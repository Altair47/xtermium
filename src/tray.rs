//! The tray icon (Settings > General > Keep running): with it on, closing the window hides it and
//! the sessions go on running. Its menu opens the window, lists the open tabs (background ones
//! marked) to bring one up, and exits, which closes every session first (main.rs `RunEvent::Exit`).

use crate::App;
use serde::Deserialize;
use tauri::image::Image;
use tauri::menu::{IconMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, WindowEvent, Wry};

const ID: &str = "main";

/// An open tab as the UI sends it: its key there, name, whether it runs in the background, and
/// its session's icon as a PNG `data:` address.
#[derive(Deserialize)]
pub struct Task {
    key: u32,
    name: String,
    background: bool,
    icon: String,
}

/// The menu's words, in the interface's language.
#[derive(Deserialize)]
pub struct Labels {
    open: String,
    tasks: String,
    none: String,
    background: String,
    exit: String,
}

impl Default for Labels {
    fn default() -> Self {
        let s = |t: &str| t.to_string();
        Self { open: s("Open"), tasks: s("Tasks"), none: s("No tasks"), background: s("background"), exit: s("Exit") }
    }
}

/// Menus take `&` as a mnemonic mark: a name's own stays a character.
fn plain(s: &str) -> String {
    s.replace('&', "&&")
}

fn png(data_url: &str) -> Option<Image<'static>> {
    use base64::Engine;
    let b64 = data_url.strip_prefix("data:image/png;base64,")?;
    let bytes = base64::engine::general_purpose::STANDARD.decode(b64).ok()?;
    let img = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png).ok()?.to_rgba8();
    let (w, h) = img.dimensions();
    Some(Image::new_owned(img.into_raw(), w, h))
}

fn menu(h: &AppHandle, tasks: &[Task], l: &Labels) -> tauri::Result<Menu<Wry>> {
    let list = Submenu::with_id(h, "tasks", plain(&l.tasks), true)?;
    if tasks.is_empty() {
        list.append(&MenuItem::with_id(h, "none", plain(&l.none), false, None::<&str>)?)?;
    }
    for t in tasks {
        let name = if t.background { format!("{} ({})", t.name, l.background) } else { t.name.clone() };
        list.append(&IconMenuItem::with_id(h, format!("task:{}", t.key), plain(&name), true, png(&t.icon), None::<&str>)?)?;
    }
    let open = MenuItem::with_id(h, "open", plain(&l.open), true, None::<&str>)?;
    let exit = MenuItem::with_id(h, "exit", plain(&l.exit), true, None::<&str>)?;
    Menu::with_items(h, &[&open, &list, &PredefinedMenuItem::separator(h)?, &exit])
}

/// The mark has no tile behind it, so each OS theme needs its own: white diamonds vanish on a
/// light taskbar and charcoal ones on a dark taskbar.
const MARK_LIGHT: &[u8] = include_bytes!("../icons/mark-light.png");
const MARK_DARK: &[u8] = include_bytes!("../icons/mark-dark.png");

/// How big a window's icon is made, in pixels; Windows scales it down for the taskbar and title bar.
const WINDOW_ICON: u32 = 64;

/// The tray's slot before display scaling, in pixels.
const TRAY_ICON: f64 = 16.0;

/// A PNG made `size` pixels square.
fn sized(png: &[u8], size: u32) -> Option<Image<'static>> {
    let img = image::load_from_memory_with_format(png, image::ImageFormat::Png).ok()?.to_rgba8();
    let img = image::imageops::resize(&img, size, size, image::imageops::FilterType::Lanczos3);
    Some(Image::new_owned(img.into_raw(), size, size))
}

/// The mark for a light or dark OS theme, `size` pixels square.
fn mark(light: bool, size: u32) -> Option<Image<'static>> {
    sized(if light { MARK_LIGHT } else { MARK_DARK }, size)
}

/// A picture chosen as the app's icon (`Settings::app_icon`), `size` pixels square; None for "".
fn picture(data_url: &str, size: u32) -> Option<Image<'static>> {
    use base64::Engine;
    let b64 = data_url.strip_prefix("data:image/png;base64,")?;
    sized(&base64::engine::general_purpose::STANDARD.decode(b64).ok()?, size)
}

/// The app's icon, `size` pixels square: the picture chosen in Settings › Appearance, else the
/// mark for the OS theme.
fn icon(h: &AppHandle, size: u32) -> Option<Image<'static>> {
    let own = h.state::<App>().settings.lock().unwrap().app_icon.clone();
    picture(&own, size).or_else(|| mark(light(h), size))
}

/// Where Windows keeps its theme, under HKEY_CURRENT_USER.
#[cfg(windows)]
const THEME_KEY: windows::core::PCWSTR = windows::core::w!(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize");

/// Whether the taskbar is light. Windows themes apps and the taskbar apart, and a window's own theme
/// follows the apps' setting, so this reads the taskbar's. Windows too old to have it are dark.
#[cfg(windows)]
fn light(_: &AppHandle) -> bool {
    use windows::Win32::System::Registry::{HKEY_CURRENT_USER, RRF_RT_REG_DWORD, RegGetValueW};
    let (mut value, mut size) = (0u32, 4u32);
    // SAFETY: the value and its size are valid for the call, which writes at most 4 bytes.
    let read = unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            THEME_KEY,
            windows::core::w!("SystemUsesLightTheme"),
            RRF_RT_REG_DWORD,
            None,
            Some((&raw mut value).cast()),
            Some(&raw mut size),
        )
    };
    read.is_ok() && value != 0
}

/// Whether the OS theme is light.
#[cfg(not(windows))]
fn light(h: &AppHandle) -> bool {
    h.get_window("main").and_then(|w| w.theme().ok()) == Some(tauri::Theme::Light)
}

/// Gives one window the app's icon. Windows has no large icon set for our windows, so its taskbar
/// shows this one too.
pub fn window_icon(w: &tauri::WebviewWindow, h: &AppHandle) {
    if let Some(icon) = icon(h, WINDOW_ICON) {
        let _ = w.set_icon(icon);
    }
}

/// Puts the app's icon on the tray and on every window: after the OS theme or the chosen icon changes.
///
/// ponytail: the tray's size is worked out here, so moving to a display with another scale keeps
/// the old size until the theme next changes. Windows scales it, so it's only a little softer.
pub fn show_theme(h: &AppHandle) {
    if let Some(t) = h.tray_by_id(ID) {
        let scale = h.get_window("main").and_then(|w| w.scale_factor().ok()).unwrap_or(1.0);
        let _ = t.set_icon(icon(h, (TRAY_ICON * scale).round() as u32));
    }
    for w in h.webview_windows().values() {
        window_icon(w, h);
    }
}

/// Follows the taskbar's theme. A window's theme event doesn't fire when only the taskbar's setting
/// changes, so this waits on the registry key itself, on a thread that lasts as long as the app.
#[cfg(windows)]
fn follow_theme(h: &AppHandle) {
    use windows::Win32::System::Registry::{
        HKEY, HKEY_CURRENT_USER, KEY_NOTIFY, REG_NOTIFY_CHANGE_LAST_SET, RegNotifyChangeKeyValue, RegOpenKeyExW,
    };
    let h = h.clone();
    std::thread::spawn(move || {
        let mut key = HKEY::default();
        // SAFETY: the handle is ours and stays open until the process ends. The wait is synchronous,
        // so it needs no event.
        unsafe {
            if RegOpenKeyExW(HKEY_CURRENT_USER, THEME_KEY, None, KEY_NOTIFY, &mut key).is_err() {
                return;
            }
            let mut was = light(&h);
            while RegNotifyChangeKeyValue(key, false, REG_NOTIFY_CHANGE_LAST_SET, None, false).is_ok() {
                let now = light(&h);
                if now != was {
                    was = now;
                    let again = h.clone();
                    let _ = h.run_on_main_thread(move || show_theme(&again));
                }
            }
        }
    });
}

/// Elsewhere the windows' theme event says when the OS theme changes (`setup`).
#[cfg(not(windows))]
fn follow_theme(_: &AppHandle) {}

/// Brings the window back and gives it the keyboard.
fn open(h: &AppHandle) {
    if let Some(w) = h.get_window("main") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
    }
}

/// Makes the tray icon, and has closing the window hide it while Keep running is on.
pub fn setup(app: &tauri::App) -> tauri::Result<()> {
    let h = app.handle();
    let mut tray = TrayIconBuilder::with_id(ID)
        .tooltip("xtermium")
        .menu(&menu(h, &[], &Labels::default())?)
        .show_menu_on_left_click(false)
        .on_menu_event(|h, e| match e.id().as_ref() {
            "open" => open(h),
            "exit" => h.exit(0),
            id => {
                if let Some(key) = id.strip_prefix("task:").and_then(|k| k.parse::<u32>().ok()) {
                    open(h);
                    let _ = h.emit_to("main", "tray-show", key);
                }
            }
        })
        .on_tray_icon_event(|t, e| {
            if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = e {
                open(t.app_handle());
            }
        });
    if let Some(icon) = icon(h, TRAY_ICON as u32) {
        tray = tray.icon(icon);
    }
    tray.build(app)?;
    show_theme(h);
    follow_theme(h);
    if let Some(w) = app.get_window("main") {
        let h = h.clone();
        w.on_window_event(move |e| {
            #[cfg(not(windows))]
            if let WindowEvent::ThemeChanged(_) = e {
                show_theme(&h);
            }
            if let WindowEvent::CloseRequested { api, .. } = e
                && h.state::<App>().settings.lock().unwrap().keep_running
            {
                api.prevent_close();
                if let Some(w) = h.get_window("main") {
                    let _ = w.hide();
                }
            }
            // Closed for good: detached tabs' windows don't keep xtermium running.
            if let WindowEvent::Destroyed = e {
                h.exit(0);
            }
        });
    }
    apply(h, h.state::<App>().settings.lock().unwrap().keep_running);
    Ok(())
}

/// The icon shows while Keep running is on.
pub fn apply(h: &AppHandle, keep_running: bool) {
    if let Some(t) = h.tray_by_id(ID) {
        let _ = t.set_visible(keep_running);
    }
}

/// The open tabs changed (or the language): the menu follows.
#[tauri::command]
pub fn tray_tasks(h: AppHandle, tasks: Vec<Task>, labels: Labels) -> Result<(), String> {
    let Some(t) = h.tray_by_id(ID) else { return Ok(()) };
    t.set_menu(Some(menu(&h, &tasks, &labels).map_err(|e| e.to_string())?)).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Swapped files would put white diamonds on a light taskbar, and a background would bring back
    /// the tile that made the icon too small to see.
    #[test]
    fn each_theme_gets_its_own_mark_with_nothing_behind_it() {
        for light in [true, false] {
            let icon = mark(light, 64).unwrap();
            assert_eq!((icon.width(), icon.height()), (64, 64));
            let pixels: Vec<&[u8; 4]> = icon.rgba().as_chunks::<4>().0.iter().collect();
            for corner in [0, 63, 64 * 63, 64 * 64 - 1] {
                assert_eq!(pixels[corner][3], 0, "corner {corner} isn't see-through");
            }
            let solid = || pixels.iter().filter(|p| p[3] == 255);
            // Amber is the only colour with far more red than blue; the rest is the diamonds.
            assert!(solid().any(|p| p[0] > 200 && p[2] < 80), "no amber diamond");
            let grey = solid().find(|p| p[0].abs_diff(p[2]) < 8).unwrap();
            assert_eq!(grey[0] < 128, light, "the diamonds should contrast with the taskbar");
        }
    }

    /// A chosen icon comes out at the size asked for; no choice, or a broken one, gives none.
    #[test]
    fn a_chosen_icon_is_resized_and_a_bad_one_ignored() {
        use base64::Engine;
        let png = base64::engine::general_purpose::STANDARD.encode(MARK_DARK);
        let icon = picture(&format!("data:image/png;base64,{png}"), 32).unwrap();
        assert_eq!((icon.width(), icon.height()), (32, 32));
        assert!(picture("", 32).is_none());
        assert!(picture("data:image/png;base64,bm90IGEgcGljdHVyZQ==", 32).is_none());
    }
}

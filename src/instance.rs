//! One xtermium per data folder (config::dir): two would overwrite each other's sessions and
//! settings. A second start asks the running one to show its window, then quits. Copies with a
//! data folder of their own (`XTERMIUM_HOME`: portable installs, tests) run side by side.
//!
//! The running one holds `instance.lock` and listens on a local port for "show", which may carry
//! a folder to open a local shell in ("Open in xtermium", explorer.rs). `instance.port` holds the
//! port and, on the next line, a random secret made at each start: a message is the secret, a
//! line break, "show", then a line break and the folder. One without the secret is dropped
//! unanswered, so another user of this computer, who can reach 127.0.0.1 but not this user's
//! data folder (config::private), can't open shells here.

use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

/// This run's claim on its data folder, held until it exits.
pub struct Instance {
    _lock: Option<File>,
}

/// The claim, or None when another xtermium already runs on this data folder (it has been asked
/// to show itself, and to open a shell in `here`). A lock that can't be made at all (a read-only
/// folder) doesn't stop xtermium.
pub fn claim(here: Option<&str>) -> Option<Instance> {
    claim_in(&crate::config::dir(), here)
}

fn claim_in(dir: &Path, here: Option<&str>) -> Option<Instance> {
    let _ = crate::config::private(dir);
    // Not truncated: whoever holds it may be writing it.
    let Ok(lock) = File::options().create(true).truncate(false).write(true).open(dir.join("instance.lock")) else {
        return Some(Instance { _lock: None });
    };
    if lock.try_lock().is_ok() {
        return Some(Instance { _lock: Some(lock) });
    }
    let _ = wake(&port_file(dir), here);
    None
}

fn port_file(dir: &Path) -> PathBuf {
    dir.join("instance.port")
}

/// Asks the running one to show its window (and open a shell in `here`).
fn wake(port_file: &Path, here: Option<&str>) -> std::io::Result<()> {
    let text = std::fs::read_to_string(port_file)?;
    let (port, secret) = text.split_once('\n').unwrap_or((&text, ""));
    let port: u16 = port.trim().parse().map_err(std::io::Error::other)?;
    let mut s = TcpStream::connect_timeout(&([127, 0, 0, 1], port).into(), Duration::from_secs(2))?;
    let secret = secret.trim();
    match here {
        Some(dir) => s.write_all(format!("{secret}\nshow\n{dir}").as_bytes()),
        None => s.write_all(format!("{secret}\nshow").as_bytes()),
    }
}

/// A local shell starting in `dir`, named after the folder.
pub fn shell_in(dir: &str) -> crate::Session {
    let path = Path::new(dir);
    let name = path.file_name().map_or_else(|| dir.to_string(), |n| n.to_string_lossy().to_string());
    crate::Session::new(&name, "", crate::Kind::Shell { program: String::new(), dir: dir.to_string() })
}

/// Shows the window when another start asks (on its own thread, for the life of the app), and
/// opens the shell it asks for.
pub fn listen(handle: AppHandle) {
    serve(&crate::config::dir(), move |here| {
        if let Some(w) = handle.get_window("main") {
            let _ = w.unminimize();
            let _ = w.show();
            let _ = w.set_focus();
        }
        if let Some(dir) = here.filter(|d| Path::new(d).is_dir()) {
            let _ = handle.emit_to("main", "open-here", shell_in(&dir));
        }
    });
}

fn serve(dir: &Path, show: impl Fn(Option<String>) + Send + 'static) {
    let Ok(listener) = TcpListener::bind("127.0.0.1:0") else { return };
    let Ok(addr) = listener.local_addr() else { return };
    let secret = crate::vault::hex(&crate::vault::random::<32>());
    if std::fs::write(port_file(dir), format!("{}\n{secret}", addr.port())).is_err() {
        return;
    }
    let want = format!("{secret}\nshow");
    std::thread::spawn(move || {
        for mut s in listener.incoming().flatten() {
            let _ = s.set_read_timeout(Some(Duration::from_secs(2)));
            let mut buf = Vec::new();
            let _ = (&mut s).take(64 * 1024).read_to_end(&mut buf);
            // Compared in constant time, so the answer's timing doesn't give the secret away.
            if let Some((head, rest)) = buf.split_at_checked(want.len())
                && crate::automation::same(head, want.as_bytes())
            {
                let here = rest.strip_prefix(b"\n").map(|d| String::from_utf8_lossy(d).to_string());
                show(here);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

    #[test]
    fn second_start_wakes_the_first() {
        let dir = std::env::temp_dir().join(format!("xtermium-instance-test-{}", std::process::id()));
        let first = claim_in(&dir, None).expect("the first start runs");
        let shown = Arc::new(AtomicBool::new(false));
        let asked = Arc::new(std::sync::Mutex::new(None));
        let (s2, a2) = (shown.clone(), asked.clone());
        serve(&dir, move |here| {
            *a2.lock().unwrap() = here;
            s2.store(true, Relaxed);
        });
        let wait = |shown: &AtomicBool| {
            for _ in 0..50 {
                if shown.load(Relaxed) {
                    break;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            shown.swap(false, Relaxed)
        };
        // Someone else on this computer finds the port but not the secret: nothing happens.
        let port = std::fs::read_to_string(port_file(&dir)).unwrap().lines().next().unwrap().to_string();
        let forged = dir.join("forged.port");
        for file in [format!("{port}\nnot-the-secret"), port.clone()] {
            std::fs::write(&forged, file).unwrap();
            wake(&forged, Some("/tmp")).unwrap();
            assert!(!wait(&shown), "a message without the secret is dropped");
        }
        assert!(claim_in(&dir, None).is_none(), "a second start on the same folder doesn't run");
        assert!(wait(&shown), "the first one was asked to show itself");
        assert_eq!(*asked.lock().unwrap(), None);
        assert!(claim_in(&dir, Some(r"C:\Users\me\proj")).is_none());
        assert!(wait(&shown), "and again, with a folder");
        assert_eq!(asked.lock().unwrap().as_deref(), Some(r"C:\Users\me\proj"), "the folder came along");
        drop(first);
        assert!(claim_in(&dir, None).is_some(), "once it's gone, the next start runs");
        let _ = std::fs::remove_dir_all(dir);
    }
}

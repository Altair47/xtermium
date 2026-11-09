//! AMI MegaRAC SP-X BMCs (ASUS ASMB10 and ASMB11 iKVM, Gigabyte, ASRock Rack and other boards
//! with the MegaRAC web interface): the H5Viewer console.

/// The web interface, a single-page app that starts at its login page.
pub const HOME: &str = "/";

/// What to do when it hands over the Java viewer (JViewer, a .jnlp file) instead.
pub const JAVA: &str = "This BMC handed over its Java viewer (JViewer), which xtermium can't run. \
     xtermium opens the HTML5 one (H5Viewer); on the BMC, Remote Control must allow it.";

/// After the shared helpers (mod.rs `COMMON`); runs in the page (and does nothing in frames).
/// Fills the login form; once signed in, goes to Remote Control and presses Launch H5Viewer, whose
/// pop-up (`viewer.html`) opens as a frame over the page instead. The viewer reads its settings
/// from the page that opened it (`opener`) and its access flags from its own window: they're set
/// on the frame's window as soon as the viewer's page starts, before its scripts run (a frame made
/// by script gets no copy of this script, and its first, blank page lends its window to the next
/// same-site page). The viewer's picture is sized to fill the tab, in the middle (`fit`).
pub const SCRIPT: &str = r##"
  if (window.top !== window) return;
  const arm = (w, t0 = Date.now()) => {
    try {
      if (/viewer\.html/.test(w.location.pathname) && w.document.readyState === "loading") {
        Object.defineProperty(w, "opener", { value: window, configurable: true });
        for (const k of ["privilege_id", "kvm_access", "vmedia_access"]) w[k] = window[k];
        return;
      }
    } catch {}
    if (Date.now() - t0 < 5000) setTimeout(() => arm(w, t0), 2);
  };
  // The viewer draws the screen on a canvas sized by CSS (its mouse maps through that size, its
  // zoom buttons only change it in steps of 10%): the canvas and its cursor layer take the size
  // that fits the room under the toolbar, in the middle of it, again whenever the tab or the
  // screen's resolution changes. The viewer's own zoom state follows, so its buttons still work.
  const fit = (w) => {
    const d = w.document;
    const css = d.createElement("style");
    css.textContent = ".remote-view { display: flex; align-items: center; justify-content: center; } #scroll_div { overflow: hidden !important; }";
    d.head.append(css);
    setInterval(() => {
      const v = w._video, box = d.querySelector(".remote-view"), sd = d.getElementById("scroll_div");
      if (!v?.running || !box || !sd || !v.resolution_x || !v.resolution_y) return;
      const bar = (sel) => d.querySelector(sel)?.getBoundingClientRect().height ?? 0;
      const [aw, ah] = [box.clientWidth, w.innerHeight - bar(".h5-toolbar") - bar(".statusbar") - 20];
      const s = Math.min(aw / v.resolution_x, ah / v.resolution_y);
      const [cw, ch] = [Math.floor(v.resolution_x * s), Math.floor(v.resolution_y * s)];
      if (!(s > 0) || (sd.style.width === cw + "px" && sd.style.height === ch + "px")) return;
      v.videoScale = s;
      v.devicePixelRatio = 2 - s;
      for (const c of [v.canvas, v.cursor_canvas]) Object.assign(c.style, { width: cw + "px", height: ch + "px" });
      box.style.height = ah + "px";
      Object.assign(sd.style, { width: cw + "px", height: ch + "px" });
      const z = d.getElementById("zoom_value");
      if (z) z.textContent = " Zoom " + Math.round(s * 100) + "%";
    }, 300);
  };
  const open = window.open;
  window.open = function (u, ...rest) {
    if (!/viewer\.html/.test(String(u))) return open.call(this, u, ...rest);
    const w = overlay(String(u), fit).contentWindow;
    arm(w);
    return w;
  };
  let tries = 0;
  const look = () => {
    const pw = passwordField();
    if (pw && pass && !seen("tried")) {
      mark("tried");
      fillForm(pw);
    }
    if (!pw && !/login/.test(location.hash) && document.cookie.includes("refresh_disable=")) {
      if (!/remote_control/.test(location.hash)) location.hash = "remote_control";
      const launch = document.getElementById("download");
      if (launch && shown(launch) && !launch.disabled && !document.getElementById("xtm-console")) return void launch.click();
    }
    if (++tries < 600) setTimeout(look, 500);
  };
  addEventListener("DOMContentLoaded", look);
"##;

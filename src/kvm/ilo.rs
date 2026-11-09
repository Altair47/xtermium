//! HPE iLO 4 (firmware 2.70 and newer) and iLO 5: the HTML5 Integrated Remote Console.

/// The web interface: the index page keeps the login page, then the application, in a frame.
pub const HOME: &str = "/";

/// What to do when it hands over a Java console (.jnlp) instead.
pub const JAVA: &str = "This iLO handed over its Java console, which xtermium can't run. \
     xtermium shows the HTML5 console of iLO 4 (firmware 2.70 or newer) and iLO 5.";

/// After the shared helpers (mod.rs `COMMON`); runs in every frame. The login form is drawn by
/// script in a frame of the index page, so that frame's copy of the script fills it in. Once the
/// index shows the application (the session is good), the standalone console page (`/irc.html`,
/// which takes the session from its cookie and connects over the web port) goes over it, in a frame.
pub const SCRIPT: &str = r#"
  if (window.top !== window) {
    let tries = 0;
    const look = () => {
      const pw = passwordField();
      if (pw && pass && !seen("tried")) {
        mark("tried");
        return fillForm(pw);
      }
      if (!pw && ++tries < 40) setTimeout(look, 300);
    };
    addEventListener("DOMContentLoaded", look);
    return;
  }
  const look = () => {
    if (document.getElementById("xtm-console")) return;
    if ([...document.querySelectorAll("iframe")].some((f) => /application\.html/.test(f.src))) return void overlay("/irc.html", () => {});
    setTimeout(look, 500);
  };
  addEventListener("DOMContentLoaded", look);
"#;

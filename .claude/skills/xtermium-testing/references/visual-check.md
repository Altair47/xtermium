# Visual check over CDP

How to see a change in the real app without touching anyone's own xtermium, desktop or browser. Source: `CLAUDE.md` "Build, test, run".

## Ground rules

- Build and run from `target/work`, not `target/debug` (`xtermium-developing`, "Building").
- Give the test exe its own scratch `XTERMIUM_HOME`: one xtermium runs per data folder, so without it the exe opens your real data, or just asks an xtermium already running on that folder to show its window and quits.
- Never capture the desktop and never send OS clicks or keys. Everything goes through the webview's DevTools protocol (CDP).
- Don't click links that open a URL: they open the system browser.
- Close test prompts by their own button. A stray `.backdrop button.primary` click can be Trust on a host-key prompt.
- Stop only the process you started.

## 1. Build

Git Bash or any POSIX shell:

```sh
CARGO_TARGET_DIR=target/work npm run tauri build -- --debug --no-bundle
```

PowerShell:

```powershell
$env:CARGO_TARGET_DIR = 'target/work'; npm run tauri build -- --debug --no-bundle
```

Result: `target/work/debug/xtermium.exe` (and `xtermium-rdp.exe` beside it). The exe is locked while it runs: stop the test process before rebuilding.

## 2. Launch with scratch folders and CDP on 9333

PowerShell, in one command (environment variables set in one tool call don't reach the next):

```powershell
$s = "$env:TEMP\xtm-visual"   # any scratch folder
New-Item -ItemType Directory -Force "$s\home", "$s\wv2" | Out-Null
$env:XTERMIUM_HOME = "$s\home"
$env:WEBVIEW2_USER_DATA_FOLDER = "$s\wv2"
$env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = '--remote-debugging-port=9333'
Start-Process .\target\work\debug\xtermium.exe
```

Then list the debuggable pages:

```sh
curl -s http://127.0.0.1:9333/json/list
```

The interface is the page at `http://tauri.localhost/`. Connect a WebSocket to its `webSocketDebuggerUrl` (Node 22 and newer have a global `WebSocket`).

## 3. Drive it

| Goal | CDP call |
|---|---|
| Read or change the page, call commands | `Runtime.evaluate` with `awaitPromise: true` |
| Screenshot a region | `Page.captureScreenshot` with a `clip` (x, y, width, height, scale) |
| Type into a terminal or field | `Input.insertText` (synthetic `keyDown` text never reaches the terminal) |
| Press a key for a field or button | `Input.dispatchKeyEvent` `keyDown` (with `text: "\r"` for Enter), then `keyUp` |
| Passkeys | `WebAuthn.enable`, then `WebAuthn.addVirtualAuthenticator`, in the same CDP session as the page actions |

Seed data from `Runtime.evaluate`:

```js
const b = await window.__TAURI_INTERNALS__.invoke("boot");          // b.store, b.workspace ("" = the default)
b.store.session.push(/* a session object shaped like the existing ones */);
await window.__TAURI_INTERNALS__.invoke("save_store", { store: b.store, workspace: b.workspace });
location.reload();
```

`save_store` takes `{ store, workspace }` as `ui/lib/state.svelte.ts` sends it. Session shapes: `ui/lib/api.ts`. Demo data uses the user `sotiris` and reserved addresses (`xtermium-designing`, "Screenshots").

## 4. Traps

- A hidden or minimised test window doesn't run `requestAnimationFrame`: terminals and animations stop painting.
- No local SSH server: `github.com:22` gives a real unknown-host-key prompt (`known_keys` / `forget_key` reset it); for a working session use the throwaway `asyncssh` server in the SKILL.md gotchas.
- `WEBVIEW2_USER_DATA_FOLDER` overrides every view's data folder, website session profiles included. To check website profiles, launch without the two WebView2 variables (keep the scratch `XTERMIUM_HOME`) and drive the app with the CLI (Settings › MCP on; `xtermium-mcp`).
- With automation on, the test exe listens on the port in its own scratch settings (default 8721). An assistant already set up for `http://127.0.0.1:8721/mcp` talks to whichever xtermium holds that port, which may be the test one.

## 5. Stop

Stop only the test process:

```powershell
Get-Process xtermium | Where-Object Path -like '*\target\work\*' | Stop-Process
```

Then delete the scratch folder if nothing in it is needed.

## 6. Report

Say what was checked in the running app and what could not be (for example Windows Hello, a real RDP host, a real BMC).

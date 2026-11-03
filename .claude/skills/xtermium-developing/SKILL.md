---
name: xtermium-developing
description: Use when changing code in the xtermium app repository (the Rust/Tauri backend in src/, the Svelte 5 UI in ui/, the RDP helper in rdp/), adding a Settings or Store field, an MCP tool or CLI word, UI text, a menu or a dialog, building the app, or touching storage, keys, the lock, the automation server, windows and webviews, SSH trust or tunnels.
---

# Developing xtermium

## Overview

xtermium is a Rust backend on tokio inside Tauri 2, with a Svelte 5 + TypeScript UI drawn in the system webview, and an RDP helper process in `rdp/` (IronRDP, its own crate and `Cargo.lock`). The backend owns all state and connections; the UI calls Tauri commands and listens to events. `CLAUDE.md` is the authoritative description of how it fits together; this skill points into it, lists the conventions the tests enforce, the recipes for common additions, the build commands, and the security model every change must keep.

## When to use

- Before editing anything under `src/`, `ui/`, `rdp/`, `capabilities/` or `tauri.conf.json`.
- Adding a Settings or Store field, an automation tool and its CLI word, UI text, a menu or dialog.
- Building the app or the standalone debug exe.
- Changing storage, the data key, the lock, the automation server, the single-instance port, website tabs, SSH host-key trust or tunnels.
- Reviewing a pull request for convention or security problems.

## Quick reference

| Task | Command (from the repo root) |
|---|---|
| RDP helper, once before any plain `cargo` command | `npm run rdp` |
| UI type check (must print 0 errors, 0 warnings) | `npm run check` |
| UI tests (locale keys, spacing and type scale, …) | `npm test` |
| Rust lint and tests | `cargo clippy --all-targets && cargo test` |
| RDP helper lint, when `rdp/` changed | `(cd rdp && cargo clippy --all-targets)` |
| Dev mode with UI reload | `npm run tauri dev` |
| Standalone debug exe | `CARGO_TARGET_DIR=target/work npm run tauri build -- --debug --no-bundle` |
| Release build and installers | `npm run tauri build` |
| Audits | `cargo audit && (cd rdp && cargo audit) && npm audit` |
| Third-party notices after dependency changes | `npm run notices` (needs `cargo install --locked cargo-about --features cli`) |

| File | What it holds |
|---|---|
| `CLAUDE.md` | how it fits together, data and storage, UI conventions, automation, build/test/run |
| `README.md` "Architecture" | the path table (`src/main.rs`, `src/term.rs`, `src/ssh.rs`, …) and the frame model |
| `README.md` "Configuration", "Security notes" | data folder, file list, encryption, accepted advisories |
| `DESIGN.md` | the UI's tokens, components and named rules (`xtermium-designing`) |
| `docs/sync.md` | sealed-file formats and the sync client's API contract |
| `CONTRIBUTING.md` | branches, pull requests, commits, versions (`xtermium-contributing`) |

## Architecture: where to read

Read the `CLAUDE.md` section instead of re-deriving it:

| Need | `CLAUDE.md` section |
|---|---|
| Backend owns state; commands (`#[tauri::command]` in `src/main.rs`, `generate_handler!`), events, which commands must be `async` | "How it fits together", first bullets |
| Terminal hot path: `Inner::frame()`, `advance_bytes`, `draw()` in `ui/lib/Terminal.svelte` (WebGL glyph cache in `glyphs.ts`); measure before and after | "How it fits together", Terminal tabs |
| RDP helper over stdin/stdout (`src/rdp_proto.rs`, `src/rdp.rs`); VNC in-process (`src/vnc.rs`); websites and BMC consoles as child webviews (`src/web.rs`, `src/kvm/`) | "How it fits together", RDP |
| Detached tabs: `tab-<id>` windows, `Tab::attach`, `ui/lib/Detached.svelte` | "How it fits together", Tabs moving between windows |
| Broadcast, file panel, tunnels, automation, sync, tray, single instance, importers, support | "How it fits together", the long bullet |
| Data folder, encrypted files, workspaces in the Store, logins, settings, restore points | "Data and storage" |
| Runes, state, components, look, menus, dialogs, Enter behaviour, i18n, copy | "UI conventions" |
| New MCP tool or CLI word | "Automation (MCP + CLI)" |

Rules from those sections that bite most often:

- Commands that create windows must be `async` (a sync one deadlocks on Windows). Sync commands run on the main thread; one that may read the clipboard (`term_key`, `term_mouse`) is `async`.
- One data folder holds everything (`config::dir`: `~/.xtermium`, or `XTERMIUM_HOME`). Never write to the OS temp or app-data folders: use `config::temp` (`temp/<pid>/`) and `config::webview_dir()`. Tests get a scratch folder from `dir()`, never the real one.
- `App.store` holds only the workspace in use and `App.rest` the others; `save_store` carries the workspace a save belongs to, so a save racing a switch lands in the right one.
- Every UI field passes `cleanInput` / `clampNumber` (`ui/App.svelte`); the backend cleans again (`src/clean.rs`).

## Conventions

### UI text and i18n

- All UI text goes through `_("…")` (or `N_("…")` in tables), keyed by the English.
- Locales: `ui/locales/<code>.json` for ar (right to left), de, el, es, fi, fr, hi, ja, nb, pl, sv, zh-CN; English is built in and the default. New text may stay untranslated (it shows in English).
- `npm test` fails on locale keys whose English no longer exists: when changing or removing text, delete those keys from every locale file, keeping the files' 2-space JSON format.
- Copy style: short plain sentences, second person, "Settings › Page" for places, no jargon in labels (details in `xtermium-designing`).

### Spacing and type

- `ui/lib/scale.test.ts` (in `npm test`) reads `ui/app.css`, `ui/App.svelte` and every `ui/lib/*.svelte` style block. It fails when padding, margin, gap, inset, `top`/`left`/`right`/`bottom` or border-radius use a px value off the Fibonacci scale (0 1 2 3 5 8 13 21 34, plus 55 and 89), and on a raw px `font-size`.
- Use the tokens (`--t-s` … `--t-xxl`, `--row`, `--control`, `--bar`, `--r-*`, `--gap`) and write derived values as `calc(…)`. Values and rules: `xtermium-designing`.

### Svelte 5, menus and dialogs

- Svelte 5 runes. App state lives in `ui/lib/state.svelte.ts` (`app`), the window in `ui/App.svelte` (tabs, panes, layouts, detach, dialogs), components in `ui/lib/`.
- Svelte transitions don't obey `app.css`'s reduced-motion rule: give them `duration: 0` when it is on.
- Menus: `.menu` + `{@attach menu(close, x, y)}` (keyboard-driven, kept on screen); items without an icon indent to line up; `.sep` starts a group; the destructive item (`.danger`) comes last.
- Dialogs: `Modal.svelte` (takes the keyboard, Tab stays inside, Escape closes the top one). Menus and dialogs move to `<body>` while open (`portal` in `ui/lib/menu.ts`), so a stacking context can't hide them under the panes.
- Terminals and RDP refuse keys while any dialog or the palette is open (`dialogTakesKeys`) and get the keyboard back only after it is gone (`refocus` waits a tick).
- Enter does the obvious thing: a filter field acts on its first hit; a yes/no question focuses yes (`{@attach (el) => el.focus()}`), except when yes can't be undone or is a security call (trust a new host key, delete folders or sessions, restore, a new automation token, discard an edit): those focus Cancel or Reject. A handler that moves the keyboard to a terminal calls `e.preventDefault()`.
- Text that names another place links to it: `openSettings(page)`, `button.link` or `button.link.quiet`; status messages naming a page use `tell(text, page)`; a moment's confirmation is `toast(text)` (`copyText` copies and says so).
- Icons: Boxicons SVGs in `assets/icons` through `Icon.svelte`; only session and folder icons (`SessionIcon.svelte`) animate, on hover.

### Adding a Store or Settings field

1. The Rust field and its `Default`: `Settings` in `src/settings.rs` (`#[serde(default)]`), `Store` in `src/config.rs` (each field with its own `#[serde(default…)]`).
2. Cleaning in `src/clean.rs` (`sanitize`); for Store fields also `Store::merge`.
3. The TypeScript type in `ui/lib/api.ts`.
4. For Store arrays, the `filled()` default in `ui/lib/state.svelte.ts`.
5. If terminals must restyle on the change: `Settings::restyles`.
6. The control on its page in `ui/lib/Settings.svelte` (one component holds every Settings page): text through `_()`, values through `cleanInput` / `clampNumber`.
7. Backgrounds have one writer (`background_set`); `save_settings` ignores the UI's `settings.workspace` (only the `workspace` command changes it).

### Adding an automation tool and CLI word

1. Schema and description in `automation::tools()` and the handler in `call()` (`src/automation.rs`). `call()` starts with `unlocked()?`, so a new tool respects the lock without extra code; never route around it.
2. A CLI word in `src/cli.rs`: `USAGE`, `COMMANDS`, `parse` (and `VALUED` for a flag that takes a value), and a case in the `commands` test.
3. The list sentence on the Settings › MCP page (`ui/lib/Settings.svelte`).
4. A README line ("Agents and scripts").
5. Never return a password; AI-agent sessions and tabs stay invisible to automation. UI-side actions go as events to the main window (`automation-open`, `automation-detach`, `automation-layout`).

How the tools look from the outside: `xtermium-mcp`.

### Code style

- Comments are full sentences saying why; `///` / `/** */` on most items; `ponytail:` notes on deliberate shortcuts, naming their ceiling.
- `cargo fmt --check` already reports about 100 diffs on main: don't reformat whole files; keep the diff to the change (`rustfmt.toml` is the config).
- Measure before and after changing the terminal hot path (a throwaway `#[ignore]` timing test works; `term::tests::bench_flood` exists for this).

## Building

- `npm run tauri build` builds `xtermium-rdp` (`rdp/`), the UI into `dist/`, then the app with the UI embedded, and the installers (`target/release/bundle/msi` and `nsis` on Windows). `-- --no-bundle` skips the installers; `-- --debug --no-bundle` gives a standalone debug exe.
- Build and test with `CARGO_TARGET_DIR=target/work` (`CLAUDE.md` "Build, test, run"): a running `target/debug/xtermium.exe` locks `target/debug`, so builds there fail while it runs. The test exe is then `target/work/debug/xtermium.exe`, locked in turn while it runs: stop it before rebuilding.
- A plain `cargo build`, `cargo test` or `cargo clippy` needs `npm run rdp` once first (Tauri checks the files it bundles). A plain `cargo build` gives the dev-mode binary, which expects the Vite server on port 1420.
- Requirements: Rust 1.89 or newer, Node.js 20 or newer; on Linux the WebKitGTK packages Tauri needs. On Windows the WezTerm git dependency needs `git config --global core.longpaths true`.
- Dependencies compile with optimizations even in dev builds, so the first build takes a few minutes.

## The security model

| Area | Control | Where |
|---|---|---|
| Storage | every `.xtm` file sealed with XChaCha20-Poly1305 under a random 256-bit data key in the OS keychain; Argon2id for passphrases; `recovery.key` restores the key | `src/vault.rs`, `src/config.rs` |
| Data folder | one folder (`~/.xtermium` or `XTERMIUM_HOME`), created 0700 on Linux and macOS | `src/config.rs` |
| Lock | password, PIN (Argon2id hashes), passkey, Windows Hello; `lock.xtm` never sent to the UI, changed only after an unlock; refuses every MCP tool and CLI command while locked | `src/lock.rs`, `src/automation.rs` `unlocked()` |
| Automation | off by default; bearer token compared in constant time (`same()`); loopback: `Host` must be `127.0.0.1`/`localhost` with the port (DNS rebinding), a non-local `Origin` is refused (web pages); only `POST /mcp`; 16 KiB head and 1 MiB body caps; network mode is HTTPS with a self-signed certificate | `src/automation.rs` |
| Single instance | loopback port plus a per-run secret on the next line of `instance.port`; a message without it is dropped | `src/instance.rs` |
| Windows | CSP with `frame-ancestors 'none'`; the interface mounts only in `main` and `tab-*` windows, unframed (`ownWindow()`); capabilities limited to those windows | `tauri.conf.json`, `ui/main.ts`, `capabilities/default.json` |
| SSH and RDP | host and server keys trusted on first use; a changed key refuses; trusting a new key focuses Reject | `src/ssh.rs`, `known.xtm` |
| Tunnels | new tunnels bind `127.0.0.1` | `src/ssh.rs` |
| BMC consoles | local relay on `127.0.0.1` with a token in the path; TLS to the BMC trusting only its pinned certificate | `src/kvm/relay.rs` |

Deliberately open (don't "fix" these without a maintainer):

| What | Why it stays |
|---|---|
| Network-mode assistant commands start Node-based assistants with `NODE_TLS_REJECT_UNAUTHORIZED=0` | the certificate is self-signed; its SHA-256 fingerprint is shown instead |
| `initialize`, `ping` and `tools/list` answer while locked | assistants keep the server and carry on after an unlock |
| `fetch_favicon` skips certificate checks | admin pages are mostly self-signed; only a picture comes back, shown in `<img>` |
| Imported tunnels keep the imported bind address (often `0.0.0.0`) | the README warns; new tunnels default to `127.0.0.1` |
| VNC is unencrypted | the app says so over the desktop unless an SSH gateway or tunnel carries it |
| `cargo audit` accepts RUSTSEC-2023-0071 (rsa "Marvin") and RUSTSEC-2026-0253 (lru) | reasons in `.cargo/audit.toml` and `rdp/.cargo/audit.toml` |
| `patches/glib` (glib 0.18 with the fix for GHSA-wrw7-89jp-8q8g) | until Tauri moves to glib 0.20 or newer |

Files, formats, lock, automation checks, network pieces and advisories in detail: `references/security-model.md`.

## Rules for touching security

- Fix at the chokepoint: one guard in the shared function every caller passes through (`unlocked()` at the top of `call()`, `ownWindow()` in `ui/main.ts`, `same()` for every secret compare), never per caller.
- Leave one runnable test per fix that fails if the guard goes (models: `automation::tests::locked_xtermium_refuses_tools`, `automation::tests::only_local_callers`, the 0700 check in `src/config.rs`).
- If a fix has to change core components, functionality or usability, ask a maintainer first (an issue or the pull request).
- Never log, print, export or return secrets: passwords, tokens, key passphrases, the recovery key. A tool never returns a password; agent sessions and tabs stay invisible.
- Keep data inside the data folder; new windows are `main` or `tab-*`, or `ownWindow()` and `capabilities/default.json` change with them.
- Dependency alerts and vulnerability reports: `xtermium-contributing`.

## Common mistakes

- Building into `target/debug`, or running a plain `cargo build` before `npm run rdp`.
- Expecting a standalone exe from a plain `cargo build` (it wants Vite on port 1420).
- Changing UI text without removing the stale keys from all 12 locale files.
- `padding: 4px 10px` or `font-size: 12px` (the scale test fails).
- A Store field added to the Rust struct only: dropped by `clean.rs`, unknown to `api.ts`, or empty in the UI.
- A new tool without its CLI word and `commands` test case, or a tool path that skips `unlocked()`.
- Returning a session object with its password or key passphrase in it.
- Comparing secrets with `==` instead of `same()`.
- Writing logs or temp files to the OS temp or app-data folders.
- Running `cargo fmt` over whole files (the README's Development block starts with it; on this codebase it rewrites about 100 places).
- A Svelte transition without the reduced-motion check.

## Read next

- `CLAUDE.md`: the full architecture and conventions.
- `CONTRIBUTING.md` and `xtermium-contributing`: branches, commits, versions, alerts.
- `DESIGN.md` and `xtermium-designing`: tokens, components, copy, screenshots.
- `xtermium-testing`: what the tests cover, the visual check over CDP, test gotchas.
- `xtermium-mcp`: the automation server and CLI as users and agents see them.
- `references/security-model.md`: the security controls in detail.

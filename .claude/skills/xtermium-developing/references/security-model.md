# The app's security model in detail

Sources: `README.md` ("Keeping it safe", "Configuration", "Security notes"), `CLAUDE.md`, `docs/sync.md`, `tauri.conf.json`, `capabilities/default.json`, `.cargo/audit.toml`, and the source files named below.

## Files in the data folder

The data folder is `~/.xtermium` (`%USERPROFILE%\.xtermium` on Windows) or `XTERMIUM_HOME` (portable mode; the Scoop manifest points it at Scoop's persist folder). It is created 0700 on Linux and macOS (`src/config.rs`, tested), because website profiles, logs and bundles had been world-readable.

| File | Contents | Synced |
|---|---|---|
| `sessions.xtm` | sessions, folders and tags, shared logins, tunnels, snippets, saved layouts, workspaces | yes |
| `known.xtm` | trusted SSH host keys and RDP server keys | yes |
| `recovery.key` | the data key sealed under the recovery passphrase | yes |
| `settings.xtm` | Settings, including the default SSH password and the sync token | no |
| `logins.xtm` | passwords remembered at a prompt, every workspace, this device only | no |
| `lock.xtm` | the lock: Argon2id hashes of password and PIN, the passkey's public key, when it locks | no |
| `sync.xtm` | the revision this device last matched and a digest | no |
| `tabs.xtm` | tabs to reopen | no |
| `background.xtm`, `background-sidebar.xtm`, `background-bar.xtm` | backgrounds (a `data:` picture or shader source) | no |
| `automation-cert.xtm` | the self-signed certificate for network-mode automation | no |
| `restore-points/<ms>-<reason>.xtm` | a full backup's text plus saved logins, sealed with the data key (re-sealed by `rekey`), newest 10 | no |
| `websites/<profile>` | each saved website session's WebView2 data folder; unused ones swept at load | no |
| `webview/`, `logs/`, `temp/<pid>/`, `custom.css`, `instance.lock`, `instance.port` | the windows' browser data, logs, this run's passing data (swept at the next start), the user stylesheet, single-instance state | no |

- Remembered passwords stay on the device; shared logins live in `sessions.xtm` and sync; exports carry them only inside the passphrase-protected `.xtermium` file.
- The sync token is kept in `settings.xtm` (sealed) and left out of backups, exports and troubleshooting bundles.
- Settings › Troubleshoot writes plain-text files about the installation (version, system, settings without secrets, counts of what is stored and open).

## Formats (`docs/sync.md`)

- Sealed file: `XTM1` | 24-byte nonce | XChaCha20-Poly1305 ciphertext and tag. Associated data is the file's name without extension, so files can't be swapped. Plaintext is UTF-8 TOML. A fresh random nonce per write.
- Data key: 32 random bytes in the OS keychain (Windows Credential Manager, macOS Keychain, Secret Service), service `xtermium`, account `data-key-<first 8 bytes of SHA-256 of the data folder path, hex>`, so portable installs keep their own key.
- `recovery.key`: TOML with `salt` (16 bytes), Argon2id `m_cost = 65536` KiB, `t_cost = 3`, parallelism 1, 32-byte output, `nonce`, `sealed_key` (associated data `recovery`). Without a passphrase, a lost keychain entry means the files can't be read.
- Folder export `*.xtermium`: `XTMX` | salt | m_cost | t_cost | nonce | ciphertext; key from Argon2id over the export passphrase.
- Readers refuse Argon2 parameters outside m_cost 8 MiB–1 GiB and t_cost 1–10, so a hostile file can't exhaust memory.
- Sync requires a recovery passphrase of at least 12 characters (`recovery.key` rides along with the vault).
- Only symmetric cryptography is used, so the README calls the storage post-quantum safe.
- Upgrading from plain-text versions: the first start encrypts `sessions.toml`, `settings.toml`, `known_hosts`, `known_rdp`, checks the result and deletes the plain files.

## The lock (`src/lock.rs`)

- Methods: password, PIN, passkey (a security key or the computer's own), Windows Hello (face, fingerprint or Windows' PIN). Passkeys run in WebView2 at `http://tauri.localhost`; their signatures are checked with `ring`.
- Locks when xtermium starts (if set), on Ctrl+Shift+K, on the lock button beside Settings, and after the chosen idle minutes (`GetLastInputInfo` on Windows, which is system-wide).
- Locked: every window shows only the lock screen; sessions keep running.
- Changing the lock takes an unlock first; wrong passwords and PINs make the next try wait. `lock.xtm` is never sent to the UI.
- Automation gate: `unlocked()` runs at the top of `call()` in `src/automation.rs`, so every tool and CLI command answers "xtermium is locked. Unlock it on this computer to use the MCP server and the command line." The handshake (`initialize`, `ping`, `tools/list`) still answers. Test: `automation::tests::locked_xtermium_refuses_tools` (a wrong password doesn't open it; an unlock does).

## Automation server (`src/automation.rs`)

| Check | Behaviour |
|---|---|
| Listener | `127.0.0.1:<port>` (default 8721), or `0.0.0.0:<port>` with Settings › MCP › Listen on the network |
| TLS | network mode: HTTPS with a self-signed certificate (rcgen, kept in `automation-cert.xtm`), SHA-256 fingerprint shown; plain HTTP accepted only from a loopback peer (the CLI, agent tabs) |
| Request head | at most 16 KiB; a line that never ends can't fill memory |
| Body | `Content-Length` at most 1 MiB, else the connection is dropped |
| Host | loopback mode: must be `127.0.0.1:<port>` or `localhost:<port>`, else `403 wrong Host` (DNS rebinding) |
| Origin | if present, must be `http://127.0.0.1…` or `http://localhost…`, else `403 requests from web pages are not allowed` |
| Path and method | only `POST /mcp` (404 / 405 otherwise; 405 answered before the token check) |
| Token | `Authorization: Bearer <token>`; empty token or mismatch → 401; compared with `same()` (no early exit), also used for the instance secret |
| Lock | every tool call refused while locked; handshake still answers |
| Secrets | tools never return passwords; AI-agent sessions and tabs are invisible |
| Website tabs | driven through their own view (`src/web.rs` `page_*`, `web_*` tools); screenshots through WebView2 DevTools, Windows only |

Tests in the same file: `only_local_callers` (Host and Origin), the oversized-line case, the lock test.

## Windows and webviews

- CSP (`tauri.conf.json` `app.security.csp`): `default-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob: https://cdn.jsdelivr.net; connect-src ipc: http://ipc.localhost; frame-ancestors 'none'`. `devCsp` adds `http://localhost:8400` for development.
- Capabilities (`capabilities/default.json`): windows `main` and `tab-*` only; permissions `core:default`, `core:webview:allow-set-webview-zoom`, `core:window:allow-set-fullscreen`, `core:window:allow-is-fullscreen`, `core:window:allow-set-title`.
- `ui/main.ts` throws unless `ownWindow(label, framed)` says the webview is `main` or `tab-*` and not framed: website tabs share the app's address (`http://tauri.localhost`) and command rights, so the interface refuses to start anywhere but xtermium's own unframed windows.
- Website views step aside while a dialog or menu overlaps them; pages can't reach xtermium itself; BMC consoles and unsaved websites browse privately; saved websites keep their own profile.
- Reload, print and find shortcuts and the webview's context menu are blocked in the interface (a reload would drop every session).
- `custom.css` that changed is applied for 30 s and removed unless kept, so a stylesheet can't lock the window.

## Network-facing pieces

- SSH (`src/ssh.rs`): agent (OpenSSH, Pageant), keys with passphrase prompt, passwords, keyboard-interactive; agent forwarding per session, off by default; host keys trusted on first use, a changed key refuses; trusting a new key focuses Reject, so Enter never trusts; keepalives; connect timeouts for SSH and jump hosts.
- Tunnels: local, remote, SOCKS5; new ones bind `127.0.0.1`; imported ones keep the file's bind address (often `0.0.0.0`).
- RDP (`rdp/`, IronRDP): NLA/NTLM, server-key trust on first use; reconnects with the same trusted key and no prompt.
- VNC (`src/vnc.rs`): unencrypted; the app says so over the desktop unless an SSH gateway or a tunnel carries it.
- BMC relay (`src/kvm/relay.rs`): `ws://127.0.0.1:<port>/<token>/<host>/<port>/<path>`, onward TLS trusting only the certificate pinned for that BMC (`src/kvm/mod.rs` `trust`), with a connect timeout.
- WinRM: the password reaches PowerShell in an environment variable PowerShell removes at once, never on the command line.
- Single instance (`src/instance.rs`): `instance.lock` claims the folder; `instance.port` holds the port and a per-run secret; a message is the secret, a line break, `show`, then optionally a folder; without the secret it is dropped unanswered, so another local user can't open shells here.
- HTTPS from the app (the GitHub release check, icon downloads, sync) goes through `reqwest` on rustls with ring, with 5 s connect and 10 s total timeouts (`fetch` in `src/main.rs`); the app never updates itself.
- `fetch_favicon` (the session editor's "Website icon") calls `fetch(…, insecure = true)`: certificates aren't checked, because admin pages are mostly self-signed and only a picture comes back, shown in `<img>`. Every other fetch checks certificates.

## Dependencies and advisories

- `cargo audit` accepted advisories (`.cargo/audit.toml`, `rdp/.cargo/audit.toml`): RUSTSEC-2023-0071 (rsa "Marvin" timing; no fixed release; one signature per SSH login), RUSTSEC-2026-0253 (lru `pop()` unsound only if a key's Drop panics; wezterm-term's only cache keys are `[u8; 32]`). `cargo audit` fails on anything new.
- Warnings only: `adler`, `paste`, `proc-macro-error`, `glib` (Linux).
- `patches/glib`: glib 0.18.5 with gtk-rs's one-line fix for GHSA-wrw7-89jp-8q8g (a `ponytail:` note: drop it once Tauri moves to glib 0.20 or newer).
- Dependabot's `deepmerge` alert has no patched version; it is build-time only, inside vite-plugin-svelte.
- `rdp/` has its own `Cargo.lock` (IronRDP and russh pin incompatible pre-release crypto crates): dependency fixes go into both lockfiles.
- Licences: all permissive except the MPL-2.0 crates `serialport`, `cssparser`, `selectors`, `dtoa-short`, `option-ext`, and `unescaper` (MIT OR GPL-3.0-only; MIT taken). `THIRD-PARTY-NOTICES.txt` ships in the installers.

## Review checklist for a security-relevant change

- [ ] The guard sits in the shared path every caller uses, not in one caller.
- [ ] A test fails if the guard is removed (name it in the pull request).
- [ ] Nothing new is logged, printed, exported or returned that could hold a secret (passwords, tokens, key passphrases, the recovery key).
- [ ] A new tool or command respects the lock and never exposes agent tabs or passwords.
- [ ] New windows are `main` or `tab-*`, or `ownWindow()` and `capabilities/default.json` changed with them; the CSP is not relaxed without a reason.
- [ ] Data stays inside the data folder.
- [ ] Checks run: `npm run check`, `npm test`, `cargo clippy --all-targets && cargo test`, and `cargo audit && (cd rdp && cargo audit) && npm audit` after dependency changes.
- [ ] Core behaviour unchanged, or a maintainer agreed first.

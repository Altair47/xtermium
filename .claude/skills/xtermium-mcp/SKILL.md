---
name: xtermium-mcp
description: Use when connecting an AI assistant or script to xtermium, turning on Settings › MCP, adding the server to Claude Code, Gemini CLI, Copilot CLI, opencode, Codex or VS Code, using the xtermium command line, choosing a tool to open sessions, run commands, read terminals, drive remote desktops or website tabs, tunnels, workspaces, layouts or restore points, running agents in xtermium tabs, or when a tool answers that xtermium is locked.
---

# Using xtermium with AI agents and scripts

## Overview

xtermium has an MCP server and a command line that call the same tools: open sessions, run commands and read terminals, drive remote desktops and website tabs, start tunnels, switch workspaces, open layouts, detach tabs and keep restore points. The server is off until Settings › MCP turns it on; it listens on `http://127.0.0.1:8721/mcp` behind a token, or on the network over HTTPS with a self-signed certificate. Tools never reveal passwords, never see AI-agent tabs, and refuse while xtermium is locked. Sources: `README.md` ("Agents and scripts"), the wiki's AI agents page, `src/automation.rs` (the tools) and `src/cli.rs` (the CLI).

## When to use

- Setting up an assistant to drive xtermium, on this computer or another one.
- Writing a script against `xtermium <command>`.
- Picking the right tool or CLI word for a task.
- Running Claude Code, Codex or another agent inside an xtermium tab.
- A tool says "xtermium is locked", returns empty output, or can't see a tab.

## Quick reference

| Thing | Value |
|---|---|
| Turn it on | Settings › MCP › **Let AI assistants and scripts control xtermium** |
| Address (this computer) | `http://127.0.0.1:8721/mcp` (port changeable on the page) |
| Address (network) | `https://<this computer's address>:<port>/mcp`, listening on `0.0.0.0`, self-signed certificate; the page shows its SHA-256 fingerprint |
| Auth | `Authorization: Bearer <token>`; Show, Copy or New token on the page (a new token stops the old one) |
| Protocol | MCP over HTTP, `POST /mcp` only |
| CLI | `xtermium <command>`, JSON output, `xtermium help` lists everything; needs Settings › MCP on |
| Locked | every tool answers "xtermium is locked. Unlock it on this computer to use the MCP server and the command line." |
| Never shown | passwords, AI-agent sessions and tabs |

> Anything that has the token can open your sessions, type into your terminals and start tunnels, as you. Keep it like a password. Web pages are always refused.

## Adding the server to an assistant

The page shows a ready command per assistant, with Copy and Run buttons (Run starts it in a local shell). On this computer they read:

| Assistant | Command |
|---|---|
| Claude Code | `claude mcp add --transport http xtermium http://127.0.0.1:8721/mcp --header "Authorization: Bearer <token>"` |
| Gemini CLI | `gemini mcp add --transport http --header "Authorization: Bearer <token>" xtermium http://127.0.0.1:8721/mcp` |
| Copilot CLI | `copilot mcp add --transport http --header "Authorization: Bearer <token>" xtermium http://127.0.0.1:8721/mcp` |
| opencode | `opencode mcp add xtermium --global --url http://127.0.0.1:8721/mcp --header "Authorization=Bearer <token>"` |
| Codex | Windows: `setx XTERMIUM_TOKEN <token> && codex mcp add xtermium --url http://127.0.0.1:8721/mcp --bearer-token-env-var XTERMIUM_TOKEN`; elsewhere the token goes into `~/.profile` (`echo 'export XTERMIUM_TOKEN=<token>' >> ~/.profile && export XTERMIUM_TOKEN=<token> && codex mcp add …`) |
| VS Code | `code --add-mcp '{"name":"xtermium","type":"http","url":"http://127.0.0.1:8721/mcp","headers":{"Authorization":"Bearer <token>"}}'` (on Windows the inner quotes are escaped as `\"` inside double quotes) |

- Codex reads the token from the environment variable: open a new terminal before starting it, and run the command again after making a new token.
- Other assistants that take MCP servers over HTTP mostly read a configuration like this, which the page copies with the token in it:

  ```json
  {
    "mcpServers": {
      "xtermium": {
        "type": "http",
        "url": "http://127.0.0.1:8721/mcp",
        "headers": { "Authorization": "Bearer <token>" }
      }
    }
  }
  ```

## On the network

- Settings › MCP › Listen on: The network binds `0.0.0.0` and speaks HTTPS with a self-signed certificate (kept in `automation-cert.xtm`); the page shows an Address field (this computer's addresses to pick from) and the certificate's SHA-256 fingerprint. Plain HTTP is still taken from loopback only (the CLI, agent tabs).
- The commands are then meant for the other machine (a macOS or Linux shell, mostly), use the `https://` address, and start the Node-based assistants (Claude Code, Gemini CLI, Copilot CLI, opencode) with `NODE_TLS_REJECT_UNAUTHORIZED=0` for that run, because the certificate is self-signed. Compare the fingerprint the page shows.
- Codex and VS Code can't be told to skip the certificate check. For them, forward the port over SSH from that machine (`ssh -L 8721:127.0.0.1:8721 <this computer>`) and use `http://127.0.0.1:8721/mcp` there.

## The tools and CLI words

| Family | MCP tools | CLI words |
|---|---|---|
| Sessions | `sessions`, `session_save`, `session_delete`, `folders`, `folder_save`, `logins` | `sessions`, `logins` |
| Tabs | `tabs`, `open` (`session`, `shell` + `dir`, `url`, or `address` like `user@host`, `ssh user@host:2222`, `rdp://host`, `vnc host:5901`, `telnet switch1`), `close` | `tabs`, `open <session>`, `open --shell [program] [--dir <folder>]`, `open --address <address>`, `open --url <address>`, `close <tab>` |
| Run and read | `run` (`until`, `wait_ms`, `lines`), `read`, `wait_for`, `type`, `key` | `run <tab> <command>`, `read <tab> [--lines <n>]`, `wait <tab> <regex> [--timeout <ms>]`, `type <tab> <text>`, `key <tab> <keys>` (`ctrl+c`, `Enter`, `"up up Enter"`, `alt+F4`, `meta`) |
| Remote desktops (RDP, VNC, BMC consoles) | `screenshot`, `click`, `move`, `drag`, `scroll`, plus `type` and `key` | `screenshot <tab> [--out <file.png>] [--max-width <px>] [--region <x,y,w,h>]`, `click <tab> <x> <y> [--right\|--middle] [--double]`, `move`, `drag`, `scroll <tab> <lines> [--at <x,y>]` |
| Websites | `open` with `url`, `web_read`, `web_click`, `web_fill`, `web_go`, `web_eval`, `screenshot` (Windows), `close` | `web-read <tab> [--chars <n>]`, `web-click`, `web-fill [--submit]`, `web-go <address\|back\|forward\|reload>`, `web-eval <tab> <script>` |
| Tunnels | `tunnels`, `tunnel_start`, `tunnel_stop` | `tunnels`, `tunnel start\|stop <name>` |
| Workspaces | `workspaces`, `workspace_switch`, `workspace_clone`, `workspace_sync`, `move_to_workspace` | `workspaces`, `workspace <name>`, `clone-workspace <name>`, `sync-workspace <from> <to>`, `move-to <workspace> <session\|folder/>...` |
| Layouts | `layouts`, `layout_open` | `layouts`, `layout <name>` |
| Detach | `detach`, `attach` (terminal and RDP tabs) | `detach <tab>`, `attach <tab>` |
| Restore points | `restore_points`, `restore_point_take` | `restore-points`, `restore-point` |

- `session_save`, `session_delete`, `folders` and `folder_save` have no CLI word.
- Looks: `sessions` and `folders` list each one's name colour, icon (a picture shows as `"picture"`), and for sessions the terminal style and notes, so a change can see what was there first. `session_save` and `folder_save` take `color` (`#rrggbb`, `""` for none), `icon` (a built-in name, an `https://` picture link fetched once, `""` for the default, `"picture"` to keep it) and `still`; `folder_save` also renames or moves a folder with everything inside (`to`) and sets its `tags`.
- Screenshots are scaled to `max_width` (default 1568); the result gives the desktop's size and the scale, and every x and y is in desktop pixels: divide what you measure on the picture by `scale`. In terminals, `click` takes a column and a row.
- Website tabs are driven through the tab itself (no browser or DevTools of their own); elements are a number from `web_read`, a CSS selector, or `--text` and the name the element shows. Website screenshots work on Windows only.
- `workspace_sync` updates and adds, never removes, and keeps a restore point first. `""` is the default workspace. `sessions`, `logins`, `tunnels`, `open` and `session_save` work in the workspace in use.
- Restoring a restore point is done by hand in Settings › Import and export.

A script that opens a shell, runs a command, waits for its output and closes the tab:

```
$ xtermium open --shell
{
  "tab": 2
}
$ xtermium run 2 "echo hello from xtermium"
{
  "sent": true
}
$ xtermium wait 2 "hello from" --timeout 3000
{
  "match": "hello from",
  "matched": true,
  "output": "…"
}
$ xtermium close 2
{
  "closed": true
}
```

## What tools never reveal

- Passwords: `logins` lists shared logins (id, name, user, whether they hold a password or key, their SSH gateway) and the hosts a password was remembered for, never the passwords themselves. No tool returns a password.
- AI-agent sessions and tabs don't exist for automation: one agent can't read or type into another agent's tab, or its own.

## The lock

- While xtermium is locked (Settings › Security), every tool call and CLI command answers "xtermium is locked. Unlock it on this computer to use the MCP server and the command line."
- `initialize`, `ping` and `tools/list` still answer, so assistants keep the server and carry on after an unlock. Don't treat a locked answer as a broken server.

## Agents in xtermium tabs

- An AI agent session runs an agent's command-line tool in a local terminal tab: Ctrl+Shift+N › AI agent. Tools: Claude Code, Codex, OpenCode, GitHub Copilot, Cursor CLI (command `agent`), or a custom command. Its command must be on the PATH.
- Fields: Mode (Ask before edits and commands · Edit freely, ask before commands · Run everything without asking · The tool's default), Effort (Claude Code and Codex: low, medium, high, xhigh or max), Model, Arguments (they come last, so they win), Working folder.
- While Settings › MCP is on, an agent started from an AI agent session gets xtermium's MCP server without setup, with the token in its environment rather than on its command line.
- Right-click an agent tab › Open the folder in VS Code opens its working folder.

## Known bug: `run --until` and `--wait`

`xtermium run <tab> <cmd> --until <regex>` and `--wait <ms>` (the `run` tool's `until` / `wait_ms`) return `"output": ""` (and `"settled": false` after 30 s) although the command ran. Until it is fixed: `run` without them, then `wait <tab> <regex>` (`wait_for`) or `read` to get the text. Never show `--until` or `--wait` in docs or screenshots (`xtermium-designing`, "Show what is there").

## Common mistakes

- Expecting the CLI to work with Settings › MCP off (it is a client of the running app).
- Putting the network address in a Codex or VS Code config without the SSH port forward.
- Skipping certificate checks without comparing the fingerprint the page shows.
- Reading a "locked" answer as a broken server, or retrying the handshake in a loop.
- Using `run --until` / `--wait` and trusting the empty output.
- Clicking at picture pixels on a scaled screenshot instead of desktop pixels.
- Looking for an agent's tab in `tabs` (it is invisible on purpose).
- Sharing the token in a command line others can read; it acts as you.

## Read next

- `README.md` "Agents and scripts": the user-facing summary.
- `CLAUDE.md` "Automation (MCP + CLI)": how tools are added.
- `CONTRIBUTING.md`: proposing a new tool.
- `xtermium-developing`: adding a tool and CLI word; the automation server's security checks.
- `xtermium-testing`: driving a test instance over CDP or the CLI.
- `xtermium-designing`: which commands may be shown.

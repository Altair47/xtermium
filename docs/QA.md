# Not yet tested

What has shipped (on `main`, in the nightly or a release) but hasn't been seen working on
real hardware or a real setup yet. Automated tests pass for all of it; what's missing is a
person, a second account or a system we don't have at hand. If you can check an item, say how
it went in an issue or a pull request, and remove the item once it has held up.

## Terminal Co-op

Needs two (or three) Pro accounts on separate machines, joined in one ring through the relay.

- Shared selections: everyone selects on their own, and each selection is drawn in its
  person's ring colour in every window.
- Copy and paste between members (sizes up to the limits on the wiki's Usage page), and a
  paste from an owner on an older version.
- The owner renames a ring member (needs the sync server with the rename, live since
  2026-11-07).
- A removed member rejoining.
- Remote desktop tabs shared in a ring: guests can't copy, paste or drop files.
- Ring colours: the owner amber and each guest their own colour, the same on every member's
  computer, round the pictures in the Co-op dialog and on pointers and selections (checked
  here only with a faked ring event in one window).
- Sharing an AI agent tab, a BMC console (its screen as pictures) and a website, and a guest
  with control driving the website (DevTools input, Windows) or the console: the owner's side
  was exercised with faked ring state in one window; a real ring hasn't driven either yet.
- Click rings and the moving pointer's name in every member's view, and the owner's pointer
  and clicks read from inside a shared page.
- A shared terminal drawn at each viewer's own font and zoom while the owner resizes or zooms.
- The picture quality setting's three levels as a viewer sees them.

## BMC consoles

Checked on 2026-11-09 against real BMCs (read only): iLO 4 (autologin, console over the page,
video), a Gigabyte MegaRAC SP-X and ASUS ASMB10 and ASMB11 (sign-in, H5Viewer launched in the
tab, scaled to fill it), a Supermicro X11 (autologin, console), and pasting into a console.
Still to see:

- Autologin on the ASUS boards with their own passwords (they were signed in by hand; the
  form filled and submitted, and the BMC refused the other board's password, as expected).
- The mouse through the MegaRAC cursor layer (automation and Co-op desktop control).
- iDRAC 7 and 8 after the shared changes (notes kept on the top window, key target).
- The Java hints: an iLO with firmware older than 2.70, a MegaRAC with the H5Viewer disabled.

## macOS

The macOS build ships unsigned, and nobody has run these on a Mac yet.

- The fix for the SSH key picker hanging ([issue #40](https://github.com/xtermium/xtermium/issues/40)): native dialogs now open with the calling window as their parent (PR #39). Not confirmed on a Mac.
- The app's own title bar: the overlay style with its three buttons, dragging.
- The per-system shortcuts (⌘ in place of Ctrl), as the wiki's shortcut table lists them.
- Opening the unsigned `.dmg` past Gatekeeper, as the Installation page describes it.

## Linux

- The app's own title bar (no system decorations): dragging, resizing from the edges,
  maximising, and the polish pass's sizes; the window menu (Minimize, Maximize, Close) a
  right-click on the bar's free space opens.
- The clipboard on X11 and Wayland, in terminals and in Co-op.

## Right to left

- The Arabic locale with the new title bar, the sidebar row on the tab line and the
  Getting started guide.

## Windows

- Unlocking with Windows Hello (needs a camera or a fingerprint reader).

## Remote desktops

- RDP to xrdp: the picture is drawn sheared, row by row (an open decoder bug). Then the
  resize that keeps the session and the login with a saved password, on a real xrdp host.

## Automation

- `run … --until` and `--wait` in an SSH tab right after `clear` (checked in a local cmd.exe
  tab on Windows).

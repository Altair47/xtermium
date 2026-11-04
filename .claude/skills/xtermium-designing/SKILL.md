---
name: xtermium-designing
description: Use when changing anything people see in the xtermium app (Svelte components, ui/app.css, icons, menus, dialogs, motion), writing UI text, README or docs copy, using the xtermium mark, colours or wordmark, making or replacing a screenshot or recording, or reviewing a visual change against DESIGN.md.
---

# Designing xtermium

## Overview

The app's look is "The Operator's Rack": the brand's world (the website's operator's terminal and the brand kit) carried into a modern VS Code, dense, in golden-ratio proportions. Near-black neutral chrome in six tonal steps, one phosphor amber accent the user can change, a cable colour per kind of connection, JetBrains Mono for short text and the system face for long text, Fibonacci spacing and five type sizes enforced by a test. `DESIGN.md` is the full record (tokens in its front matter, components, named rules, do's and don'ts) and `ui/app.css` holds the live tokens. This skill is the map, the rules that are easy to break, the brand basics, the copy style and the screenshot rules.

## When to use

- Editing styles or markup in `ui/app.css`, `ui/App.svelte` or `ui/lib/*.svelte`.
- Adding a component, menu, dialog, icon or animation.
- Writing UI text, README text or docs.
- Placing the logo, colours or wordmark; changing the README hero.
- Making a screenshot, capture or recording of the app.
- Reviewing a design change.

## Quick reference

| Scale | Values |
|---|---|
| Spacing (Fibonacci) | 1 2 3 5 8 13 21 34 px (55 and 89 allowed, unused); padding pairs 1 : φ (`3px 8px` controls, `5px 8px` compact blocks, `8px 13px` cards, `13px` dialog body) |
| Type tokens | `--t-s` 11.5, `--t` 13, `--t-l` 14.5, `--t-xl` 16.5, `--t-xxl` 18.5 px |
| Heights | `--row` 21 (list rows, status bar), `--control` 26 (buttons, fields, menu items), `--bar` 34 (tab strip) |
| Radii | `--r-control` 3, `--r-menu` 5, `--r-window` 8 px; nested = `calc(outer - padding)` |
| Gap | `--gap` 5 px |
| Dialog widths | 377, 610, 987 px; Settings is 987 × 610 |
| Shadows | `--shadow-float`, `--shadow-menu`, `--shadow-dialog`, on floating layers only |
| Accent | `var(--gold)`, phosphor amber (`#ffb000` dark, `#754b00` light, by default; the token keeps its old name; never hard-code it) |
| Long face (`--ui`) | Segoe UI Variable Text, Segoe UI, system-ui, -apple-system, Ubuntu; 13px; body text, prose, field labels, menus and lists |
| Short face (`--short`, `--mono`) | JetBrains Mono Variable (`@fontsource-variable/jetbrains-mono`): titles, buttons, tags, the status bar, the start screen's lead, code and keys; `--short` falls back to `--ui` for ar, hi, ja, zh |
| Logo | `ui/lib/Logo.svelte` (`assets/logo-horizontal.svg`); its tiles and wordmark in `--mark` |

| `DESIGN.md` section | For |
|---|---|
| Overview | the north star and key characteristics |
| Colors (Primary, Secondary, Tertiary, Neutral, Named Rules) | phosphor amber, kind hues, status colours, near-black neutrals and `--mark`, the One Accent, Logo Colours, Cable Colour and See-Through Chrome rules |
| Typography | faces, hierarchy, the Five Sizes, Golden Line and Short-in-Mono rules |
| Layout | panels, padding pairs, heights, the start screen, forms, no breakpoints, the Fibonacci and Derived Value rules |
| Elevation & Depth | tonal depth, the three shadows, the Float-Only Shadow rule |
| Shapes | three corners, the Nested Curve rule |
| Components | buttons, fields, menus, navigation, dialogs and palette, tags and toasts, kind glyph, broadcast outline, backgrounds, session icons |
| Do's and Don'ts | the checklist in prose |

## The app's look rules

- **Spacing and type only from the scales.** `ui/lib/scale.test.ts` in `npm test` fails on off-scale spacing (4, 6, 10, 12, 16 px…) and on a raw px `font-size`. A value worked out from others is a `calc(…)`.
- **One accent.** Phosphor amber (`--gold`) marks focus, picked options, toggles that are on, links to another place, drop targets, the focused pane's ring and the single primary button of a dialog (and folder tags, and the start screen's prompt sigil and cursor). No amber headings, icons or decorative strokes. Read it as `var(--gold)`: the user can change it.
- **Cable colours.** Each kind of connection has a hue (`--k-ssh` cobalt, `--k-rdp` violet, `--k-serial` chartreuse, …), read through a local `--kind` and used only to say "this is an SSH/RDP/… thing": the session icon, the 2px tab connector, the glyph tile. State is always `ok` / `warn` / `error`, never a kind hue. User marks tint names and tab lines, never a session's icon.
- **See-through chrome.** Chrome is `rgba(var(--panel-rgb), var(--chrome-alpha))`, never an opaque hex, so the Transparent effect shows through. Any place is two layers, the window's ground and a panel or terminal on it, that together make the Opacity setting, so don't paint a third between them.
- **Panels** are rounded and float a gap apart on a darker window, flat at rest. Only floating layers (menus, dialogs, palette, toasts, find and reconnect bars) cast a shadow, from the three tokens, and arrive with `float-in`; menus open instantly.
- **Menus** end with their destructive item (`.danger`); `.sep` starts a group; items without an icon indent to line up; keyboard-driven.
- **Dialogs** (`Modal.svelte`): 377 / 610 / 987 wide; they hold the keyboard; Escape closes the top one; one amber primary; a red danger primary only when yes ends or deletes something; long bodies put the buttons in a fixed `foot`. Enter focuses yes, except irreversible or security answers (trust a new host key, delete, restore, new automation token, discard), which focus Cancel or Reject.
- **Undo over asking**: something that can be put back just happens, and a toast offers Undo.
- **Icons**: Boxicons SVGs in `assets/icons` through `Icon.svelte`; tool buttons are icon-only with a tooltip. Only session and folder icons (`SessionIcon.svelte`) animate (blink, flow, spin, bob), only while the pointer is over their row or tab, never under reduced motion, never on the interface's own buttons.
- **Motion**: 0.1–0.2 s, arrivals on `--ease-out`, exits faster than entrances; CSS motion stops under `prefers-reduced-motion` through `ui/app.css`; a Svelte transition must check it itself (`duration: 0`).
- **Right to left**: logical properties throughout (`inset-inline-*`, `margin-inline-*`), so Arabic mirrors for free.
- **Short in mono.** Titles, buttons, tags, keys, the status bar and code are in `var(--short)` (JetBrains Mono); body text, prose, field labels, menus (`.menu button`), list rows (`button.item`), the Settings page list and the session editor's five-to-a-row kind picker stay in `var(--ui)`.
- **Light theme** (`:root[data-theme="light"]`) redefines every token with the same roles, in neutral greys with charcoal `#141414` text; the amber deepens to `#754b00`.
- **The logo and the prompt.** The start screen shows the website's prompt (`~/xtermium $`, an amber `$` and a blinking amber block cursor, hidden from assistive tech) above the brand kit's horizontal logo through `Logo.svelte`; Settings › About shows the logo at 233px. There is no gradient and no ASCII art.

Tokens, kind hues, type roles and components condensed: `references/app-look.md`.

## Copy style

- Short plain sentences, second person, no jargon in labels.
- Places are written "Settings › Page", and text that names a place links to it (`openSettings(page)`, `button.link`, `tell(text, page)`).
- All UI text goes through `_("…")` (`xtermium-developing`).
- The name is **xtermium**, lower case, in the app, its README, code, commands and paths (`PRODUCT.md`, "Brand Commitments"). The wordmark and the website write "XTermium"; the website's own rules live in the `xtermium-web` repository's `DESIGN.md`.
- Claim only what the README and the app show: there are no customers, testimonials, case studies or benchmarks on hand (`PRODUCT.md`, "Evidence on Hand"), so none are invented.
- Comments read the same way: full sentences saying why.
- Commit messages and release notes: for a developer reading what changed (`xtermium-contributing`).

## Brand basics

| Colour | Hex | Use |
|---|---|---|
| Amber | `#FFC20E` | the small diamond of the mark; README badges |
| White | `#F2F2F2` | the mark's large diamonds on dark backgrounds |
| Charcoal | `#141414` | the mark on light backgrounds; badge labels |
| Ink | `#050506` | dark backgrounds behind the mark |

- **The mark**: four rounded diamonds in a diamond arrangement, three large ones white (`#F2F2F2` on dark, charcoal `#141414` on light) and one small amber `#FFC20E` diamond at the foot. Never recolour the amber diamond, stretch, rotate or add effects; keep clear space of at least the small diamond's height.
- **The wordmark**: "XTermium" in JetBrains Mono Bold (700).
- The brand amber `#FFC20E` is not the app's accent (`--gold`, the phosphor amber `#ffb000`, user-changeable) and is never used for app UI state; it and `--mark` live only inside the logo.
- **In this repo**: `assets/logo.svg` and `assets/logo.png`; the horizontal logo `assets/logo-horizontal.svg` (the kit's on-dark horizontal logo), shown through `ui/lib/Logo.svelte`, which swaps its `#F2F2F2` fills for `currentColor` (`--mark`) so one file serves both themes and never touches the amber tile; the Tauri icons in `icons/` (`32x32.png`, `128x128.png`, `128x128@2x.png`, `icon.icns`, `icon.ico`, `icon.png`); the README hero `assets/readme/hero.svg` (1200 × 420, about 14 KB) with its poster `assets/readme/hero.png`.
- **The README hero** types `~/xtermium $ cat readme.txt`, lights the mark's four tiles as a 2×2 split and turns them into the mark, then holds with a blinking cursor. It is CSS animation only, no script and nothing fetched, so it works as an `<img>` on GitHub; JetBrains Mono (SIL OFL) is embedded at weights 500 and 700; under reduced motion it shows the last frame at once. The source files are generated outside this repository: replace `hero.svg` and `hero.png` together, never hand-edit them.

## Screenshots and captures

- **Show what is there.** A command, its output or a feature shown in a screenshot, the README or docs must have been seen working in the real app and is transcribed verbatim from a real run. Leave out anything broken or unverified even if the help text lists it: today that means `xtermium run --until` / `--wait` (empty output, `xtermium-mcp`), RDP against an xrdp server (renders sheared), and any feature you could not exercise (for example WinRM without a Windows host).
- **Demo user `sotiris`**, never `demo`: SSH containers' user name, the VNC desktop's user, shared logins. Captions may still say "demo host".
- **Demo data only**: local containers on 127.0.0.1 and reserved names and addresses (192.0.2.x, 2001:db8::, `home.arpa`); demo folder tags such as `prod`, `bmc`, `stg`, `lab`, `net`, `win`.
- **The real app, never a mock**: captures come from a debug build with a throwaway `XTERMIUM_HOME`, driven over CDP (`xtermium-testing`). Never capture the desktop. When the real thing isn't on hand (a BMC), show the real app opening a local stand-in page and say so in the caption, rather than mocking an app screen.
- **The caption says exactly what the picture shows.**
- The README's pictures are served from the website (`https://xtermium.com/assets/shots/…`); recapture them when the UI they show changes.

## Accessibility

Full keyboard operation (every action has a rebindable shortcut; the palette reaches everything; dialogs hold the keyboard so nothing typed reaches a server); right-to-left layout for Arabic; reduced motion respected; text contrast WCAG AA in dark and light (`PRODUCT.md`).

## Review checklist for a visual change

- [ ] Only the app's own tokens from `ui/app.css`; no new hex, no hard-coded accent.
- [ ] Spacing and type on the scales (`npm test` green).
- [ ] Amber only where the One Accent rule allows; kind hues only for kinds; state in `ok` / `warn` / `error`.
- [ ] Shadows only on floating layers, from the three tokens.
- [ ] Reduced motion leaves the result complete and still; Svelte transitions check it.
- [ ] Keyboard path works; focus is visible (a 1px amber outline).
- [ ] Copy follows the copy style; text goes through `_()`; stale locale keys removed.
- [ ] Seen in the running app (`xtermium-testing`); any new screenshot follows the rules above.
- [ ] `DESIGN.md` still describes what shipped; update it in the same pull request if not.

## Common mistakes

- `padding: 4px 12px` or `font-size: 12px` (off-scale; the test fails).
- Hard-coding the accent's hex, or using a kind hue to show state.
- A shadow on a resting panel, tab, row or button, or a new shadow value.
- Animating the interface's own toolbar icons.
- A menu whose destructive item isn't last.
- Enter focused on yes in an irreversible or security dialog.
- Writing "Xtermium" or "XTERMIUM" in the app or README (it is `xtermium`), or using the brand amber as an app colour.
- Recolouring, stretching or adding effects to the mark.
- Setting a list, a menu or a five-to-a-row grid (the kind picker) in the mono: names stop fitting and it no longer reads down.
- Using `#FFC20E` or `--mark` outside the logo.
- A screenshot of a mocked screen, of the user `demo`, or of a broken command.

## Read next

- `DESIGN.md`: the full design system.
- `PRODUCT.md`: users, principles, accessibility, brand commitments.
- `CLAUDE.md` "UI conventions": the conventions in code terms.
- `CONTRIBUTING.md`: how a design change is proposed.
- `references/app-look.md`: tokens and components, condensed.
- `xtermium-developing`: the scale test, i18n, components in code.
- `xtermium-testing`: the visual check that makes screenshots.
- `xtermium-mcp`: which commands may be shown.

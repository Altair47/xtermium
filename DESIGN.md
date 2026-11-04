---
name: xtermium
description: A terminal and session manager for sysadmins, in the brand's near-black chrome with one phosphor amber accent and a cable colour per kind of connection.
colors:
  gold: "#ffb000"
  gold-ink: "#050506"
  gold-soft: "rgba(255, 176, 0, 0.14)"
  mark: "#f2f2f2"
  k-ssh: "#5c8df0"
  k-shell: "#33b8a6"
  k-serial: "#b5c14a"
  k-rdp: "#9c70e6"
  k-browser: "#dd6b9c"
  k-telnet: "#d98a4a"
  k-winrm: "#4aa8d9"
  k-vnc: "#c46fd1"
  k-kvm: "#8fa3b8"
  k-agent: "#ec6a6a"
  k-coop: "#c9b48a"
  ok: "#4fc08d"
  warn: "#e0b04a"
  error: "#ec6a6a"
  error-ink: "#200a0a"
  window: "#050506"
  deep: "#08080a"
  bg: "#0b0b0d"
  panel: "#111114"
  raised: "#1d1d22"
  hover: "#26262c"
  line: "#2c2c33"
  fg: "#e6e6e1"
  weak: "#9d9da6"
typography:
  prompt:
    fontFamily: "JetBrains Mono Variable, Cascadia Mono, Consolas, monospace"
    fontSize: "13px"
    fontWeight: 400
    lineHeight: 1.4
  headline:
    fontFamily: "JetBrains Mono Variable, Cascadia Mono, Consolas, monospace"
    fontSize: "16.5px"
    fontWeight: 600
    lineHeight: 1.618
    letterSpacing: "-0.025em"
  title:
    fontFamily: "JetBrains Mono Variable, Cascadia Mono, Consolas, monospace"
    fontSize: "14.5px"
    fontWeight: 600
    lineHeight: 1.4
    letterSpacing: "-0.01em"
  short:
    fontFamily: "JetBrains Mono Variable, Cascadia Mono, Consolas, monospace"
    fontSize: "13px"
    fontWeight: 400
    lineHeight: 1.4
  body:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, -apple-system, Ubuntu, sans-serif"
    fontSize: "13px"
    fontWeight: 400
    lineHeight: 1.4
  prose:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, -apple-system, Ubuntu, sans-serif"
    fontSize: "13px"
    fontWeight: 400
    lineHeight: 1.618
  label:
    fontFamily: "Segoe UI Variable Text, Segoe UI, system-ui, -apple-system, Ubuntu, sans-serif"
    fontSize: "11.5px"
    fontWeight: 400
    lineHeight: 1.4
  label-short:
    fontFamily: "JetBrains Mono Variable, Cascadia Mono, Consolas, monospace"
    fontSize: "11.5px"
    fontWeight: 400
    lineHeight: 1.4
  mono:
    fontFamily: "JetBrains Mono Variable, Cascadia Mono, Consolas, monospace"
    fontSize: "0.92em"
    fontWeight: 400
rounded:
  control: "3px"
  menu: "5px"
  window: "8px"
  round: "50%"
spacing:
  "1": "1px"
  "2": "2px"
  "3": "3px"
  "5": "5px"
  "8": "8px"
  "13": "13px"
  "21": "21px"
  "34": "34px"
  gap: "5px"
  row: "21px"
  control: "26px"
  bar: "34px"
components:
  button:
    backgroundColor: "{colors.raised}"
    textColor: "{colors.fg}"
    typography: "{typography.short}"
    rounded: "{rounded.control}"
    padding: "3px 8px"
    height: "{spacing.control}"
  button-hover:
    backgroundColor: "{colors.hover}"
  button-primary:
    backgroundColor: "{colors.gold}"
    textColor: "{colors.gold-ink}"
    rounded: "{rounded.control}"
    padding: "3px 8px"
    height: "{spacing.control}"
  button-primary-hover:
    backgroundColor: "color-mix(in srgb, #ffb000 86%, white)"
  button-danger:
    backgroundColor: "{colors.error}"
    textColor: "{colors.error-ink}"
    rounded: "{rounded.control}"
    padding: "3px 8px"
    height: "{spacing.control}"
  button-on:
    backgroundColor: "{colors.gold-soft}"
    textColor: "{colors.gold}"
  button-flat:
    backgroundColor: "transparent"
    textColor: "{colors.weak}"
  button-flat-hover:
    backgroundColor: "{colors.raised}"
  button-icon-only:
    padding: "3px 5px"
    height: "{spacing.control}"
  input:
    backgroundColor: "{colors.deep}"
    textColor: "{colors.fg}"
    rounded: "{rounded.control}"
    padding: "3px 8px"
    height: "{spacing.control}"
  menu:
    backgroundColor: "{colors.panel}"
    rounded: "{rounded.menu}"
    padding: "3px"
    width: "190px"
  menu-item:
    backgroundColor: "transparent"
    rounded: "2px"
    padding: "3px 8px"
    height: "{spacing.control}"
  menu-item-hover:
    backgroundColor: "{colors.raised}"
  tab:
    backgroundColor: "{colors.panel}"
    textColor: "{colors.weak}"
    padding: "0 5px"
    height: "{spacing.bar}"
  tab-active:
    backgroundColor: "{colors.bg}"
    textColor: "{colors.fg}"
  rack-row:
    textColor: "{colors.fg}"
    padding: "0 5px"
    height: "{spacing.row}"
  rack-row-hit:
    backgroundColor: "{colors.gold-soft}"
  panel:
    backgroundColor: "{colors.panel}"
    rounded: "{rounded.window}"
  dialog:
    backgroundColor: "{colors.panel}"
    rounded: "{rounded.window}"
    padding: "13px"
    width: "610px"
  palette:
    backgroundColor: "{colors.panel}"
    rounded: "{rounded.menu}"
    padding: "5px"
    width: "610px"
  status-bar:
    backgroundColor: "{colors.panel}"
    textColor: "{colors.weak}"
    typography: "{typography.label-short}"
    padding: "0 8px"
    height: "{spacing.row}"
  tag:
    backgroundColor: "{colors.gold-soft}"
    textColor: "{colors.gold}"
    typography: "{typography.label-short}"
    rounded: "{rounded.control}"
    padding: "0 5px"
  toast:
    backgroundColor: "{colors.panel}"
    textColor: "{colors.fg}"
    rounded: "{rounded.menu}"
    padding: "5px 13px"
  kbd:
    backgroundColor: "{colors.deep}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "0 5px"
---

# Design System: xtermium

## Overview

**Creative North Star: "The Operator's Rack"**

xtermium is a rack of live connections, in the brand's world: the operator's terminal of the
website (xtermium.com) and the brand kit, carried into the app. The chrome is the website's
near-black ground and surfaces, neutral and a hair apart, getting lighter the nearer they sit to
the hand. Each kind of connection has its own hue, the way cables on a patch panel are
colour-coded: SSH is cobalt, RDP violet, serial chartreuse. One phosphor amber accent marks where
you are and what you picked. Nothing else is coloured, so a glance at the rack tells you which
kind of session each row and tab is, and a glance at the amber tells you where the keyboard is.
The start screen waits at the website's prompt, `~/xtermium $` and a blinking amber block cursor,
over the brand kit's logo.

The feel is quiet and exact, as dense as a modern code editor: 13px text, 26px controls, 21px
rows, small precise corners. JetBrains Mono, the brand's face, says everything short (titles,
buttons, tags, the status bar, code); the system face says everything long (text, menus and
lists), so the app still reads as a native tool. The sidebar (the session rack), the tab area
with its panes, and the file panel are rounded panels floating a few pixels apart on a darker
window. They are flat at rest and are told apart by tone and hairline, not by shadow. Only things
that float above the work cast a shadow: menus, dialogs, the command palette, toasts.

Every proportion comes from the golden ratio (φ ≈ 1.618). Spacing steps along the Fibonacci
scale, padding pairs go 1 : φ from vertical to horizontal, type steps by a quarter of φ, and
dialogs are 377, 610 or 987px wide. Settings is a golden rectangle, 987 × 610. `ui/lib/scale.test.ts`
enforces the spacing and type scales in `npm test`, so they are rules, not guidelines.

**Key Characteristics:**
- Near-black neutrals in six tonal steps (the website's ground, surfaces and hairlines), a dark
  theme by default and a full light theme.
- One accent, Phosphor Amber, which the user can change. It is never decoration.
- Ten cable colours, one per kind of connection, used only to say "this is an SSH/RDP/… thing".
- JetBrains Mono for short text, the system face for long text and lists.
- The brand kit's horizontal logo on the start screen and in Settings › About; its own amber and
  white stay inside it.
- Fibonacci spacing (1 2 3 5 8 13 21 34px) and five type sizes, both enforced by a test.
- Rounded panels on a darker window. Flat at rest; shadows only on floating layers.
- Icon-only tool buttons with tooltips. Boxicons SVGs, animated only on session rows and tabs.
- Right-to-left ready: logical properties throughout, with flips for chevrons, menus and carets.

## Colors

Near-black neutral greys, one phosphor amber, and a set of connection hues that act as cable
colours.

### Primary
- **Phosphor Amber** (`gold`): the accent and the only one, the website's phosphor (`#ffb000`;
  `#754b00` on the light theme). The token keeps its earlier name, `--gold`. It marks focus
  (`:focus-visible` outline, field borders), toggles that are on and options that are picked
  (with `gold-soft` behind amber text), links that go to another place, drop targets, the focused
  pane's outline, the active Settings page's edge, the start screen's prompt sigil and block
  cursor, and fill on the single primary button of a dialog. Users can change it (Settings ›
  Appearance shows the default as "Amber (default)"): the presets are Brushed gold `#d8a84e` (the
  app's accent before the brand's), then Platinum `#c3ccd8`, Bronze `#a99a78` and Rose gold
  `#e2a395`, metals apart from every kind hue and the error red, each with a darker light-theme
  value (`ACCENTS` in `api.ts`); any other colour is darkened toward the text on the light theme,
  as the amber is. The app sets `--gold` as `light-dark(light, dark)` and derives `gold-ink` and
  `gold-soft` from it at runtime, so never hard-code its hex in a component; use `var(--gold)`.
- **Amber Ink** (`gold-ink`): text on a solid amber fill (the primary button): the ground,
  `#050506`, on dark; white on light.
- **Amber Wash** (`gold-soft`): the translucent tint (14%) behind picked rows, search hits, a
  setting a search points at, tags and drop zones.

The broadcast marker in a status bar reads the accent through `--broadcast`, an alias of `--gold`
(as `--fwd` and `--back` alias kind hues for tunnels).

### Secondary
The connection hues, one per kind of session (`--k-*`), keyed like cable colours:
- **Cobalt** (`k-ssh`): SSH. Also forward tunnels (`--fwd`).
- **Teal** (`k-shell`): local shells. Also reverse tunnels (`--back`).
- **Chartreuse** (`k-serial`): serial lines.
- **Violet** (`k-rdp`): RDP desktops.
- **Rose** (`k-browser`): websites.
- **Copper** (`k-telnet`): telnet.
- **Cyan** (`k-winrm`): WinRM.
- **Orchid** (`k-vnc`): VNC.
- **Bare Titanium** (`k-kvm`): BMC consoles, deliberately the least coloured.
- **Ember** (`k-agent`): AI-agent tabs. It shares its value with `error`.
- **Champagne** (`k-coop`): tabs shared with you in a Terminal Co-op ring.

They colour a session's icon (always), the 2px connector on top of its tab (unless the tab
carries a mark), and the tile behind a kind glyph (15% of the hue). Components read them through
a local `--kind`.

**Marks.** A person can mark a session, a folder or a tab with a colour (`COLORS` in `api.ts`:
nine swatches, no amber, plus any colour). A mark never takes a kind hue's place: it tints the
session's or folder's name (`.tinted`, its OKLCH lightness held to at least 0.72 on dark and at
most 0.48 on light, so every swatch reads at 4.8:1 or better while keeping its hue) and the tab's
connector line. A folder has no kind, so its icon wears its mark too. A folder's mark carries
down to every name inside it, under each session's own.

### Tertiary
Status, kept apart from the kind hues:
- **Live Green** (`ok`): connected (the status dot, the dot on a session's icon in the rack).
- **Amber** (`warn`): waiting, slow, or a dead tab's title; the border of the reconnect bar.
- **Signal Red** (`error`): disconnected, failures, the hover of a menu's
  destructive item (14% wash behind red text), and the fill of a danger button.
- **Red Ink** (`error-ink`): text on a solid red fill (a danger button).

### Neutral
From the back of the window to the hand, the website's near-black, with no blue in it:
- **Ground** (`window`): the window itself, showing in the gaps between panels and split panes.
- **Inset Well** (`deep`): wells set into a panel: text fields, `kbd`, segmented controls, the
  sidebar's status pill, code snippets. In the light theme it is white, the lightest step.
- **Base** (`bg`): the content area under the tabs, and the active tab that joins it.
- **Surface** (`panel`): the sidebar, the tab strip, the file panel, the status bar, and every
  floating layer (menus, dialogs, palette, toasts at 98% opacity, or the Opacity setting with the
  Transparent effect on, up to 98%).
- **Raised** (`raised`): button faces, menu-item hover, the current Settings page, scrollbar
  thumbs.
- **Lit** (`hover`): a button under the pointer.
- **Hairline** (`line`): every 1px border and divider.
- **Ink** (`fg`): primary text.
- **Muted** (`weak`): secondary text, form labels, idle tabs and tool buttons, counts, the
  start screen's prompt.
- **Mark** (`mark`): the logo's three large tiles and its wordmark, `#f2f2f2` on dark and the
  brand's charcoal `#141414` on light, as the brand kit draws them. Nothing else.

The light theme (`:root[data-theme="light"]` in `ui/app.css`) redefines every token above with
the same roles, in neutral greys with no blue tint and text in the brand's charcoal `#141414`.
The amber deepens to `#754b00` (white ink, a 14% wash), dark enough to hold 4.5:1 on every light
surface, a button under the pointer included, and the kind hues darken to hold contrast on the
light greys. The light values are also kept in `.impeccable/design.json`
(`colorMeta.*.light`).

### Named Rules
**The One Accent Rule.** Amber says "you are here" or "you picked this", and nothing else. Two
exceptions: the fill of a dialog's single primary button, and a folder's tag (`.tag`, `prod`,
`lab`…), which is a fact about where you are and shows on the folder's row and under a pane. No
amber headings, icons or decorative strokes. The start screen's prompt sigil and block cursor are
the website's: they say where you type.

**The Logo Colours Rule.** The logo's own amber, `#FFC20E`, and its white (`--mark`) live only in
the logo. The brand amber is not the accent and is never UI state, as the website's One Phosphor
Rule says; the accent is the phosphor `--gold`. `ui/lib/Logo.svelte` swaps the artwork's
`#F2F2F2` fills for `currentColor`, which is `--mark`, so one file serves both themes; the small
amber tile is never recoloured.

**The Cable Colour Rule.** A kind hue identifies a kind of connection and nothing else, and
nothing else takes its place: a session's icon is always in its kind's hue, whatever it is marked
with. State (connected, waiting, failed) is always `ok` / `warn` / `error`, never a kind hue.

**The See-Through Chrome Rule.** Chrome backgrounds are written
`rgba(var(--panel-rgb), var(--chrome-alpha))` (or the `window`/`bg`/`deep` triplet), never as a
hex. `--chrome-alpha` drops below 1 while the Transparent effect is on, and the desktop shows through.
Any place is two layers, the window's ground and a panel or terminal on it, and `--chrome-alpha` is
worked out so the two together make the Opacity setting, in every window: don't paint a third layer
between them. A floating layer is one layer at `--float-alpha`, over what's already there, as a window
is over another. The lock screen stays solid: it hides what's behind it.

## Typography

**Short Font:** JetBrains Mono Variable, the brand's face (with Cascadia Mono, Consolas), bundled
from `@fontsource-variable/jetbrains-mono`
**Long Font:** Segoe UI Variable Text (with Segoe UI, system-ui, -apple-system, Ubuntu)

**Character:** the website's split. JetBrains Mono, the face the logo's wordmark is drawn in
(Bold, as outlines in the artwork), says everything short: titles, buttons, tags, the pane's
status bar, the start screen's lead, and code and keys. The operating system's own face says
everything long: body text, prose, field labels, menus and lists, so the app still reads as a
native tool. `--short` names the short face (`var(--mono)`), `--ui` the long one. The terminal's
own font is the user's setting and is not part of this system.

### Hierarchy
- **Prompt** (mono 400, 13px `--t`, 1.4, in `weak`): the start screen's `~/xtermium $`, with the
  `$` at 500 in amber and a blinking amber block cursor after it; decorative, hidden from
  assistive tech. Nowhere else.
- **Headline** (mono 600, 16.5px `--t-xl`, 1.4, -0.025em, the website's title tracking): the
  lead sentence under the logo.
- **Title** (mono 600, 14.5px `--t-l`, 1.4, -0.01em): dialog and page titles (`h2`). Section
  heads inside a page (`h3`) are mono 600 at body size.
- **Short** (mono 400, 13px `--t`, 1.4): buttons.
- **Body** (400, 13px `--t`, 1.4, system face): everything else by default: fields, field
  labels, rows, menus.
- **Prose** (400, 13px, line-height φ = 21px): paragraphs and text areas, so a line of text is
  as tall as a list row. Measure is capped at 62–70ch.
- **Label** (400, 11.5px `--t-s`): counts, hints and secondary lines in the system face; the
  status bar, tags and `kbd` at the same size in the mono. Counts use `tabular-nums`.
- **Mono** (0.92em, JetBrains Mono): `code`, `kbd`, `pre`, and tunnel addresses (`.addr` in the
  tunnel drawing). It always runs left to right, even in Arabic.

`--t-xxl` (18.5px) is used twice: the status centred over an RDP desktop while it connects or
after it closes, and the sign-in code a person approves in the browser (in the mono, letters
0.1em apart). The logo is drawn, not set: it takes its size from its width (`min(377px, 100%)` on
the start screen, 233px in Settings › About).

### Named Rules
**The Five Sizes Rule.** Every `font-size` is a type token (`var(--t-s)` … `var(--t-xxl)`),
`inherit`, an `em`, or a `clamp()`. A raw `px` size fails `npm test`.

**The Golden Line Rule.** Prose and text areas set `line-height: var(--phi)`. Dense UI text
stays at 1.4.

**The Short-in-Mono Rule.** As on the website: if it is a title, a button, a tag, a key, a status
line or code, it is JetBrains Mono (`var(--short)`, or `var(--mono)` for code). If it is read as
text or down a list, it is the system face (`var(--ui)`). Lists are long even when each entry is
short, so these stay in the system face: menu items (`.menu button`), the rows of the session
rack and the palette (`button.item`), the Settings page list (`.pages button`) and the session
editor's kind picker (`.kinds button`, five to a row, where the narrower face keeps the names
whole). In Arabic, Hindi, Japanese and Chinese (`lang` ar, hi, ja, zh) `--short` falls back to
`--ui`, since JetBrains Mono has no letters for those scripts.

## Layout

The window is padded by the gap (`--gap`, 5px) and holds up to three floating panels in a row:
the session rack (resizable; it collapses to a 38px rail), the center (a 34px tab strip over
the panes), and the file panel (340px). The splitter between rack and center is the gap itself.
Split panes are each their own panel (5px corners, 1px hairline) with the window's dark showing
between them. Pane sizers are 6px hit areas centred on that gap, amber-tinted on hover.

Spacing is the Fibonacci scale only: 1 2 3 5 8 13 21 34px (55 and 89 are allowed but unused).
Padding pairs go 1 : φ, vertical to horizontal: `3px 8px` for controls, `5px 8px` for compact
blocks, `8px 13px` for cards and asides, `13px` for a dialog body. Heights come from three
tokens: `--row` 21px (list rows, status bar, small buttons), `--control` 26px (buttons, fields,
menu items), `--bar` 34px (the tab strip). Dialogs are 377, 610 or 987px wide and keep 21px from
the window's edge.

The start screen sits on the golden section: the room above the welcome and below it go 1 : φ,
in a column at most 610px wide, 13px between its parts. The welcome starts at the website's
prompt (`~/xtermium $` and a blinking amber block cursor), then the brand kit's horizontal logo
(the mark beside the "XTermium" wordmark, `assets/logo-horizontal.svg` through
`ui/lib/Logo.svelte`) at `min(377px, 100%)` wide, arriving with `float-in`, then the lead
sentence and the ways to start, and a `weak` line that says "Start typing to open a local
shell." Settings › About shows the same logo at 233px.

The prompt is real. A character typed there (outside any field, with no dialog, palette or menu
open) opens a local shell, and every key typed is held until the shell can take it, then handed
over in order, the first included (Settings › General › Sessions turns this off, and the line with
it). The tab shows in the strip at once, as the key's acknowledgement. The welcome leaves in
0.12s, accelerating away (ease-in), starting at the key with no delay. The terminal stays unseen
until the shell can take keys and the welcome has gone, then fades in over 0.2s on `--ease-out`:
one after the other, never a crossfade of the logo over the shell's text. Under reduced motion
it is a cut.

Forms are a two-column grid (labels in `weak`, minimum 120px, gaps `8px 13px`), and a label
beside a multi-line field aligns to its first baseline.

This is a desktop app with no breakpoints: things shrink, ellipsize and scroll rather than
reflow. Long names truncate with an ellipsis; Settings page names wrap to two lines instead.
Layout uses logical properties (`inset-inline`, `margin-inline-start`, `padding-inline-end`) so
right-to-left languages mirror for free.

### Named Rules
**The Fibonacci Rule.** Padding, margin, gap, inset and border-radius values are Fibonacci pixels.
A stray 4px or 12px fails `npm test`.

**The Derived Value Rule.** A value worked out from others is written as `calc(…)` so it reads as
derived: `calc(8px + 16px + 5px)` is a field's text start past its icon, and
`calc(var(--r-menu) - 3px)` is a menu item's corner.

## Elevation & Depth

Depth is tonal first. The window is the darkest tone and each layer nearer the hand is a step
lighter (window → bg → panel → raised → hover). Text fields are set *into* the surface as darker
wells (`deep`). Resting panels have a 1px hairline and no shadow. Shadows are reserved for
layers that float above the work and close again, and they are soft and dark, never coloured.
Hairline edges inside chrome (the tab strip's bottom, the current Settings page's amber edge,
drag-insertion bars) are drawn as inset `box-shadow` lines so they take no room.

### Shadow Vocabulary
Three tokens on `:root` in `ui/app.css`, from lowest to highest:
- **Float** (`--shadow-float`: `0 4px 14px rgba(0, 0, 0, 0.3)`): bars floating over a view
  (find, reconnect) and the hint over a detached window.
- **Menu** (`--shadow-menu`: `0 6px 20px rgba(0, 0, 0, 0.36)`): context menus, dropdown lists
  and toasts.
- **Dialog** (`--shadow-dialog`: `0 8px 28px rgba(0, 0, 0, 0.4)`): modal dialogs and the command
  palette.
- **Scrim**: `rgba(5, 5, 6, 0.4)` (the ground) behind a dialog, `0.32` behind the palette.

### Named Rules
**The Float-Only Shadow Rule.** A surface casts a shadow only if it floats over the work and goes
away again, and takes it from one of the three tokens. Panels, tabs, rows and buttons never do.

## Shapes

Three corners, all small: 3px (`--r-control`) for controls, fields, tags and panes; 5px
(`--r-menu`) for menus, toasts, the palette, split panes and cards; 8px (`--r-window`) for the
floating panels and dialogs. Status dots (7px), colour swatches (16–18px) and the dot on a live
session's icon are circles. Borders are a 1px hairline everywhere. Dashed hairlines mean "empty
or droppable" (an empty pane, a drop zone). A 2px amber border means "picked" (a colour scheme
card).

### Named Rules
**The Nested Curve Rule.** A rounded thing inside another keeps the same curve: its radius is
the outer one less the room between them, written as `calc()` (a menu item is
`calc(var(--r-menu) - 3px)`, a segmented button `calc(var(--r-control) - 1px)`).

## Components

### Buttons
Quiet and exact: small, flat faces that brighten a step under the pointer.
- **Shape:** gently squared (3px), 26px tall, `3px 8px` padding, 5px between icon and text; the
  label is in the mono (`--short`); menu items (`.menu button`) and list rows (`button.item`)
  keep the system face.
- **Default:** a `raised` face with no visible border (a transparent 1px keeps sizes equal when a
  state adds one). Hover moves to `hover`. Disabled is 45% opacity.
- **Primary:** amber fill with `gold-ink` text; hover mixes in 14% white. One per dialog, on its
  main action.
- **Danger:** the primary of a question whose yes ends or deletes something (close a running
  tab, delete a folder, replace everything from a backup, replace the MCP token): red
  fill with `error-ink` (`button.primary.danger`). Something that can be put back is not asked
  about at all: it happens, and a toast offers Undo.
- **On / selected:** amber wash, amber text, and a 55% amber border, for toggles and picked options.
- **Flat:** transparent at rest, `raised` on hover. Tool buttons in bars are flat, icon-only
  (`3px 5px`), in `weak`, turning amber when on, and always carry a tooltip.
- **Link:** text that names another place is a borderless amber `button.link` (underline on
  hover). A whole sentence that links is `.quiet`: body colour, dotted underline, amber on hover.
- **Focus:** a 1px amber outline drawn inside the edge (`outline-offset: -1px`).

### Inputs / Fields
- **Style:** an inset well (`deep`) with a 1px hairline, 3px corners, 26px tall, `3px 8px`
  padding. Selects use the app's own chevron, set in 8px from the edge.
- **Focus:** the border turns amber; no glow.
- **Placeholder:** an example, in `weak` (never the engine's default grey, which fails contrast
  on the wells). Labels stay outside the field.
- **With an icon:** search fields put a 16px icon at 8px in and start text at
  `calc(8px + 16px + 5px)`.
- **Checks, radios, ranges, progress:** native controls tinted with `accent-color: var(--gold)`.
- **Too long:** a value, placeholder or choice that doesn't fit ends in `…`; fields never widen
  their dialog. In a `.form` the labels take what the longest needs, from 120 to 233px, and end
  in `…` past that; the fields take the rest. Boxes of text don't resize.

### Menus
- **Style:** panel at 98%, hairline, 5px corners, 3px padding, at least 190px wide, Menu shadow.
- **Items:** 26px, `3px 8px`, transparent; `raised` on hover. In a menu with icons, items
  without one are indented to line up with the others' text.
- **Groups:** `.sep` starts a group with a hairline 8px in from each side.
- **Destructive:** comes last, as `.danger`, and turns red on a 14% red wash when pointed at.
- Keyboard-driven, kept on screen, and opening leftwards in right-to-left languages.
- Menus open instantly, with no arrival motion: they follow the pointer and the keyboard, and
  any delay there reads as lag.

### Navigation
- **Tab strip:** a 34px panel bar. Tabs are flat, split by hairlines, `weak` when idle. The
  active tab takes the content's `bg` and drops its bottom line so it joins its pane. A 2px
  connector sits on top of each tab, in its mark (the tab's own, else its session's or folder's)
  or else its kind hue: hidden when idle, at 50% when the tab is visible in a split, and full on
  the active one. The tab's icon is always in its kind hue. A drag shows a 2px amber insertion bar.
- **Session rack:** 21px rows, folders with a rotating caret (0.12s) and a hairline guide under
  each level. Leaves indent past the caret so icons in one level line up. A session with an open
  tab gets a 7px green dot on its icon's corner, hollow once the connection is gone. Search hits
  and chosen rows take the amber wash. A marked session's or folder's name is in its mark. Its
  search opens out of the magnifier; Enter closes it and puts the keyboard on the hit's row in
  the tree, its folders opened (a second Enter opens the session); Escape gives the tree back the
  keyboard.
- **Sidebar foot:** the status pill (an inset well, label size in `weak`, fading in over 0.16s
  when a message arrives), then Settings and
  the account, in that order, at the foot's trailing end. The account is a flat icon button; signed
  in, it shows an 18px round avatar with the first letter of the name, ringed 2px in `error` when
  sync fails or the quota is full and in `warn` when it is nearly full. Its menu opens upwards,
  led by a `.head` block (name, address, last sync, the quota bar) under a hairline.
- **Focus ring:** one 2px amber ring around the focused pane, outside it in the gap (5px corners
  in a split). When the focus moves it glides to the next pane (0.2s, `--ease-out`; Static in
  Settings › Appearance makes it jump), so you see which way it went. While a pane edge is
  dragged it follows the edge with no delay.
- **Settings pages:** a 183px column. The current page is `raised` with a 2px amber edge on its
  leading side. Its search replaces the pages with hits; while it's open the page follows the
  best hit and lights that setting (`.lit`: its label and field in the amber wash, ringed 3px in
  it), and the hit is lit in the list too. Enter closes the search and puts the keyboard on the
  setting's control, whose light then fades over 2.6s (`.found`). Pages: General, Appearance,
  Shortcuts, Security (encryption, recovery passphrase, the lock), Sync, MCP (sub-sections "AI
  assistants (MCP)" and "Command line", whose commands and configuration sit in `.snippet`
  wells: `deep`, `8px 13px`, 3px corners, in the mono) and About.
- **Status bar:** a 21px panel strip under a tab's view, label size in the mono, in `weak`, items
  13px apart.
  It leads with where the keyboard goes: `user@host`, then the session's folder with the folder's
  tags as amber chips (`Production/Web · prod`), so a production box says so under its pane. Then a
  state dot (green / amber / red / grey) and, at the end, the amber broadcast marker.
- **Tab-strip tools** come as the tabs do: Saved layouts always (it opens sessions), File
  Browser and Snippets once a tab is open, Broadcast and the layout segment with two or more.

### Dialogs and the Command Palette
- **Dialog:** a panel at 98% with 8px corners, a hairline header (`8px 8px 8px 13px`), and a 13px
  body with 8px between blocks; Dialog shadow over a 40% scrim. It holds the keyboard: Tab stays
  inside and Escape closes the top one. Widths: 377 / 610 / 987. A dialog whose body can run
  long puts its buttons in `foot`, a fixed footer under a hairline (`8px 13px`), so Save never
  scrolls out of reach. A dialog whose fields come and go while it's used (the session editor)
  is `top`: it hangs at 12vh, where the palette does, and grows downwards instead of
  re-centring, so what's under the pointer stays put. Its keyboard lands on the first field a
  person can see, never one in a closed `<details>` or a shortcut that fills the others in
  (`data-not-first`, as "Copy the server from"), else on the dialog itself. Dialogs and menus
  draw over everything wherever they're opened from (they move to the end of the page while open).
- **Sign in:** a 377 dialog. Sync needs an account and nothing else does; the browser opens on
  the sign-in page (Google, GitHub or Discord) and the dialog shows the code to approve there,
  centred, in the mono at `--t-xxl`, with a flat copy button beside it and an amber spinner while
  it waits. Cancel is flat; after a failure, Try again is the primary.
- **Lock screen:** not a dialog but a whole opaque window (`rgb(var(--window-rgb))`, over
  everything, dialogs included) that fades in over 0.16s. A centred 377px column, 13px between
  parts: a 34px lock icon in amber, a 600 title at `--t-xl`, a password or PIN well with the
  primary Unlock beside it (Password / PIN switch as flat buttons, the picked one amber), then
  passkey and Windows Hello as plain buttons, and "Your sessions keep running." in `weak`.
- **Session editor:** 610 wide whatever it holds. The types in two even rows of five (a name
  that doesn't fit ends in `…`, the whole one in its tooltip), then where it connects (one
  "Host / port" row for every type that has a host), then the login, then Name and Folder; Advanced and "Icon, colour and notes"
  closed unless used; Save in the fixed footer.
- **Arrival:** dialogs, the palette, the bars over a terminal (find, reconnect) and toasts share
  one entrance, `float-in` in `ui/app.css`: a fade from 5px short of where the layer hangs,
  in 0.16s on `--ease-out`. The offset `--from` is 5px below by default and -5px for things
  hanging from the top (the palette, the find bar). The scrim behind a dialog fades in
  (`fade-in`) over the same 0.16s.
- **Palette:** 610px wide, 12vh from the top, at most 1/φ of the window tall, with 5px corners.
  A search well over 26px rows; the picked row takes the amber wash, with its shortcut in a `kbd`
  at the end of the row. Empty, it lists the open tabs, then the saved sessions, then the actions:
  a machine before a verb. A typed word has to sit in one stretch of a row's text (an abbreviation
  like `prd` for `prod`), never letters scattered across it.

### Tags, Keys and Toasts
- **Tag:** amber text on the amber wash, 3px corners, `0 5px`, label size, at most 80px wide.
- **Key cap (`kbd`):** an inset well with a hairline and a 2px bottom border so it reads as a
  key, in the mono at label size.
- **Toast:** panel, hairline, 5px corners, `5px 13px`, Menu shadow, 34px above the bottom edge.
  It arrives with `float-in` and leaves faster than it came, fading out in 0.1s. One with an
  action (Undo after a delete, Reopen after Close other tabs) carries an amber link button,
  takes the pointer, and stays 8s instead of 2.4s.
- **Tooltip (`.tooltip`, `ui/lib/tooltip.ts`):** a tool button's title in the app's own panel at
  98%, hairline, 5px corners, `3px 8px`, label size on golden lines, Menu shadow, at most 377px
  wide; its shortcut follows in a `kbd` in `weak`. It hangs under what it explains (above when
  there is no room) and takes no pointer.
- **Quota bar (`Quota.svelte`):** sync storage used, in the account menu and Settings › Sync. A
  3px track in `line` with 2px corners, filled in amber; the fill turns `warn` when nearly full
  and `error` when full, and a label-size line under it says how much. Not a native `progress`:
  the webview paints that flat grey whatever the accent.

### Kind Glyph (signature)
A 21px tile with 3px corners, the kind's hue as the icon colour on 15% of the same hue (`.glyph`
in `ui/app.css`). It sits beside each way to start on the start screen and makes a connection
type recognisable before you read its name. The session editor's Type picker is a row of them:
one button per kind, glyph and name, the picked one in the amber wash. The type comes first in
the editor because it decides every field below it; changing it keeps the host typed so far.

### Broadcast Outline (signature)
Every terminal that receives broadcast keystrokes gets a 2px inset outline in a chosen colour
(`--oc`), drawn inside the pane at 3px corners. Its style is the user's (steady, breathe, fade,
ring, march, rainbow, off). While broadcast is on, the focus ring around the pane you type in
turns to a 2px dashed ring in the text colour (`fg`) and steps 2px further out, so it never reads
as one band with an outline in the same colour. All animated styles stop under reduced motion, and
the preview in Settings › Appearance holds still while it is scrolled out of view.

### Co-op Ring and Pointers
A tab shared in a Terminal Co-op ring wears a small ring (8px, 2px stroke in `ok`) after its name;
a tab shared with you is a `k-coop` tab. The ring's dialog is a 377px `Modal`: the passcode in
`--t-xxl` mono, people as 21px round pictures (the first letter when there is none), and "End the
ring" last, in `error`. Over a shared or remote pane, each other person's pointer is an 8px ring in
a mark colour (`COLORS`, held per person) with their 13px picture beside it, the name on hover. It
moves in 80ms and stands still under reduced motion.

### Backgrounds
A picture or a GLSL shader behind the terminals, the sidebar or the tab bar (each its own),
drawn under the place's own colour at the strength set (a range of 5–95% in steps of 5,
`Backdrop.svelte`). A picture
fills its place by default, cut to fit; it can also stretch, fit whole, tile at its size, or sit
centred at its size (`FITS`). A shader always fills. The place isolates its stacking, so the
backdrop stays under everything in it.

### Session Icons
Boxicons SVGs through `Icon.svelte`, 16–18px. Session and folder icons (`SessionIcon.svelte`)
have small moving parts (blink, flow, spin, bob…) that play only while the pointer is over that
session's row or tab, never on the interface's own buttons, and never under reduced motion.

## Do's and Don'ts

### Do:
- **Do** use only Fibonacci pixels for spacing and radii (1 2 3 5 8 13 21 34), with padding in
  1 : φ pairs (`3px 8px`, `5px 8px`, `8px 13px`).
- **Do** set every text size from the type tokens (`--t-s --t --t-l --t-xl --t-xxl`).
- **Do** size things with `--row` (21px), `--control` (26px) and `--bar` (34px), and write any
  derived value as `calc(…)`.
- **Do** keep amber for focus, selection, toggles that are on, links, drop targets and the one
  primary action. Read it as `var(--gold)`, since the user can change it.
- **Do** colour a session by its kind hue (icon, tab connector, glyph tile) through `--kind`.
- **Do** paint chrome as `rgba(var(--…-rgb), var(--chrome-alpha))` so the Transparent effect shows through.
- **Do** end a menu with its destructive item (`.danger`) and start groups with `.sep`.
- **Do** offer Undo instead of a question when the thing can be put back, and ask (with a
  danger button) only when it can't: a running connection, a folder of sessions, a restore.
- **Do** focus the safe answer when yes can't be undone or is a security call (Reject on a new
  host key, Cancel on a delete or restore, Keep editing on discard), so Enter never does the
  irreversible thing; an ordinary yes (Close tab, which Reopen can undo) keeps the focus.
- **Do** make tool buttons icon-only with a tooltip.
- **Do** keep motion to state and continuity: 0.1–0.2s, arrivals on `--ease-out`, a new floating
  layer through `float-in`, exits faster than entrances. When one view replaces another (the
  start screen giving way to the shell typed into it), the old one leaves first and the new one
  arrives after it. CSS motion stops under `prefers-reduced-motion` through `ui/app.css`; a
  Svelte transition must check it itself.
- **Do** use logical properties (`inset-inline-*`, `margin-inline-*`) so right-to-left mirrors.
- **Do** set short text (titles, buttons, tags, status lines) in `var(--short)` and anything read
  as text or down a list (menus, rows, the Settings pages, the kind picker) in `var(--ui)`.
- **Do** show the logo through `Logo.svelte`, so its tiles and wordmark follow the theme through
  `--mark`.

### Don't:
- **Don't** use off-scale spacing (4px, 6px, 10px, 12px, 16px) or a raw `font-size` in px; the
  scale test fails on them.
- **Don't** put a shadow on a resting panel, tab, row or button, or write a new shadow value for
  a floating layer: use `--shadow-float`, `--shadow-menu` or `--shadow-dialog`.
- **Don't** use amber for headings, icons, illustration or large fills other than the primary
  button.
- **Don't** use a kind hue to show state, or a status colour to mean a kind of connection.
- **Don't** let a user's colour recolour a session's icon: marks go on names (`.tinted`) and tab
  lines, and accent presets stay off the kind hues and the error red.
- **Don't** hard-code the accent's hex, or the chrome as an opaque hex.
- **Don't** animate the interface's own icons. Only session and folder icons move, and only on
  hover.
- **Don't** add big padding or wide gaps. The densest comfortable layout is the house style.
- **Don't** set a list, a menu or a five-to-a-row grid in the mono: the names stop fitting and it
  no longer reads down.
- **Don't** use the logo's amber `#FFC20E` or `--mark` outside the logo, or recolour, stretch,
  rotate or add effects to the logo.

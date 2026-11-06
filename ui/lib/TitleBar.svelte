<script lang="ts">
  // The window's title bar, drawn by xtermium so it looks the same on every system (src/main.rs
  // `own_bar` takes the system's away): the mark, in the centre what the window is about (the
  // main window's focused tab, a detached window's name), room to drag the window by (Tauri's
  // `data-tauri-drag-region`: a double click maximizes), and the three window buttons.
  // macOS keeps its own three buttons, laid over the bar's left end, as it does in VS Code.
  // Everything else starts under the bar (`--titlebar` in app.css), dialogs and the lock included,
  // so the window can always be moved, minimized and closed.
  import type { Snippet } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { OS } from "./actions";
  import { _ } from "./i18n.svelte";
  import Logo from "./Logo.svelte";

  /** `title`: the window's name, shown in the centre unless `children` draw the centre themselves.
   *  `tools`: buttons beside the mark (the main window's local shell and tunnels). */
  let { title = "", children, tools }: { title?: string; children?: Snippet; tools?: Snippet } = $props();

  const w = getCurrentWindow();
  let max = $state(false);
  let full = $state(false);
  /** Another window has the keyboard: the bar steps back, as a system's own does. */
  let away = $state(false);
  const look = async () => ([max, full] = await Promise.all([w.isMaximized(), w.isFullscreen()]));
  look();
  w.onResized(look);
  // A window can open without the keyboard (started from another program), so it is asked first.
  w.isFocused().then((f) => (away = !f));
  w.onFocusChanged((e) => (away = !e.payload));
  // Full screen has no bar: the page gets its room back.
  $effect(() => void document.documentElement.classList.toggle("fullscreen", full));

  /** A window button keeps the keyboard where it was (in the terminal). */
  const keep = (e: MouseEvent) => e.preventDefault();
</script>

{#if !full}
  <header class="titlebar" class:mac={OS === "macos"} class:away data-tauri-drag-region="deep">
    <div class="start">
      {#if OS !== "macos"}<div class="mark"><Logo mark /></div>{/if}
      {@render tools?.()}
    </div>
    <div class="centre">
      {#if children}{@render children()}{:else if title}<span class="name">{title}</span>{/if}
    </div>
    <div class="end">
      {#if OS !== "macos"}
        <button tabindex="-1" title={_("Minimize")} aria-label={_("Minimize")} onmousedown={keep} onclick={() => w.minimize()}>
          <svg viewBox="0 0 10 10" aria-hidden="true"><path d="M0 5.5h10" /></svg>
        </button>
        <button tabindex="-1" title={max ? _("Restore down") : _("Maximize")} aria-label={max ? _("Restore down") : _("Maximize")} onmousedown={keep} onclick={() => w.toggleMaximize()}>
          <svg viewBox="0 0 10 10" aria-hidden="true"><path d={max ? "M2.5 2.5v-2h7v7h-2M.5 2.5h7v7h-7z" : "M.5 .5h9v9h-9z"} /></svg>
        </button>
        <button class="close" tabindex="-1" title={_("Close")} aria-label={_("Close")} onmousedown={keep} onclick={() => w.close()}>
          <svg viewBox="0 0 10 10" aria-hidden="true"><path d="M0 0l10 10M10 0L0 10" /></svg>
        </button>
      {/if}
    </div>
  </header>
{/if}

<style>
  /* On the window's ground, with no panel of its own: the panels float under it. Three parts,
     the middle one centred on the window whatever stands at the ends. */
  .titlebar {
    position: fixed;
    top: 0;
    inset-inline: 0;
    height: var(--titlebar);
    display: grid;
    grid-template-columns: 1fr minmax(0, auto) 1fr;
    align-items: center;
    color: var(--weak);
  }
  /* The mark stands over the sidebar's first column of icons. */
  .start {
    display: flex;
    align-items: center;
    gap: 8px;
    padding-inline-start: 21px;
  }
  /* macOS's own buttons sit on the left, whichever way the text runs. */
  .mac .start {
    padding-left: calc(55px + 21px);
  }
  /* An even size in an even bar: its edges land on whole pixels. */
  .mark {
    width: 22px;
  }
  .centre {
    min-width: 0;
    display: flex;
    justify-content: center;
    padding: 0 13px;
  }
  /* The same line the centre's button sets in the main window (App.svelte `.where`). */
  .name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--fg);
    font-family: var(--short);
    font-weight: 500;
  }
  .end {
    display: flex;
    justify-content: flex-end;
    height: 100%;
  }
  /* Windows' own caption buttons: 46px wide, square, the glyphs hairlines. */
  .end button {
    width: 46px;
    height: 100%;
    min-height: 0;
    display: grid;
    place-items: center;
    padding: 0;
    border: 0;
    border-radius: 0;
    background: none;
    color: inherit;
  }
  .end button:hover {
    background: var(--hover);
    color: var(--fg);
  }
  .end button:active {
    background: var(--raised);
  }
  .end .close:hover {
    background: var(--error);
    color: var(--error-ink);
  }
  .end .close:active {
    background: color-mix(in srgb, var(--error) 86%, black);
  }
  svg {
    width: 10px;
    height: 10px;
    fill: none;
    stroke: currentColor;
    stroke-width: 1;
  }
  .start,
  .centre,
  svg {
    transition: opacity 0.15s;
  }
  .away :is(.start, .centre, .end button:not(:hover) svg) {
    opacity: 0.5;
  }
</style>

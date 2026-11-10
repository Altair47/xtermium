<script lang="ts" module>
  import { _ } from "./i18n.svelte";
  import { app, lock, shortcut } from "./state.svelte";

  /** The getting-started steps, most important first. Each is done when the state it leads to is
   *  there (a saved session, a tunnel, a lock, an account), or else once it was done
   *  (`guideDone` records those: the shell, the palette, panes, broadcast). */
  export const STEPS = ["shell", "session", "palette", "split", "tunnel", "broadcast", "lock", "sync"] as const;
  export type Step = (typeof STEPS)[number];
  /** A step as the list shows it. `known`: whether `done` can be told yet (the lock's state
   *  arrives a moment after the window opens), so a tick that only arrived late isn't news. */
  export type GuideRow = { id: Step; title: string; hint: string; done: boolean; known: boolean; keys: string };

  /** The steps and where each stands, read where it's called (in a `$derived`, it follows them).
   *  App.svelte reads it too: for the sidebar's progress button and the toast a new tick brings. */
  export function guideRows(): GuideRow[] {
    const s = app.settings!;
    const did = (id: Step) => s.guide_done.includes(id);
    const l = lock.info;
    return [
      { id: "shell", title: _("Open a local shell"), hint: _("A terminal on this computer."), done: did("shell"), known: true, keys: shortcut("local-shell") },
      { id: "session", title: _("Save your first server"), hint: _("SSH, RDP, VNC, serial, telnet or a website, kept in the sidebar."), done: app.store.session.length > 0, known: true, keys: shortcut("new-session") },
      { id: "palette", title: _("Find anything from the keyboard"), hint: _("The command palette opens sessions, runs actions and connects to user@host."), done: did("palette"), known: true, keys: shortcut("palette") },
      { id: "split", title: _("Work side by side"), hint: _("Put tabs in panes with the layout buttons at the title bar's end."), done: did("split"), known: true, keys: "" },
      { id: "tunnel", title: _("Forward a port over SSH"), hint: _("Tunnels reach a database or a web page behind a server."), done: app.store.tunnel.length > 0, known: true, keys: shortcut("tunnels") },
      { id: "broadcast", title: _("Type into many servers at once"), hint: _("Broadcast sends your keys to every ticked tab."), done: did("broadcast"), known: true, keys: shortcut("toggle-broadcast") },
      { id: "lock", title: _("Lock xtermium"), hint: _("A password, PIN or passkey before your sessions show."), done: !!l && (l.password || l.pin || !!l.passkey || l.biometrics), known: !!l, keys: "" },
      { id: "sync", title: _("Keep your devices in step"), hint: _("Sign in to sync sessions, end-to-end encrypted."), done: !!s.sync_token, known: true, keys: "" },
    ];
  }
</script>

<script lang="ts">
  // Getting started: a short list for someone new, on the start screen in place of its four start
  // buttons, and with tabs open in a popover from the sidebar's foot (App.svelte). Each row does
  // what it says, and ticks itself once that's done. Only the step to do next says more (its hint
  // and keys); the others are one line each, their hint in the row's tooltip. Hide puts it away
  // (with Undo); Settings › General brings it back.
  import Icon from "./Icon.svelte";
  import { toast } from "./state.svelte";

  /** `inset`: inside a popover, which draws the edge and background itself. */
  let { onstep, onimport, inset = false }: { onstep: (s: Step) => void; onimport: () => void; inset?: boolean } = $props();

  const s = $derived(app.settings!);
  const rows = $derived(guideRows());
  const count = $derived(rows.filter((r) => r.done).length);
  /** The first step not done yet: the one to do next stands out. */
  const next = $derived(rows.find((r) => !r.done)?.id);

  function hide() {
    s.guide = false;
    toast(_("Getting started is hidden. Settings › General brings it back."), { label: _("Undo"), run: () => (s.guide = true) });
  }
</script>

<section class="guide" class:inset aria-label={_("Getting started")}>
  <header>
    <h2>{_("Getting started")}</h2>
    <span class="progress">{count === rows.length ? _("All done") : _("{n} of {total} done", { n: count, total: rows.length })}</span>
    <button class="flat hide" onclick={hide}>{_("Hide")}</button>
  </header>
  <ol>
    {#each rows as r, i (r.id)}
      <li>
        <button
          class="flat step"
          class:done={r.done}
          class:next={r.id === next}
          aria-current={r.id === next ? "step" : undefined}
          title={r.id === next ? null : r.hint}
          onclick={() => onstep(r.id)}
        >
          <span class="mark" aria-hidden="true">{#if r.done}<Icon name="check" size={16} />{:else}{i + 1}{/if}</span>
          <span class="text">
            <span class="title">{r.title}<span class="sr">{r.done ? _(" (done)") : ""}</span></span>
            <span class="hint">{r.hint}</span>
          </span>
          {#if r.keys}<kbd>{r.keys}</kbd>{/if}
        </button>
        {#if r.id === "session" && r.id === next}
          <button class="link quiet also" onclick={onimport}>{_("Or import them from MobaXterm or ~/.ssh/config")}</button>
        {/if}
      </li>
    {/each}
  </ol>
</section>

<style>
  .guide {
    display: flex;
    flex-direction: column;
    border: 1px solid var(--line);
    border-radius: var(--r-menu);
    background: rgba(var(--panel-rgb), var(--chrome-alpha));
  }
  /* in a popover, which has its own edge and background */
  .guide.inset {
    border: 0;
    border-radius: 0;
    background: none;
  }
  header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 5px;
    padding-inline-start: 13px;
    border-bottom: 1px solid var(--line);
  }
  h2 {
    margin: 0;
    font-family: var(--short);
    font-size: var(--t-l);
    font-weight: 600;
    letter-spacing: -0.01em;
  }
  .inset h2 {
    font-size: var(--t);
  }
  .progress {
    color: var(--weak);
    font-family: var(--short);
    font-size: var(--t-s);
    font-variant-numeric: tabular-nums;
  }
  .hide {
    margin-inline-start: auto;
    color: var(--weak);
  }
  ol {
    margin: 0;
    padding: 3px;
    list-style: none;
    display: flex;
    flex-direction: column;
  }
  li {
    display: flex;
    flex-direction: column;
  }
  /* a row: the step's number (a tick once done), what to do, its keys; one line, but the step to
     do next also says why, so the eye has one place to go. The words wrap (a button keeps them on
     one line), so a narrow window never puts them under the keys. */
  .step {
    display: grid;
    grid-template-columns: var(--row) minmax(0, 1fr) auto;
    white-space: normal;
    align-items: center;
    gap: 8px 13px;
    padding: 3px 8px;
    border: 0;
    border-radius: calc(var(--r-menu) - 3px);
    text-align: start;
    color: var(--fg);
  }
  .step.next {
    padding-block: 5px;
    background: var(--hover);
  }
  /* the number, plain; only the next step's has a ring */
  .mark {
    width: var(--row);
    height: var(--row);
    display: grid;
    place-items: center;
    border: 1px solid transparent;
    border-radius: 50%;
    font-family: var(--short);
    font-size: var(--t-s);
    color: var(--weak);
    font-variant-numeric: tabular-nums;
  }
  .next .mark {
    border-color: var(--fg);
    color: var(--fg);
    font-weight: 600;
  }
  .done .mark {
    background: color-mix(in srgb, var(--ok) 18%, transparent);
    color: var(--ok);
  }
  .text {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-width: 0;
  }
  .title {
    font-family: var(--short);
    font-weight: 500;
  }
  .hint {
    font-family: var(--ui);
    font-size: var(--t-s);
    color: var(--weak);
    line-height: 1.4;
  }
  /* only the next step shows its hint; the others keep it for screen readers (and the tooltip) */
  .step:not(.next) .hint,
  .sr {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
  }
  /* done: still there to do again, but stepped back */
  .done .title {
    color: var(--weak);
    font-weight: 400;
  }
  /* keys as quiet text, not key caps: eight caps down the side were the loudest thing here */
  kbd {
    padding: 0;
    border: 0;
    background: none;
    box-shadow: none;
    font-size: var(--t-s);
    color: var(--weak);
  }
  .next kbd {
    color: var(--fg);
  }
  .done kbd {
    opacity: var(--dim);
  }
  /* the import link under its step, lined up with the step's text */
  .also {
    align-self: flex-start;
    min-height: 0; /* in the popover, a menu's items are taller */
    padding: 0;
    margin-block: 3px;
    margin-inline-start: calc(8px + var(--row) + 13px);
    font-size: var(--t-s);
  }
</style>

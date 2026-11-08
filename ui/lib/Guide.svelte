<script lang="ts" module>
  /** The getting-started steps, most important first. Each is done when the state it leads to is
   *  there (a saved session, a tunnel, a lock, an account), or else once it was done
   *  (`guideDone` records those: the shell, the palette, panes, broadcast). */
  export const STEPS = ["shell", "session", "palette", "split", "tunnel", "broadcast", "lock", "sync"] as const;
  export type Step = (typeof STEPS)[number];
</script>

<script lang="ts">
  // Getting started: a short list on the start screen for someone new, in place of its four
  // start buttons. Each row does what it says, and ticks itself once that's done. Hide puts it
  // away (with Undo); Settings › General brings it back.
  import { _ } from "./i18n.svelte";
  import Icon from "./Icon.svelte";
  import { app, lock, shortcut, toast } from "./state.svelte";

  let { onstep, onimport }: { onstep: (s: Step) => void; onimport: () => void } = $props();

  const s = $derived(app.settings!);
  const keys = (action: string) => shortcut(action);
  const rows = $derived.by(() => {
    const did = (id: Step) => s.guide_done.includes(id);
    const l = lock.info;
    return [
      { id: "shell", title: _("Open a local shell"), hint: _("A terminal on this computer. A command typed on this screen runs in one."), done: did("shell"), keys: keys("local-shell") },
      { id: "session", title: _("Save your first server"), hint: _("SSH, RDP, VNC, serial, telnet or a website, kept in the sidebar."), done: app.store.session.length > 0, keys: keys("new-session") },
      { id: "palette", title: _("Find anything from the keyboard"), hint: _("The command palette opens sessions, runs actions and connects to user@host."), done: did("palette"), keys: keys("palette") },
      { id: "split", title: _("Work side by side"), hint: _("Put tabs in panes with the layout buttons at the title bar's end."), done: did("split"), keys: "" },
      { id: "tunnel", title: _("Forward a port over SSH"), hint: _("Tunnels reach a database or a web page behind a server."), done: app.store.tunnel.length > 0, keys: keys("tunnels") },
      { id: "broadcast", title: _("Type into many servers at once"), hint: _("Broadcast sends your keys to every ticked tab."), done: did("broadcast"), keys: keys("toggle-broadcast") },
      { id: "lock", title: _("Lock xtermium"), hint: _("A password, PIN or passkey before your sessions show."), done: !!l && (l.password || l.pin || !!l.passkey || l.biometrics), keys: "" },
      { id: "sync", title: _("Keep your devices in step"), hint: _("Sign in to sync sessions, end-to-end encrypted."), done: !!s.sync_token, keys: "" },
    ] satisfies { id: Step; title: string; hint: string; done: boolean; keys: string }[];
  });
  const count = $derived(rows.filter((r) => r.done).length);
  /** The first step not done yet: the one to do next stands out. */
  const next = $derived(rows.find((r) => !r.done)?.id);

  function hide() {
    s.guide = false;
    toast(_("Getting started is hidden. Settings › General brings it back."), { label: _("Undo"), run: () => (s.guide = true) });
  }
</script>

<section class="guide" aria-labelledby="guide-title">
  <header>
    <h2 id="guide-title">{_("Getting started")}</h2>
    <span class="progress">{count === rows.length ? _("All done") : _("{n} of {total} done", { n: count, total: rows.length })}</span>
    <button class="flat hide" onclick={hide}>{_("Hide")}</button>
  </header>
  <ol>
    {#each rows as r, i (r.id)}
      <li>
        <button class="flat step" class:done={r.done} class:next={r.id === next} aria-current={r.id === next ? "step" : undefined} onclick={() => onstep(r.id)}>
          <span class="mark" aria-hidden="true">{#if r.done}<Icon name="check" size={16} />{:else}{i + 1}{/if}</span>
          <span class="text">
            <span class="title">{r.title}<span class="sr">{r.done ? _(" (done)") : ""}</span></span>
            <span class="hint">{r.hint}</span>
          </span>
          {#if r.keys}<kbd>{r.keys}</kbd>{/if}
        </button>
        {#if r.id === "session" && !r.done}
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
    gap: 1px;
  }
  li {
    display: flex;
    flex-direction: column;
  }
  /* a row: the step's number (a tick once done), what to do and why, its keys; the words wrap
     (a button keeps them on one line), so a narrow window never puts them under the keys */
  .step {
    display: grid;
    grid-template-columns: var(--row) minmax(0, 1fr) auto;
    white-space: normal;
    align-items: center;
    gap: 8px 13px;
    padding: 5px 8px;
    border: 0;
    border-radius: calc(var(--r-menu) - 3px);
    text-align: start;
    color: var(--fg);
  }
  .mark {
    width: var(--row);
    height: var(--row);
    display: grid;
    place-items: center;
    border: 1px solid var(--line);
    border-radius: 50%;
    font-family: var(--short);
    font-size: var(--t-s);
    color: var(--weak);
    font-variant-numeric: tabular-nums;
  }
  /* the step to do next: its number in the text colour, the others step back */
  .next .mark {
    border-color: var(--fg);
    color: var(--fg);
    font-weight: 600;
  }
  .done .mark {
    border-color: transparent;
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
  /* done: still there to do again, but stepped back */
  .done .title {
    color: var(--weak);
    font-weight: 400;
  }
  kbd {
    font-size: var(--t-s);
  }
  .done kbd {
    opacity: var(--dim);
  }
  /* the import link under its step, lined up with the step's text */
  .also {
    align-self: flex-start;
    margin-block: 0 3px;
    margin-inline-start: calc(8px + var(--row) + 13px);
    font-size: var(--t-s);
  }
  .sr {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
  }
</style>

<script lang="ts">
  // Command palette: actions, saved sessions, open tabs and tunnels, fuzzy-searched. An address
  // ("me@host:2222", "rdp 192.0.2.19") offers to connect to it directly. One of the filters above
  // the list (Sessions, Snippets, Hosts) narrows the search to its own items.
  import { untrack } from "svelte";
  import { _ } from "./i18n.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { ACTIONS, type Filter, find, isWheel, keysOf, quickConnect, shown as keysShown } from "./actions";
  import { addressSession, host, KIND_COLOR, KIND_ICON, type SavedLayout, type Session, type Snippet, type TunnelStats } from "./api";
  import Icon from "./Icon.svelte";
  import SessionIcon from "./SessionIcon.svelte";
  import { app, inside, lookOf } from "./state.svelte";

  let {
    tabs,
    onaction,
    onsnippet,
    onlayout,
    onopen,
    onshow,
    onclose,
    start = null,
  }: {
    tabs: { key: number; title: string; session: Session }[];
    onaction: (id: string) => void;
    onsnippet: (s: Snippet) => void;
    onlayout: (l: SavedLayout) => void;
    onopen: (s: Session) => void;
    onshow: (key: number) => void;
    onclose: () => void;
    /** The filter picked as it opens (Sessions, to find one). */
    start?: Filter | null;
  } = $props();

  /** `in`: the filters that keep the item; an action is in none, so only the unfiltered list has it. */
  type Item = { label: string; hint: string; tags?: string; icon: string; color: string; keys?: string; in?: Filter[]; run: () => void };

  let query = $state("");
  let sel = $state(0);
  let filter = $state<Filter | null>(untrack(() => start));
  const FILTERS: [Filter, string][] = [
    ["sessions", _("Sessions")],
    ["snippets", _("Snippets")],
    ["hosts", _("Hosts")],
  ];
  let list: HTMLDivElement;
  let live = $state<Record<string, TunnelStats>>({});
  invoke<Record<string, TunnelStats>>("tunnel_stats").then((s) => (live = s));

  function tunnel(name: string, on: boolean) {
    invoke(on ? "tunnel_stop" : "tunnel_start", { name }).catch((e) => (app.status = String(e)));
  }

  const items = $derived.by((): Item[] => {
    const shortcuts = app.settings?.shortcuts ?? {};
    return [
      ...tabs.map((t) => ({ label: t.title, hint: _("Open tab"), ...lookOf(t.session), in: ["sessions"] as Filter[], run: () => onshow(t.key) })),
      // the sessions before the actions: the keyboard's front door is reached for a machine first
      ...app.store.session.map((s) => ({
        label: s.folder ? `${s.folder}/${s.name}` : s.name,
        hint: host(s.kind),
        // a folder's tags find every session in it and below
        tags: app.store.folder.filter((f) => f.tags.length && inside(s.folder, f.path)).flatMap((f) => f.tags).join(" "),
        ...lookOf(s),
        // a session that reaches a machine on the network is a host too, found by its address
        in: ("host" in s.kind ? ["sessions", "hosts"] : ["sessions"]) as Filter[],
        run: () => onopen(s),
      })),
      // every action but opening the palette, which is open, and the ones only the mouse wheel runs
      ...ACTIONS.filter((a) => a.id !== "palette" && !isWheel(a.id)).map((a) => ({ label: _(a.label), hint: "", icon: "play", color: "var(--weak)", keys: keysShown(keysOf(a.id, shortcuts)), run: () => onaction(a.id) })),
      ...app.store.layout.map((l) => ({ label: _("Open layout: {name}", { name: l.name }), hint: l.pane.map((s) => s.name).join(", "), icon: "grid-alt", color: "var(--weak)", run: () => onlayout(l) })),
      ...app.store.snippet.map((s) => ({ label: _("Run snippet: {name}", { name: s.name }), hint: s.text.split("\n")[0], icon: "play", color: "var(--weak)", in: ["snippets"] as Filter[], run: () => onsnippet(s) })),
      ...app.store.tunnel.map((t) => {
        const on = !!live[t.name]?.live;
        return { label: on ? _("Stop tunnel {name}", { name: t.name }) : _("Start tunnel {name}", { name: t.name }), hint: `${t.kind} :${t.listen_port}`, icon: "transfer-alt", color: "var(--weak)", run: () => tunnel(t.name, on) };
      }),
    ];
  });

  const q = $derived(query.trim());
  // A typed address is a host to connect to, so the Sessions and Snippets filters leave it out.
  const addr = $derived(quickConnect(q));
  /** How many items each filter holds for the query, on its card. */
  const counts = $derived(Object.fromEntries(FILTERS.map(([f]) => [f, find(items, q, f).length + (f === "hosts" && addr ? 1 : 0)])));

  const shown = $derived.by((): Item[] => {
    const found = find(items, q, filter);
    if (!addr || (filter && filter !== "hosts")) return found.slice(0, 60);
    const label = `${addr.user ? addr.user + "@" : ""}${addr.host}:${addr.port}`;
    const connect = () => onopen(addressSession(addr));
    return [{ label: _("Connect to {address}", { address: label }), hint: addr.type.toUpperCase(), icon: KIND_ICON[addr.type], color: KIND_COLOR[addr.type], run: connect }, ...found.slice(0, 59)];
  });

  function pick(it: Item | undefined) {
    if (!it) return;
    onclose();
    it.run();
  }

  /** Picks a filter, or none when it is the one picked; the list starts again from its top. */
  function choose(f: Filter | null) {
    filter = f === filter ? null : f;
    sel = 0;
  }

  function keydown(e: KeyboardEvent) {
    if (e.key === "Tab") {
      // Tab and Shift+Tab step through the filters (and none) without leaving the field, so
      // Enter still runs the first hit.
      e.preventDefault();
      const order = [null, ...FILTERS.map(([f]) => f)];
      choose(order[(order.indexOf(filter) + (e.shiftKey ? order.length - 1 : 1)) % order.length]);
    } else if (e.key === "ArrowDown" || e.key === "ArrowUp") {
      e.preventDefault();
      sel = Math.max(0, Math.min(shown.length - 1, sel + (e.key === "ArrowDown" ? 1 : -1)));
      requestAnimationFrame(() => list.querySelector(".on")?.scrollIntoView({ block: "nearest" }));
    } else if (e.key === "Enter") {
      // The keyboard goes back to the terminal before this key's keypress: without this, an Enter
      // would follow a snippet that isn't meant to run (or any command typed there).
      e.preventDefault();
      pick(shown[sel]);
    } else if (e.key === "Escape") {
      onclose();
    }
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="backdrop" onclick={onclose}>
  <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
  <div class="palette" role="dialog" aria-label={_("Command palette")} tabindex="-1" onclick={(e) => e.stopPropagation()}>
    <label class="query">
      <Icon name="search" />
      <!-- Focused outright: Svelte's autofocus leaves focus alone while the terminal has it. -->
      <input bind:value={query} oninput={() => (sel = 0)} onkeydown={keydown} {@attach (el) => el.focus()} placeholder={_("Type a command, a session, or user@host to connect")} />
    </label>
    <div class="filters" role="group" aria-label={_("Filters")}>
      {#each FILTERS as [f, name] (f)}
        <!-- Out of the Tab order, and a click doesn't take focus: Tab in the field steps through these, and the field keeps the keyboard. -->
        <button
          class="filter"
          class:on={filter === f}
          aria-pressed={filter === f}
          tabindex="-1"
          title={_("Search only these; pick again for everything. Tab and Shift+Tab step through the filters.")}
          onmousedown={(e) => e.preventDefault()}
          onclick={() => choose(f)}
        >
          {name}<span class="count">{counts[f]}</span>
        </button>
      {/each}
    </div>
    <div class="list" bind:this={list}>
      {#each shown as it, i (i)}
        <button class="flat item" class:on={i === sel} onclick={() => pick(it)} onpointermove={() => (sel = i)}>
          <SessionIcon icon={it.icon} color={it.color} />
          <span class="label">{it.label}</span>
          {#if it.hint}<span class="hint">{it.hint}</span>{/if}
          {#if it.keys}<kbd>{it.keys}</kbd>{/if}
        </button>
      {:else}
        <p class="weak none">{_("Nothing matches.")}</p>
      {/each}
    </div>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: var(--titlebar) 0 0;
    z-index: 30;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding-top: 12vh;
    background: var(--scrim-soft);
    animation: fade-in 0.16s var(--ease-out);
  }
  .palette {
    width: min(610px, calc(100vw - 2 * 21px));
    max-height: calc(100vh / var(--phi));
    display: flex;
    flex-direction: column;
    background: rgba(var(--panel-rgb), var(--float-alpha));
    border: 1px solid var(--line);
    border-radius: var(--r-menu);
    box-shadow: var(--shadow-dialog);
    overflow: hidden;
    /* it hangs from the top, so it drops into place */
    --from: -5px;
    animation: float-in 0.16s var(--ease-out);
  }
  .query {
    position: relative;
    display: flex;
    align-items: center;
    padding: 5px;
    color: var(--weak);
  }
  .query :global(.icon) {
    position: absolute;
    inset-inline-start: 13px;
    pointer-events: none;
  }
  .query input {
    flex: 1;
    padding-inline-start: calc(8px + 16px + 5px);
  }
  .filters {
    display: flex;
    gap: 5px;
    padding: 0 5px 5px;
  }
  .filter {
    flex: 1;
    justify-content: space-between;
  }
  .count {
    font-size: var(--t-s);
    color: var(--weak);
  }
  .filter.on .count {
    color: inherit;
  }
  .list {
    overflow: auto;
    display: flex;
    flex-direction: column;
    padding: 0 5px 5px;
  }
  .item {
    justify-content: flex-start;
    gap: 8px;
    min-height: var(--control);
    padding: 3px 8px;
    border-radius: var(--r-control);
    text-align: start;
    border: 0;
  }
  /* the picked row (the global `.on` would turn the text gold, and `.flat` hides its background) */
  .item.on {
    color: var(--fg);
    background: var(--gold-soft);
  }
  /* on the gold wash the weak grey loses contrast: the hint takes the row's colour */
  .item.on .hint {
    color: inherit;
  }
  .label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .hint {
    flex-shrink: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: var(--t-s);
    color: var(--weak);
  }
  kbd {
    margin-inline-start: auto;
    flex: none;
  }
  .none {
    margin: 8px 13px;
  }
</style>

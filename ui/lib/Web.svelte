<script lang="ts">
  // A website tab: the system's browser view (src/web.rs), a native view laid over `area`. It
  // follows the area as panes move, and hides while the tab is hidden or a dialog is open: a
  // native view always draws above the page, so a dialog would sit under it.
  import { _ } from "./i18n.svelte";
  import { onDestroy, onMount, untrack } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { openUrl, type Session } from "./api";
  import Icon from "./Icon.svelte";
  import { app, coopColor } from "./state.svelte";

  let {
    session,
    active,
    visible,
    shared = 0,
    onid,
    ontitle,
    onfocus,
  }: {
    session: Session;
    /** In the focused pane: takes the keyboard. */
    active: boolean;
    visible: boolean;
    /** Your tab's number in the ring while you share it: the others' pointers and clicks are drawn
     *  in the page, and who drives it shows under the address bar. */
    shared?: number;
    onid: (id: number) => void;
    ontitle: (title: string) => void;
    onfocus: () => void;
  } = $props();

  /** The address the tab opens at; later navigation doesn't change the session. A BMC console
   *  (kind "kvm") opens at the BMC's web interface, which signs in and goes on to the console. */
  const home = untrack(() => {
    const k = session.kind;
    // A console's address: its vendor's start page goes after it (src/kvm `home`).
    return k.type === "browser" ? k.url : k.type === "kvm" ? `https://${k.host}${k.port === 443 ? "" : `:${k.port}`}` : "";
  });
  const kvm = untrack(() => (session.kind.type === "kvm" ? session.kind : null));
  let id = 0;
  let area: HTMLDivElement;
  let address = $state(home);
  let editing = false;
  let loading = $state(true);
  let error = $state("");

  // A click in the page gives it the keyboard without xtermium's page seeing it: the backend
  // says so (Windows), and this pane becomes the focused one.
  const unfocus = listen<number>("web-focus", ({ payload }) => payload === id && onfocus());
  // What the view can't show, said in the status line (the view covers the tab's own area).
  const unnotice = listen<{ id: number; message: string }>("web-notice", ({ payload: n }) => n.id === id && (app.status = n.message));

  type Page = { id: number; url: string; title: string | null; loading: boolean };
  const unlisten = listen<Page>("web-page", ({ payload: p }) => {
    if (p.id !== id) return;
    loading = p.loading;
    if (!editing) address = p.url;
    if (p.title) ontitle(p.title);
  });

  // ---- placement: checked every frame while shown; sent only when it changes ----
  let frame = 0;
  let placed = "";
  function place() {
    frame = requestAnimationFrame(place);
    if (!id) return;
    const r = area.getBoundingClientRect();
    // The page would cover the broadcast outline (2px, inside the tab's edge): it stays clear of it.
    const o = area.closest(".outline")?.getBoundingClientRect();
    const edges = o
      ? [Math.max(r.left, o.left + 2), Math.max(r.top, o.top + 2), Math.min(r.right, o.right - 2), Math.min(r.bottom, o.bottom - 2)]
      : [r.left, r.top, r.right, r.bottom];
    // Physical pixels: devicePixelRatio holds both the display scale and the interface zoom.
    const [x, y, right, bottom] = edges.map((v) => Math.round(v * devicePixelRatio));
    // Dialogs, the palette, right-click menus and pane drop zones would draw under the page: it steps aside while
    // one is over it.
    const over = (o: DOMRect) => o.left < r.right && o.right > r.left && o.top < r.bottom && o.bottom > r.top;
    const blocked = [...document.querySelectorAll(".backdrop, .menu, .drops")].some((el) => over(el.getBoundingClientRect()));
    const covered = !visible || r.width < 2 || r.height < 2 || blocked;
    const rect = covered ? null : { x, y, w: right - x, h: bottom - y };
    const key = JSON.stringify(rect);
    if (key === placed) return;
    placed = key;
    invoke("web_place", { id, rect }).catch((e) => (error = String(e)));
  }
  $effect(() => {
    if (visible) {
      cancelAnimationFrame(frame);
      place();
    } else {
      cancelAnimationFrame(frame);
      if (id) invoke("web_place", { id, rect: null }).then(() => (placed = "null"));
    }
  });
  $effect(() => {
    if (active && visible && id) invoke("web_go", { id, action: "focus" });
  });

  const go = (action: string, url?: string) => id && invoke("web_go", { id, action, url: url ?? null }).catch((e) => (error = String(e)));

  // ---- Terminal Co-op: the ring's pointers and clicks are drawn inside the page (src/web.rs `web_marks`),
  // since nothing of this window can lie over the native view; who drives the tab shows in a bar. ----
  const controller = $derived(shared ? (app.coop.control.find((c) => c.tab === shared)?.user_id ?? "") : "");
  const nameOf = (uid: string) => app.coop.ring?.members.find((m) => m.id === uid)?.name ?? _("Someone");
  /** The guests asking for control of this tab. */
  const asks = $derived(shared ? app.coop.wants.filter((w) => w.tab === shared) : []);
  const give = (user: string) => invoke("coop_control", { tab: shared, user }).catch((e) => (error = String(e)));
  const decline = (user: string) => invoke("coop_decline_control", { tab: shared, user }).catch((e) => (error = String(e)));
  // A pointer that moved in the last second carries its owner's name; a click is drawn once.
  const MOVING = 1000;
  let last: Record<string, string> = {};
  let moved: Record<string, number> = {};
  let drawn = 0;
  let tick = $state(0);
  $effect(() => {
    // Read first, so the effect follows them even while the view isn't open yet (`id` isn't state).
    const [cursors, clicks] = [app.coop.cursors, app.coop.clicks];
    void tick;
    if (!id) return;
    if (!shared) {
      if (drawn) invoke("web_marks", { id, marks: [] }).catch(() => {});
      [drawn, last, moved] = [0, {}, {}];
      return;
    }
    const t = performance.now();
    type Mark = { id: string; x: number; y: number; color: string; name: string; initial: string; moving: boolean; clicks: [number, number, number][] };
    const mark = (uid: string, name: string, x: number, y: number): Mark => ({ id: uid, x, y, color: coopColor(uid), name, initial: [...(name || "?")][0].toUpperCase(), moving: t - (moved[uid] ?? 0) < MOVING, clicks: [] });
    const marks: Mark[] = cursors
      .filter((c) => c.tab === shared)
      .map((c) => {
        const at = `${c.col},${c.row}`;
        if (last[c.user_id] !== at) [last[c.user_id], moved[c.user_id]] = [at, t];
        return mark(c.user_id, c.name, c.col, c.row);
      });
    for (const k of clicks.filter((c) => c.tab === shared && c.at > drawn)) {
      const m = marks.find((m) => m.id === k.user_id) ?? marks[marks.push(mark(k.user_id, k.name, -1, -1)) - 1];
      m.clicks.push([k.col, k.row, k.button]);
      drawn = Math.max(drawn, k.at);
    }
    drawn ||= 1;
    invoke("web_marks", { id, marks }).catch(() => {});
    const timer = setTimeout(() => (tick = performance.now()), MOVING + 50);
    return () => clearTimeout(timer);
  });

  function submit(e: SubmitEvent) {
    e.preventDefault();
    editing = false;
    let url = address.trim();
    if (!/^https?:\/\//i.test(url)) url = "https://" + url;
    error = "";
    go("go", url);
  }

  // ---- BMC consoles: paste as key presses ----
  /** Types the clipboard into the console (src/kvm.rs): the console only takes keys. */
  const paste = () => {
    error = "";
    invoke("web_type", { id, text: null })
      .then(() => go("focus"))
      .catch((e) => (error = String(e)));
  };
  let gone = false;
  onMount(async () => {
    try {
      const k = session.kind;
      const credential = k.type === "browser" || k.type === "kvm" ? k.credential : undefined;
      const profile = k.type === "browser" ? (k.profile ?? null) : null;
      id = await invoke<number>("web_open", { url: home, name: session.name, credential, vendor: kvm?.vendor ?? null, profile });
    } catch (e) {
      error = String(e);
      return;
    }
    if (gone) return void invoke("web_close", { id });
    onid(id);
  });
  onDestroy(() => {
    gone = true;
    cancelAnimationFrame(frame);
    unlisten.then((f) => f());
    unfocus.then((f) => f());
    unnotice.then((f) => f());
    if (id) invoke("web_close", { id });
  });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="web" onpointerdown={onfocus}>
  <form class="row bar" onsubmit={submit}>
    <button type="button" class="flat icon-only" title={_("Back")} aria-label={_("Back")} onclick={() => go("back")}><Icon name="chevron-right" size={15} /></button>
    <button type="button" class="flat icon-only fwd" title={_("Forward")} aria-label={_("Forward")} onclick={() => go("forward")}><Icon name="chevron-right" size={15} /></button>
    <button type="button" class="flat icon-only" title={_("Reload")} aria-label={_("Reload")} onclick={() => go("reload")}>
      <Icon name={loading ? "loader-alt" : "refresh"} spin={loading} size={15} />
    </button>
    <input class="grow" bind:value={address} onfocus={() => (editing = true)} onblur={() => (editing = false)} spellcheck="false" aria-label={_("Address")} />
    {#if kvm}
      <button type="button" class="flat icon-only" title={_("Paste: types the clipboard into the console as key presses (US layout)")} aria-label={_("Paste into the console")} onclick={paste}>
        <Icon name="paste" size={15} />
      </button>
    {/if}
    <button type="button" class="flat icon-only" title={_("Open in your web browser")} aria-label={_("Open in your web browser")} onclick={() => openUrl(address).catch((e) => (error = String(e)))}>
      <Icon name="world" size={15} />
    </button>
  </form>
  <!-- Who drives a shared website or console: shown while a guest drives or asks (the owner's side; a guest sees the picture in Rdp.svelte). -->
  {#if shared && (controller || asks.length)}
    <div class="row bar control" role="status">
      <Icon name="group" size={15} />
      {#if controller}
        <span>{_("{name} has control.", { name: nameOf(controller) })}</span>
        <button type="button" class="flat" onclick={() => give("")}>{_("Take control")}</button>
      {/if}
      {#each asks as a (a.user_id)}
        <span>{_("{name} asks for control.", { name: nameOf(a.user_id) })}</span>
        <button type="button" class="flat" onclick={() => give(a.user_id)}>{_("Give control")}</button>
        <button type="button" class="flat" onclick={() => decline(a.user_id)}>{_("Not now")}</button>
      {/each}
    </div>
  {/if}
  <div class="area" bind:this={area}>
    {#if error}<p class="error">{error}</p>{/if}
  </div>
</div>

<style>
  .web {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    background: rgba(var(--deep-rgb), 1);
  }
  .bar {
    flex: none;
    gap: 2px;
    padding: 3px 5px;
    border-bottom: 1px solid var(--line);
    background: rgba(var(--panel-rgb), var(--chrome-alpha));
  }
  .bar input {
    height: var(--control);
    margin: 0 5px;
  }
  /* one chevron icon, turned around for Back */
  .bar button:first-child :global(.icon) {
    transform: scaleX(-1);
  }
  .control {
    gap: 8px;
    padding: 3px 8px;
    font-size: var(--t-s);
  }
  .control button {
    min-height: var(--row);
    padding: 0 5px;
  }
  .area {
    position: relative;
    flex: 1;
    min-height: 0;
  }
  .error {
    margin: 13px;
  }
</style>

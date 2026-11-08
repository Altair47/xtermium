<script lang="ts">
  // The Terminal Co-op dialog (Ctrl+Shift+G): join a ring or start one, then who is in it, drawn as
  // a ring (CoopRing.svelte) with each person's traffic. It polls nothing itself: while it is open it
  // asks the backend for a fresh look every 2 seconds and for the traffic every second, and what it
  // draws is the `coop` event's picture, kept in `app.coop` (state.svelte.ts).
  import { invoke } from "@tauri-apps/api/core";
  import { _ } from "./i18n.svelte";
  import { openUrl, type CoopMember, type CoopTraffic } from "./api";
  import { clock, passcode, PASSCODE_LENGTH, rate, RING_SEATS, type RingFlow } from "./coop";
  import CoopPic from "./CoopPic.svelte";
  import CoopRing from "./CoopRing.svelte";
  import Icon from "./Icon.svelte";
  import { menu } from "./menu";
  import Modal from "./Modal.svelte";
  import { app, CHECKOUT, coopColor, copyText, loadAccount } from "./state.svelte";

  let { onclose }: { onclose: () => void } = $props();

  const ring = $derived(app.coop.ring);
  const signedIn = $derived(!!app.settings?.sync_token);
  /** The server said so, the account says it has no Pro, or starting a ring was refused for want of it. */
  let refused = $state(false);
  const needsPro = $derived(refused || app.sync.needsPro || app.sync.account?.pro.active === false);
  const pending = $derived(ring?.members.filter((m) => m.state === "pending") ?? []);
  // The owner hears of the people turned away too ("kicked"); they are no longer in the ring.
  const inside = $derived(ring?.members.filter((m) => m.state === "approved") ?? []);

  /** More people than the ring is drawn for: they are listed instead, without their traffic. */
  const crowded = $derived(inside.length > RING_SEATS);

  let code = $state("");
  let error = $state("");
  let busy = $state(false);
  let now = $state(Date.now());
  /** Left to the passcode, in seconds. */
  const left = $derived(ring?.passcode_until ? ring.passcode_until - now / 1000 : 0);

  /** Only the owner asks the server who is waiting; a viewer hears of changes from the `coop` event. */
  const owns = $derived(!!ring?.owner);
  const refresh = () => owns && invoke("coop_refresh").catch(() => {});
  /** What each person sends and receives, by their id (the backend says whose it knows). */
  let flows = $state<Record<string, RingFlow>>({});
  /** The counters as read over the last 3 seconds, oldest first. A rate is the growth since the
   *  oldest, so the figures move calmly instead of leaping with every burst. */
  let reads: { at: number; by: Map<string, CoopTraffic> }[] = [];
  async function measure() {
    if (!ring || crowded) return void (flows = {});
    const list = await invoke<CoopTraffic[]>("coop_traffic").catch(() => [] as CoopTraffic[]);
    const at = performance.now();
    flows = Object.fromEntries(
      list.map((t) => {
        const old = reads.find((r) => r.by.has(t.id));
        const was = old?.by.get(t.id);
        const secs = old ? (at - old.at) / 1000 : 0;
        return [t.id, { up: was ? rate(t.up, was.up, secs) : 0, down: was ? rate(t.down, was.down, secs) : 0, sent: t.up, received: t.down }];
      }),
    );
    reads = [...reads.filter((r) => at - r.at < 3500), { at, by: new Map(list.map((t) => [t.id, t])) }];
  }
  $effect(() => {
    refresh();
    measure();
    const [look, tick] = [setInterval(refresh, 2000), setInterval(() => ((now = Date.now()), measure()), 1000)];
    return () => (clearInterval(look), clearInterval(tick));
  });
  // Whether the account has Pro is asked afresh each time the dialog opens; a refusal is forgotten once it has it.
  $effect(() => void (signedIn && loadAccount()));
  $effect(() => void (app.sync.account?.pro.active && (refused = false)));

  /** Runs one of the backend's commands, then looks again at once; its refusal is shown in words.
   *  True when it went through. */
  async function act(command: string, args?: Record<string, unknown>) {
    [busy, error] = [true, ""];
    try {
      await invoke(command, args);
      refresh();
      return true;
    } catch (e) {
      error = String(e);
      // The server's answer to a ring without Pro names it ("needs XTermium Pro"): show the link instead.
      if (command === "coop_create" && /\bPro\b/.test(error)) refused = true;
      return false;
    } finally {
      busy = false;
    }
  }

  /** The backend answers at once; the ring then waits ("pending") until the owner says yes. */
  async function join() {
    const typed = passcode(code);
    if (typed.length === PASSCODE_LENGTH && !busy && (await act("coop_join", { passcode: typed }))) code = "";
  }

  /** Whose ring it is, for someone who joined it. */
  const owner = $derived(ring?.members.find((m) => m.owner)?.name);
  /** Before there is a ring it is drawn with you alone on it. */
  const alone = $derived.by((): CoopMember[] => {
    const a = app.sync.account;
    return signedIn && a ? [{ id: "", name: a.name || a.email, image: a.image, state: "approved", you: true }] : [];
  });

  /** The person the owner picked on the ring, and where their menu opens. */
  let picked = $state<{ m: CoopMember; x: number; y: number } | null>(null);
  /** The person being given another name, and the name typed so far. */
  let renaming = $state<{ id: string; was: string; name: string } | null>(null);
  const keep = () => ([renaming, error] = [null, ""]);
  async function rename() {
    const r = renaming;
    const name = r?.name.trim();
    if (!r || !name || busy) return;
    if (name === r.was || (await act("coop_rename", { user: r.id, name }))) renaming = null;
  }
</script>

<!-- A click anywhere closes the menu, but the click on a seat that opened it. -->
<svelte:window onclick={(e) => (e.target as Element).closest?.(".seat") || (picked = null)} />

{#snippet person(m: { id: string; name: string; image: string | null })}
  <CoopPic name={m.name} image={m.image} color={coopColor(m.id)} />
  <span class="grow name" title={m.name}>{m.name}</span>
{/snippet}

{#snippet renameFoot()}
  <button onclick={keep}>{_("Cancel")}</button>
  <button class="primary" disabled={busy || !renaming?.name.trim()} onclick={rename}>{_("Rename")}</button>
{/snippet}

{#snippet foot()}
  {#if ring?.owner}
    <button class="danger" disabled={busy} onclick={() => act("coop_end")}><Icon name="x-circle" />{_("End the ring")}</button>
  {:else if ring?.state === "joined"}
    <button disabled={busy} onclick={() => act("coop_leave")}><Icon name="log-out" />{_("Leave the ring")}</button>
  {:else}
    <button disabled={busy} onclick={() => act("coop_leave")}>{_("Cancel")}</button>
  {/if}
{/snippet}

<Modal title={_("Terminal Co-op")} width={377} {onclose} foot={ring ? foot : undefined}>
  {#if !ring}
    <p class="weak">{_("Share tabs with up to two more people. What you share goes straight between your computers, end-to-end encrypted.")}</p>
    {#if !signedIn}
      <p>
        {_("Terminal Co-op needs an account.")}
        <button class="link" onclick={() => (app.signIn = true)}>{_("Sign in")}</button>
      </p>
    {:else if needsPro}
      <p>
        {_("Terminal Co-op needs xtermium Pro.")}
        <button class="link" onclick={() => openUrl(CHECKOUT).catch((e) => (app.status = String(e)))}>{_("Get Pro")}</button>
      </p>
    {/if}
    <h3>{_("Join a ring")}</h3>
    <p class="weak">{_("Ask the person who started it for the passcode.")}</p>
    <div class="row">
      <input
        class="grow code"
        value={code}
        placeholder="A7K2M9QX"
        autocomplete="off"
        autocapitalize="characters"
        spellcheck="false"
        aria-label={_("Passcode")}
        oninput={(e) => (code = e.currentTarget.value = passcode(e.currentTarget.value))}
        onkeydown={(e) => e.key === "Enter" && join()}
      />
      <button disabled={!signedIn || busy || passcode(code).length !== PASSCODE_LENGTH} onclick={join}>{_("Join")}</button>
    </div>
    <h3>{_("Start a ring")}</h3>
    <p class="weak">{_("You get a passcode. The people you give it to ask to join, and you say yes.")}</p>
    <div class="row">
      <button class="primary" disabled={!signedIn || needsPro || busy} onclick={() => act("coop_create")}>
        {#if busy}<Icon name="loader-alt" spin />{/if}{_("Start a ring")}
      </button>
    </div>
    {#if alone.length}<CoopRing people={alone} />{/if}
  {:else if ring.owner}
    <p class="weak">{_("Give people this passcode. They ask to join, and you say yes.")}</p>
    <div class="passcode row">
      <output class="grow" aria-label={_("Passcode")}>{ring.passcode ?? ""}</output>
      <button class="flat icon-only" disabled={!ring.passcode} title={_("Copy the passcode")} aria-label={_("Copy the passcode")} onclick={() => ring.passcode && copyText(ring.passcode)}><Icon name="copy" /></button>
    </div>
    <div class="row">
      <span class="grow weak">{left > 0 ? _("Works for {time} more.", { time: clock(left) }) : _("The passcode has expired.")}</span>
      <button disabled={busy} onclick={() => act("coop_passcode")}><Icon name="refresh" />{_("New passcode")}</button>
    </div>
    {#if pending.length}
      <h3>{_("Asking to join")}</h3>
      {#each pending as m (m.id)}
        <div class="row person">
          {@render person(m)}
          <!-- Quiet, and Approve the one gold button: Enter never lets anyone in by itself (the keyboard starts on the dialog). -->
          <button class="flat" disabled={busy} onclick={() => act("coop_decline", { user: m.id })}>{_("Decline")}</button>
          <button class="primary" disabled={busy} onclick={() => act("coop_approve", { user: m.id })}>{_("Approve")}</button>
        </div>
      {/each}
    {/if}
    <h3>{_("In the ring")}</h3>
    {#if crowded}
      {#each inside as m (m.id)}
        <div class="row person">
          {@render person(m)}
          {#if !m.owner}
            <button class="flat" disabled={busy} onclick={() => (renaming = { id: m.id, was: m.name, name: m.name })}>{_("Rename…")}</button>
            <button class="flat" disabled={busy} onclick={() => act("coop_kick", { user: m.id })}>{_("Kick")}</button>
          {/if}
        </div>
      {/each}
    {:else}
      <CoopRing people={inside} {flows} picked={picked?.m.id} onpick={(m, x, y) => (picked = { m, x, y })} />
      <p class="weak">{inside.length < 2 ? _("Nobody else is in yet.") : _("Click a person on the ring to rename or kick them.")}</p>
    {/if}
    <p class="weak">{_("Right-click a tab › Share with the ring puts it in front of everyone here.")}</p>
  {:else if ring.state !== "joined"}
    <p class="row weak">
      <Icon name="loader-alt" spin /><span class="grow">{ring.state === "pending" ? _("Waiting for the owner to say yes…") : _("Connecting to the owner…")}</span>
    </p>
  {:else}
    <p>{owner ? _("You're in {name}'s ring.", { name: owner }) : _("You're in a ring.")}</p>
    {#if crowded}
      {#each inside as m (m.id)}
        <div class="row person">{@render person(m)}</div>
      {/each}
    {:else}
      <CoopRing people={inside} {flows} />
    {/if}
    <p class="weak">{_("The tabs the owner shares open as tabs here. Type and click in them as if they were yours.")}</p>
  {/if}
  {#if error && !refused && !renaming}<p class="error">{error}</p>{/if}
</Modal>

{#if picked}
  {@const m = picked.m}
  <div class="menu" {@attach menu(() => (picked = null), picked.x, picked.y)}>
    <button onclick={() => (renaming = { id: m.id, was: m.name, name: m.name })}><Icon name="edit" />{_("Rename…")}</button>
    <button class="danger" disabled={busy} onclick={() => act("coop_kick", { user: m.id })}><Icon name="x" />{_("Kick")}</button>
  </div>
{/if}

{#if renaming}
  <Modal title={_("Rename")} onclose={keep} foot={renameFoot}>
    <p class="weak">{_("Give {name} another name. Everyone in the ring sees it instead of the one they chose.", { name: renaming.was })}</p>
    <div class="row">
      <input class="grow" bind:value={renaming.name} maxlength="120" autocomplete="off" spellcheck="false" aria-label={_("Name")} onkeydown={(e) => e.key === "Enter" && rename()} />
    </div>
    {#if error}<p class="error">{error}</p>{/if}
  </Modal>
{/if}

<style>
  .code {
    font-family: var(--mono);
    letter-spacing: 0.2em;
    text-align: center;
  }
  .passcode {
    padding: 8px 13px;
    border: 1px solid var(--line);
    border-radius: var(--r-menu);
    background: rgba(var(--deep-rgb), 1);
  }
  /* the passcode, big enough to read out across a room */
  output {
    font-family: var(--mono);
    font-size: var(--t-xxl);
    font-weight: 600;
    letter-spacing: 0.2em;
    user-select: text;
    -webkit-user-select: text;
  }
  .person {
    gap: 8px;
    min-height: var(--control);
  }
  .name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  /* the end of the dialog, as far from the safe buttons as it goes */
  button.danger {
    color: var(--error);
  }
  p {
    margin: 0;
  }
</style>

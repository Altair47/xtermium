<script lang="ts">
  // The Terminal Co-op dialog (Ctrl+Shift+G): start a ring or join one, then who is in it. It polls
  // nothing itself: while it is open it asks the backend for a fresh look every 2 seconds, and what
  // it draws is the `coop` event's picture, kept in `app.coop` (state.svelte.ts).
  import { invoke } from "@tauri-apps/api/core";
  import { _ } from "./i18n.svelte";
  import { openUrl } from "./api";
  import { clock, passcode, PASSCODE_LENGTH } from "./coop";
  import CoopPic from "./CoopPic.svelte";
  import Icon from "./Icon.svelte";
  import Modal from "./Modal.svelte";
  import { app, CHECKOUT, copyText, loadAccount } from "./state.svelte";

  let { onclose }: { onclose: () => void } = $props();

  const ring = $derived(app.coop.ring);
  const signedIn = $derived(!!app.settings?.sync_token);
  /** The server said so, the account says it has no Pro, or starting a ring was refused for want of it. */
  let refused = $state(false);
  const needsPro = $derived(refused || app.sync.needsPro || app.sync.account?.pro.active === false);
  const pending = $derived(ring?.members.filter((m) => m.state === "pending") ?? []);
  const inside = $derived(ring?.members.filter((m) => m.state !== "pending") ?? []);

  let code = $state("");
  let error = $state("");
  let busy = $state(false);
  let now = $state(Date.now());
  /** Left to the passcode, in seconds. */
  const left = $derived(ring?.passcode_until ? ring.passcode_until - now / 1000 : 0);

  const refresh = () => invoke("coop_refresh").catch(() => {});
  $effect(() => {
    refresh();
    const [look, tick] = [setInterval(refresh, 2000), setInterval(() => (now = Date.now()), 1000)];
    return () => (clearInterval(look), clearInterval(tick));
  });
  // Whether the account has Pro is asked afresh each time the dialog opens; a refusal is forgotten once it has it.
  $effect(() => void (signedIn && loadAccount()));
  $effect(() => void (app.sync.account?.pro.active && (refused = false)));

  /** Runs one of the backend's commands, then looks again at once; its refusal is shown in words. */
  async function act(command: string, args?: Record<string, unknown>) {
    [busy, error] = [true, ""];
    try {
      await invoke(command, args);
      refresh();
    } catch (e) {
      error = String(e);
      // The server's answer to a ring without Pro names it ("needs XTermium Pro"): show the link instead.
      if (command === "coop_create" && /\bPro\b/.test(error)) refused = true;
    } finally {
      busy = false;
    }
  }

  /** Cancelled by the person while the owner has not answered: the join's failure isn't news. */
  let cancelled = false;
  async function join() {
    const typed = passcode(code);
    if (typed.length !== PASSCODE_LENGTH || app.coop.joining) return;
    [cancelled, error, app.coop.joining] = [false, "", true];
    try {
      // Resolves once the owner has said yes and the connection is up.
      await invoke("coop_join", { passcode: typed });
      code = "";
      refresh();
    } catch (e) {
      if (!cancelled) error = String(e);
    } finally {
      app.coop.joining = false;
    }
  }
  function cancel() {
    [cancelled, app.coop.joining] = [true, false];
    invoke("coop_leave").catch(() => {});
  }

  /** Whose ring it is, for someone who joined it. */
  const owner = $derived(ring?.members.find((m) => m.owner)?.name);
</script>

{#snippet person(m: { name: string; image: string | null })}
  <CoopPic name={m.name} image={m.image} />
  <span class="grow name" title={m.name}>{m.name}</span>
{/snippet}

{#snippet foot()}
  {#if ring?.owner}
    <button class="danger" disabled={busy} onclick={() => act("coop_end")}><Icon name="x-circle" />{_("End the ring")}</button>
  {:else}
    <button disabled={busy} onclick={() => act("coop_leave")}><Icon name="log-out" />{_("Leave the ring")}</button>
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
    <h3>{_("Start a ring")}</h3>
    <p class="weak">{_("You get a passcode. The people you give it to ask to join, and you say yes.")}</p>
    <div class="row">
      <button class="primary" disabled={!signedIn || needsPro || busy} onclick={() => act("coop_create")}>
        {#if busy}<Icon name="loader-alt" spin />{/if}{_("Start a ring")}
      </button>
    </div>
    <h3>{_("Join a ring")}</h3>
    {#if app.coop.joining}
      <p class="row weak"><Icon name="loader-alt" spin /><span class="grow">{_("Waiting for the owner to say yes…")}</span></p>
      <div class="row"><button onclick={cancel}>{_("Cancel")}</button></div>
    {:else}
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
        <button disabled={!signedIn || passcode(code).length !== PASSCODE_LENGTH} onclick={join}>{_("Join")}</button>
      </div>
    {/if}
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
          <button class="flat" disabled={busy} onclick={() => act("coop_kick", { user: m.id })}>{_("Decline")}</button>
          <button class="primary" disabled={busy} onclick={() => act("coop_approve", { user: m.id })}>{_("Approve")}</button>
        </div>
      {/each}
    {/if}
    <h3>{_("In the ring")}</h3>
    {#each inside as m (m.id)}
      <div class="row person">
        {@render person(m)}
        {#if !m.owner}
          <button class="flat" disabled={busy} title={_("Take {name} out of the ring", { name: m.name })} onclick={() => act("coop_kick", { user: m.id })}>{_("Kick")}</button>
        {/if}
      </div>
    {:else}
      <p class="weak">{_("Nobody else is in yet.")}</p>
    {/each}
    <p class="weak">{_("Right-click a tab › Share with the ring puts it in front of everyone here.")}</p>
  {:else}
    <p>{owner ? _("You're in {name}'s ring.", { name: owner }) : _("You're in a ring.")}</p>
    {#each inside as m (m.id)}
      <div class="row person">{@render person(m)}</div>
    {/each}
    <p class="weak">{_("The tabs the owner shares open as tabs here. Type and click in them as if they were yours.")}</p>
  {/if}
  {#if error && !refused}<p class="error">{error}</p>{/if}
</Modal>

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

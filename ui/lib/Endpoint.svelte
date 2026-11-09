<script lang="ts">
  // Host / port / user / key fields, as rows of a `.form` grid. They can be filled in from a
  // saved SSH session (its login too), e.g. to use it as a jump host: that pick comes first, above
  // what it fills, and a dialog's keyboard still lands on the host (SshPick is `data-not-first`).
  // `hint`: point people with several computers at shared logins (the session editor's target).
  import { _ } from "./i18n.svelte";
  import type { Endpoint } from "./api";
  import { app } from "./state.svelte";
  import Hint from "./Hint.svelte";
  import Login from "./Login.svelte";
  import PathInput from "./PathInput.svelte";
  import SshPick from "./SshPick.svelte";

  let { e, label = _("Host / port"), hint = false }: { e: Endpoint; label?: string; hint?: boolean } = $props();
</script>

<SshPick onpick={(k) => Object.assign(e, { host: k.host, port: k.port, user: k.user, key: k.key, credential: k.credential ?? "" })} />
<span>{label}</span>
<div class="row"><input class="grow" bind:value={e.host} /><input type="number" min="1" max="65535" bind:value={e.port} /></div>
<Login bind:value={() => e.credential ?? "", (v) => (e.credential = v)} />
{#if !e.credential}
  <span>{_("User")}</span>
  <input bind:value={e.user} />
  <span>{_("Private key")}</span>
  <PathInput bind:value={e.key} start="~/.ssh/" placeholder={_("empty: your SSH agent, keys in ~/.ssh or a password")} />
  {#if hint}
    <Hint id="shared-login">
      {_("Using xtermium on several computers? Put the user, key and password in a shared login once, with a key path per system, and every session that uses it follows.")}
      <button class="link" onclick={() => (app.showCredentials = true)}>{_("Manage shared logins…")}</button>
    </Hint>
  {/if}
{/if}

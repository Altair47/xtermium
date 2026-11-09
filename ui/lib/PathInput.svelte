<script lang="ts">
  // A file or folder path: the disk's matching entries drop down as you type (folders end in a
  // separator, so the next level follows), and Browse opens the system's picker. `exec`: a
  // program, also found on PATH; `start`: where suggestions begin while the field is empty.
  // A stored path can hold one value for each system, since sessions sync between Windows, macOS
  // and Linux computers; `single` is for this computer's own settings, which never need that.
  import { _ } from "./i18n.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { here, system, SYSTEMS, type Portable } from "./api";
  import Icon from "./Icon.svelte";

  let {
    value = $bindable(),
    dirs = false,
    exec = false,
    start = "",
    placeholder = "",
    single = false,
  }: { value: Portable; dirs?: boolean; exec?: boolean; start?: string; placeholder?: string; single?: boolean } = $props();

  const list = `paths-${Math.random().toString(36).slice(2)}`;
  let options = $state<string[]>([]);
  let timer = 0;
  // This computer's slot; other systems read Linux's, as `here` does.
  const os = SYSTEMS.find(([k]) => k === system.os)?.[0] ?? "linux";

  /** Writes a system's value, this computer's by default; a plain string stays a string. */
  function set(v: string, k = os) {
    value = typeof value === "string" ? v : { ...value, [k]: v };
  }

  /** One value for every system, or one for each: switching back keeps this computer's. */
  function toggle() {
    value = typeof value === "string" ? { windows: "", macos: "", linux: "", [os]: value } : here(value);
  }

  function suggest() {
    clearTimeout(timer);
    timer = setTimeout(async () => {
      options = await invoke<string[]>("complete_path", { typed: here(value) || start, dirs, exec }).catch(() => []);
    }, 80);
  }

  async function browse() {
    const picked = await invoke<string | null>("pick_path", { dirs, start: here(value) || start }).catch(() => null);
    if (picked) set(picked);
  }
</script>

{#snippet field(v: string, k: typeof os, label?: string)}
  <input
    class="grow"
    list={k === os ? list : undefined}
    value={v}
    {placeholder}
    aria-label={label}
    oninput={(e) => (set(e.currentTarget.value, k), k === os && suggest())}
    onfocus={() => k === os && suggest()}
    autocomplete="off"
    spellcheck="false"
  />
{/snippet}
{#snippet browseButton()}
  <button class="icon-only" title={dirs ? _("Choose a folder") : _("Choose a file")} onclick={browse}><Icon name="folder-open" size={16} /></button>
{/snippet}
{#snippet eachButton()}
  {#if !single}
    <button class="icon-only" class:on={typeof value !== "string"} title={_("A value for each system")} aria-pressed={typeof value !== "string"} onclick={toggle}><Icon name="world" size={16} /></button>
  {/if}
{/snippet}

{#if typeof value === "string"}
  <div class="row path">
    {@render field(value, os)}
    {@render browseButton()}
    {@render eachButton()}
    <datalist id={list}>{#each options as o (o)}<option value={o}></option>{/each}</datalist>
  </div>
{:else}
  <div class="col each">
    {#each SYSTEMS as [k, name], i (k)}
      <span class="system">{name}</span>
      {@render field(value[k], k, name)}
      {#if k === os}{@render browseButton()}{:else}<span></span>{/if}
      {#if i === 0}{@render eachButton()}{:else}<span></span>{/if}
    {/each}
    <p class="weak note">{_("Sessions synced to another system use that system's value. Empty means the default there.")}</p>
    <datalist id={list}>{#each options as o (o)}<option value={o}></option>{/each}</datalist>
  </div>
{/if}

<style>
  .path {
    gap: var(--gap);
  }
  .path input {
    min-width: 0;
  }
  .each {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr) auto auto;
    gap: var(--gap) 8px;
    align-items: center;
  }
  .system {
    font-size: var(--t-s);
    color: var(--weak);
  }
  .note {
    grid-column: 1 / -1;
    margin: 0;
  }
</style>

<script lang="ts">
  // A person's picture, round, or their first letter when they have none or it doesn't load (the
  // window's CSP lets https pictures through, tauri.conf.json `img-src`).
  let { name, image, size = 21, color = "" }: { name: string; image: string | null; size?: 34 | 21 | 13; color?: string } = $props();

  /** The address that failed to load, so a new picture is tried afresh. */
  let failed = $state("");
</script>

<span class="pic" class:small={size < 21} class:big={size > 21} style:--size="{size}px" style:--c={color || null}>
  {#if image && image !== failed}
    <img src={image} alt="" onerror={() => (failed = image)} />
  {:else}
    {[...(name || "?")][0].toUpperCase()}
  {/if}
</span>

<style>
  .pic {
    display: grid;
    place-items: center;
    flex: none;
    width: var(--size);
    height: var(--size);
    overflow: hidden;
    border-radius: 50%;
    background: var(--c, var(--raised));
    color: var(--fg);
    font-size: var(--t-s);
    font-weight: 700;
    line-height: 1;
  }
  .small {
    font-size: 0.6em;
  }
  .big {
    font-size: var(--t-xl);
  }
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
</style>

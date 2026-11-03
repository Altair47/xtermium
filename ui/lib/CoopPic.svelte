<script lang="ts">
  // A person's picture, round, or their first letter when they have none or it doesn't load. The
  // window only lets `data:` pictures through (tauri.conf.json `img-src`), so a picture at an
  // https address shows as the letter until the backend sends it inline.
  let { name, image, size = 21, color = "" }: { name: string; image: string | null; size?: 21 | 13; color?: string } = $props();

  /** The address that failed to load, so a new picture is tried afresh. */
  let failed = $state("");
</script>

<span class="pic" class:small={size < 21} style:--size="{size}px" style:--c={color || null}>
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
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
</style>

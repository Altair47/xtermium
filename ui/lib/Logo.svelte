<script lang="ts">
  // The brand kit's horizontal logo: the mark beside the "XTermium" wordmark. Its three large
  // tiles and the wordmark take `--mark`, so one file serves both themes; the small tile keeps
  // the brand amber, which the kit says is never recoloured. Its width comes from the parent.
  // A picture chosen as the app's icon (Settings › Appearance) takes the mark's place.
  import horizontal from "../../assets/logo-horizontal.svg?raw";
  import alone from "../../assets/logo.svg?raw";
  import { app } from "./state.svelte";

  /** `mark`: the mark alone, where there is no room for the wordmark to read (the title bar). */
  let { mark = false }: { mark?: boolean } = $props();
  /** The chosen picture, if it is a plain base64 PNG (it goes into the logo's markup). */
  const own = $derived(/^data:image\/png;base64,[A-Za-z0-9+/=]+$/.test(app.settings?.app_icon ?? "") ? app.settings!.app_icon : "");
  // In the horizontal logo the mark's box runs from x 40 to 728 and y 40 to 550: the picture is a
  // square as tall, ending where the mark ends, and the view starts as far before it as the mark
  // did, so the padding and the gap to the wordmark stay the same.
  const art = $derived(
    (own
      ? horizontal
          .replace(/<g[^>]*>[\s\S]*?<\/g>/, `<image href="${own}" x="218" y="40" width="510" height="510"/>`)
          .replace('viewBox="0 0 2305 590" width="2305"', 'viewBox="178 0 2127 590" width="2127"')
      : mark
        ? alone
        : horizontal
    ).replaceAll('fill="#F2F2F2"', 'fill="currentColor"'),
  );
</script>

<div class="logo" role="img" aria-label="xtermium">
  {#if own && mark}<img src={own} alt="" />{:else}{@html art}{/if}
</div>

<style>
  .logo {
    color: var(--mark);
    line-height: 0;
  }
  .logo :global(svg),
  img {
    width: 100%;
    height: auto;
  }
</style>

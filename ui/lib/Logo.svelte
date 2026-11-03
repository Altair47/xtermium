<script lang="ts">
  // The brand kit's horizontal logo: the mark beside the "XTermium" wordmark. Its three large
  // tiles and the wordmark take `--mark`, so one file serves both themes; the small tile keeps
  // the brand amber, which the kit says is never recoloured. Its width comes from the parent.
  import svg from "../../assets/logo-horizontal.svg?raw";

  const art = svg.replaceAll('fill="#F2F2F2"', 'fill="currentColor"');
</script>

<div class="logo" role="img" aria-label="xtermium">{@html art}</div>

<style>
  .logo {
    color: var(--mark);
    line-height: 0;
  }
  .logo :global(svg) {
    width: 100%;
    height: auto;
  }
</style>

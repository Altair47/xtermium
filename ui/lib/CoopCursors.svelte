<script lang="ts">
  // Over a tab that is shared or shared with you: where each other person's pointer is, a small
  // marker in their colour with their picture beside it; their name shows on hover. The host
  // (Terminal.svelte, Rdp.svelte) says where a cell, or a pixel of a desktop, is in its box.
  import { COLORS } from "./api";
  import { memberColor } from "./coop";
  import CoopPic from "./CoopPic.svelte";
  import { app } from "./state.svelte";

  /** `tab`: the tab's number in the ring (the owner's). `at`: a cell (col, row) or a desktop pixel
   *  (x, y) → where that is in the host's box, in CSS pixels. */
  let { tab, at }: { tab: number; at: (col: number, row: number) => [number, number] } = $props();

  /** A pointer reported at -1 has left the tab. */
  const here = $derived(app.coop.cursors.filter((c) => c.tab === tab && c.col >= 0 && c.row >= 0));
  /** Everyone keeps one colour, the same on every computer, from their id. */
  const colorOf = (id: string) => COLORS[memberColor(id, COLORS.length)];
</script>

{#each here as c (c.user_id)}
  {@const [x, y] = at(c.col, c.row)}
  <div class="cursor" style:--c={colorOf(c.user_id)} style:translate="{x}px {y}px">
    <i class="dot"></i>
    <span class="who" title={c.name}><CoopPic name={c.name} image={c.image} size={13} color={colorOf(c.user_id)} /></span>
  </div>
{/each}

<style>
  .cursor {
    position: absolute;
    left: 0;
    top: 0;
    z-index: 2;
    pointer-events: none;
    transition: translate 80ms linear;
  }
  /* the point itself, a ring in the person's colour */
  .dot {
    display: block;
    width: 8px;
    height: 8px;
    border: 2px solid var(--c);
    border-radius: 50%;
    outline: 1px solid rgba(var(--window-rgb), 0.7);
  }
  /* beside it, down and to the right; the only part that takes the pointer, for the name */
  .who {
    position: absolute;
    left: 8px;
    top: 8px;
    pointer-events: auto;
  }
  .who :global(.pic) {
    outline: 1px solid rgba(var(--window-rgb), 0.7);
  }
  @media (prefers-reduced-motion: reduce) {
    .cursor {
      transition: none;
    }
  }
</style>

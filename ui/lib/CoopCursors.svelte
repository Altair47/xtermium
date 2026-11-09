<script lang="ts">
  // Over a tab that is shared or shared with you: where each other person's pointer is, a small
  // marker in their colour with their picture beside it; their name shows on hover. The host
  // (Terminal.svelte, Rdp.svelte) says where a cell, or a pixel of a desktop, is in its box.
  import CoopPic from "./CoopPic.svelte";
  import { app, coopColor } from "./state.svelte";

  /** `tab`: the tab's number in the ring (the owner's). `at`: a cell (col, row) or a desktop pixel
   *  (x, y) → where that is in the host's box, in CSS pixels. */
  let { tab, at }: { tab: number; at: (col: number, row: number) => [number, number] } = $props();

  /** A pointer reported at -1 has left the tab. */
  const here = $derived(app.coop.cursors.filter((c) => c.tab === tab && c.col >= 0 && c.row >= 0));
  /** The ring's clicks over this tab, of the last second. */
  const clicks = $derived(app.coop.clicks.filter((c) => c.tab === tab));

  // A pointer that moves shows its owner's name for a second, so it's clear who is doing what; at
  // rest, only the picture.
  let last: Record<string, string> = {};
  let moved: Record<string, number> = {};
  let now = $state(performance.now());
  $effect(() => {
    const t = performance.now();
    for (const c of here) {
      const at = `${c.col},${c.row}`;
      if (last[c.user_id] !== at) [last[c.user_id], moved[c.user_id]] = [at, t];
    }
    now = t;
    const timer = setTimeout(() => (now = performance.now()), MOVING + 50);
    return () => clearTimeout(timer);
  });
  const MOVING = 1000;
  const moving = (id: string) => now - (moved[id] ?? 0) < MOVING;
</script>

{#each clicks as k (k.user_id + k.at)}
  {@const [x, y] = at(k.col, k.row)}
  <i class="click" class:other={k.button > 0} style:--c={coopColor(k.user_id)} style:translate="{x - 17}px {y - 17}px"></i>
{/each}

{#each here as c (c.user_id)}
  {@const [x, y] = at(c.col, c.row)}
  <div class="cursor" class:moving={moving(c.user_id)} style:--c={coopColor(c.user_id)} style:translate="{x}px {y}px">
    <i class="dot"></i>
    <span class="who" title={c.name}>
      {#if moving(c.user_id)}<b class="name">{c.name}</b>{:else}<CoopPic name={c.name} image={c.image} size={13} color={coopColor(c.user_id)} />{/if}
    </span>
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
  /* while it moves: a bigger point and the name instead of the picture */
  .moving .dot {
    width: 13px;
    height: 13px;
  }
  .name {
    display: block;
    padding: 0 3px;
    border-radius: 8px;
    color: #fff;
    background: var(--c);
    font-size: var(--t-s);
    font-weight: 700;
    line-height: 1.4;
    white-space: nowrap;
    outline: 1px solid rgba(var(--window-rgb), 0.7);
  }
  /* a click: a ring in the person's colour growing from the point and fading; dashed for the other buttons */
  .click {
    position: absolute;
    left: 0;
    top: 0;
    z-index: 2;
    width: 34px;
    height: 34px;
    border: 2px solid var(--c);
    border-radius: 50%;
    pointer-events: none;
    animation: ripple 700ms var(--ease-out) forwards;
  }
  .click.other {
    border-style: dashed;
  }
  @keyframes ripple {
    from {
      scale: 0;
      opacity: 1;
    }
    to {
      scale: 1;
      opacity: 0;
    }
  }
  @media (prefers-reduced-motion: reduce) {
    .cursor {
      transition: none;
    }
    .click {
      animation: none;
      opacity: 0.7;
    }
  }
</style>

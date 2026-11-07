<script lang="ts">
  // Over a terminal tab in a ring: what each other person has selected, in their colour (the one
  // their pointer wears), under the pointers. Everyone selects on their own; everyone sees all of it.
  // The host (Terminal.svelte) says where a cell is in its box and which stable row is on top.
  import { COLORS } from "./api";
  import { memberColor, selectionRows } from "./coop";
  import { app } from "./state.svelte";

  /** `tab`: the tab's number in the ring. `top`: the stable row number of the top row on screen;
   *  `cols` × `rows`: the screen. `at`: a cell (col, row) → where it is in the host's box (CSS pixels). */
  let { tab, top, cols, rows, at }: { tab: number; top: number; cols: number; rows: number; at: (col: number, row: number) => [number, number] } = $props();

  const boxes = $derived(
    app.coop.selections
      .filter((s) => s.tab === tab)
      .flatMap((s) =>
        selectionRows(s, top, cols, rows).map(([y, from, to]) => {
          const [x0, y0] = at(from, y);
          const [x1, y1] = at(to, y + 1);
          return { key: `${s.user_id} ${y}`, color: COLORS[memberColor(s.user_id, COLORS.length)], x: x0, y: y0, w: x1 - x0, h: y1 - y0 };
        }),
      ),
  );
</script>

{#each boxes as b (b.key)}
  <div class="sel" style:--c={b.color} style:left="{b.x}px" style:top="{b.y}px" style:width="{b.w}px" style:height="{b.h}px"></div>
{/each}

<style>
  .sel {
    position: absolute;
    z-index: 1;
    pointer-events: none;
    background: color-mix(in srgb, var(--c) 30%, transparent);
  }
</style>

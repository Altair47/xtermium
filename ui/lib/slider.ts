/**
 * The light theme draws its own sliders (app.css), and CSS can't see a slider's value, so this
 * hands it over as `--f`, the filled share from 0 to 1, for the amber up to the knob. Pass the
 * bound value: an attachment re-runs when its argument changes, so a value set in code (a reset,
 * Undo, sync) moves the fill as well as one dragged.
 *
 *   <input type="range" bind:value={v} {@attach filled(v)} />
 */
export const filled = (_value: unknown) => (el: HTMLInputElement) => {
  const set = () => {
    const min = Number(el.min) || 0;
    const max = el.max === "" ? 100 : Number(el.max);
    el.style.setProperty("--f", String(max > min ? (Number(el.value) - min) / (max - min) : 0));
  };
  set();
  el.addEventListener("input", set);
  return () => el.removeEventListener("input", set);
};

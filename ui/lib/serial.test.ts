// Run with `npm test`: node runs TypeScript directly.
import assert from "node:assert/strict";
import { test } from "node:test";
import { oneAtATime } from "./serial.ts";

test("one run at a time; calls during one share the next run", async () => {
  let [runs, now, most] = [0, 0, 0];
  const run = oneAtATime(async () => {
    runs++;
    most = Math.max(most, ++now);
    await new Promise((r) => setTimeout(r, 5));
    now--;
  });
  const [a, b, c] = [run(), run(), run()];
  assert.equal(b, c, "the calls during the first share one run");
  await Promise.all([a, b, c]);
  assert.deepEqual([runs, most], [2, 1]);
  await run();
  assert.equal(runs, 3, "idle again: a call runs at once");
});

test("a run that fails doesn't stop the next", async () => {
  let runs = 0;
  const run = oneAtATime(async () => {
    if (++runs === 1) throw new Error("disk full");
  });
  const first = run().catch(() => "failed");
  await run();
  assert.equal(await first, "failed");
  assert.equal(runs, 2);
});

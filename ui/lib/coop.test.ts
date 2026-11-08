// Run with `npm test`: node runs TypeScript directly.
import assert from "node:assert/strict";
import { mock, test } from "node:test";
import { clock, memberColor, pace, passcode, PASSCODE_ALPHABET, rate, selectionRows, throttle, traffic } from "./coop.ts";

test("a passcode is upper case, from the alphabet, and 8 characters at most", () => {
  assert.equal(passcode("a7k2m9qx"), "A7K2M9QX");
  assert.equal(passcode("a7k2-m9qx"), "A7K2M9QX");
  assert.equal(passcode("  a7k2 m9qx "), "A7K2M9QX");
  // 0, O, 1 and I aren't in the alphabet; lower case o and i go with their capitals
  assert.equal(passcode("0O1Iio"), "");
  assert.equal(passcode("ABCDEFGHJKLM"), "ABCDEFGH");
  assert.equal(passcode("é€ßﬆ"), ""); // ß and the "st" ligature upper-case to two letters each
  assert.equal(passcode(""), "");
  assert.equal(passcode(PASSCODE_ALPHABET), PASSCODE_ALPHABET.slice(0, 8));
});

test("a countdown reads minutes and seconds", () => {
  assert.equal(clock(600), "10:00");
  assert.equal(clock(485.9), "8:05");
  assert.equal(clock(-3), "0:00");
});

test("traffic reads in b, Kb, Mb, Gb and Tb", () => {
  assert.equal(traffic(0), "0 b");
  assert.equal(traffic(0.4), "0 b");
  assert.equal(traffic(1023), "1023 b");
  assert.equal(traffic(1024), "1.0 Kb");
  assert.equal(traffic(12.4 * 1024), "12.4 Kb");
  assert.equal(traffic(5 * 1024 ** 2), "5.0 Mb");
  assert.equal(traffic(1.5 * 1024 ** 3), "1.5 Gb");
  assert.equal(traffic(3 * 1024 ** 4), "3.0 Tb");
  assert.equal(traffic(2048 * 1024 ** 4), "2048 Tb"); // nothing above Tb
  // Four characters at most: the decimal goes from 100 on, and 99.96 doesn't round up to five.
  assert.equal(traffic(99.9 * 1024), "99.9 Kb");
  assert.equal(traffic(99.96 * 1024), "100 Kb");
  assert.equal(traffic(148.7 * 1024), "149 Kb");
  assert.equal(traffic(1023.9 * 1024), "1024 Kb");
});

test("a rate is the counter's growth a second, and never negative", () => {
  assert.equal(rate(3000, 1000, 2), 1000);
  assert.equal(rate(1000, 1000, 1), 0);
  assert.equal(rate(10, 5000, 1), 0); // the person reconnected: their counter began again
  assert.equal(rate(10, 0, 0), 0);
});

test("a quiet ring has one slow packet, a busy one more and faster", () => {
  assert.deepEqual(pace(0), [1, 0.05]);
  assert.equal(pace(500)[0], 1);
  const [few, slow] = pace(2000);
  const [many, fast] = pace(5_000_000);
  assert.deepEqual([few, many], [2, 4]);
  assert.ok(fast > slow && slow > 0.05);
  // Never a blur: a lap takes 4 seconds at least, whatever the traffic.
  assert.equal(pace(1e15)[1], 0.25);
});

test("throttle sends the first call, then the last one of the wait", () => {
  mock.timers.enable({ apis: ["setTimeout"] });
  const seen: number[] = [];
  const send = throttle((n: number) => seen.push(n), 50);
  send(1);
  send(2);
  send(3);
  assert.deepEqual(seen, [1]);
  mock.timers.tick(50);
  assert.deepEqual(seen, [1, 3]);
  // quiet for a wait: the next call goes out at once
  mock.timers.tick(50);
  send(4);
  assert.deepEqual(seen, [1, 3, 4]);
  mock.timers.reset();
});

test("a ring member keeps one colour, the same everywhere, and members differ", () => {
  const n = 3;
  const ring = [{ id: "owner", owner: true }, { id: "a" }, { id: "b" }, { id: "c" }];
  assert.equal(memberColor("owner", ring, n), -1);
  const got = ["a", "b", "c"].map((id) => memberColor(id, ring, n));
  // same again, and three guests on three colours wear them all
  assert.deepEqual(got, ["a", "b", "c"].map((id) => memberColor(id, [...ring], n)));
  assert.deepEqual([...got].sort(), [0, 1, 2]);
  // someone not listed yet still gets a colour; so does a ring with no owner row
  for (const id of ["", "x".repeat(200)]) assert.ok(memberColor(id, [], n) >= 0 && memberColor(id, [], n) < n);
  // a guest's colour doesn't change when a guest listed after them leaves
  assert.equal(memberColor("a", ring, n), memberColor("a", ring.slice(0, 2), n));
});

test("a ring member's selection is drawn row by row, only what is on screen", () => {
  // Screen 10 × 3 whose top row is stable row 100.
  const rows = (a: [number, number], b: [number, number], block = false) => selectionRows({ a, b, block }, 100, 10, 3);
  assert.deepEqual(rows([100, 2], [100, 5]), [[0, 2, 6]]);
  // Backwards is the same; middle rows run to the edge.
  assert.deepEqual(rows([102, 1], [100, 8]), [
    [0, 8, 10],
    [1, 0, 10],
    [2, 0, 2],
  ]);
  // A rectangle: the same columns on each row.
  assert.deepEqual(rows([101, 6], [100, 3], true), [
    [0, 3, 7],
    [1, 3, 7],
  ]);
  // Scrolled off above or below: nothing; partly: the part on screen.
  assert.deepEqual(rows([50, 0], [60, 4]), []);
  assert.deepEqual(rows([200, 0], [210, 4]), []);
  assert.deepEqual(rows([90, 0], [100, 4]), [[0, 0, 5]]);
  assert.deepEqual(rows([102, 3], [1_000_000, 0]), [[2, 3, 10]]);
});

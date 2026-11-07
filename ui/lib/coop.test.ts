// Run with `npm test`: node runs TypeScript directly.
import assert from "node:assert/strict";
import { mock, test } from "node:test";
import { clock, memberColor, pace, passcode, PASSCODE_ALPHABET, rate, throttle, traffic } from "./coop.ts";

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
  const n = 12;
  assert.equal(memberColor("user_abc123", n), memberColor("user_abc123", n));
  for (const id of ["", "a", "user_abc123", "x".repeat(200)]) assert.ok(memberColor(id, n) >= 0 && memberColor(id, n) < n);
  assert.notEqual(memberColor("user_abc123", n), memberColor("user_abc124", n));
});

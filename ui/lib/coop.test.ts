// Run with `npm test`: node runs TypeScript directly.
import assert from "node:assert/strict";
import { mock, test } from "node:test";
import { clock, passcode, PASSCODE_ALPHABET, throttle } from "./coop.ts";

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

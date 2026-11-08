// Terminal Co-op helpers that need no window, so `npm test` can run them (coop.test.ts).

/** The characters a passcode is made of (xtermium-api: no 0, O, 1 or I, which read alike). */
export const PASSCODE_ALPHABET = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
export const PASSCODE_LENGTH = 8;

/** What was typed or pasted as a passcode: upper case, only the passcode alphabet, at most 8
 *  characters. Spaces and dashes people paste in ("A7K2-M9QX") fall away with the rest. */
export const passcode = (typed: string) =>
  [...typed]
    .map((c) => c.toUpperCase()) // per character: "ß" becomes "SS", which is no letter of the alphabet
    .filter((c) => c.length === 1 && PASSCODE_ALPHABET.includes(c))
    .slice(0, PASSCODE_LENGTH)
    .join("");

/** Seconds as "8:05", for a passcode counting down. */
export const clock = (secs: number) => {
  const s = Math.max(0, Math.floor(secs));
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
};

/** What a person in the ring sends and receives: bytes a second now (`up`, `down`), and bytes in all
 *  since their connection opened. */
export type RingFlow = { up: number; down: number; sent: number; received: number };

/** Bytes as the ring shows them, "12.4 Kb": steps of 1024, in b, Kb, Mb, Gb and Tb. The number
 *  is four characters at most (one decimal under 100, none from there), so it fits a fixed room. */
export const traffic = (bytes: number) => {
  const units = ["b", "Kb", "Mb", "Gb", "Tb"];
  const i = bytes < 1 ? 0 : Math.min(Math.floor(Math.log2(bytes) / 10), units.length - 1);
  const v = bytes / 2 ** (10 * i);
  return `${v.toFixed(i && v < 99.95 ? 1 : 0)} ${units[i]}`;
};

/** Bytes a second between two readings of a counter, `secs` apart. A counter that started again (the
 *  person reconnected) reads 0 once, not a negative rate. */
export const rate = (now: number, before: number, secs: number) => (secs > 0 && now >= before ? (now - before) / secs : 0);

/** How many people the ring is drawn for (CoopRing.svelte: the owner in the middle, one at each
 *  end). A ring of more is a plain list: there is no room to draw them and their traffic. */
export const RING_SEATS = 3;

/** How many packets travel the ring and how many laps a second they make, for `rate` bytes a
 *  second. One slow packet while the ring is quiet (a lap in 20 seconds), two from a kilobyte a
 *  second, four from a megabyte, and never faster than a lap in 4 seconds: it shows that traffic
 *  flows and roughly how much, the figures say the rest. */
export const pace = (rate: number): [number, number] => [rate < 1024 ? 1 : rate < 1024 ** 2 ? 2 : 4, Math.min(0.25, 0.05 + 0.03 * Math.log10(Math.max(rate, 1)))];

/** `fn` at most once every `ms`: the first call goes out at once, and of the calls during the wait
 *  the last one goes out when it's over, so a pointer that came to rest is where the others see it. */
export function throttle<A extends unknown[]>(fn: (...args: A) => void, ms: number): (...args: A) => void {
  let waiting: ReturnType<typeof setTimeout> | 0 = 0;
  let later: A | null = null;
  const go = (args: A) => {
    fn(...args);
    waiting = setTimeout(() => {
      waiting = 0;
      const next = later;
      later = null;
      if (next) go(next);
    }, ms);
  };
  return (...args) => (waiting ? void (later = args) : go(args));
}

/** A selection in a shared terminal: its ends (stable row, column) and whether it is a rectangle. */
export type RingSelection = { a: [number, number]; b: [number, number]; block: boolean };

/** The rows of selection `s` on a screen `cols` wide and `rows` high whose top row is stable row
 *  `top`: [row on screen, first column, one past the last]. Only what is on screen. */
export function selectionRows(s: RingSelection, top: number, cols: number, rows: number): [number, number, number][] {
  let [a, b] = [s.a, s.b];
  if (s.block)
    [a, b] = [
      [Math.min(a[0], b[0]), Math.min(a[1], b[1])],
      [Math.max(a[0], b[0]), Math.max(a[1], b[1])],
    ];
  else if (b[0] < a[0] || (b[0] === a[0] && b[1] < a[1])) [a, b] = [b, a];
  const out: [number, number, number][] = [];
  for (let y = Math.max(0, a[0] - top); y < rows && top + y <= b[0]; y++) {
    const r = top + y;
    const from = s.block || r === a[0] ? a[1] : 0;
    const to = Math.min(cols, s.block || r === b[0] ? b[1] + 1 : cols);
    if (to > from) out.push([y, from, to]);
  }
  return out;
}

const hash = (id: string) => [...id].reduce((h, c) => (h * 31 + c.charCodeAt(0)) >>> 0, 7);

/** Which of `n` colours a ring member keeps, the same on every computer, so everyone knows who is
 *  who: their pointer (CoopCursors.svelte), their selection in a shared terminal
 *  (CoopSelections.svelte) and the ring round their picture (CoopPic.svelte) wear it. The owner
 *  is -1, the ring's amber. Guests take, in the order the server lists them (the same list
 *  everywhere), the first colour still free from the one their id hashes to, so no two share one
 *  while colours are left. Someone the list hasn't got yet keeps their hashed colour. */
export function memberColor(id: string, members: { id: string; owner?: boolean }[], n: number): number {
  const taken = new Set<number>();
  for (const m of members) {
    if (m.owner) {
      if (m.id === id) return -1;
      continue;
    }
    let at = hash(m.id) % n;
    for (let tries = 0; tries < n && taken.has(at); tries++) at = (at + 1) % n;
    taken.add(at);
    if (m.id === id) return at;
  }
  return hash(id) % n;
}

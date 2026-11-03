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

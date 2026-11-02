// One run at a time of an async job, for work whose runs must land in order.

/**
 * `job`, run one at a time: a call while it runs waits for it, and however many calls came
 * meanwhile, they share one run after it (which sees the state as it is then). Every promise
 * returned settles once a run that started after its call is done.
 */
export function oneAtATime(job: () => Promise<void>): () => Promise<void> {
  let running: Promise<void> | null = null;
  let waiting: Promise<void> | null = null;
  const run = (): Promise<void> => {
    if (running) {
      const next = () => ((waiting = null), run());
      return (waiting ??= running.then(next, next));
    }
    return (running = job().finally(() => (running = null)));
  };
  return run;
}

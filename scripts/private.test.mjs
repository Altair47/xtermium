// Nothing personal or secret is tracked: no home folder with a user name in its path (a tool's
// hook or snapshot written on one machine), no private key, certificate or API token. The demo
// user `sotiris` and placeholders are fine. Run by `npm test`; the same check lives in the other
// xtermium repositories. A secret that got in is rotated, not only removed: history keeps it.
import { test } from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";

/** User names a home path may carry: the demo user, the tests' made-up ones, placeholders, dot folders. */
const ALLOWED = /^(sotiris|bob|alice|bakdim|you|user|name|me|…|\*|\$\{?USER\}?|%USERNAME%|<[^>]+>|Applications|\..*)$/i;
const PATTERNS = {
  "a home folder with a user name": /(?:[A-Za-z]:\\\\?Users\\\\?|\/home\/|\/Users\/)([^\s"'`\\/:<>|,)]+)/g,
  "a private key": /-----BEGIN [A-Z ]*PRIVATE KEY-----\r?\n[A-Za-z0-9+/=\r\n]{64,}/g,
  "a certificate": /-----BEGIN CERTIFICATE-----\r?\n[A-Za-z0-9+/=\r\n]{64,}/g,
  "a GitHub token": /\b(?:ghp|gho|ghu|ghs)_[A-Za-z0-9]{20,}|\bgithub_pat_[A-Za-z0-9_]{20,}/g,
  "an API key": /\bsk-(?:ant-)?[A-Za-z0-9_-]{24,}/g,
  "an AWS key": /\bAKIA[0-9A-Z]{16}\b/g,
  "a Slack token": /\bxox[baprs]-[A-Za-z0-9-]{10,}/g,
};
const SKIP = /\.(png|jpe?g|webp|gif|ico|woff2?|ttf|otf|icns|zip|exe|dll|pdf|lock)$|-lock\.json$|^patches\/|(^|\/)private\.test\.(mjs|ts)$/;

test("nothing personal or secret is tracked", () => {
  const files = execFileSync("git", ["ls-files", "-z"], { encoding: "utf8" }).split("\0").filter((f) => f && !SKIP.test(f));
  const found = [];
  for (const f of files) {
    let text;
    try {
      text = readFileSync(f, "utf8");
    } catch {
      continue;
    }
    for (const [what, re] of Object.entries(PATTERNS)) {
      for (const m of text.matchAll(re)) {
        if (m[1] !== undefined && ALLOWED.test(m[1])) continue;
        const line = text.slice(0, m.index).split("\n").length;
        found.push(`${f}:${line}: ${what} (${m[0].slice(0, 48)})`);
      }
    }
  }
  assert.deepEqual(found, [], "take it out of the commit, or ignore the file (CONTRIBUTING.md, \"What never goes into a commit\")");
});

---
name: xtermium-contributing
description: Use when preparing a change to the xtermium app repository for review, creating a branch, writing commit messages or a pull request, deciding whether to change the version, handling a Dependabot alert or audit finding, reporting a vulnerability, or answering a licence, NOTICE or third-party-notices question.
---

# Contributing to xtermium

## Overview

Every change reaches `main` as a pull request from a topic branch, merged with a merge commit. Commit bullets and pull request titles become the release notes, so they are written for a developer reading what changed. Versions follow semver and only maintainers raise them, because a merge that raises the version publishes a release. This skill is `CONTRIBUTING.md` plus the repository's settings and release machinery, in one place.

## When to use

- Starting work on a fix or feature.
- Writing commit messages or a pull request description.
- Wondering whether to touch `Cargo.toml` / `package.json` versions.
- A Dependabot alert, `cargo audit` or `npm audit` finding.
- A security problem found in the app.
- A question about the licence, `NOTICE`, `THIRD-PARTY-NOTICES.txt` or the name.

## Quick reference

| Step | What |
|---|---|
| Large change | open an issue first to agree on the approach |
| Branch | `feat/<topic>` or `fix/<topic>` from an up-to-date `main` (outside contributors: in a fork) |
| Scope | one topic per pull request; don't reformat or reorder code you didn't touch |
| Checks | `npm run check && npm test`, `cargo clippy --all-targets && cargo test`, `(cd rdp && cargo clippy --all-targets)` when `rdp/` changed; `npm run rdp` once before plain `cargo` |
| Commit | short summary line, blank line, bullets saying what changed |
| Pull request | against `xtermium/xtermium` `main`; merged with a merge commit; the branch is deleted |
| Versions | leave them alone; maintainers raise them to release |
| Security problems | privately: Security › Report a vulnerability on the repository |

## Branches and pull requests

- Nobody pushes to `main`, maintainers included: the repository's ruleset requires a pull request and forbids force pushes and deleting `main`.
- Merge commits only: squash and rebase merges are off, and the merged branch is deleted.
- Start the branch from an up-to-date `main`:

  ```sh
  git switch main && git pull --ff-only
  git switch -c fix/<topic>
  ```

- One topic per pull request. A fix and an unrelated clean-up are two pull requests.
- Keep the diff to the change. `cargo fmt --check` already reports about 100 diffs on `main`, so never run `cargo fmt` over whole files; format only what you wrote.
- Describe in the pull request what changed, which checks passed (with counts where a tool prints them), and anything a maintainer must still do.
- Conventions come from `CLAUDE.md` (written for coding agents, but for everyone) and `DESIGN.md`; `npm test` enforces some of them (`xtermium-developing`, `xtermium-testing`).

## Commits and release notes

Each commit has a short summary line, then a body of bullets:

```
Lock gates automation, secure the instance port, keep website tabs out of the interface

- The app lock now refuses every MCP tool call and CLI command until someone unlocks xtermium on this computer; initialize, ping and tools/list still answer so assistants reconnect on their own
- The single-instance loopback port takes a per-run secret written next to the port; a message without it is dropped
```

- `scripts/changelog.mjs` turns history into release notes: one section per merged pull request (its title, then its commits' bullet lists), then commits made on `main` directly. A wrapped line joins its bullet; an indented `- ` stays nested; trailer lines such as `Co-Authored-By:` are dropped. `scripts/changelog.test.mjs` (in `npm test`) covers it.
- Preview the notes for everything since the last tag: `node scripts/changelog.mjs` (arguments: `[ref] [previous tag]`).
- So write the pull request title and the bullets for a developer reading what changed, not as a diary of the work.

## Versions and releases

- Leave the version alone (`CONTRIBUTING.md`). Maintainers raise it when they release.
- Semver, pre-1.0: a new feature or a breaking change raises the minor (0.3.0 → 0.4.0), a fix the patch (0.3.0 → 0.3.1).
- A bump changes `Cargo.toml` (the source of truth, read by the release workflow) and `package.json` (kept in step), plus their lockfiles' `xtermium` entries (`Cargo.lock`, `package-lock.json`). `tauri.conf.json` has no version; `rdp/Cargo.toml` is its own crate and is not bumped.
- `.github/workflows/release.yml` runs on every push to `main` (and by hand). Its `version` job reads the version from `Cargo.toml`; if tag `v<version>` exists it prints "v<version> is released already: nothing to publish." and stops. Otherwise it builds on `windows-latest` and `ubuntu-22.04` (the macOS entry is commented out), regenerates `THIRD-PARTY-NOTICES.txt` with cargo-about, writes the notes with `scripts/changelog.mjs`, and publishes `v<version>` named `xtermium <version>`, not a draft, not a prerelease, with the build number in `XTERMIUM_BUILD`.
- Assets: `xtermium_<v>_x64-setup.exe`, `xtermium_<v>_x64_en-US.msi`, `xtermium_<v>_x64-portable.zip`, `xtermium_<v>_amd64.deb`, `xtermium-<v>-1.x86_64.rpm`, `xtermium_<v>_amd64.AppImage`, each with a sha256 digest on GitHub.
- So any merge that raises the version publishes a release; a merge that doesn't publishes nothing.

## Dependencies, Dependabot and audits

- The app has two Rust lockfiles: `Cargo.lock` and `rdp/Cargo.lock` (IronRDP and russh pin incompatible pre-release crypto crates, so `rdp/` is its own crate). A Rust dependency fix goes into both.
- Run the audits after any dependency change:

  ```sh
  cargo audit && (cd rdp && cargo audit) && npm audit
  ```

- When an alert has no fix, or doesn't apply, say so and why in the commit or pull request. Accepted `cargo audit` advisories carry their reason in `.cargo/audit.toml` and `rdp/.cargo/audit.toml`; the current list is in `xtermium-developing`'s security reference.
- After dependency changes, `npm run notices` rewrites `THIRD-PARTY-NOTICES.txt` (needs `cargo install --locked cargo-about --features cli`); release builds regenerate it anyway.

## Security problems

- Don't open a public issue. Report privately: **Security › Report a vulnerability** on the repository (private vulnerability reporting is on).
- Fixes follow the rules in `xtermium-developing` ("Rules for touching security"): one guard at the chokepoint, one test that fails without it, and a maintainer's agreement before changing core behaviour.

## Licence and credits

- The app is Apache-2.0 (`LICENSE`); your contribution is licensed under it.
- `NOTICE` holds the author's credit: every copy and fork keeps `LICENSE` and `NOTICE`, and a fork that shows credits shows that one among them.
- Third-party licences are quoted in full in `THIRD-PARTY-NOTICES.txt`; the installers carry it next to `LICENSE` and `NOTICE` (`tauri.conf.json` `bundle.resources`), and Settings › About opens it.
- "xtermium" and its logo are trademarks of the author; forks must use another name.
- The sync server is proprietary and not in this repository; the client talks to it through the API in `docs/sync.md`.

## Common mistakes

- Pushing to `main`, or asking for a squash or rebase merge.
- Two topics in one pull request, or a whole-file reformat inside a fix.
- Commit bodies without bullets, or bullets that describe the work instead of the change.
- Raising the version in a contributor pull request (it would publish a release on merge).
- Changing `package.json`'s version without `Cargo.toml`'s, or the reverse.
- Fixing a Rust advisory in `Cargo.lock` only and leaving `rdp/Cargo.lock` behind.
- Discussing a vulnerability in a public issue.
- Shipping a fork under the name "xtermium".

## Read next

- `CONTRIBUTING.md`: the source of these rules.
- `CLAUDE.md`: the project's conventions.
- `DESIGN.md`: the UI's rules.
- `xtermium-developing`: architecture, conventions, the security model.
- `xtermium-testing`: what to run and how to check a change in the running app.
- `xtermium-designing`: look, copy and screenshots.

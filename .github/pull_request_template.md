## What changed

<!-- For a developer reading the release notes: the title and your commit bullets become them. -->

Fixes #

## Why

<!-- The problem, or the issue where the approach was agreed. -->

## Checks

<!-- Tick what you ran, with the counts the tools print. -->

- [ ] `npm run check` and `npm test`
- [ ] `cargo clippy --all-targets` and `cargo test`
- [ ] `(cd rdp && cargo clippy --all-targets)`, if `rdp/` changed
- [ ] `cargo audit`, `(cd rdp && cargo audit)` and `npm audit`, if dependencies changed
- [ ] Seen working in the running app (say how: OS, connection kind)

## Screenshots

<!-- For anything you can see: before and after. Hide host names, addresses and anything private. -->

## Before you send it

- [ ] One topic; no reformatting of code I didn't touch
- [ ] The version in `Cargo.toml` and `package.json` is unchanged (maintainers raise it to release)
- [ ] UI text goes through the translation files; README and docs say what changed
- [ ] My contribution is licensed under the Apache License 2.0

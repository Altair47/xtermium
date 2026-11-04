# Security policy

## Supported versions

Only the [latest release](https://github.com/xtermium/xtermium/releases/latest) gets security fixes. xtermium never updates itself: Settings › About tells you when a newer release is out, and fixes ship as a new release.

## Reporting a vulnerability

Don't open a public issue, discussion or pull request for a security problem. Report it privately, either way:

- **On GitHub:** [Security › Report a vulnerability](https://github.com/xtermium/xtermium/security/advisories/new) on this repository.
- **By email:** [abuse@xtermium.com](mailto:abuse@xtermium.com).

Include what you can of:

- the version and build (Settings › About) and your operating system;
- what an attacker can do, and what they need first (local access, a malicious server, a crafted file, a network position);
- the steps to reproduce it, or a proof of concept;
- logs or a troubleshooting bundle (Settings › Troubleshoot › Create bundle), if they help. The bundle holds no passwords, tokens, names or addresses; read it before you send it.

You get a reply as soon as the report has been read. A confirmed problem is fixed in a new release, published as a GitHub security advisory, and credited to you unless you'd rather not be named. Please give a reasonable time to fix it before you make it public.

## Scope

- **In scope:** the app in this repository (the Rust backend, the UI, the RDP helper, the MCP server and the `xtermium` command line, the installers and the install scripts), and the hosts under xtermium.com and xtermium.gr, including the sync server at account.xtermium.com.
- **Out of scope:** Creem, the sign-in providers, GitHub, Cloudflare's own platform and our hosting provider's networks; denial of service and load testing; spam, social engineering and physical access; findings that need an attacker who already controls your user account or your OS keychain.
- **Known and accepted:** the `cargo audit` advisories listed with their reasons in the README's [Security notes](https://github.com/xtermium/xtermium#security-notes).

## Testing

You may test the app on your own machines as you like, under its licence. For the hosted services, the rules are in the terms under [Security research](https://xtermium.com/terms#security): test only with accounts you made for the test, never open, change or delete anyone else's data (if you reach any, stop there and tell us), and keep to the scope above. Testing that keeps to these rules is authorised, and we will not take legal action over it.

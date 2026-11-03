# Agent Skills for xtermium

These are Agent Skills (agentskills.io format: `<name>/SKILL.md` plus `references/`) for anyone, human or coding agent, working on this repository: `xtermium-developing`, `xtermium-contributing`, `xtermium-testing`, `xtermium-designing` and `xtermium-mcp`.

Claude Code loads them from `.claude/skills` when it runs in this repository. Other runtimes read skills from `~/.agents/skills`: link each folder there (`ln -s "$PWD/.claude/skills/<name>" ~/.agents/skills/<name>`, or a directory junction on Windows).

To add one, make `<name>/SKILL.md` with frontmatter `name` (the folder's name) and `description` (starting "Use when…", triggers only, under 1024 characters), then Overview, When to use, Quick reference, the body, Common mistakes and Read next. Keep each fact in one skill and point to it by name from the others; put long material in `references/` and use repo-relative paths only.

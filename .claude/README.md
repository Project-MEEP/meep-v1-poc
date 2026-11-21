# `.claude/` settings

`settings.json` holds permission rules for Claude Code in this repository: files it must not read or edit (`data/`, env and key files), commands it must not run (pushes to `main`, force pushes, skipping hooks), and commands it must ask about first (`gh pr merge`, `gh pr ready`, and `gh api` calls that change things).

**These rules are agent guardrails, not a security boundary.** Pattern matching on command text can be bypassed (another spelling, another tool, a script). The control of record is branch protection on `main`: a pull request is required, the `all-green` check is required, administrators are included (enforce admins), and force pushes are blocked. See CLAUDE.md rules 5 and 11 and ENG-03.

JSON does not allow comments, so the explanation lives here.

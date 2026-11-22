# ENG-03: Pull requests only

- **ID:** ENG-03
- **Status:** Confirmed
- **Date:** 2026-11-19
- **Owner:** Owner decision (spec Engineering rules)

## Decision

Every change reaches main through a pull request, enforced by a branch protection rule; no direct pushes by anyone, including the owner and AI agents.

Branch protection on main: every change arrives by pull request, with no direct pushes by anyone; pull request required; "All checks green" required; branch must be up to date; applies to administrators; no force-push or deletion. Third-party actions are pinned to full commit hashes. The one exception is the repository's very first commit: GitHub can only protect a branch that exists, so the rule is turned on immediately after that commit, before any other work.

## Notes

Copied from the master spec (Engineering rules row ENG-03, the branch-protection paragraph, and the 2026-11-19 change-log entry), no new content. Added to fill a gap under STD-01.

# Project MEEP: rules for AI assistants

This repository is PUBLIC.

1. **Build only v1.0** (the names feature, app shell, shared UI, minimum
   foundation). Anything labeled v1.1, v2, v3, v4 or Deferred is context, not work.
2. **Only "Confirmed" decisions are binding.** If a default has not been
   confirmed by the owner, ask before building on it.
3. **Ambiguity rule:** if the spec does not answer a question, stop and ask the
   owner. Do not fill gaps with assumptions or your own platform preferences.
4. **No real personal data in the repo, ever.** No personal emails, phone
   numbers, addresses, account IDs, keys, tokens, or real records, in code, docs,
   comments, commits, issues or PRs. Use the synthetic fixtures in the spec.
   Real data lives only in the git-ignored `data/` folder. The one public
   identity is the owner's chosen publishing identity (spec PUB-ID).
5. **Pull requests only (ENG-03).** Never commit or push to `main`. Use a
   feature branch and open a PR. Merge only when every CI check is green.
6. **Commit identity** is the GitHub noreply address only.
7. **Warn, never block** (principle 26), except for data integrity and
   authorization.
8. Reference spec IDs (ID-04, R7, AT-28, ...) in commits, tests and PRs.
9. If PII is ever committed, stop and tell the owner. History must be rewritten.
10. Do not commit older spec revisions until their bylines are scrubbed.

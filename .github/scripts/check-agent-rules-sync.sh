#!/usr/bin/env bash
# VZ-02: CLAUDE.md and AGENTS.md carry the same shared rules between markers.
# Keep them identical; edit one, copy to the other.
set -u
ext() { sed -n '/<!-- shared-rules:start -->/,/<!-- shared-rules:end -->/p' "$1"; }
a=$(ext CLAUDE.md); b=$(ext AGENTS.md)
if [ -z "$a" ] || [ -z "$b" ]; then echo "FAIL: shared-rules markers missing in CLAUDE.md or AGENTS.md"; exit 1; fi
if [ "$a" != "$b" ]; then
  echo "FAIL: shared rules differ between CLAUDE.md and AGENTS.md"
  diff <(printf '%s\n' "$a") <(printf '%s\n' "$b")
  exit 1
fi
echo "CLAUDE.md and AGENTS.md shared rules match."

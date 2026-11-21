#!/usr/bin/env bash
# SEC-05 items 1 and 2: SQL only as static text through the storage wrapper; templates escape by default.
# Fails on: Box::leak, String::leak or Vec::leak anywhere; set_use_isolating(false) outside test code; rusqlite use outside crates/meep-storage; `|safe` in any template.
# Passes while no Rust or templates exist.
set -u
fail=0
rs=$(git ls-files '*.rs')
if [ -n "$rs" ]; then
  hits=$(printf '%s\n' "$rs" | xargs -d '\n' grep -nE 'Box::leak|String::leak|Vec::leak' 2>/dev/null || true)
  [ -n "$hits" ] && { echo "FAIL: Box::leak, String::leak or Vec::leak (SEC-05 item 1):"; echo "$hits"; fail=1; }
  iso=""
  while IFS= read -r f; do
    # join lines so a call split across lines is still matched
    if tr '\n\r' '  ' < "$f" | grep -qE 'set_use_isolating[[:space:]]*\([[:space:]]*false'; then iso="$iso$f"$'\n'; fi
  done < <(printf '%s\n' "$rs" | grep -vE '/tests/|_test\.rs$')
  [ -n "$iso" ] && { echo "FAIL: Fluent isolation turned off outside tests (SEC-05 item 5):"; echo "$iso"; fail=1; }
  outside=$(printf '%s\n' "$rs" | grep -v '^crates/meep-storage/' || true)
  if [ -n "$outside" ]; then
    hits=$(printf '%s\n' "$outside" | xargs -d '\n' grep -nE 'use rusqlite|rusqlite::|\.(execute_batch|prepare|prepare_cached)\(' 2>/dev/null || true)
    [ -n "$hits" ] && { echo "FAIL: database access outside crates/meep-storage (SEC-05 item 1):"; echo "$hits"; fail=1; }
  fi
fi
tpl=$(git ls-files | grep -E '\.(html|htm|jinja|j2|askama)$|/templates/' | grep -v '^specs/\|^docs/' || true)
if [ -n "$tpl" ]; then
  hits=$(printf '%s\n' "$tpl" | xargs -d '\n' grep -nE '\|[[:space:]]*safe\b|escape[[:space:]]*=[[:space:]]*"?none' 2>/dev/null || true)
  [ -n "$hits" ] && { echo "FAIL: |safe or disabled escaping in a template (SEC-05 item 2):"; echo "$hits"; fail=1; }
fi
[ "$fail" -eq 0 ] && echo "check-sec05: clean"
exit "$fail"

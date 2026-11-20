#!/usr/bin/env bash
# LOG-01 item 6: the only way to log is crates/meep-core/src/log (typed API).
# Fails on print/println/eprint/eprintln/dbg and direct tracing or log macros elsewhere in library and binary code.
# Tests may print. Passes while no Rust exists.
set -u
rs=$(git ls-files '*.rs' | grep -v '/tests/\|^tests/\|/benches/\|/fuzz/\|^crates/meep-core/src/log/' || true)
[ -z "$rs" ] && { echo "check-logging: no Rust yet"; exit 0; }
hits=$(printf '%s\n' "$rs" | xargs grep -nE '\b(println|eprintln|print|eprint|dbg)!|\b(tracing|log)::(trace|debug|info|warn|error)!|use (tracing|log)\b' 2>/dev/null || true)
if [ -n "$hits" ]; then echo "FAIL: logging outside the typed log module (LOG-01 item 6):"; echo "$hits"; exit 1; fi
echo "check-logging: clean"

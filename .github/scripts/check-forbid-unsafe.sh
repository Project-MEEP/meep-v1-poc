#!/usr/bin/env bash
# VZ-01 item 2: forbid unsafe code in every Meep crate. Run from the repository root.
set -u
if [ ! -f Cargo.toml ]; then
  echo "No Cargo.toml yet; unsafe-code check skipped."
  exit 0
fi
fail=0

# 1. The workspace lint forbids unsafe code and every member inherits it (this covers tests too).
grep -Eq '^unsafe_code *= *"forbid"' Cargo.toml || { echo 'FAIL: [workspace.lints.rust] must set unsafe_code = "forbid"'; fail=1; }
meta=$(cargo metadata --format-version 1 --no-deps)
while IFS= read -r manifest; do
  if ! { grep -Eq '^\[lints\]' "$manifest" && grep -Eq '^workspace *= *true' "$manifest"; }; then
    echo "FAIL: $manifest does not inherit the workspace lints"; fail=1
  fi
done < <(printf '%s' "$meta" | jq -r '.packages[].manifest_path')

# 2. Every library and binary crate root declares #![forbid(unsafe_code)].
while IFS= read -r src; do
  head -n 20 "$src" | grep -q '#!\[forbid(unsafe_code)\]' || { echo "FAIL: $src lacks #![forbid(unsafe_code)]"; fail=1; }
done < <(printf '%s' "$meta" | jq -r '.packages[].targets[] | select(.kind | any(. == "lib" or . == "bin" or . == "proc-macro")) | .src_path')

# 3. No overrides of the lint.
if grep -rEn 'allow\(unsafe_code\)|unsafe_code *= *"(allow|warn)"' --include='*.rs' --include='Cargo.toml' crates tests Cargo.toml 2>/dev/null; then
  echo "FAIL: an unsafe_code override was found"; fail=1
fi

[ "$fail" -eq 0 ] && echo "forbid-unsafe: clean"
exit "$fail"

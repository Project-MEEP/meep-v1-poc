#!/usr/bin/env bash
# FOUND-07: fail if a test-only cargo feature is enabled in a release build or shipped in an artifact.
# Run from the repository root. Spec 015's release and signed artifacts must run this too.
set -u
FEATURES="plaintext-test-store in-memory-key-store"
MARKERS="MEEP_TEST_FEATURE_PLAINTEXT_STORE MEEP_TEST_FEATURE_IN_MEMORY_KEY_STORE"

if [ ! -f Cargo.toml ]; then
  echo "No Cargo.toml yet; release-feature check skipped."
  exit 0
fi

fail=0

# 1. Never in a default feature set of any workspace package.
defaults=$(cargo metadata --format-version 1 --no-deps | jq -r '.packages[] | .features.default[]?')
for f in $FEATURES; do
  if printf '%s\n' "$defaults" | grep -qx "$f"; then
    echo "FAIL: $f is in a default feature set"; fail=1
  fi
done

# 2. Not in the resolved feature tree of a normal release build.
tree=$(cargo tree --workspace --edges normal,build -e features --prefix none 2>/dev/null)
for f in $FEATURES; do
  if printf '%s\n' "$tree" | grep -q "feature \"$f\""; then
    echo "FAIL: $f is enabled in the release feature tree"; fail=1
  fi
done

# 3. Not in any built artifact: build in release mode and look for the marker strings.
cargo build --release --workspace --locked || { echo "FAIL: release build failed"; exit 1; }
for a in $(find target/release -maxdepth 1 -type f \( -perm -u+x -o -name '*.rlib' -o -name '*.so' -o -name '*.dll' -o -name '*.dylib' \) 2>/dev/null); do
  for m in $MARKERS; do
    if grep -aq "$m" "$a"; then echo "FAIL: marker $m found in $a"; fail=1; fi
  done
done

[ "$fail" -eq 0 ] && echo "release-features: clean"
exit "$fail"

#!/usr/bin/env bash
# Guard: keep real data, secrets and personal contact info out of the public repo.
# Run from the repository root. Exits non-zero on any finding.
set -u
fail=0

files=$(git ls-files)

# 1. Real-data folder must never be tracked
if printf '%s\n' "$files" | grep -E '^data/' >/dev/null; then
  echo "FAIL: files under data/ are tracked:"; printf '%s\n' "$files" | grep -E '^data/'; fail=1
fi

# 2. Images and PDFs only inside synthetic test fixtures
bad=$(printf '%s\n' "$files" | grep -Ei '\.(png|jpe?g|gif|webp|heic|bmp|tiff?|pdf)$' | grep -Ev '^tests/fixtures/' || true)
if [ -n "$bad" ]; then
  echo "FAIL: image/PDF outside tests/fixtures/:"; echo "$bad"; fail=1
fi

# 3. Secrets and key material (this script and the workflow are excluded: they name the patterns)
scan=$(printf '%s\n' "$files" | grep -Ev '^\.github/(scripts/guard\.sh|workflows/ci\.yml)$' || true)
secrets=$(printf '%s\n' "$scan" | xargs -r grep -InE \
  '(gh[pousr]_[A-Za-z0-9]{20,}|sk-[A-Za-z0-9_-]{20,}|AKIA[0-9A-Z]{16}|-----BEGIN [A-Z ]*PRIVATE KEY-----)' 2>/dev/null || true)
if [ -n "$secrets" ]; then
  echo "FAIL: possible secret:"; echo "$secrets"; fail=1
fi

# 4. Personal email addresses (example.* and GitHub noreply are allowed)
emails=$(printf '%s\n' "$scan" | xargs -r grep -InEo \
  '[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}' 2>/dev/null \
  | grep -Eiv '@(example\.(com|org|net)|users\.noreply\.github\.com)$' || true)
if [ -n "$emails" ]; then
  echo "FAIL: email address found:"; echo "$emails"; fail=1
fi

[ "$fail" -eq 0 ] && echo "guard: clean"
exit "$fail"

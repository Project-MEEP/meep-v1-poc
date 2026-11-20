#!/usr/bin/env bash
# VZ-01 item 4 (Proposed): AI security review of every pull request, part of the all-green gate.
#
# Provider-neutral harness (principle 11). The owner chooses a provider, adds its secret and sets
# the repository variable AI_REVIEW_ENFORCE=true. Until then this runs in REPORT-ONLY mode: it
# passes but writes a visible warning, so a missing review is never silent.
#
# Adapter contract: .github/scripts/ai-review-adapters/<AI_REVIEW_PROVIDER>.sh <diff-file> <findings-file>
#   exit 0 = no finding at or above "high"; exit 1 = findings at or above "high"; any other = error (fails).
set -u
summary() { echo "$1"; [ -n "${GITHUB_STEP_SUMMARY:-}" ] && printf '%s\n' "$1" >> "$GITHUB_STEP_SUMMARY"; return 0; }

base="${BASE_SHA:-}"; head="${HEAD_SHA:-}"
if [ -z "$base" ] || [ -z "$head" ]; then
  summary "AI security review: not a pull request; nothing to review."
  exit 0
fi

git diff --name-only "$base" "$head" > changed-files.txt
relevant=$(grep -E '\.(rs|toml|lock|sh|ps1|yml|yaml)$|^deny\.toml$' changed-files.txt || true)
if [ -z "$relevant" ]; then
  summary "AI security review: no source, build or workflow files changed; nothing to review."
  exit 0
fi

provider="${AI_REVIEW_PROVIDER:-}"
enforce="${AI_REVIEW_ENFORCE:-false}"
adapter=".github/scripts/ai-review-adapters/${provider}.sh"

if [ -z "$provider" ] || [ ! -f "$adapter" ]; then
  if [ "$enforce" = "true" ]; then
    summary "FAIL: AI security review is required but no provider adapter is configured (AI_REVIEW_PROVIDER='$provider')."
    exit 1
  fi
  summary "WARNING: AI security review is NOT configured, so this pull request was not reviewed. Files that need review: $(printf '%s' "$relevant" | tr '\n' ' '). The gate is not enforced until the owner chooses a provider, adds its secret and sets AI_REVIEW_ENFORCE=true (VZ-01)."
  exit 0
fi

git diff "$base" "$head" > pr.diff
bash "$adapter" pr.diff findings.json
code=$?
if [ "$code" -eq 0 ]; then
  summary "AI security review: no findings at or above high."
  exit 0
fi
[ -f findings.json ] && summary "$(head -c 4000 findings.json)"
if [ "$code" -eq 1 ]; then summary "FAIL: AI security review found issues at or above high."; else summary "FAIL: the AI review adapter failed (exit $code)."; fi
exit 1

#!/usr/bin/env bash
# VZ-02: the enforcing gate for the Claude Code security review.
# Inputs (env): PLAN_RUN (true|false), PLAN_REASON, ACTION_OUTCOME (success|failure|skipped),
#   STRUCTURED_OUTPUT (JSON {"findings":[{"severity","file","summary"}]}), AI_REVIEW_ENFORCE (true|false).
# Until the owner sets AI_REVIEW_ENFORCE=true this is REPORT-ONLY: it passes but warns loudly.
# Once enforced it fails closed: a review that should have run and did not, or any high/critical finding, fails.
set -u
summary() { echo "$1"; [ -n "${GITHUB_STEP_SUMMARY:-}" ] && printf '%s\n' "$1" >> "$GITHUB_STEP_SUMMARY"; return 0; }
enforce="${AI_REVIEW_ENFORCE:-false}"
fail() { if [ "$enforce" = "true" ]; then summary "FAIL: $1"; exit 1; fi; summary "WARNING (report-only, VZ-02): $1"; exit 0; }

if [ "${PLAN_RUN:-}" != "true" ]; then
  case "${PLAN_REASON:-}" in
    "not a pull request"|"no source, build, workflow or rules files changed") summary "AI security review: nothing to review (${PLAN_REASON})."; exit 0 ;;
    *) fail "the AI security review did not run: ${PLAN_REASON:-unknown}." ;;
  esac
fi
[ "${ACTION_OUTCOME:-}" = "success" ] || fail "the review action did not complete (${ACTION_OUTCOME:-unknown})."
command -v jq >/dev/null || { summary "FAIL: jq missing"; exit 1; }
if ! printf '%s' "${STRUCTURED_OUTPUT:-}" | jq -e '.findings | type == "array"' >/dev/null 2>&1; then
  fail "the review returned no valid structured result."
fi
blocking=$(printf '%s' "$STRUCTURED_OUTPUT" | jq -r '[.findings[] | select(.severity=="high" or .severity=="critical")] | length')
total=$(printf '%s' "$STRUCTURED_OUTPUT" | jq -r '.findings | length')
summary "AI security review: $total finding(s), $blocking at high or critical."
printf '%s' "$STRUCTURED_OUTPUT" | jq -r '.findings[] | "- [\(.severity)] \(.file): \(.summary)"' | while IFS= read -r l; do summary "$l"; done
[ "$blocking" -gt 0 ] && fail "$blocking finding(s) at high or critical."
exit 0

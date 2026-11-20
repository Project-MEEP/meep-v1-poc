#!/usr/bin/env bash
# VZ-02: decide whether the Claude Code security review runs on this pull request.
# Writes run=true|false and reason=... to $GITHUB_OUTPUT. Never fails the job by itself; the gate does.
# The review prompt is read from the BASE commit, so a pull request cannot rewrite its own reviewer.
set -u
out="${GITHUB_OUTPUT:-/dev/null}"
emit() { echo "run=$1" >> "$out"; echo "reason=$2" >> "$out"; echo "plan: run=$1 ($2)"; }

base="${BASE_SHA:-}"; head="${HEAD_SHA:-}"
[ -z "$base" ] || [ -z "$head" ] && { emit false "not a pull request"; exit 0; }

relevant=$(git diff --name-only "$base" "$head" | grep -E '\.(rs|toml|lock|sh|ps1|yml|yaml|md)$|^deny\.toml$|^\.github/' || true)
[ -z "$relevant" ] && { emit false "no source, build, workflow or rules files changed"; exit 0; }

if ! git cat-file -e "$base:.github/ai-review/security-review-prompt.md" 2>/dev/null; then
  emit false "no review prompt on the base commit yet (first pull request that adds it)"; exit 0
fi
git show "$base:.github/ai-review/security-review-prompt.md" > "${RUNNER_TEMP:-/tmp}/review-prompt.md"

[ -z "${HAS_TOKEN:-}" ] && { emit false "CLAUDE_CODE_OAUTH_TOKEN is not available to this run (not set, or a fork pull request)"; exit 0; }
emit true "review required"

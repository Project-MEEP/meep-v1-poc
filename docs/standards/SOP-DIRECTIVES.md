# SOP: directives and conformance audits

Status: Confirmed (STD-02, owner 2026-11-22). Amended 2026-11-27.
Home: `docs/standards/SOP-DIRECTIVES.md`

## 1. Purpose

Owner decisions are made in conversation and handed to an orchestrator agent, often running on a low-effort model, which turns them into code and specs. Two things can go wrong:

1. **The directive was underspecified,** so the agent filled gaps with its own guesses.
2. **The directive was clear, but the result drifted** silently, partly or through stale context.

This SOP covers both: how to **write** directives an agent will execute reliably (Part A), and how to **audit** what was built against what was asked (Part B).

## 2. Roles

| Role | Who | Owns |
|---|---|---|
| Owner | The repository owner | Decisions. Confirms or rejects. Gives merge go-ahead. |
| Reviewer | The assistant in the owner's planning chat | Drafts directives, runs pre-send checks, runs audits. Independent of the executor. |
| Orchestrator | The Claude Code session | Executes directives, reports with evidence, never self-certifies an audit. |
| Second reviewer | Codex on PRs | Adversarial code review. Not a conformance auditor. |

**Independence rule:** the agent that built something never audits its own conformance. It supplies evidence; the reviewer judges.

---

## Part A. Writing a directive

### A1. Every directive is a file

- Named `Meep-Directive-<Target>_<YYYY-MM-DD>_<HHMM>-CT.md` (NAMING.md §3).
- The orchestrator commits it to `docs/directives/` as the **first commit** of the PR it drives, before any other change.
- Chat-only pastes are not directives. A short answer that changes scope gets folded into a directive file, or recorded as an addendum file. Otherwise it can't be audited.
- Every instruction has an ID: `<directive file stem>#S<step>.<item>`, for example `Meep-Directive-ORC_2026-11-22_1119-CT#S1.3`. PR bodies and reports cite these IDs.

### A2. Required sections, in order

1. **Header:** target branch or PR, **expected head SHA** (the orchestrator stops if it differs), and which earlier directives this one supersedes.
2. **Owner confirmation line:** which items sending this confirms.
3. **Standing rules:** a reference to the standing-rules block, or a copy of it.
4. **Verified facts:** what the reviewer checked, with sources. This tells the *executing agent* not to re-derive them; if the work shows a fact is false, it stops and reports. It does not limit reviewers or CI, which always verify independently.
5. **Steps:** numbered, in order, one action per item. Each step has:
   - **Do:** the action, with exact file paths and IDs.
   - **EXACT** blocks for any text that must land verbatim: Fluent strings, record text, task text. Everything outside an EXACT block is instruction, not content.
   - **Accept when:** an observable check. Prefer a command (see A4).
   - **Stop if:** the conditions under which the agent stops and reports instead of deciding.
6. **Finish:** CI, the ready and merge rules, and the report format.

### A3. Wording rules for low-effort executors

| Don't write | Write instead | Why |
|---|---|---|
| "X or Y" (offering a choice) | One instruction, plus "Stop if X is impossible" | The agent will pick, and usually the easier path |
| "as discussed", "per my earlier answer" | Restate it in full | The agent can't see the chat |
| "un-ignore so it must fail" | "Remove `#[ignore]`. The test must PASS by asserting the refusal." | Pass/fail direction gets inverted |
| "add a method for X" | First check that X doesn't already exist, and name the existing method if it does | Prevents duplicates (`NoticeId::as_str` case) |
| "record it in FOUND" | "Take the next free FOUND number after checking `docs/decisions/`" | Prevents ID collisions (LANG-01, SEC-01, FOUND-07 cases) |
| "rebase onto main" | "Merge main into the branch; never force-push" | Force-push is denied by design |
| A reference to a decision or file | Check that it exists first; if not, the step creates it or stops | Dangling references (NAMES-07, UI-02 case) |
| "keep it simple", "as appropriate" | A concrete limit or rule | Vague modifiers get ignored |
| "and also…" inside a step | A separate numbered item | Compound items get done halfway |
| Notes indented under file content | Notes outside the EXACT block | Indented lines become content (Fluent continuation case) |
| "open the PR ready for review" | "Open as a draft; mark ready only after all-green (CLAUDE.md rule 11)" | A directive must not contradict the repo's standing rules |
| "report review findings, don't fix" | "Fix a review finding only if it is under 20 lines and has a test; otherwise answer the thread and stop (CLAUDE.md rule 11)" | Rule 11 requires every review thread fixed or answered before merge |

### A4. Acceptance checks

Write checks the auditor can run without judgment:

- A file contains text: `grep -q 'E-ADDRESS-UNREGISTERED' specs/002-foundation/contracts/diagnostic-codes.md`
- A test exists and passes: `cargo test -p meep-core save_refuses_unregistered_address`
- A record's status: `grep -q '^Status: Confirmed' docs/decisions/FOUND-11.md`
- Something is absent: `! grep -rq 'PoisonError::into_inner' crates/meep-storage`

Put the checks in the directive under **Accept when**. The orchestrator copies them into the PR body with their results.

### A5. Reviewer pre-send checklist

Run all of these before the owner sends a directive.

1. **Fresh state:** read the repo through the API or `gh`, not a cached raw copy. Record the head SHA in the header.
2. **Existence:** every referenced ID, file, type, function and task exists, or the step creates it.
3. **Conflicts:** no step contradicts a Confirmed record or an earlier owner answer. If one does, surface it to the owner as a decision before sending (Q1/Q2 toast case).
4. **Placement:** each check sits where the data is available (registry check at the port, not in a pure function).
5. **Dry-read as the executor:** read each step and ask whether it requires a judgment call. If yes, either decide it with a recommendation to the owner, or add a Stop-if.
6. **Truncation:** if the directive is longer than about 60 lines, send it as a file, not a paste.
7. **Supersession:** say which earlier directives or pastes this one replaces.
8. **Decisions are closed:** every owner-facing choice was presented as options, consequences and a recommendation, and has been answered.
9. **Standing rules:** read CLAUDE.md and AGENTS.md at the current head. No step may contradict them; if one must, surface it to the owner as a decision first.

### A6. Report template (orchestrator to owner)

```
Directive: <file stem>   Head before: <sha>   Head after: <sha>   PR: #<n>
Per step: <ID> — Done | Partial | Stopped — evidence (file:line / test / command output)
Deviations: <ID> — what differs and why
Not verified: what and why
Conflicts found: <record or file> — description
Owner questions: as decisions (options + recommendation)
```

---

## Part B. Conformance audit

### B1. When to audit

| Trigger | Scope |
|---|---|
| A PR driven by a directive is about to merge | That directive's steps |
| Before each phase gate (e.g. foundation done, before 010/001 code) | Every directive since the last gate |
| Owner asks, or a surprise shows up in a report | Targeted |

### B2. Procedure

1. **Build the intent ledger.** One row per instruction ID from the directive files in `docs/directives/`. Also include any owner answers given only in chat that changed scope; flag those rows "chat-only", since they're a process defect in themselves.
2. **Gather evidence.** For each row, run its acceptance check against `main`, or against the PR head if it's pre-merge. Record the file:line, test name or command output.
3. **Classify each row:**

   | Status | Meaning |
   |---|---|
   | Conforms | Built as asked, and the check passes |
   | Partial | Some of it was done |
   | Deviated, reported | Different, and the orchestrator said so |
   | Deviated, silent | Different, and not reported. Most serious. |
   | Not done | Missing |
   | Superseded | Replaced by a later owner decision; cite it |
   | Underspecified | The directive allowed more than one reading; the agent's reading was reasonable |

4. **Find the root cause** of every non-Conforms row:
   - **Directive defect:** it broke a rule in A2–A5.
   - **Execution defect:** the agent ignored a clear step.
   - **Context defect:** stale or cached state, or a truncated paste.
   - **Upstream defect:** a spec or record contradiction.
5. **Disposition** of each row, decided by the owner from the reviewer's recommendation:
   - fix in a follow-up PR
   - accept the deviation and record it in a decision record
   - re-issue the step in a new directive
6. **Feed back:** each directive defect adds a row to the A3 table, or an item to the A5 checklist.

### B3. Audit output

File: `docs/reviews/Meep-Review-Conformance-<scope>_<date>_<HHMM>-CT.md`, containing:

- the ledger table: ID, instruction summary, status, evidence, root cause, recommended disposition
- totals per status
- the owner decisions needed, as options with a recommendation

---

## Appendix: defects already seen (2026-11-19 to 2026-11-22)

| Case | Type | Mitigation now in this SOP |
|---|---|---|
| Directive cited NAMES-07, UI-02/03 and FOUND-09, which were never recorded | Directive / context | A5.2 existence check |
| SEC-01, LANG-01 and FOUND-07 ID collisions | Directive | A3 "next free number" rule |
| Reviewer read stale tasks.md through a cached raw URL | Context | A5.1 fresh state |
| Paste truncated mid-sentence | Context | A5.6 send as a file |
| Q1/Q2 defaults contradicted earlier owner answers | Directive | A5.3 conflicts |
| "Rebase" instruction collided with the force-push deny rule | Directive | A3 merge-main rule |
| `from_stored` told to check a registry it can't reach | Directive | A5.4 placement |
| "Un-ignore so it must fail" | Directive | A3 pass/fail wording |
| Fluent notes indented under content | Directive | A3 EXACT blocks |
| The same report re-sent; answers re-issued twice | Context | A1 directive files plus IDs make re-sends idempotent |
| Directive defaults sent as Confirmed on the owner's behalf | Directive | A2.2 an explicit owner-confirmation line |
| Directive said "ready for review"; CLAUDE.md rule 11 says draft first (#47) | Directive | A5.9 standing-rules check; A3 row |
| Directive said "don't fix" review findings; rule 11 requires fix or answer (#44) | Directive | A5.9; A3 row |
| "Don't re-derive" read by Codex as telling reviewers to skip checks (#44) | Directive | A2.4 scoped to the executor |

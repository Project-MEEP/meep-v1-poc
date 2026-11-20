# Meep Constitution

These 27 principles come from the owner's Meep Spec v1.0 (Constitution section,
2026-11-19). They bind every spec, plan and task; no assistant may override them.
Each is **Confirmed** unless marked **Proposed**. A Proposed principle is context,
not a binding rule, until the owner confirms it.

## Core Principles

### Identity and ownership

1. One human = one account = one logical instance, across any number of devices, apps and workers.
2. Every person owns their own data. Caregiver access never makes a patient's records the caregiver's.
3. Owner/Patient and Caregiver are base roles a person can hold separately or together. Admin is hidden, belongs to the app team only, and grants no general access to private records.
4. The owner grants and revokes all delegation, and can always revoke. An in-app role is application authorization, not proof of legal POA.
5. Caregiver access is least-privilege, task-scoped, view-only and temporary by default. Persistent copies need a separate, attributable opt-in.

### Structure

6. Domain-driven design: Domain → Namespace → Repository, addressed `INSTANCE.NAME.Domain.Namespace.Repository`. The owner defines domains one at a time.
7. Repositories are data-agnostic; source, provenance and authority are modeled explicitly.
8. POA exists both as a domain (legal documents) and as an access role (label provisional).
9. Views are standardized by role and authorization state. MVC is the analogy, not a chosen framework.

### Runtime and AI

10. The AI persona is a conversational skin over the application. It never holds authoritative permissions, data or workflow state.
11. Meep is AI-platform-agnostic. No vendor, model or product is assumed, and an assistant may not propose its own platform as a default.
12. Meep runs on its own owner-built, self-hosted system and harness, not on My Pal Clara or Indi.
13. Workers are replaceable; durable state survives the failure of any one of them.
14. Accounts interact peer-to-peer; access across accounts never transfers ownership.

### Security and privacy

15. *(Proposed, pending SEC-01)* Secrets stay isolated from the AI persona; vault adapters act on scoped credentials. The persona never asks for a credential in conversation.
16. Encryption in transit and at rest, audit logs, threat modeling and a secure SDLC wherever feasible. No certification is claimed.
17. Patient workspaces never mix data, AI context or job results. The one exception is the minimized cross-patient appointment view (CARE-07).
18. Free and open source under MIT, standards first, no mandatory paid cloud or credential provider.

### Scope

19. No diagnosis, treatment, dosing, triage or prescribing; Meep never presents itself as a provider.
20. Providers with their own systems document there. Meep imports and attributes; it never replaces their record.
21. v1 serves self-governing adults in mutual support. Minors wait for v4; wards for v3 or v4.
22. BYOx: if an adequate tool or standard exists, use it.

### Process

23. Requirements, specifications and engineering recommendations stay separate. Only the owner marks anything Confirmed.
24. Whiteboards are transcribed verbatim before anyone interprets them.
25. Spec Kit order: constitution, specify, clarify, plan, checklist, tasks, analyze, implement, converge.

### Authority

26. **Warn, never adjudicate.** Meep records, attributes and warns. It never blocks an owner's change on legal or factual grounds and never claims a document is valid. Every uploaded artifact is informational only. Meep still blocks for data integrity (a record that would be corrupt) and authorization (a delegate acting outside granted scope).
27. *(Proposed)* **No lock-in.** Every record can be exported in an open, documented format at any time, without Meep running.

## Engineering Constraints

- **Language (LANG-01):** Rust.
- **Clean code and patterns (ENG-01):** Robert C. Martin's clean code and Gang of Four patterns, adapted to Rust traits and enums rather than class inheritance.
- **Tests (ENG-02):** every feature ships unit tests and acceptance tests (Given/When/Then, from the AT list). CI runs them on every push and pull request, and `main` merges only when green.
- **License (LICENSE-01):** MIT. The MEEP name and logo are protected separately by `TRADEMARKS.md`, not by editing the MIT text (NAME-POL).
- **Egress (EGRESS-01):** AI extraction runs locally by default; cloud use is opt-in per item.
- **No personal data in this repository, ever.** Use only synthetic fixtures. Real data lives in a git-ignored `data/` folder. The one public identity is the owner's chosen publishing identity (PUB-ID).

## Development Workflow

- **Pull requests only (ENG-03):** every change reaches `main` through a pull request, enforced by branch protection, with no direct pushes by anyone, including the owner and AI agents. Merge only when every CI check is green.
- **One feature per point release (APPROACH):** v1.0 is names; the next feature starts only once the prior one is feature complete and merged with all tests green.
- **Only "Confirmed" is binding:** if a default has not been confirmed by the owner, ask before building on it. If the spec does not answer a question, stop and ask the owner.
- **Spec Kit runs per feature,** in the order of principle 25. IDs (ID-04, R7, AT-28 and so on) are cited in commits, tests and pull requests.
- **Decision records** live in `docs/decisions/`; every new confirmation lands as a reviewable pull request.

## Governance

This constitution supersedes other practices. Only the owner marks anything Confirmed
or changes a principle. Amendments are made by pull request, state what changed and
why, and bump the version below. All reviews verify compliance with it, and any
deviation must be justified in the pull request and approved by the owner. Day-to-day
rules for AI assistants are in `CLAUDE.md`.

**Version**: 1.0.0 | **Ratified**: 2026-11-19 | **Last Amended**: 2026-11-19

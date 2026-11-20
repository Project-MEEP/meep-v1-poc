# NAMES-07: Keystone changes need explicit confirmation

- **ID:** NAMES-07
- **Status:** Confirmed
- **Date:** 2026-11-20
- **Owner:** Owner

## Decision

Keystone changes require an explicit confirmation step and raise a toast (UI-02) plus an entry in the notifications tray (UI-03). A confirmation is an extra step, never a refusal.

**Plain confirmation:**
- changing the Legal name's text;
- making a different name the Legal name;
- marking the Legal name no longer used;
- deleting any name.

**Typed confirmation (irreversible):**
- Replace everything;
- permanent delete;
- restoring over an existing store.

The typed confirmation shows an explicit warning that the loss cannot be undone and offers to make a backup first.

**Not keystone:** editing Preferred, Nickname or Also known as entries; adding names; the pronoun opt-in; display settings.

## Notes

Spec 001 and its tasks are not yet amended to this list; the foundation's "Replace everything" uses the typed confirmation (spec 002 FR-017). That alignment is a follow-up.

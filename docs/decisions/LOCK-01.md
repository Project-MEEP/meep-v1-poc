# LOCK-01: vNext app auto-lock and idle

- **ID:** LOCK-01
- **Status:** Confirmed (direction, vNext)
- **Date:** 2026-11-20
- **Owner:** Owner
- **Source:** [Meep Access Review 2026-11-20 08:48 CT](../reviews/Meep-Review-Access_2026-11-20_0848-CT.md) (the owner's decisions of 09:46 CT)

## Decision

- Auto-lock after 30 seconds of idle by default; the owner may raise it to at most 2 minutes. It locks even if the device never does, and whenever the device locks.
- WCAG 2.2.1 through "extend": a warning at least 20 seconds before the lock; any key or tap extends; at least 10 extensions. The warning is `role="alertdialog"`, assertive (it interrupts screen readers).
- Lock is not close-out: it hides the screen and keeps drafts; resuming needs the device check.
- **Idle** means no key, pointer, touch or scroll and Meep's own read-aloud not playing. Other apps' audio never counts. There is no screen-reader detection. On Android (later), the TalkBack state may lengthen the warning window only and is never logged.

## Notes

Not v1.0. Belongs with AUTH-01 and spec 010's accessibility work.

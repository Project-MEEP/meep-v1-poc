# NAMES-01: Retired names in outbound exports

- **ID:** NAMES-01
- **Status:** Confirmed
- **Date:** 2026-11-19
- **Owner:** Owner decision (spec 001 clarification)

## Decision

1. Retired (No longer used) names ARE included in outbound exports: "old" in the standard healthcare format and as a Previous Name in the US health-data form (USCDI Patient Demographics data class: https://isp.healthit.gov/uscdi-data-class/patient-demographicsinformation).
2. Each name carries an outbound-sharing flag (default: share), with no control for it in v1.0.
3. A "Don't share this name outside Meep" control must ship before any automatic outbound sharing. **Entry condition for v2 provider exchange (HCP-X).**
4. The owner backup (the owner's own full JSON export) always includes every name (principle 27, Confirmed by CONST-27).

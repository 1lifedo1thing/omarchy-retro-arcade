# FreeSki delivery plan

Current implementation: `13fa3bf`, with evidence recorded in `4dc1f8f`.
Practice and seeded endless Free Ski are playable; movement and save hardening
have automated evidence. Tyler said the revised movement felt good and gave a
positive overall response to the endless pass. That does not establish full
chase, Slalom, difficulty or release acceptance.

Tracking issue: [#14](https://github.com/tcballard/omarchy-retro-arcade/issues/14).
The [requirements snapshot](REQUIREMENTS.md) preserves its proposal.
[System design](SYSTEM.md) owns responsibilities; [verification](VERIFICATION.md)
owns evidence and remaining acceptance gaps. Keep issue #14 open until its scope
and actual desktop acceptance are complete.

## Next: optional pursuit, through one shared run path

Use [NEXT.md](NEXT.md) as the implementation brief. Deliver these slices in order:

1. Extract one UI-independent session transition and bounded replay/evidence
   runner. Preserve current physics and schema-2 saves; demonstrate equivalence.
2. Add the pre-run chase option, warning, deterministic pursuit, swept catch,
   independent records and faithful paused restoration.
3. Prove the boundary cases, then tune warning and escape opportunities through
   human playtesting. Complete native, migration and staged-upgrade checks.

These slices form one feature pass. Avoid a broad framework rewrite or a second
physics path for reference runs. Current speed and turn behavior remain the
baseline unless new feedback justifies a documented revision.

## Delivery map

| Stage | Current status | Next exit evidence |
| --- | --- | --- |
| 1. Practice and movement | Implemented; positive feedback after speed/turn revision | Retain current regressions and continue mouse/readability testing as needed |
| 2. Simulation and saves | Implemented hardening; input replay is test-local | Shared session and reusable bounded evidence runner prove behavior equivalence before pursuit |
| 3a. Endless terrain | Implemented, separate record, 128 seeds × 5 km reference corpus | Human terrain-variety/difficulty observations; preserve bounded generation and sampled reachability |
| 3b. Optional creature | Planned in NEXT.md | Warning, pursuit, catch, chase-off isolation, records, saved continuation and credible evasive play |
| 4. Slalom | Not implemented | First course proves ordered gates, event timing, penalties, finish and save behavior; then expand to five courses and calibrate medals |
| 5. Presentation and release | Original art and native flows exist; sound/final acceptance pending | Original event-driven sound, mute/reduced effects, provenance, full native/package checks and hands-on Omarchy acceptance |

## After pursuit: one complete Slalom course

Extend the same session with course identity and objective state. Ordered gate
crossings, pole collisions, missed-gate resolution and finish must share the
chronological event contract. Slalom disables pursuit and keeps its own results.
Demonstrate direction/order, exactly-once penalties and save/replay equivalence
on one course before authoring four more. Medal targets need measured reference
runs and human calibration, not invented completion times.

## Integration and release

FreeSki is already registered in the Rust workspace, appended to the Arcade shelf,
and included in the existing desktop identity, native tests and package staging.
Preserve those integrations and all other games; there is no registration task to
repeat. Add new modes through the same lifecycle and independent FreeSki save.

Root [CONTRIBUTING.md](../../../CONTRIBUTING.md) and the
[CI workflow](../../../.github/workflows/arcade.yml) define combined gates.
A missing system Rust package currently blocks local Arch packaging; verify live
prerequisites when doing packaging work. Local builds/staged checks are separate
from a real package result and from human acceptance.

Use focused commits. Record integration choices in root DECISIONS.md, tuning
observations in TUNING.md and revision-specific results in VERIFICATION.md. Keep
runtime instructions in [AGENTS.md](../AGENTS.md) short and task-routed. When a
failure teaches a reusable lesson, retain the smallest regression case alongside
the owning code rather than adding another broad checklist.

Online play, leaderboards, trick combos, equipment, weather, moving NPC skiers and
course editing remain outside this delivery plan. The design pass changes plans;
it does not itself implement pursuit, replay tooling or the proposed session API.

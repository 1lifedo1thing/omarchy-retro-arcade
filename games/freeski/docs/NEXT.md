# Next delivery: an optional creature chase

Planned, not implemented. This is the next player-facing feature after the current
practice and endless build. The [system design](SYSTEM.md) defines its shared
contracts; [PLAN.md](PLAN.md) owns delivery status.

The intended experience is a readable warning followed by a pursuit the player can
influence through skiing. A catch ends the attempt clearly. Normal Free Ski keeps
its current feel and remains available without pursuit. Use original creature art.

## Slice A: one run path and reproducible cases

Before adding the creature, extract the session owner described in SYSTEM.md.
Move native full-run stepping, both fixture examples and terrain reference runs to
it. Keep the current physics and schema-2 save representation unchanged. Add the
small bounded headless replay/evidence entry point as a caller of that owner.

Acceptance:

- Capture representative practice/endless traces from `13fa3bf`, including a ramp,
  crash reset and chunk boundary; compare the extracted implementation tick by tick.
  Generate baselines from that revision, not from the implementation being checked.
- The 128-seed corpus and existing schema migrations still pass. Paused restores
  and future/invalid-file retention remain intact.
- Native input, 30/60/120 Hz, render resize and release-after-crash evidence all use
  the same session path. Failed cases identify the first divergent tick.
- App/examples no longer own their own chunk refresh/step sequence. There is one
  stored copy of run configuration and no new framework or service.

This is a bounded refactor supporting the chase, not a separate architecture
project. Code ownership can then divide cleanly at the session/actor/view boundary.

## Slice B: complete chase behavior behind a pre-run option

Implement chase as run configuration and explicit pursuit state, rather than an
independent animation loop or a second ticking UI object.

| Area | Proposed behavior / evidence |
| --- | --- |
| Choice | Chase defaults off; enable before a new Free Ski run. Changing it during an attempt requires the existing replacement flow. Practice/Slalom cannot enable it. |
| Trigger | Begin with the issue's proposed 1,000 m threshold as a tuning candidate. Crossing it starts one visible warning; use simulation ticks for its duration. |
| State | Dormant → warning → pursuit. A catch is a terminal run outcome with a distinct reason, not a fourth obstacle crash. Persist phase, timer and creature motion. |
| Spawn | Deterministic valid terrain position behind the skier after warning. Find it within a bounded query/search; if no fair site fits, stay warned and retry on a documented bounded schedule. Never teleport an active pursuer. |
| Movement | Bounded speed/acceleration/turning in world coordinates. Evaluate candidate values with production runs before choosing them. No hidden speed multiplier keyed to the player's screen, test mode or elapsed wall time. |
| Terrain | Use declared world queries and collision rules for the creature. Start with ordinary obstacle contact and deterministic local steering; document its footprint and any later exception. No silent phasing through trees. |
| Catch | Swept relative contact between both moving actors, ordered against other tick events. Jumping alone is not blanket immunity. One catch stops simulation and awards its mode's record once. |
| Recovery | Initial policy: catch protection covers the existing tumble/protection interval. The pursuer brakes outside a configured safe gap while protection is active; it does not teleport or overlap the skier and catch instantly on expiry. Validate this with ordinary recovery trajectories. |
| Pause/restore | Warning, pursuit, recovery and catch all freeze under pause/focus loss. Reopening is paused at the same positions/timers; no respawn or extra warning allowance. |
| Records | Preserve current Free Ski record as chase-off; add a distinct chase-on record. Show the selected category in HUD/results. No toggle can transfer an active run's score between categories. |
| View | Warning works with sound muted. The creature and offscreen direction/proximity indicator derive from the same world position used for catch. Keep the downhill route readable. |

The warning duration, spawn gap, actor geometry and pursuit speeds are provisional
until measured. Store selected values and reasons in TUNING.md when implemented;
do not scatter speculative numbers across plans and code.

## Slice C: fairness and lifecycle evidence, then human tuning

Use a small named set of production-input policies with explicit tick/distance
limits, and record seed, catch tick, closest separation and terminal reason.
Policies are test drivers, never player assistance shipped in the game.

| Scenario | What it must establish |
| --- | --- |
| Chase disabled | Current Free Ski continuation stays identical; no hidden pursuit state or record changes |
| Threshold crossed / paused during warning | Exactly one warning, no hidden countdown under pause, reproducible spawn |
| Straight descent versus committed turns | Quantify closing pressure; demonstrate a deliberate evasive line that gains separation for a stated interval, then assess it by hand |
| Braking, sideways stop and edge corridor | No accidental infinite immunity or discontinuous catch; recovery remains usable |
| Ramp and tree contact during pursuit | Same flight/obstacle rules; correct earliest event and a readable outcome |
| Both actors cross within one tick | Catch cannot tunnel or depend on render rate |
| Save during warning, pursuit and post-crash protection | Restored continuation matches uninterrupted state/events |
| Catch on record boundary / repeated close and reopen | Correct category, no duplicate award or resumed terminal run |
| Spawn query fails | Safe bounded fallback, with no actor appearing on the skier |

A policy surviving some seeds is feasibility evidence, not proof the chase is fun.
Tyler's next playtest should establish whether the warning is understood, a catch
has an apparent cause, and intentional steering gives a credible chance to escape.
Record actual observations; tune one explanatory variable at a time. Do not make
ordinary Free Ski harsher to compensate for a weak pursuit policy.

## Delivery boundary

Ship the local playable chase with help, original readable creature geometry,
save migration, independent records and the applicable native/upgrade evidence.
A visible warning is required; sound can use the later audio work without blocking
this slice. Do not wait for five Slalom courses or final art to test pursuit.

The next implementation request can use this brief directly. It need not repeat
settled movement values, existing feature setup or the full historical transcript.

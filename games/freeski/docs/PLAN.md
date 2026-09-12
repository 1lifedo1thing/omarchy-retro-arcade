# FreeSki delivery plan

Status: milestone 1 implemented and under acceptance; first human feedback implemented; revised feel/difficulty acceptance remains pending.
Tracking issue: [#14](https://github.com/tcballard/omarchy-retro-arcade/issues/14).
Working branch: `feat/freeski`.

The [requirements snapshot](REQUIREMENTS.md) preserves the full proposal.
This plan sequences that scope; passing an intermediate milestone does not close
the issue. Keep numerical defaults provisional in [TUNING.md](TUNING.md).

## Delivery sequence

| Milestone | Deliverable | Exit evidence | Status |
| --- | --- | --- | --- |
| 1. Playable practice slope | Native integration, one authored practice slope, carving, braking, look-ahead, ramps, crashes, keyboard/mouse, pause and basic save/resume | A human can play several minutes, understand hazards and recovery, and suspend/reopen the attempt; focused engine/storage and native checks | Implemented; human acceptance pending |
| 2. Simulation and persistence hardening | Tick-stamped replay, chronological physical events, versioned state validation, exactly-once results and complete lifecycle handling | Equivalent continuations across render schedules and save/reload; adverse input, save and event-order tests | Not started |
| 3. Endless Free Ski | Seeded bounded chunks, traversable connections, capped difficulty, distance records, optional creature pursuit | Documented production-engine seed corpus, bounded generation/memory, chase and recovery tests, human evasion evidence | Not started |
| 4. Five Slalom courses | One complete course first, then four more; ordered gates, penalties, finish, unlocks and medals | Production-engine reference completion for each course, timing/records tests and human-calibrated medals | Not started |
| 5. Presentation and release acceptance | Original final art/audio, readable effects, polished flows, help/About, provenance and packaging | Actual layout captures, workspace/native/package checks and separately recorded Omarchy/Wayland playtests | Not started |

Foundations are incremental: milestone 1 already uses a fixed-step engine and
serializable state. Milestone 2 proves and completes those contracts. Presentation
and input readability are developed throughout, with final polish in milestone 5.

## Proposed code boundaries

The Rust library is registered and hosted by Arcade. Simulation, collision, world,
input, app, rendering and storage modules are implemented. Course and audio
modules remain proposed for later milestones; no empty modules are added.

| Area | Responsibility |
| --- | --- |
| `src/lib.rs` | Library exports; engine usable without desktop features |
| `src/engine.rs` | Authoritative state, fixed ticks, movement, jumping and outcomes |
| `src/collision.rs` | Swept checks and deterministic chronological event resolution |
| `src/world.rs` | Practice terrain, later bounded seeded chunks and route validation |
| `src/course.rs` | Authored Slalom definitions, gates, finish and course versions |
| `src/input.rs` | Keyboard/mouse handover and normalized tick inputs |
| `src/app.rs` | Arcade lifecycle, accumulator, menus, focus and pause |
| `src/render.rs` | Camera, theme-aware native drawing and bounded visual effects |
| `src/storage.rs` | Versioned validated saves, private atomic writes and recovery |
| `src/audio.rs` | Original event-driven cues and owned playback lifecycle |
| `tests/` | Engine/replay/storage coverage and production-engine reference runs |

State should distinguish Ready, Running, Paused and Results, with explicit jump,
crash recovery and protection state. The practice implementation uses 60 Hz ticks and serializable f64 state.
Persist simulation state, not wall-clock instants or renderer state.

## Integration touchpoints

- Root `Cargo.toml`, `Cargo.lock`, `arcade/Cargo.toml`: register the library when
  implementation begins; preserve a headless engine build.
- `arcade/src/main.rs`: game ID, appended shelf order, preview, construction,
  suspend/exit behavior and About credit; inspect shelf assumptions as needed.
- Reuse `shared/presentation` for cabinet materials and established theme/input
  conventions. Inspect existing bounded-read/private atomic-write helpers before
  choosing a storage adapter; avoid a broad storage refactor for this game.
- `scripts/native-check.py`, `scripts/polish-renders.py`, a FreeSki native script,
  and `.github/workflows/arcade.yml`: switching, controls, saves and captures.
- `scripts/install.sh` and `packaging/`: inspect how assets/notices are staged;
  include FreeSki provenance and exercise installed-game upgrade preservation.
- Root README, DECISIONS, presentation and verification docs: update actual
  supported behavior and evidence as milestones land.

## Review and completion

Use focused, reviewable commits and link the eventual draft PR to #14. Keep
milestone status and [evidence](VERIFICATION.md) current. Each acceptance row must
identify its tested revision and an inspectable artifact or test before passing.
Close #14 only after the complete scope and desktop acceptance are satisfied.

Online play, global score submission, tricks, equipment, weather, moving NPC
skiers and course editing remain outside this v1 plan.

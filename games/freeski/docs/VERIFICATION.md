# FreeSki acceptance and evidence

Milestone 1 is implemented. Automated practice-slope checks pass; human control
feel and difficulty acceptance remains pending. Later milestones are not complete.
The full issue #14 remains open.

## Acceptance tracker

| ID | Acceptance area | Current evidence / status |
| --- | --- | --- |
| A1 | Movement/braking, continuous collision, jumping and single-event recovery | Practice engine tests pass, including high-speed sweeps, overlapping hazards, descent into a rock and safe recovery at every authored hazard |
| A2 | Gates, penalties, finish ordering and exactly-once records | Practice fatal-crash precedence and records tests pass; Slalom gates/timing remain later work |
| A3 | Connected seeds, recovery and bounded generation/memory | Authored practice recovery checked; 240-segment trail cap; endless generator/corpus not implemented |
| A4 | Creature warning, spawn, catch and isolation | Not implemented |
| A5 | Five complete courses and calibrated medals/chase | Practice reference completes; Slalom and human calibration not implemented |
| A6 | Mouse/keyboard flows, handover, overlays, focus and compact targets | Frontend and native scenarios pass; human feel evaluation pending |
| A7 | Save/reopen during jumps, recovery, pursuit and Slalom | Practice jump/recovery continuation and native restoration pass; unsupported saves retained; pursuit/Slalom not implemented |
| A8 | Render-rate equivalence and resize independence | Frontend replay at 30/60/120 Hz and alternating window sizes produces identical tick state; backlog pause tested |
| A9 | Actual theme/size/scale captures | Dark, light, compact, 200% X11 and local Wayland captures inspected |
| A10 | Workspace, native switching, package/upgrade and desktop acceptance | Workspace/build/native/staged reinstall checks pass; actual Arch package build/install acceptance and human Wayland playtest pending |
| A11 | Shelf/help/About, provenance, rules, controls and decisions | Added and reviewed for the practice milestone; later-mode help/assets remain later work |

Implementation source commit: `acf6286` (`feat/freeski`). Later documentation
commits add evidence without changing gameplay.

## 12 September 2026 implementation evidence

Environment: local x86_64 Omarchy 4.0.3; Rust 1.98.1. Native automation uses
Xvfb and Mesa software rendering. Desktop capture uses the actual Wayland backend.
The Rust toolchain was installed user-locally; Xvfb, Python test dependencies and
Stockfish were held in temporary/user-local test locations, with no privileged
system package installation.

| Check | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --locked --all-targets -- -D warnings` | Passed |
| `REQUIRE_STOCKFISH=1 cargo test --workspace --locked --all-targets` | Passed with a real Stockfish 17.1 executable; 231 tests including 18 FreeSki tests |
| `cargo test -p omarchy-freeski --locked --no-default-features` | Passed: 12 engine/storage tests; no desktop dependencies |
| `scripts/build.sh` | Passed: optimized Arcade and preserved C++ Pinball engine |
| Pinball `theme-palette`, `theme-path`, `authored-upstream-table` | Passed: all three |
| Desktop entry and script syntax | Passed |
| `scripts/native-freeski.py` | Passed normal controls, focus/help isolation, input ownership, pause/reopen, same-window shelf return and invalid-save retention |
| Production `practice-evidence` example + native fixture reopen | Passed mid-jump and crash-recovery restoration; inactive restored runs remain unchanged |
| `scripts/native-check.py` against staged installation | Passed singleton, eleven games in one window, Stockfish response, existing saves and clean shutdown |
| `scripts/install.sh` staging and reinstall | Passed: one desktop entry, all eleven game licences, FreeSki provenance; suspended mid-jump file unchanged across reinstall and reopen |
| Actual local Wayland launch/capture | Passed rendering and clean capture exit; interactive playtesting remains separate |
| `packaging/build-arch.sh` | Blocked at dependency validation: pacman has no installed `rust>=1.98` package. The user-local toolchain builds the app but does not satisfy the package database. No Arch package was built or installed; staged installation is separate evidence |
| Hands-on human playtest | Pending; no human findings have been invented |

The production reference in [engine tests](../tests/engine.rs) follows authored
waypoints with ordinary steering targets. It finishes in **3,397 ticks / 56.62 s**,
uses all three ramps and takes zero crashes. Separate tests compare uninterrupted
and save-restored trajectories for 400 subsequent ticks during flight/recovery.

Frontend checks in [app.rs](../src/app.rs) cover mouse start/pause/settings/restart,
held brake, focus loss, overlay isolation, new-run records, backlog handling and
render/resize equivalence. [Input tests](../src/input.rs) check intentional source
handover and mandatory release of held controls after suspension.

## Actual app captures

- [Dark skiing](captures/dark.png)
- [Light skiing](captures/light.png)
- [Compact skiing](captures/compact.png)
- [200% skiing](captures/200.png)
- [Restored jump](captures/mid-jump.png)
- [Restored crash recovery](captures/recovery.png)
- [Local Wayland ready screen](captures/wayland.png)

All are native application captures, not mockups. X11 layout captures show release
builds at 1280 × 900, 900 × 760 and 1800 × 1520 (200%). The Wayland compositor chose
a tiled 941 × 1030 window. Review checked unclipped controls, hazard silhouettes,
contrast and separation of HUD from the slope. Paused restore captures intentionally
show the resume overlay; the simulation state is verified separately.

## Findings and practical limits

- Concurrent debug software rendering at 200% during compilation triggered the
  intended backlog pause. An isolated release run passed. This is not a low-end
  performance certification; actual playing latency still needs a human check.
- Stockfish 19 timed out in the existing full-suite engine handshake on this host,
  while its isolated test passed. The full suite passed with Stockfish 17.1.
  No Chess timeout or engine behavior was changed for FreeSki.
- The existing all-game native harness assumed default Chess board colours but
  picked up the real user's theme. It now writes the expected palette into its
  temporary XDG state before making pixel assertions. User theme files are untouched.
- Existing Pinball/SDL output included `triangle area overflow` warnings during
  switching; its native checks and C++ tests passed. This is not FreeSki output.
- This milestone has no sound. Endless skiing, chase and Slalom remain later scope.

## Human playtest record

Record tester, revision, input method, theme, size/scale and desktop/backend.
Assess turn commitment, braking, hazard reaction time, jump and crash readability,
difficulty and resume behavior. Record findings and resulting numerical changes
in [TUNING.md](TUNING.md). Until that evidence exists, milestone 1 is implemented
but not fully accepted.

## Rules 2: first human feedback revision (12 September 2026)

Revision: the `fix(freeski): build speed and turn through full sideways headings`
commit following `cef046c` on `feat/freeski`. Tyler's initial keyboard playtest
requested much higher top speed and turning through 90° instead of leaning.
The resulting parameters and remaining human acceptance are in TUNING.md.

- PASS: workspace formatting, strict Clippy, all 234 workspace tests with
  Stockfish 17.1, and release Rust/native build. FreeSki has 21 frontend/engine/
  storage tests; its separate UI-free build passes all 14 engine/storage tests.
- PASS: production reference finishes in 1,714 ticks / 28.57 seconds with three
  jumps and zero crashes. Tests measure gradual speed buildup, true left/right
  traverses, keyboard release retaining heading, and legacy-save migration.
- PASS: native X11 dark, light, compact and 200% runs: visible start, mouse/keyboard
  handover, full 90° turn and retained heading after release/resume, pause/focus/
  help isolation, same-window shelf switching, close/reopen, mid-jump/recovery
  fixtures and corrupt-file retention. Actual captures were inspected at
  `/tmp/freeski-captures/v2-{dark,light,compact,200}/quarter-turn.png`.
- Native harness correction: at the faster speed its old five-second opening
  entered the tree section during the input test, causing a crash/reset in the
  compact run. The handover sequence now begins on open snow after 2.3 seconds;
  the compact rerun passes. No collision behavior was bypassed or relaxed.
- PASS: updated build opened as a native Wayland window. The actual user's save
  migrated exactly (only rules version changed; restored paused), and its
  original was verified in the migration backup. Revised human feel/readability
  acceptance remains pending.
- Packaging files are unchanged. The previous staged install/upgrade checks
  remain baseline evidence; package build remains blocked by the missing system
  `rust>=1.98` prerequisite documented above. Packaging and unchanged Pinball
  C++ tests were not repeated for this tuning-only revision.

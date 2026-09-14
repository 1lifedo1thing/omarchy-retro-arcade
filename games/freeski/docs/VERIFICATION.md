# FreeSki acceptance and evidence

The full local game is implemented: practice, endless terrain, optional creature
pursuit, five Slalom courses, medals, sound and resumable runs. Tyler previously
accepted the revised movement and responded positively to endless skiing. Human
ratings of the new pursuit/course difficulty and final desktop acceptance remain
open. The completion pass below records current checks; earlier sections are
historical evidence for their named revisions. Issue #14 remains open.

## Choosing checks

Use these task routes for the inner development loop. Root
[CONTRIBUTING.md](../../../CONTRIBUTING.md) and the
[CI workflow](../../../.github/workflows/arcade.yml) still define combined gates.
Run those applicable gates after integration; do not rerun unrelated suites merely
because another document changed.

| Changed responsibility | Discriminating evidence |
| --- | --- |
| Pure physics/contact | Named engine regression, then FreeSki headless suite |
| Terrain/window bounds | Deterministic overlap/ID checks and seed corpus in `tests/endless.rs` |
| Save/schema/records | Migration, original retention, record separation, invalid versions and exact continuation in `tests/hardening.rs` |
| Input/lifecycle/session integration | Frontend tests, especially multi-tick crash reset and chunk/render equivalence; real native input and reopen |
| Visuals or new controls | Actual dark/light/compact/200% app captures and readable keyboard/mouse flow |
| Package/install boundary | Committed-source package build, staged/native install and exact save preservation on upgrade, reported separately |
| Docs/plans/instructions only | Content and current-vs-proposed review, local links, referenced commands/symbols and `git diff --check`; report runtime tests as not rerun |

Existing commands, run from the repository root with the project's Rust/native
prerequisites available:

```sh
cargo test -p omarchy-freeski --locked --no-default-features
cargo test -p omarchy-freeski --locked --lib
freeski_evidence=$(mktemp -d /tmp/freeski-evidence.XXXXXX)
cargo run -p omarchy-freeski --locked --no-default-features --example practice-evidence -- "$freeski_evidence/fixtures"
cargo run -p omarchy-freeski --locked --no-default-features --example endless-evidence -- "$freeski_evidence/fixtures"
cargo run -p omarchy-freeski --locked --no-default-features --example completion-evidence -- "$freeski_evidence/fixtures"
```

The examples create synthetic saves through ordinary inputs. They are current
commands. The `replay` example consumes an initial Save, absolute consecutive
input tick numbers, a relative tick limit (1–1,000,000), optional per-tick expected
Sim and optional final expected phase/ticks/distance. Files are limited to 1 MiB;
Ready/Paused initial states resume deliberately. It reports the first divergence
or a compact summary to stdout. See examples/replay.rs for the JSON structures.
The native script takes the release binary, output directory and layout variant;
its `FREESKI_FIXTURES` variable points at the generated fixture directory. See CI
for the full invocation and native/Pillow dependencies. On a Wayland desktop,
unset `WAYLAND_DISPLAY` for Xvfb tests so the real desktop is not selected.

Avoid rediscovering these demonstrated environment conditions:

- The local Rust toolchain has been run through `mise exec rust@stable -- ...`.
  Check what exists on the current host; historical `/tmp` dependency paths may
  have expired. User-local Rust does not satisfy makepkg's package-database checks.
- Run native layout variants sequentially or allocate separate virtual displays;
  simultaneous `xvfb-run -a` startup has raced. Avoid concurrent heavy compilation
  during software-renderer timing acceptance. Do not relax the backlog pause to
  conceal an overloaded test environment.
- A real Stockfish engine is needed for required Chess checks; an engine timeout
  is a concrete failing prerequisite, not a FreeSki test exemption.
- All scripted gameplay uses isolated XDG state. Native layout evidence is not a
  human difficulty rating. An installed release may differ from the development
  binary; identify which executable is being tested.

For new evidence record revision/dirty status, command and dependencies, input
identity, outcome and retained artifact location. A local `/tmp` path alone is
transient. Keep small causal regressions in the repository and retain bulky
captures through CI artifacts. Promote the minimal failing example into a test
before treating the lesson as reusable evidence.

## Acceptance tracker

| ID | Acceptance area | Current evidence / status |
| --- | --- | --- |
| A1 | Movement/braking, continuous collision, jumping and single-event recovery | Practice engine tests pass, including high-speed sweeps, overlapping hazards, descent into a rock and safe recovery at every authored hazard |
| A2 | Gates, penalties, finish ordering and exactly-once records | Ordered downhill gates, one-time five-second misses, pole collision, finish precedence and record/unlock tests pass |
| A3 | Connected seeds, recovery and bounded generation/memory | 128 seeds × 5 km production runs, zero reference crashes; four chunks / 72 obstacles and 240-segment trail cap; recovery corridor and overlap tests pass |
| A4 | Creature warning, spawn, catch and isolation | Warning, bounded/blocked spawn, relative catch, physical terrain, protected recovery, chase-off and causal catch-before-crash tests pass |
| A5 | Five complete courses and calibrated medals/chase | All five courses complete with zero reference crashes/misses; thresholds measured. Human medal/chase calibration pending |
| A6 | Mouse/keyboard flows, handover, overlays, focus and compact targets | Frontend and native scenarios pass; rules-2 movement received positive human feedback; endless feel evaluation pending |
| A7 | Save/reopen during jumps, recovery, pursuit and Slalom | Practice and endless jump/recovery continuation and native restoration pass; unsupported saves retained; pursuit warning/active/recovery/caught and Slalom progress/results also restored exactly |
| A8 | Render-rate equivalence and resize independence | Frontend replay at 30/60/120 Hz and alternating window sizes produces identical tick state; backlog pause tested |
| A9 | Actual theme/size/scale captures | Dark, light, compact, 200% X11 and local Wayland captures inspected |
| A10 | Workspace, native switching, package/upgrade and desktop acceptance | Workspace/build/native/staged reinstall checks pass; user-local Arch package build and extracted-package checks pass; system pacman installation and human difficulty acceptance not claimed |
| A11 | Shelf/help/About, provenance, rules, controls and decisions | All modes have shelf/help/About, original geometry/sound provenance, rules and controls |

Current completion evidence is in the final dated section. The preceding endless
implementation was `13fa3bf`, with documentation in `4dc1f8f`; initial practice was
`acf6286`. Earlier results do not imply acceptance of later features.

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

These committed captures belong to the initial practice revision, not the later
endless build. All are native application captures, not mockups. X11 layout captures show release
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


## 13 September 2026 endless terrain and hardening

Implementation: `13fa3bf` on `feat/freeski`. Two GPT-5.6 Sol medium workers ran
in parallel for terrain and engine/storage, followed by parent integration,
source review and independent combined checks. Issue #14's updated timestamp still
matches the requirements snapshot. This completes the requested hardening and
endless terrain pass; it does not complete creature, Slalom or full release scope.

| Check | Result |
| --- | --- |
| Workspace fmt and strict Clippy, all targets | Passed |
| Workspace tests, with Stockfish 17.1 required | 252 passed |
| FreeSki desktop-independent suite | 29 passed |
| Production seed corpus | Seeds 0–127 × 5 km from rest, zero crashes, over 1,000 jumps |
| Tick-stamped input continuation | Exact states after save/reopen and crossing chunk boundaries |
| Render schedule and crash reset | 30/60/120 Hz produce identical states across resize, chunk crossings and released-key crash recovery |
| Rust release and native Pinball build | Passed |
| Pinball C++ theme/path/authored-table tests | All three passed |
| Desktop entry validation | Passed |
| Staged eleven-game native switching | Passed, including singleton, Chess engine response and existing save paths |
| FreeSki native dark/light/compact/200% | Passed mode choice, start, steering, overlays/focus, quarter-turn, shelf switching, save/reopen and invalid-file retention |
| Suspended endless fixtures | Native restore beyond 1,600 m, mid-jump and crash recovery; generated through ordinary production inputs |
| Staged reinstall and reopen | Exact endless mid-jump save retained through reinstall and reopening |
| Existing user save | Schema-1 practice run/records/preferences preserved exactly with new schema fields; original backup verified |
| Omarchy desktop | Updated native Wayland FreeSki window opened; endless hands-on acceptance remains pending |
| Arch package | Blocked by missing system Rust prerequisite; user-local mise Rust builds succeed. Staged installation is separate evidence |

Captures inspected under `/tmp/freeski-captures/endless-{dark,light,compact,200}/`:
`free-ready.png`, `free-long-run.png`, `free-mid-jump.png`, `free-recovery.png`,
`free-skiing.png`. Local logs: `/tmp/freeski-endless-workspace.log`,
`/tmp/freeski-endless-headless.log`, `/tmp/freeski-endless-native-all.log`.
The staged upgrade evidence is `/tmp/freeski-endless-upgrade/{before,after}.png`.
These are local acceptance artifacts; CI uploads its corresponding native captures.

Two simultaneously launched `xvfb-run -a` checks initially selected the same virtual
display and failed window isolation. Reruns on separate display numbers passed;
CI already runs variants sequentially. Existing Pinball SDL triangle warnings are
unchanged, with its native/C++ checks passing.

The replay evidence uses explicit tick-stamped production inputs; no player-facing
replay importer/exporter is shipped. Statistical route checks establish feasibility,
not difficulty calibration. Test several minutes of the endless mode before tuning
its density or beginning creature pursuit.


## 13 September 2026 system design pass

Documentation-only revision: introduced SYSTEM.md and NEXT.md, replaced stale
setup work in PLAN.md, routed AGENTS.md by task, and distinguished active rules,
historical tuning and revision-specific evidence. The next planned work is shared
session/reproduction support followed by optional pursuit; no proposed interface
or gameplay behavior was implemented in this pass.

Checked local Markdown links/anchors, referenced current commands and symbols,
source consistency, scope/status language and whitespace. Runtime tests and native
captures were not rerun because executable code and assets did not change; the
252-test runtime result above remains evidence for `13fa3bf`, not a new run.


## 13 September 2026 — whole-game completion

The implementation pass adds Session, optional physical pursuit, five-course
Slalom Cup, original audio, schema 3 migration and production-input replay/fixtures.
Player physics remains rules 2 and terrain generator 1. Exact practice ramp/crash
and endless chunk snapshots were captured independently from `13fa3bf` and are
retained as Session regressions. Chase contact before a predicted crash must not
retain that unrealized crash, recovery position, speed or heading.

Completed checks before the implementation commit:

| Check | Result |
| --- | --- |
| Workspace tests with required Stockfish 17.1 | PASS: 287 tests |
| FreeSki without UI | PASS: 59 tests |
| FreeSki UI/input/audio tests | PASS: 17 tests, including three new complete-mode flows |
| Strict workspace Clippy / formatting | PASS |
| Release Rust/native build | PASS |
| Pinball theme/path/authored table | PASS: all three checks |
| Desktop entry validation | PASS |
| Bounded replay CLI | PASS: 120-tick chase continuation; intentional mismatch reports first divergence at tick 1,660 |
| Production fixture generation | PASS: practice/endless jump/recovery, chase warning/active/recovery/caught, Slalom progress/finish/cup finish |
| Dark/light/compact/200% native controls and exact fixture reopen | PASS |
| Staged eleven-game native switching | PASS: singleton, all games, required Stockfish response, existing saves and clean shutdown |
| Staged reinstall | PASS: pursuit, Slalom progress and cup results retained byte for byte before/after reinstall and reopen |
| Human difficulty and Omarchy playtest | Pending; automated reference routes are not human acceptance |

Pursuit and course reference measurements are in TUNING.md. Native automation
uses isolated XDG state and a disconnected audio server; waveform bounds and
owned process cleanup are tested separately. No actual player save is used as a
fixture. CI generates all twelve fixtures and exercises all four layouts, then
checks an active-pursuit save across package reinstall.

Transient local artifacts: `/tmp/freeski-complete-fixtures`,
`/tmp/freeski-complete-native`, and `/tmp/freeski-complete-*.log`. The tracked tests,
examples and CI commands reproduce the evidence after these files expire.


### Long-escape save regression

Review after `80d8d7f` found that a validation-only 512 m pursuit separation limit
could reject a legitimate escape. Seed 17 with ordinary reference steering
reproduced the old failure at tick 7,687, separation 512.126 m. The bounded union
of two terrain windows already supports arbitrary separation within world bounds;
no extra gap limit belongs in persistence. The fix removes that limit and runs
12,000 production ticks with per-tick validation, then saves and restores exactly.
The same regression was run against the old validator to confirm that it fails.

The full native dark/light/compact/200% runs passed on `80d8d7f`. A virtual-display
startup collision on reusing display 182 failed before launching the app; distinct
displays 183 and 184 resolved it without changing the game or timing thresholds.
Installed switching and three staged upgrade cases also passed. The existing
Pinball SDL triangle warnings remain unchanged, with its native/C++ checks passing.

The standard Arch wrapper reached dependency validation and stopped because
pacman has no `rust>=1.98` package. A user-local build of the same PKGBUILD uses
mise Rust, verified remaining dependencies and checksummed sources; its final
result is recorded below. System packages and pacman installation state are not
changed by these checks.

### Final completion closeout — code `4f61399`

| Check | Result |
| --- | --- |
| FreeSki all targets after long-escape fix | PASS: 75 tests |
| FreeSki without UI | PASS: 60 tests |
| Workspace Clippy / formatting and rebuilt release | PASS |
| Package's required workspace tests | PASS: 288, with the package's built, pinned Stockfish |
| Package's Pinball checks | PASS: all three |
| Native packaged app | PASS: all eleven games, singleton, engine response, original save paths and clean shutdown |
| Packaged FreeSki | PASS: normal controls, mode choices, focus/overlays, exact fixture restore, shelf return, mute and future-file retention |
| Package re-extraction and reopen | PASS: exact active pursuit, Slalom progress and completed-cup saves retained |
| Actual Omarchy/Wayland launch | PASS: updated game opened; every legacy field in the player's schema-2 save remained exact, with an original schema-2 backup |
| Human difficulty / audio judgement | Not claimed; new modes await Tyler's playtest |
| System pacman install/upgrade | Not performed; package extraction/re-extraction is the demonstrated boundary |

The main package was built from committed source `4f61399` using the unmodified
PKGBUILD, with a computed checksum replacing its source placeholder. All sources
passed SHA-256 verification. All runtime/build dependencies except the pacman Rust
entry were verified installed. `mise` supplied Rust 1.98.1; `makepkg --nodeps`
bypassed that package-database prerequisite only. `--noextract` reused prepared
committed source and compiler/engine build caches. No build/check phase was skipped.
No system packages, identities or configuration were changed.

Artifact: `dist/arch/omarchy-retro-arcade-0.1.0-1-x86_64.pkg.tar.zst` (131,552,099
bytes), SHA-256 `48eb67577ad75eecb2b10c73226be30819f458dc97631ee1328272374aef7302`.
The corresponding app snapshot, pinned Stockfish source, NNUE and exact PKGBUILD
are in `dist/arch/sources`. App-source archive SHA-256:
`411c11171261f3e3f524c9389aa65aeb3c6c430d025068de59d8f56b419ca126`.
The package contains one desktop entry and bundled Stockfish with its licence.

Final artifacts include `/tmp/freeski-complete-package-final.log`,
`/tmp/freeski-package-native-all.log`, `/tmp/freeski-package-native-freeski.log`,
`/tmp/freeski-package-upgrade.log`, and the `package` capture directory beneath
`/tmp/freeski-complete-native`. A live close-pursuit capture in its `creature`
directory shows the original actor at its physical position, produced from seed
17 through normal Session inputs. These temporary paths may expire.

The old release executable was replaced by the tested completion build on the
actual Wayland desktop. The running player save was never used to generate test
fixtures or reset to obtain screenshots. Next work is scoped in NEXT.md; issue
#14 has not been closed or externally updated by this pass.

### Menu design pass — 2026-09-13

The menu pass replaces cramped default dialogs with padded panels, grouped
secondary actions, a prominent primary action, readable results and preferences,
and two short help pages. Start/mode/course controls now sit on an opaque themed
surface. This is a FreeSki presentation change; movement rules and save schema
are unchanged.

| Check | Result |
| --- | --- |
| Workspace formatting and strict all-target Clippy | PASS |
| Workspace tests, required Stockfish 17.1 | PASS: 289, serial execution |
| Final FreeSki all-target tests | PASS: 76, including compact help/preferences visibility and run isolation |
| Release Rust/native build | PASS |
| Pinball theme/path/authored table and desktop entry | PASS |
| Native menu captures: dark, light, compact, 200% | PASS: all four; start/modes, pause, restart, settings, help, results, recovery and shelf return |
| Full FreeSki native lifecycle with all twelve fixtures | PASS: dark; normal mouse/keyboard, exact restore, focus, mode/pursuit controls and future-save retention |
| Actual Omarchy/Wayland relaunch | PASS: current player's full parsed save unchanged after normal close/reopen |
| Human visual acceptance | Pending Tyler's review of the reopened game |
| Arch package rebuild/install | Not run for this presentation-only pass; the earlier package artifact predates these menus |

The initial parallel workspace run failed the unrelated Chess engine-timeout
assertion. The complete serial run passed with the required real engine. The
menu screenshot driver's first attempt passed floating-point coordinates to X11;
integer coordinates corrected the driver, and all four final runs passed.

Reproduce menu evidence with `scripts/native-freeski-menus.py BINARY OUTPUT
[dark|light|compact|200]` under Xvfb, with Pillow and `FREESKI_FIXTURES` pointing to
the production example's fixtures. Use distinct X displays for concurrent runs.
The existing full lifecycle driver includes the new toolbar coordinates. Native
checks use disposable XDG directories and a disconnected audio server; these are
separate from human Omarchy acceptance. Final temporary captures are in
`/tmp/freeski-menus/{dark,light,compact,200,lifecycle}`, with logs at
`/tmp/freeski-menu-*.log`.

### Compact Arcade-themed menus — 2026-09-14

Tyler's screenshot identified excessive header height and styling inconsistent
with the collection. The Free Ski header is now about 104 logical px rather than
269 px: mode/actions share a row, stats/pursuit share a row, and hints use one
line. Slalom adds course choices at Ready. Dialogs use the shared popup frame,
brass borders, square buttons and smaller typography/padding. Simulation and
save format are unchanged.

- PASS: workspace fmt, strict all-target Clippy and 289 workspace tests with the
  required Stockfish engine, run serially; 18 FreeSki library tests also passed.
- PASS: rebuilt release app; native menu checks at dark/light/compact/200%, with
  actual screenshots inspected for header, dialogs and results.
- PASS: full dark native FreeSki lifecycle and all twelve production save fixtures,
  updated visible-control coordinates, focus/handover, shelf return and restoration.
- PASS: normal close/reopen of the actual Wayland game preserved the complete
  parsed player save. The updated app was left open for Tyler's review.
- NOT RUN: new Arch package/install or unchanged Pinball C++ checks for this
  presentation-only revision. Earlier package evidence predates these changes.

Two initial concurrent menu runs failed the first mode-selection assertion while
workspace tests were active. The subsequent isolated runs passed; the capture
helper now allows 300 ms for overlay painting before saving an image. No game
input or simulation timing was changed to accommodate the driver. Final captures
are under `/tmp/freeski-compact/{dark,light,compact,200,lifecycle}` and logs use
`/tmp/freeski-compact-*.log`. Human acceptance of this revision remains separate.

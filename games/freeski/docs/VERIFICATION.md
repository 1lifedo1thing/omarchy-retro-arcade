# FreeSki acceptance and evidence

Practice, simulation/save hardening and endless terrain are implemented. Tyler
reported positive feedback on rules-2 movement and later on the endless pass
overall. Detailed difficulty/variety acceptance, creature pursuit, Slalom and
release presentation remain open.
The full issue #14 remains open.

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
```

The examples create synthetic saves through ordinary inputs. They are current
commands, distinct from the **proposed** reusable replay runner in SYSTEM.md.
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
| A2 | Gates, penalties, finish ordering and exactly-once records | Practice fatal-crash precedence and records tests pass; Slalom gates/timing remain later work |
| A3 | Connected seeds, recovery and bounded generation/memory | 128 seeds × 5 km production runs, zero reference crashes; four chunks / 72 obstacles and 240-segment trail cap; recovery corridor and overlap tests pass |
| A4 | Creature warning, spawn, catch and isolation | Not implemented |
| A5 | Five complete courses and calibrated medals/chase | Practice reference completes; Slalom and human calibration not implemented |
| A6 | Mouse/keyboard flows, handover, overlays, focus and compact targets | Frontend and native scenarios pass; rules-2 movement received positive human feedback; endless feel evaluation pending |
| A7 | Save/reopen during jumps, recovery, pursuit and Slalom | Practice and endless jump/recovery continuation and native restoration pass; unsupported saves retained; pursuit/Slalom not implemented |
| A8 | Render-rate equivalence and resize independence | Frontend replay at 30/60/120 Hz and alternating window sizes produces identical tick state; backlog pause tested |
| A9 | Actual theme/size/scale captures | Dark, light, compact, 200% X11 and local Wayland captures inspected |
| A10 | Workspace, native switching, package/upgrade and desktop acceptance | Workspace/build/native/staged reinstall checks pass; actual Arch package build/install acceptance and human Wayland playtest pending |
| A11 | Shelf/help/About, provenance, rules, controls and decisions | Practice and endless shelf/help/rules reviewed; creature/Slalom presentation remains later work |

Current runtime evidence is for `13fa3bf`, with documentation in `4dc1f8f`.
The initial practice implementation was `acf6286`. Sections below are dated
historical runs; planned APIs in [SYSTEM.md](SYSTEM.md) and [NEXT.md](NEXT.md) do
not inherit a pass from these results.

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

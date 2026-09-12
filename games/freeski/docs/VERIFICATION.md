# FreeSki acceptance and evidence

Status: documentation setup only. No FreeSki runtime checks or playtests have run.
This table follows all acceptance bullets in the [issue snapshot](REQUIREMENTS.md).

| ID | Acceptance area | Milestones | Required evidence | Status |
| --- | --- | --- | --- | --- |
| A1 | Deterministic movement/braking, continuous collisions, jump clearance, single-event safe recovery | 1–2 | Focused production-engine scenarios including high-speed and overlapping hazards | Not run |
| A2 | Gate direction/order/misses, finish precedence, timer boundaries, exactly-once records | 2, 4 | Event-order and result persistence tests; fatal event at/before finish case | Not run |
| A3 | Connected traversable seeds, safe recovery, bounded generation/memory | 3 | Versioned seed corpus, production-engine traversal including chunk seams, bounded retries/fallback and long-run measurements | Not run |
| A4 | Chase warning/spawn/catch, chase-off isolation, pause/resume | 3 | Deterministic pursuit cases and human evasion evidence | Not run |
| A5 | Five completable Slalom courses and calibrated medals/chase | 3–4 | Five repeatable production-engine completions and human playtest notes | Not run |
| A6 | Mouse-only/keyboard-only flows, handover, overlays, focus, compact targets | 1, 5 | Native input scenarios and hands-on flows through start, pause, settings, restart, results and shelf | Not run |
| A7 | Faithful save/reopen for jumps, recovery, pursuit and Slalom | 1–4 | Compare continuous and restored outcomes; records/unlocks preserved; corrupt/future/incompatible files retained; lifecycle flush checks | Not run |
| A8 | Render-rate equivalence and resize independence | 2 | Same tick inputs under multiple render schedules and sizes; backlog pauses instead of skipping simulation | Not run |
| A9 | Actual light/dark, compact and 200% app captures | 1, 5 | Captures from the built app, visually reviewed with commit and display details | Not run |
| A10 | Workspace quality, native switching, install/upgrade and desktop acceptance | 5 | CI/local check results, installed package run and save preservation; separate Omarchy/Wayland report | Not run |
| A11 | Shelf/help/About, original provenance, rules/controls and decisions | 1, 5 | Content/link review and actual app inspection | Not run |

## Checks to run when implementation exists

Use the current repository [contribution checks](../../../CONTRIBUTING.md#build-and-check):
workspace fmt, strict Clippy, all-target tests with real Stockfish, release build,
Pinball engine tests and desktop-entry validation. Add focused FreeSki engine,
storage, replay and native checks as the relevant milestones land, including an
engine build without UI features. Extend existing switching/render/package jobs.

Runtime checks are not applicable to this documentation-only setup. They must
not be reported as passing FreeSki gameplay or release acceptance.

## Evidence entry format

For every execution record:

- Date, exact commit, build/profile, platform and test command or manual steps.
- Result: passed, failed, skipped or blocked; explain skipped/blocked checks.
- Acceptance IDs covered and links to test code, logs, captures or replay fixtures.
- Rules/course/generator versions, seed and input fixture for repeatable runs.
- Findings, remaining gaps and follow-up tuning or fixes.

Keep headless engine tests, native X11 automation and hands-on Omarchy/Wayland
evidence distinct. A screenshot demonstrates rendering, not control feel or sound.

## Human playtest record

Record tester, revision, course/seed, input method, theme, window size/scale and
desktop/backend. Note steering and braking feel, hazard reaction time, jump and
crash readability, difficulty and chase evasion, audio, and resume behavior.
Link resulting tuning changes in [TUNING.md](TUNING.md).

No human playtests recorded yet.

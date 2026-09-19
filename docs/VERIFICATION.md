# Arcade verification

## FreeSki integration — 15 September 2026

Merged main `5d5c085` while preserving all thirteen games and the lean package.
Combined workspace checks, native switching and the actual system package
upgrade passed. Existing saves survived installation and native Wayland reopen.
[Current FreeSki evidence](../games/freeski/docs/VERIFICATION.md#2026-09-15-integrated-package-and-review-readiness)
separates current checks, CI status and Tyler's recorded human acceptance.
The dated reports below describe their original revisions.

## FreeSki complete local modes — 13 September 2026

FreeSki is the eleventh game in the source build. The completion pass implements
practice, seeded endless terrain, optional creature pursuit, five Slalom courses,
medals, original sound and resumable state. Native dark/light/compact/200% checks,
eleven-game switching and staged pursuit/Slalom upgrade cases passed. The actual
Wayland launch retained the user's existing schema-2 run and its original bytes.
A user-local Arch package build passed its 288 tests and all engine checks; the
extracted package passed native switching, FreeSki flows and save re-extraction.
This is separate from installing it into the system pacman database.

[FreeSki verification](../games/freeski/docs/VERIFICATION.md) records source
revisions, exact checks, package results and remaining human difficulty acceptance.
The older nine-game preview release below is unchanged.

## Nine-game presentation integration — 12 September 2026

The integrated presentation revision passed all 197 workspace tests, strict Clippy, formatting, the release build, three Pinball engine/theme tests and desktop-entry validation. The local staged installation also contains the nine game licenses and cabinet-art provenance.

[Integrated CI run](https://github.com/tcballard/omarchy-retro-arcade/actions/runs/34689873161) passed native one-window traversal of all nine games, saves, Stockfish interaction, Stack controls/focus/pause/resume, Snake controls/saves/layouts and Blast solo/local-match lifecycle. Its Arch job built the complete package with bundled Stockfish, installed one desktop entry, exercised the installed games and preserved the Solitaire session through a reinstall.

All 40 actual application captures (collection plus nine games in dark, light, compact and 200% layouts) were visually reviewed. Review caught and corrected unavailable navigation glyphs and the opening Pinball preview crop. See [presentation](PRESENTATION.md) for the screenshots and final correction build.

This session cannot open a local display socket, so its native GUI evidence comes from GitHub's Linux/X11 runners. The historical local evidence below belongs to the earlier consolidation, not this session.

## Original five-game consolidation: local evidence

- All 120 Rust tests pass across the combined workspace, including real Stockfish communication, save compatibility, deterministic rules, card artwork validation and bounded Pinball frame decoding.
- Formatting and Clippy with warnings denied pass for all workspace targets.
- Original Pinball engine tests pass for a full three-ball game, ramp, target, orbit and drain collisions, finite/bounded ball state, and operation without proprietary DAT resources.
- Native X11/XTest checks confirm a single window across five games, singleton locking, Solitaire draw/save/reopen, original save-directory identities, Pinball launch/flipper input, confirmed return to the shelf, and clean shutdown.
- The optimized release was staged with the real installation script. It contains one desktop entry; all five games open from that staged prefix. The bundled Stockfish answered an e2-e4 move entered through the native board.
- Native renders pass at 1120×860, 900×760, 200% X11 scale and a light Omarchy palette. Actual screenshots are under `docs/screenshots`.
- A five-second local software-renderer probe delivered 277 Pinball frames, averaging 56.7 fps while other builds were running. This is a local throughput measurement, not a latency or real-desktop benchmark.

## Original five-game consolidation: GitHub evidence

- Published all five games as ordinary subdirectories with their complete original Git ancestry.
- [Arch package job](https://github.com/tcballard/omarchy-retro-arcade/actions/runs/34657618469/job/103453313089): built the package and bundled Stockfish, passed all 120 Rust tests and Pinball engine tests, installed one desktop entry, exercised all five games in one window, and reinstalled without changing the Solitaire save.
- [Native Linux job](https://github.com/tcballard/omarchy-retro-arcade/actions/runs/34658145339/job/103454867412): formatting, strict Clippy, all Rust tests, release build, Pinball engine tests, desktop entry validation, native game switching and screenshot capture passed.
- The first Ubuntu GUI run exposed a test-harness race when focusing an unmapped window. The corrected harness waits for X11 viewability and passed the rerun. Application code is identical between the verified Arch package and that rerun.
- The [package](https://github.com/tcballard/omarchy-retro-arcade/actions/runs/34657618469/artifacts/10286164667) and [corresponding source plus engine inputs](https://github.com/tcballard/omarchy-retro-arcade/actions/runs/34657618469/artifacts/10286583861) are available as build artifacts.

## Remaining desktop acceptance

- Real Omarchy/Wayland desktop acceptance: sound, fractional scaling, low-end CPU/GPU performance, Pinball input latency and game difficulty.

This is a development preview. Headless Linux verification is not a real Omarchy desktop playtest.

## Host architecture refactor — 19 September 2026

Source under test: `7c0474d134d5b7a24e8240b64646ff636a72e447`, based on
`0aa746357192e488d2c2d077f279a347c1fff6e3`. Environment: Linux x86_64,
Rust/Cargo 1.98.1 (rustc 48a229cea). Reference reviewed: OmaCut
`0948c4615d45ac62727b8c69112178e09781b7a4`. See ARCHITECTURE.md.

Reproduced now:

- `cargo fmt --all --check`: exit 0.
- `cargo clippy --workspace --locked --all-targets -- -D warnings`: exit 0.
- `cargo test -p omarchy-retro-arcade --locked`: exit 0, nine passed;
  the ignored Pinball fixture was invoked successfully by its parent test.
  Includes both new save/game-drop/lock-release ordering tests.
- `cargo test --workspace --locked --all-targets`: exit 0. Stockfish was not
  installed and REQUIRE_STOCKFISH was not set, so this does not establish
  real-engine Chess acceptance. No native desktop session was used.
- `cargo metadata --locked --offline --no-deps --format-version 1`: exit 0.
- `git diff --check`: exit 0. README/architecture local Markdown links resolve.
- Whitespace/visibility-normalized comparison against the baseline confirms
  catalogue data and all game constructor/lock acquisition logic were preserved.

Environment limitations and checks not run:

- Native window switching, actual Omarchy/Wayland, package build/install and
  Stockfish-required acceptance were not run here. No installed Omarchy revision
  was tested. Existing CI has these broader gates; none is claimed from source review.
- Installing desktop development dependencies with apt failed due to the
  environment's setgroups/seteuid restrictions. Rust host compilation nevertheless
  succeeded with the available libraries. No permissions workaround was used.
- An initial offline Clippy attempt failed because ascii 1.1.0 was not cached;
  the subsequent ordinary workspace Clippy command above passed.
- Publication was blocked by automatic approval review pending explicit permission
  to push the branch to the public repository. Remote CI evidence is unavailable.

## Shared platform extraction — 19 September 2026

Source under test: `537f81c3fb8116be3daedd3d08173a883e1e25c7`, on merged
host refactor `957fc5d4d2f5b24d8139568c5fbf5743deec5fa9`. Linux x86_64,
Rust 1.98.1. Generic helper bodies and palette behaviour were compared with the
baseline; only the palette's Color32 import changed from eframe's re-export to
the same ecolor type. No registry dependency version changed.

Reproduced now:

- `cargo test --workspace --locked --all-targets`: exit 0. Stockfish was not
  required or available locally; real-engine acceptance remains with CI.

- Workspace fmt, diff whitespace and workspace Clippy (all targets, locked): exit 0.
- `cargo check --workspace --offline`: exit 0; updates the lockfile for the local crate.
- `cargo test -p arcade-platform --locked --all-features`: four passed, exit 0.
- `cargo test -p arcade-platform --locked --no-default-features`: three passed, exit 0.
- No-default-feature tests for Stack, Snake, FreeSki, Shatter and Tanks: exit 0.
- Normal dependency trees for those five games and arcade-platform with default
  features disabled contain no eframe, egui, ecolor or omarchy-chess packages.
- No game manifest outside Chess references omarchy-chess. The host retains its
  real Chess game dependency. Existing Chess theme/storage exports still compile.

Historical evidence, not rerun for this extraction:

- PR #46 workflow run 35438448573 passed all three jobs, including Stockfish-required
  workspace tests, native switching/save/input checks and Arch packaging, for
  head f3e2c4b348b3b24889edf8eb0d747bbd057b0b5a. #46 was merged after these checks.

Remaining acceptance:

- New-branch native rendering/switching and Arch build/install remain CI gates;
  the earlier #46 results do not establish this branch's runtime acceptance.
- Actual Omarchy/Wayland playtesting and a local Stockfish-required run were not
  performed here. No installed Omarchy revision was tested.

## Focus and drag lifecycle — 19 September 2026

Source under test: `2cd4447b6d213c3b3aae11aae5cbfebd45443b8f`, based on merged PR #47
(`d0a7839394f0e93aca20ca56f8255966e560dd27`). Linux x86_64, Rust 1.98.1.

- Workspace fmt and diff whitespace checks: passed.
- `cargo clippy --workspace --locked --all-targets -- -D warnings`: passed.
- `cargo test -p omarchy-retro-arcade -p omarchy-chess -p omarchy-solitaire --locked`: passed. Includes four host focus/input regressions and real Chess/Solitaire drag-cancellation regressions. Existing ignored subprocess fixture remains ignored in the ordinary test run.
- `python3 -m py_compile scripts/native-stack.py`: passed (syntax only).
- Native Stack focus/held-key script was extended; native execution, Stockfish-required checks and Arch packaging remain CI gates for this branch.
- No actual Omarchy/Wayland session or installed Omarchy revision was tested locally.
- Detached audio-cue ownership and Pinball teardown scheduling remain follow-up work; see LIFECYCLE.md.

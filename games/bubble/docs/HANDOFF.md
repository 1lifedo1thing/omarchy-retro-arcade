# Bubble implementation handoff

Date: 2026-09-11
Repository: https://github.com/tcballard/omarchy-retro-arcade
Feature branch: `feat/bubble`
Integration base: `b1dec03` on `feat/consolidated-arcade`

## Implemented

Bubble is the sixth Rust/egui shelf entry in the existing single-window Arcade app. It includes 20 data-driven levels, deterministic hex-grid collision and snapping, matching and disconnected falls, wall bounces, limited aiming guide, ceiling pressure, scoring, best scores, saved unlocks, mouse/keyboard controls, pause/focus protection, original vector artwork and synthesised audio. Colours have distinct symbols. Rendering keeps the board's aspect ratio and follows the existing Omarchy theme loader.

Existing engines, artwork and save paths are preserved. No standalone Bubble app, installer, desktop entry or release process was added. The shelf count and keyboard wraparound derive from its game list.

## Automated evidence

- Entire Rust workspace: 137 tests passed before the final three focused audio/save-extension tests were added; no failures or ignored tests. The final Bubble suite has 20 passing tests (4 app/audio unit tests, 4 persistence tests, 12 rules tests).
- All 20 authored completion routes replay through the production physics, including pressure. Each shot remains valid with ±0.25° aim variation.
- 6,260 opening-shot angles, symmetric hex neighbours, wall reflection, ceiling attachment, no overlap, crowded play, matching, falling, empty-board win, danger loss and one-shot terminal transitions.
- Native Xvfb interaction: a single window across all six games; Bubble keyboard and actual mouse shots; mid-flight pause, focus loss and explicit resume; saved unlocks/bests; reopen; existing Solitaire and legacy saves; clean shutdown.
- Native interaction passed at 1120×860 and at 900×760 logical size with 200% X11 scaling. The harness waits for a mapped window before assigning focus.
- Actual application screenshots: standard, compact, light palette, 200% scaling and first-play instructions. QA saves select later levels at zero score; these are native renders, not concept images.
- Rust release build passed. Workspace formatting and Clippy passed.
- Unchanged C++ Pinball engine rebuilt; `theme-palette`, `theme-path` and `authored-upstream-table` CTests all passed.
- The shared install script stages one desktop entry and all six game licences. Bubble is included in the workspace build and existing Arch package; CI includes native Bubble interaction and compact/200% coverage.

## Remaining limits

- Human playtesting on a real Omarchy/Wayland desktop, audible sound quality and game balance have not been verified here. The solver proves completion routes, not that the difficulty curve feels right to a player.
- This Ubuntu workspace has no `makepkg`, Arch container runtime or GitHub CI execution for this branch. A real `.pkg.tar.zst` build/install/upgrade must run in the existing Arch CI before release. Local install staging is not an Arch package pass.
- Arcade has per-game audio/settings rather than a global service. Bubble follows the existing sound convention and Ctrl+M. Actual audio requires the optional desktop `paplay` command; silent play remains available.
- Only level progress and best scores persist; an unfinished attempt restarts when reopened.
- GitHub reports this repository empty, with no branch refs. Git push fails because HTTPS credentials are unavailable. This branch is based on the local consolidation and must be published above that base once authenticated Git access is available. No PR was created, merged or released.

## Review and publication

The accompanying Git bundle retains the existing integration history and the Bubble branch. Clone it with:

```sh
git clone omarchy-arcade-bubble.bundle omarchy-retro-arcade
cd omarchy-retro-arcade
git switch feat/bubble
```

If working in the existing checkout, fetch the bundle and review its `feat/bubble` branch. Do not overwrite newer consolidation work. Compare Bubble changes against the documented integration base, then stack/rebase them onto the latest integration branch and run its gates. Publish the integration base before opening Bubble's PR. A prepared PR description is in `PR-DRAFT.md`.

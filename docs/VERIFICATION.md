# Consolidation verification

## Verified locally

- All 120 Rust tests pass across the combined workspace, including real Stockfish communication, save compatibility, deterministic rules, card artwork validation and bounded Pinball frame decoding.
- Formatting and Clippy with warnings denied pass for all workspace targets.
- Original Pinball engine tests pass for a full three-ball game, ramp, target, orbit and drain collisions, finite/bounded ball state, and operation without proprietary DAT resources.
- Native X11/XTest checks confirm a single window across five games, singleton locking, Solitaire draw/save/reopen, original save-directory identities, Pinball launch/flipper input, confirmed return to the shelf, and clean shutdown.
- The optimized release was staged with the real installation script. It contains one desktop entry; all five games open from that staged prefix. The bundled Stockfish answered an e2-e4 move entered through the native board.
- Native renders pass at 1120×860, 900×760, 200% X11 scale and a light Omarchy palette. Actual screenshots are under `docs/screenshots`.
- A five-second local software-renderer probe delivered 277 Pinball frames, averaging 56.7 fps while other builds were running. This is a local throughput measurement, not a latency or real-desktop benchmark.

## Remaining gates

- Clean Arch package installation/reinstallation and CI results for the consolidated revision. [The first consolidated build](https://github.com/tcballard/omarchy-retro-arcade/actions/runs/34657618469) is running.
- Real Omarchy/Wayland desktop acceptance: sound, fractional scaling, low-end CPU/GPU performance, Pinball input latency and game difficulty.

This is a development preview. Headless Linux verification is not a real Omarchy desktop playtest.

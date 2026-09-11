# Verification

Local checks, 11 September 2026:

- Rust formatting and Clippy (all targets, warnings denied) pass.
- 11 rules/storage tests pass: scoring, bunker erosion, boundary clamp, wave transition, damage grace, terminal state, serialisation, atomic saves, corrupt-file preservation and a 100,000-step simulation with restarts.
- Release build succeeds with Rust 1.98.1. An earlier shared-cache build hit a malformed dependency archive; the isolated release build succeeded.
- Native X11 window exercised through Xvfb with software rendering and real key events: movement, firing, scoring, pause, focus loss, quit, sound preference persistence, paused restoration and closing Settings without resuming.
- Native screenshots inspected at 860×900 and 600×680, plus a light Omarchy palette. The fixed dark playfield retains readable sprites.
- Testing found and fixed Space activating a retained button focus and Ctrl+Q deadlocking when issuing a viewport command inside the input lock.

The GitHub workflow repeats Rust checks, native input/persistence checks and an Arch package build/install/version check. Consult the PR for the current CI result; this document does not claim an unobserved run passed.

Still required before a polished release: hands-on Hyprland/Wayland playtesting, audible sound verification, fractional scaling, difficulty/feel review, and approval of the pixel artwork. Controller support and rich explosion/audio effects are not implemented. No stable release has been published.

To reproduce native checks, install Xvfb, ImageMagick and libXtst, compile `tools/xprobe.rs` using rustc into `/tmp/invaders-xprobe`, and run `INVADERS_BINARY=/absolute/path/to/omarchy-invaders node tools/native-check.mjs`.

## 8-bit Orbit implementation verification

- 14 tests pass, including sparse formation spacing and coherent movement, scoring, long simulation, legacy save decoding, bounded PNG validation and storage recovery.
- Formatting and Clippy with warnings denied pass; optimized Linux build succeeds.
- Native X11 keyboard checks pass for movement, firing, scoring, pause, focus loss, paused restore, Settings dismissal, sound preference and clean quit.
- Native artwork checks pass for editable-copy creation, reload, invalid-file preservation, saved custom-art preference, unchanged paused gameplay and game-over Enter retry with high-score preservation. The test locates the moving Settings window before clicking its controls.
- Reviewed the actual opening, compact paused, light theme, Settings and game-over renders against the approved concept. Tiny bitmap labels were switched to a nearest-neighbour font texture after compact rendering revealed missing strokes.
- `docs/orbit-opening.png` is the actual game. `docs/design/approved-orbit.png` is the concept. `docs/orbit-comparison.png` places concept on the left and implementation on the right; concept framing is normalised for comparison.
- Intentional visual differences: smaller enemies to fit seven letters and marching margins, simple 3×5 letter forms rather than exact brand typography, flat three-tone physical bunker cells, and native menu text. See DECISIONS.md and ARTWORK.md.

No claim of updated GitHub CI success is made here. Package CI must pass on the new PR head. Real Hyprland/Wayland, audible audio, fractional scaling and difficulty playtesting remain outstanding. Pixel explosion effects are implemented; a richer audio mix and controller support remain future work.

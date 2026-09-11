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

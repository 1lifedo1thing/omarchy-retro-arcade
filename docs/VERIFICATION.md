# Verification record

Native implementation built and exercised on Linux with Python 3.12 and
PySide6/Qt 6.11.2 using the offscreen platform and software rendering.

## Automated checks

`python -m unittest discover -s tests -v`

**Result: 17 tests passed.** The full suite completed in 22.4 seconds. The eight
Qt integration tests were subsequently rerun after the final keyboard/dialog
changes and passed, including real double-click-to-foundation input. No QML
warnings were emitted by the native interaction test.

- Seeded deal layout, 52-card uniqueness and face orientation.
- Draw-one/draw-three, partial packets, recycle order and undo.
- Alternating-colour sequences, king-only empty columns, revealing hidden cards.
- Same-suit ascending foundations, returning cards to the table, victory and finish eligibility.
- Invalid moves leave state unchanged.
- Random legal-play sequences across 20 seeds, checking invariants and save/undo round trips.
- Duplicate/missing cards, invalid orientation, invalid draw counts and malformed history rejected.
- Local save/resume, full undo persistence, preferences and corruption recovery copies.
- Theme-directory replacement, malformed palette, light-theme contrast and appearance lock.
- Static custom artwork imports; scripts and external resources rejected.
- Timer pause and statistics counting across undo/restart/repeated victory.
- Native Qt click-to-move, keyboard draw/undo, sequence drag-and-drop, and window capture at two sizes.

Both the source distribution and wheel build successfully using
`python -m build --no-isolation`. The wheel includes all QML and SVG assets and
was installed into an isolated local staging directory for a native startup
and screenshot check.

## Visual inspection

Native window captures are generated directly from the QML app, not mockups.
Representative sizes: 1120 × 800 and 800 × 600. Additional views cover the deck
settings and a light palette. See `docs/screenshots/`.

## Remaining environment-specific checks

This environment is not a live Omarchy/Hyprland desktop. Offscreen tests do not
establish real Wayland interaction, launcher integration, GPU rendering,
fractional scaling, assistive-technology integration or pacman installation.
The Arch CI job builds and tests inside Arch; verify its result before installing
the artifact. A real Omarchy desktop smoke test remains required before a public
release is presented as ready.

At handoff, publishing to GitHub's default branch was blocked by automatic
approval review because explicit permission to publish was required. The
commits are local; remote CI has **not run**, and the Arch package recipe has
**not been executed** in this environment. Approval to push the prepared commits
is the next step, followed by checking both CI jobs.

Suggested desktop check: install package, launch from the menu, draw and drag a
sequence, undo, switch between light/dark Omarchy themes without losing the
deal, import a team card back, close/reopen, test keyboard play and reduced
motion, then uninstall via pacman if desired.

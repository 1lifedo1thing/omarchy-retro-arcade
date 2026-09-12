# FreeSki

An original SkiFree-inspired downhill skiing game inside Omarchy Arcade.

**Milestone 1 is playable:** one 1,200-metre practice slope with carving, braking,
ramps, low rocks, trees, three crash allowances, local records and suspended runs.
Endless skiing, creature pursuit and the five Slalom courses are later milestones.
Human acceptance of the initial control feel and difficulty is still pending.

Build with `scripts/build.sh`, choose FreeSki at the end of the Arcade shelf, or run:

```sh
./target/release/omarchy-retro-arcade --game freeski
```

Enter starts. A/D or Left/Right carve; release to point downhill. Moving the mouse
left/right of the skier selects pointer steering. Hold S, Down, Space, the right
mouse button on the slope, or the visible brake button to slow down. Ramps launch
automatically. Esc pauses/resumes; Ctrl+H returns to Arcade. Settings: Ctrl+,.

Runs save on pause, close, shelf exit, results and five-second simulation
checkpoints, then reopen paused. Restarting unfinished progress requires confirmation.
Records and preferences survive restarts. Invalid saves remain untouched until an
explicit archive/reset; playing without saving is also available.

- [Rules, controls, save behavior and limitations](docs/RULES.md)
- [Delivery plan and milestone gates](docs/PLAN.md)
- [Milestone 1 implementation and human acceptance](docs/MILESTONE-1.md)
- [Issue #14 requirements snapshot](docs/REQUIREMENTS.md)
- [Initial tuning and reference run](docs/TUNING.md)
- [Acceptance and evidence](docs/VERIFICATION.md)
- [Original asset provenance](assets/README.md)

Run `cargo test -p omarchy-freeski --locked` for engine, storage and frontend checks;
add `--no-default-features` for the desktop-independent engine/storage suite.
`scripts/native-freeski.py` exercises the real app under Xvfb. The
`practice-evidence` example generates suspended-run fixtures through ordinary
production inputs for native reopen checks.

New code and original assets are GPL-3.0-or-later. No SkiFree artwork, sounds,
courses or creature design are bundled. This game has no standalone launcher.

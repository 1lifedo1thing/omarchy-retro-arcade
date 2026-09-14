# FreeSki

An original SkiFree-inspired downhill skiing game inside Omarchy Arcade.

Choose **Practice** for the authored 1,200-metre learning slope, or **Free Ski**
for a seeded endless mountain. Both use the same carving, brakes, jumps, trees,
rocks and three crash allowances. Free Ski keeps a separate distance record and
creates a new mountain for each new run. Creature pursuit, Slalom and sound remain
later milestones.

Tyler's follow-up playtest found the revised speed and turning felt good, and his
overall response to the endless pass was positive. Detailed terrain difficulty
and variety playtesting remains open.

Build with `scripts/build.sh`, choose FreeSki at the end of the Arcade shelf, or run:

```sh
./target/release/omarchy-retro-arcade --game freeski
```

Enter starts. Hold A/D or Left/Right to turn up to 90°; release to keep your heading. Moving the mouse
left/right of the skier selects pointer steering. Hold S, Down, Space, the right
mouse button on the slope, or the visible brake button to slow down. Ramps launch
automatically. Esc pauses/resumes; Ctrl+H returns to Arcade. Settings: Ctrl+,.

Choose a mode before starting. From a paused run, use **Try endless Free Ski** or
**Switch to practice**; confirm before replacing unfinished progress.

Runs save on pause, close, shelf exit, results and five-second simulation
checkpoints, then reopen paused. Restarting unfinished progress requires confirmation.
Records and preferences survive restarts. Invalid saves remain untouched until an
explicit archive/reset; playing without saving is also available.

- [Rules, controls, save behavior and limitations](docs/RULES.md)
- [What comes next](docs/PLAN.md)
- [System design and ownership](docs/SYSTEM.md)
- [Next implementation: optional pursuit](docs/NEXT.md)
- [Milestone 1 implementation and human acceptance](docs/MILESTONE-1.md)
- [Issue #14 requirements snapshot](docs/REQUIREMENTS.md)
- [Tuning and reference runs](docs/TUNING.md)
- [Acceptance and evidence](docs/VERIFICATION.md)
- [Original asset provenance](assets/README.md)

Run `cargo test -p omarchy-freeski --locked` for engine, storage and frontend checks;
add `--no-default-features` for the desktop-independent engine/storage suite.
`scripts/native-freeski.py` exercises the real app under Xvfb. The
`practice-evidence` and `endless-evidence` examples generate suspended-run fixtures
through ordinary production inputs for native reopen checks.

New code and original assets are GPL-3.0-or-later. No SkiFree artwork, sounds,
courses or creature design are bundled. This game has no standalone launcher.

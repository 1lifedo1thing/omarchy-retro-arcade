# Implementation decisions for review

1. Working title: **Omarchy Invaders**. Space Invaders-inspired play with original Rust rules and hand-authored pixel sprites. No upstream game engine, ROM or copied game assets. For this small deterministic game, a standalone Rust core gives simpler packaging than adapting an existing C/C++ game.
2. Fixed 120 Hz simulation; rendering scales an 800×700 field without changing collision geometry. Three lives, eleven columns and five rows. Holding fire is supported, with at most three player shots.
3. Four bunkers erode under either side's fire. New waves restore bunkers; lives carry over. Difficulty growth is capped. No online leaderboard, account, telemetry or multiplayer in this milestone.
4. Score tiers: 30/20/10 by row, bonus craft 150. No random bonus multipliers. Deterministic internal random sequence makes a fresh run reproducible; this is not a competitive anti-cheat system.
5. Native eframe/egui front end, following Chess's menus and shortcuts. Focus loss pauses. Resuming is explicit. Saves resume paused. New-game confirmation archives prior state.
6. Follow Omarchy by default; charcoal playfield remains dark even with a light desktop. Very dark accents fall back to sage in the playfield so enemies remain visible. Desktop files are never modified.
7. Sound off by default. Optional synthesized impact cues, inherited helper from Chess. A full arcade soundtrack and richer effects remain future polish.
8. One atomic local JSON file contains the run, high score and preferences; five-second autosave and normal-exit save. Lifetime file lock. Invalid data is preserved and requires an explicit recovery action.
9. GPL-3.0-or-later. Arch package plus standalone Linux CI binary. Rust remains the application language. No Python runtime or build dependency.
10. Artwork is an initial playable pixel direction. Visual approval and real Omarchy playtesting remain outstanding; tests passing do not establish finished game feel.

# Rust verification and remaining acceptance

## Evidence

The Python preview and its test evidence are superseded by this Rust implementation.

- Rust 1.98.1, locked dependencies.
- 33 Rust tests passed locally: 28 rules/storage/theme/UCI tests and 5 headless egui tests.
- Rules include castling, en passant, promotions, automatic and claimable draws, repetition after resume, PGN rejection and custom-FEN round trips.
- Persistence tests cover exclusive locking, legacy-lock protection, Python-session migration/backup and preservation of corrupt bytes.
- UCI tests include silent-engine timeout, cancellation, illegal engine replies and a real Stockfish move. Stockfish was compiled from upstream for testing and is not bundled.
- Headless GUI tests cover pointer-driven play, history input protection, stale engine-result rejection, archival replacement, orientation mapping and compact layout generation. They do not establish full rendered desktop correctness.
- CI is configured to run formatting, Clippy, tests with required Stockfish, release build, Xvfb screenshot and an Arch package build/install check. Consult the matching commit's Actions run for actual results; configuration alone is not passing evidence.

## Real desktop acceptance

This environment is not Tom's Dell and has no Omarchy/Hyprland session. Before a stable release:

- Install the package; confirm launcher, icon and own-window behaviour.
- Play as both colours, promote, resign and finish games.
- Test drag/drop, keyboard focus, modal dialogs and PGN file portals.
- Switch actual Omarchy themes, including dark and light palettes.
- Test Wayland, monitor scaling, resizing and resume after reboot.
- Check screen-reader access; the custom board does not yet expose 64 individually accessible squares.
- Check battery/CPU use, perceived difficulty and Stockfish installation usability.

No stable release, AUR submission or marketplace listing has been published.

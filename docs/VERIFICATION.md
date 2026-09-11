# Rust verification and remaining acceptance

## Evidence

The Python preview and its test evidence are superseded by this Rust implementation.

- Rust 1.98.1, locked dependencies. Release executable built locally and `--version` passed. Formatting and Clippy with warnings denied pass.
- 41 Rust tests passed locally: 31 rules/storage/theme/UCI/preferences/audio tests and 10 headless egui tests.
- Rules include castling, en passant, promotions, automatic and claimable draws, repetition after resume, PGN rejection and custom-FEN round trips.
- Persistence tests cover exclusive locking, legacy-lock protection, Python-session migration/backup and preservation of corrupt bytes.
- UCI tests include silent-engine timeout, cancellation, illegal engine replies and a real Stockfish move. Stockfish was compiled from upstream for testing and is not bundled.
- Headless GUI tests cover pointer-driven play, fast drag/drop, focused keyboard moves, history input protection, stale engine-result rejection, archival replacement, orientation mapping and compact layout generation. They do not establish full rendered desktop correctness.
- CI is configured to run formatting, Clippy, tests with required Stockfish, release build, Xvfb screenshot and an Arch package build/install check. Consult the matching commit's Actions run for actual results; configuration alone is not passing evidence.

## Real desktop acceptance

This environment is not Tom's Dell and has no Omarchy/Hyprland session. Before a stable release:

- Install the package; confirm launcher, icon and own-window behaviour.
- Play as both colours, promote, resign and finish games.
- Test drag/drop, keyboard focus, modal dialogs and PGN file portals.
- Switch actual Omarchy themes, including dark and light palettes.
- Test Wayland, monitor scaling, resizing and resume after reboot.
- Check screen-reader access; all 64 squares expose names and activation actions; real Orca interaction remains unverified.
- Check battery/CPU use, perceived difficulty and Stockfish installation usability.

No stable release, AUR submission or marketplace listing has been published.

## Arcade polish evidence

Local formatting, Clippy, 41 tests with required Stockfish and the 0.2.0 release build pass. Added coverage checks persistent preferences, current/legacy theme precedence, generated PCM structure, rematch archival and the 64-square AccessKit tree plus activation. CI captures normal and compact light windows and verifies the Arch package including its bundled Stockfish and audio dependencies. See the current PR run for CI results.

Normal 1060×780 and compact light 740×560 windows were rendered in CI and visually inspected. Both captures are in docs/. The release package now builds the exact pinned Stockfish source/network and tests that engine; current CI results are linked in the PR.

## Chisel integration — 11 September 2026

Imported the twelve Chisel SVGs unchanged from the selected design handoff. Native Xvfb/software-GL renders checked at 1060×780 and compact 740×560, with dark, light and coral-accent palettes. Only inlays recolour; fixed body facets remain intact. The full source viewBox is preserved, including its built-in padding. Board and drag use matching image sizes.

42 tests pass, including real Stockfish integration, existing native-widget input tests and a new all-piece recolouring regression. Formatting, Clippy and release build pass locally. These checks do not replace real Omarchy fractional-scaling or visual acceptance.

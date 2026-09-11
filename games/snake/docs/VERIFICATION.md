# Snake verification

Verified on 12 September 2026 on the isolated `feat/snake` branch, initially based on published Arcade `2fddb0f`, then rebased onto `dd5e9f8` to preserve the latest consolidation evidence and X11 viewability fix. The latter changes no application code. This is a development contribution, not a release.

## Automated checks passed

- Workspace regression: 120 existing Rust tests passed with Stockfish required, plus 16 Snake tests (136 total). The full workspace run initially had 15 Snake tests; the added full-board replay test then passed in the focused 16-test run. No failures or ignored tests.
- `cargo test -p omarchy-snake --no-default-features`: 14 pure engine/replay tests, including a full-board win reproduced by the validator. No desktop dependency is needed for the service adapter.
- `cargo fmt --all --check`, `cargo clippy --workspace --locked --all-targets -- -D warnings`, and `git diff --check` passed.
- `scripts/build.sh`: release Arcade executable, existing C++ Pinball worker and theme-test target built successfully. Local CMake used the workspace's extracted SDL headers/libraries because development packages are not installed system-wide.
- Pinball `theme-palette`, `theme-path`, `authored-upstream-table`: all three passed. Initial runtime lookup of libmodplug failed; rerunning with the existing dependency directory in the library path passed.
- Staged `scripts/install.sh` output: one Arcade executable, one `.desktop` entry, existing Pinball worker, bundled Stockfish supplied as in the Arch recipe, and Snake's licence/docs. Version reports Arcade 0.1.0; no separate Snake package or launcher.
- `scripts/native-check.py` on the staged release binary: singleton protection, one native window across all six games, Solitaire draw/save/reopen, legacy game save paths, a real Stockfish response to keyboard/mouse play, Pinball bridge and clean shutdown.
- `scripts/native-snake.py` on both development and staged release binaries: explicit start, WASD turn, eating/growth/score, collision, restart, pause, cancelled countdown, focus loss, no automatic resume, shelf navigation, exact restored run and movement phase, compact/light/dark/200% renders. The test owns a temporary state directory and never uses user saves.
- Audio lifecycle: a controlled long-lived `paplay` substitute proves the owned process is terminated on pause and on return to the shelf while audio is active. This checks ownership/cleanup; it does not claim audible device quality.

Engine tests cover all directions/walls, self-collision, departing-tail legality, scoring, growth, two-turn validation, repeats and overflow, deterministic unoccupied food, the last empty cell, full-board win without another RNG draw, replay equivalence and limits, ticket seed/rules/speed mismatch, pause/countdown timing, save phase/buffer/RNG restoration, and independent records/corrupt-file recovery. Identical timed inputs produce identical state at 15, 30, 59, 60, 120, 144 and 240 render frames/second.

## Running-game screenshots

All PNGs are captured from the actual native Arcade binary driven by XTest. No mockup or generated artwork is used as running-game evidence. Gameplay captures restore a valid local test snapshot positioned to eat one item, then continue via the ordinary UI; they are not human high scores.

| Capture | Size | Evidence |
|---|---|---|
| [Dark gameplay](snake-dark.png) | 1120 × 860 | Active snake, directional head, food, score and shared navigation |
| [Light gameplay](snake-light.png) | 1120 × 860 | Theme loading with contrasting fixed board palette |
| [Compact gameplay](snake-compact.png) | 900 × 760 | Square cells and complete 24 × 20 board |
| [200% gameplay](snake-200.png) | 2240 × 1720 | Native 2× scaling, full playfield and controls |
| [Start](snake-menu.png) | 1120 × 860 | Explicit start and speed selection |
| [Result](snake-result.png) | 1120 × 860 | Final score, Play Again, Change Speed and Return to Arcade |

## Reproduce

```sh
cargo fmt --all --check
cargo clippy --workspace --locked --all-targets -- -D warnings
REQUIRE_STOCKFISH=1 cargo test --workspace --locked --all-targets
cargo test -p omarchy-snake --no-default-features --locked
scripts/build.sh
ctest --test-dir build/pinball --output-on-failure -R 'theme-palette|theme-path|authored-upstream-table'
xvfb-run -a -s '-screen 0 1440x1100x24' env LIBGL_ALWAYS_SOFTWARE=1 SDL_AUDIODRIVER=dummy python3 scripts/native-check.py target/release/omarchy-retro-arcade
xvfb-run -a -s '-screen 0 2400x1800x24' env LIBGL_ALWAYS_SOFTWARE=1 python3 scripts/native-snake.py target/release/omarchy-retro-arcade
```

The native Snake harness needs Pillow, X11 and XTest, only as test dependencies. CI has the native Snake gate and screenshot capture. In this session, Unix display sockets were unavailable; the same Xvfb and real native app were run together over an authenticated local TCP display. This does not change the game, test inputs or rendering backend.

## Outstanding acceptance

- Shared hosted leaderboard: not integrated or deployed for Snake. See [the precise dependency handoff](LEADERBOARD-HANDOFF.md). Logical replay validation is tested; live tickets, identity, ties, deletion and retry flows are not claimed.
- Arch package build/install/reinstall through the PR workflow. The local host does not supply pacman/makepkg. The staged installation is verified but is not an Arch package acceptance result.
- Human Omarchy/Wayland desktop playtesting, audible sound quality/volume, keyboard feel on physical hardware, fractional scaling and low-end GPU behaviour. Automated XTest input is not human playtesting.

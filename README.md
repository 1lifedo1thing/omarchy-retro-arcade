# Omarchy Arcade

Five games. One native app. One more go.

**Circuit Pinball · Solitaire · Scram · Invaders · Chess**

Open Arcade, choose a game and play in the same window. `Ctrl+H` returns to the collection; `Ctrl+Q` closes Arcade. The games retain their approved artwork, settings, controls and local saves. Everything works offline.

## Install

Download the **arch-package** artifact from a passing build in this repository's Actions tab, extract it, then install the `.pkg.tar.zst` file:

```sh
sudo pacman -U ./omarchy-retro-arcade-*.pkg.tar.zst
```

The package includes every game and a private Stockfish engine. It replaces conflicting standalone game packages without deleting their user data. There is one desktop entry: **Omarchy Arcade**. No account or additional game downloads.

Development preview. Real Omarchy desktop acceptance is tracked in [verification](docs/VERIFICATION.md).

## Build

On Arch, install Rust 1.98+, CMake, SDL2, SDL2_image, SDL2_mixer and the native graphics dependencies listed in `packaging/PKGBUILD`.

```sh
scripts/build.sh
./target/release/omarchy-retro-arcade
```

For computer Chess in a source build, provide Stockfish through `OMARCHY_CHESS_ENGINE`. The Arch package builds and bundles the pinned engine automatically:

```sh
packaging/build-arch.sh
```

The package builder requires a clean committed checkout and does not install anything.

## Source layout

- `arcade/`: the Rust app, collection shelf and local Pinball transport.
- `games/`: five ordinary source directories with their full imported Git history.
- `packaging/`: one Arch package, icon and desktop entry.
- `scripts/`: shared build, staging and verification entry points.

Four Rust games draw directly into the shared window. Pinball retains the upstream C++ physics engine in a private worker whose rendering appears in that same window, including on Wayland. No browser, X11 child-window embedding or separate game launcher is used.

Existing save paths remain authoritative. Pinball preserves high scores and settings, but does not resume unfinished tables. The other games save when returning to Arcade.

See [migration provenance](docs/MIGRATION.md), [integration decisions](DECISIONS.md), and each game's licence and artwork notices. The combined application is distributed under GPL-3.0-or-later; permissively licensed components retain their notices. This is a community project.

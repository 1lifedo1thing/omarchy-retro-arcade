# Omarchy Space Cadet

Classic desktop pinball, with colours that follow Omarchy.

A native C++/SDL application based on [SpaceCadetPinball](https://github.com/k4zmu2a/SpaceCadetPinball). Independent community project. **Development preview: bring your own game resources.**

## What is implemented

- Live Omarchy colours across the menus and rendered table.
- Midnight and Amber palettes, plus original table colours.
- Separate preferences and local high scores.
- Native launcher, first-launch game-data selection and launcher action to change the folder.
- Existing pinball gameplay, keyboard/controller controls, fullscreen, audio and local multiplayer from upstream.
- Linux build checks and a local Arch package recipe.

This is currently a recoloured source port. A wholly original Omarchy table, artwork and audio pack is still outstanding; see [decisions and limitations](DECISIONS.md).

## Build on Omarchy

From a checkout with the build dependencies installed (`cmake`, `ninja`, a C++ compiler, SDL2 and SDL2_mixer development files):

```sh
cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr
cmake --build build --parallel
ctest --test-dir build --output-on-failure
```

For a package, use the local recipe:

```sh
cd packaging
makepkg -si
```

The package uses the enclosing checkout. Review the source first. It is not yet an official repository package.

## First launch

Open **Omarchy Space Cadet** from the application launcher and select your existing Space Cadet resource folder. Keep the DAT file together with the original sound/music resources and subdirectories.

Or select the folder explicitly:

```sh
omarchy-spacecadet --data-dir /path/to/game-data
```

Supported data filenames include PINBALL.DAT and CADET.DAT (also lowercase). The folder is remembered. Use `--choose-data` or the launcher's **Choose game data folder** action to change it.

To run a build before installation, launch the absolute path to `bin/omarchy-spacecadet-game` with your resource folder as its working directory. The installed launcher performs this setup for you.

## Appearance

The **Appearance** menu offers Follow Omarchy, Midnight, Amber and Original table colours. Preferences persist independently of the upstream app.

Follow Omarchy reads `${XDG_CONFIG_HOME:-~/.config}/omarchy/current/theme/colors.toml` and notices changes within about two seconds. Set `SPACECADET_THEME_FILE` to read another colours file. Missing/invalid assignments retain the fallback palette. The app does not execute theme files or write to Omarchy configuration.

## Controls and resources

Use Game > New Game to start, and the Options menu to inspect/change key bindings. Existing pause, fullscreen, sound and controller options remain available. `-sw` requests software rendering; `-noaudio` disables audio initialization.

No original external game resources are downloaded or bundled. The source retains upstream embedded resources; see [UPSTREAM.md](UPSTREAM.md) and [README.upstream.md](README.upstream.md) for provenance and original build instructions.

## Verification

CI builds on Ubuntu, runs palette and launcher tests, validates the desktop entry and stages installation. A second job builds and installs an Arch package, available as the `omarchy-spacecadet-arch-x86_64` workflow artifact. Both jobs have passed. The Ubuntu tarball is not a portable cross-distribution release. Gameplay and Omarchy desktop acceptance still require a real desktop with game data.

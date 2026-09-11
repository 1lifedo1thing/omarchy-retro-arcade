# Omarchy Space Cadet

The original Space Cadet source port, with Omarchy appearance and Arcade desktop integration. The default now runs the upstream physics, table logic and missions.

**Original game resources are required.** The executable does not contain the Windows table data, artwork or sounds.

## Install and play

Download the Arch package from a successful [Linux build](https://github.com/tcballard/omarchy-spacecadet/actions/workflows/linux.yml), unzip it, and install its package with:

    sudo pacman -U /path/to/omarchy-spacecadet-package.pkg.tar.zst

Or run makepkg -si from the checkout's packaging directory.

Open Omarchy Space Cadet from the app menu. On first launch, choose your original Space Cadet resource folder. Keep the complete folder, including PINBALL.DAT and its sounds/music, together. The folder selection is remembered. Full Tilt CADET.DAT and demo data remain supported by upstream.

You can also select the folder explicitly:

    omarchy-spacecadet --data-dir /path/to/game-resources

The app does not download or bundle the original resources. Once configured, it opens the original engine directly. Missing data never silently launches a different pinball game.

## Controls and appearance

Fresh settings use A/D for flippers, Space for the plunger, P for pause, F2 for new game and F11 for fullscreen. Escape pauses. Controller shoulders, A and Start remain available. Settings includes configurable controls, Appearance and Sound. Existing custom bindings are preserved.

Follow Omarchy reads the active palette, including the current state-directory location and older configuration fallback. Original table colours are available for comparison. The colour transformation changes rendered pixels, not collision geometry, physical constants or mission logic.

The launcher/window icon and About screen identify this independent Omarchy Arcade application. Official Omarchy assets retain their owner's rights; the source port and application code are MIT.

## Updates and local state

Install a newer package with the same pacman -U command. High scores and engine settings are local, normally under ~/.local/share/omarchy-spacecadet/ (respecting XDG_DATA_HOME). The remembered resource path is under the user's configuration directory.

The original engine does not implement the experimental table's mid-game save/resume. Its scores and rules are different, so those saved games are not converted. Experimental saves remain untouched.

## Experimental table

The separately authored table is retained explicitly for existing users:

    omarchy-spacecadet --experimental

It uses different physics/rules and retains its own save/resume. It is not a fallback or a reproduction of Space Cadet. Screenshots in docs/native-table*.png show that experimental table, not the original engine.

## Verification and rebranding

CI compiles both engines, tests launcher routing and builds/installs the Arch package. Its resource-free gameplay smoke test and saved-game upgrade test exercise only the experimental table. They do not verify original-engine gameplay.

See [resource and rebranding plan](docs/ORIGINAL_ENGINE.md) for the remaining data-dependent work, and [Arcade standard](docs/ARCADE_STANDARD.md) for the collection target.

Version tags matching the package version build GitHub prereleases with packages and checksums. No release is implied by a development PR.

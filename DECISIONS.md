# Build decisions

## 11 September 2026

- Working name: **Omarchy Space Cadet**, executable `omarchy-spacecadet`.
- Native Linux application built from the C++/SDL source port. No browser wrapper, shell plugin, account or service.
- Import a pinned upstream snapshot and preserve its licence and credits. Direct Git push was unavailable; source was imported through the authenticated GitHub connection. Upstream revision is recorded in UPSTREAM.md.
- Preserve existing physics, rules, local high scores, multiplayer, pause, fullscreen, keyboard/controller configuration and sound support.
- Follow Omarchy's `current/theme/colors.toml` through its stable path, respecting XDG_CONFIG_HOME. Re-read at most once every two seconds; use literal colour parsing only. Midnight fallback when unavailable, Amber alternative, original-table-colours option.
- Tint the final rendered table through a precomputed 32,768-entry lookup table; keep warm lights distinct and preserve bright highlights. This changes appearance without modifying physics or original assets.
- Keep settings separate from upstream under the SDL preference directory for `omarchy-spacecadet`.
- Native first-launch folder selection with Zenity; explicit `--data-dir` and SPACECADET_DATA_DIR for scripting. Persist only the directory path, never execute it as shell text. Reuse the user's resource directory without copying it.
- Local Arch PKGBUILD and a Linux CI build artifact first. No marketplace submission or stable release claim yet.
- Fix the two upstream scalar-delete/array-allocation mismatches encountered in bitmap scaling.

## Material limits / outstanding work

- This is a theme-aware port, **not yet an entirely original Omarchy table**. Existing artwork, mission names and audio remain when using original data.
- A distributable replacement resource pack requires original background, sprites, depth/occlusion data, table geometry/records, fonts, sound effects and music. An image alone cannot replace this pack. Embedded upstream icon/font/resource provenance also needs auditing.
- Full gameplay, visual contrast across actual Omarchy themes, audio, controller input and Hyprland launch identity require a real desktop run with valid game resources.
- Linux compilation, staged installation and an actual Arch makepkg build/install passed in CI (run 34622571841). Interactive Omarchy desktop acceptance still remains.
- No claim of official Omarchy affiliation. No automatic download of proprietary game resources.

## Verification evidence

- C++ palette parser/colour mapping tests: passed locally and in Linux/Arch builds.
- Launcher tests: passed, including paths containing spaces and shell syntax, persistence and invalid-folder handling.
- Ubuntu compilation, desktop entry validation and staged install: passed.
- Arch package build, installation and executable version probe: passed.
- Original-data retrieval for local gameplay testing was unavailable in this environment. No gameplay or screenshot claim is made.

## Continuation: self-contained original table

The entries above describe the first, incomplete colour-only port. The following decisions supersede its default-game dependency and feature scope.

- Default to a newly authored original Omarchy table that needs no external game files. Preserve the imported classic source port as `--classic`. The new table uses a separate small physics/rules model and the source port's SDL/ImGui foundation; it does not reproduce Space Cadet's exact physics or missions. This distinction is deliberate and user-visible.
- Use a fixed 120 Hz simulation with four collision substeps per tick. Ball speed is bounded. Rails, bumper circles and moving flipper capsules share their geometry with the drawing code.
- Ship three balls, charged launch, ball save, bumper scoring, O/M/A lane lights, targets, circuit bonuses, 1x–5x multipliers, nudge/tilt and game-over handling. No network mode or paid assets.
- Synthesize original sound effects and a short original optional ambient note sequence in code, with mute support. No original music or recordings are used in the default mode.
- Save the current game, high score and preferences locally using atomic replacement. Validate recovered state and resume paused. Keep these saves separate from classic mode.
- Include the exact official Omarchy SVG files. Use the official green mark on the playfield, header, window icon and desktop launcher. Transparent proportional rasterization is used only for SDL. Keep the brand assets' rights separate from the MIT code licence.
- Correct the initial theme-path assumption: current Omarchy uses `~/.local/state/omarchy/current/theme`. Support XDG_STATE_HOME first, current Omarchy's home path next, and legacy configuration locations last. Add a regression test for precedence and directory replacement.
- Keep the native app keyboard-operable. Support common controller shoulder/A/Start controls and pause after focus/controller loss. Confirm replacement of an active game.
- Original graphics are drawn directly from the table geometry; there is no generated mockup masquerading as an app screenshot.

### Verification and remaining hardware acceptance

- Run the actual native executable with SDL's software renderer and dummy audio, including screenshots of real game frames.
- Test physics/rules, deterministic simulation, save round trips/rejection, launcher mode selection and theme-path precedence. Address/undefined-behaviour sanitizer checks run with leak scanning disabled because the environment cannot inspect process threads.
- Inspect the normal and smaller light-theme render. Preserve official logo shape/colour and verify source hashes.
- CI must compile both engines, build/install the Arch package and run the default game with no external data before this revision is handed off.
- Hardware acceptance still includes real Omarchy/Hyprland focus/fullscreen behavior, fractional scaling, physical controller input, subjective pinball feel and listening to audio. Classic mode gameplay is still unverified here because no original data was available.

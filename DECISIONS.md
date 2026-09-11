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
- Linux compilation and packaging are checked in CI. An Ubuntu build artifact is not a verified Arch package; run makepkg and install-test on Omarchy before release.
- No claim of official Omarchy affiliation. No automatic download of proprietary game resources.

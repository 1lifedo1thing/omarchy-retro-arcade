# Blast verification handoff

Implemented on `feat/blast`, based on the committed consolidation branch at `b1dec039416e539c13901851c01727f93bb553ec`. Bubble's separate working checkout was inspected for coordination and not modified. When combining both game additions, retain both workspace members, shelf entries, dependencies, screenshots, installation notices and native checks; compute shelf navigation from the resulting game order.

## Passed locally

- 136 Rust workspace tests, including 16 new Blast simulation/preference tests. Existing Chess tests used the real Stockfish engine.
- Workspace formatting and strict Clippy for all targets.
- Optimized single Arcade executable build.
- Original Pinball engine build and all three selected palette/path/authored-table checks.
- Installation staging through the ordinary collection install script. Blast contributes no executable or desktop entry.
- The existing native collection test against the staged release: singleton lock; same-window game switching, including Blast; Solitaire draw/save/reopen; original save paths; Stockfish responding to a native move; Pinball interaction and clean shutdown.
- Blast's automated native test against the staged release: live solo movement and bombs, stable pause/focus-loss scenes, explicit resume, shelf navigation, P2 Enter/arrows, a complete first-to-three local match, one persisted record and process-restart persistence.
- Visually inspected actual gameplay screenshots at 1120×860, 900×760, 2240×1720 (200%) and a light Omarchy palette. Characters, tiles, HUD and controls stay visible without stretching the grid.

## Not claimed

- No human two-player session or Omarchy/Hyprland hardware playtest. Physical keyboard rollover, audible sound and longer bot balance testing remain manual checks.
- No Arch CI result: publishing is blocked, so GitHub Actions has not run this branch. The existing Arch workflow now includes Blast's installed local-match test. Local staging is not an Arch package test.
- No merge or release.

## Publication blocker

`git push -u origin feat/blast` fails because HTTPS Git credentials are unavailable in this workspace. The connected GitHub service confirms `tcballard/omarchy-retro-arcade` is empty (no base branch; README fetch returns “This repository is empty”). A PR cannot currently target the consolidation branch. No replacement root/main history was created and no other working checkout was overwritten.

The saved bundle contains the full branch history, including the consolidation base. Once the consolidation is published, fetch it and rebase Blast as needed, then push `feat/blast` and open a PR against the consolidation branch (or `main` after consolidation merges). Preserve Bubble when resolving shared shelf/workspace changes. Run the configured CI and address failures before merge.

# Omarchy Arcade experience standard

Tom's collection requirements: each game opens directly into something playable; consistent menus, shortcuts, theme behaviour and sound controls; matching launcher icons and About screens; local saves/settings and straightforward package updates.

This document is the implementation contract for Chess and a portable handoff for the other Arcade games. It does not claim the other repositories have already adopted it.

## Shared conventions

| Surface | Convention |
|---|---|
| Launch | Resume the saved game or open a playable board. No splash screen, account or mandatory setup wizard. |
| Menus | Game, Settings, Help. A visible New Game action is allowed beside them. |
| Shortcuts | Ctrl+N new game; Ctrl+Z undo where supported; Ctrl+, settings; Ctrl+M sound; F1 help; Ctrl+Q quit. Game-specific shortcuts remain in Help. |
| Sound | Off on first launch, explicit toggle in Settings, remembered across restarts. Short original cues; no background music by default. |
| Appearance | Follow Omarchy by default; shared restrained fallback palette when disabled/unavailable. Support light and dark colours with readable controls. Never write to Omarchy's theme files. |
| Icon | 128×128 SVG, rounded dark tile (#171c1a), pale green game silhouette (#b3cb92). Same frame, padding and small four-square Arcade family mark. Game symbol varies. |
| About | Same game icon, game name, “Omarchy Arcade”, version, concise controls, credits and license. Identify community status without implying official inclusion. |
| Storage | Per-game local session and settings; atomic writes; preserve user data on package updates. No cross-game process may overwrite another game's files. |
| Distribution | Native package, desktop entry, stable executable/icon identity. Update with pacman -U; package scripts never reset user data. |
| End of game | Clear result and visible Play Again/Rematch. Preserve the previous game before replacing it. |

Theme source: released Omarchy v4.0.3 uses `~/.local/state/omarchy/current/theme/colors.toml`. Chess also supports XDG state overrides and the legacy config location. Source: https://github.com/omacom/omarchy/blob/v4.0.3/bin/omarchy-theme-set . Poll every two seconds.

## Chess implementation

Chess remains untimed and opens directly into a board. The Arch package includes a pinned Stockfish executable, so installation includes the opponent. A source build without Stockfish starts a local game; Settings offers an executable picker. Computer strength labels do not claim an Elo rating.

`settings.json` sits beside `session.json` under the game's XDG state directory. Sound and Follow Omarchy persist independently of the game. Stockfish executable choice is local and passed as an executable path, never a shell command. Original malformed settings are preserved.

Accessible square labels and activation actions, polite move-status announcements and keyboard controls are implemented. Real assistive-technology testing remains required before claiming complete screen-reader compatibility.

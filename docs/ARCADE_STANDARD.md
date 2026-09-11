# Omarchy Arcade standard

Agreed collection baseline, 11 September 2026. These are acceptance criteria, not a claim that every game already conforms.

| Area | Shared requirement |
| --- | --- |
| Startup | Open into a ready game or restore local progress. No account, network or asset download needed for default play. Real-time games restore paused with an obvious Resume control. |
| Menus | Use Game (New game, Pause/Resume where applicable, Quit), Settings (Appearance, Sound), Help (How to play, About). Keep controls keyboard accessible and readable at supported window sizes. |
| Shortcuts | F2: New game, confirming before discarding progress. P: pause/resume real-time play. Escape: pause or dismiss a panel without losing progress. F11: fullscreen where supported. Show game-specific controls and avoid gameplay input inside text fields/dialogs. |
| Theme | Default to Follow Omarchy; update while running; provide a readable fallback and remember explicit overrides. Keep gameplay colours legible and official brand artwork unchanged. |
| Sound | Consistent Mute and Music labels where applicable. Mute silences all audio; remember choices. Optional music starts off. Missing audio must not prevent play. |
| Identity | Matching launcher icon sizing, treatment and brand placement, with a distinct symbol for each game. Preserve the official logo unchanged whenever used; identify the project as independent. |
| About | Consistent order: game name, installed version, Omarchy Arcade membership, description, project/support link, upstream credits, code licence and brand attribution. |
| Data | Local per-user saves/settings outside package-owned files. Automatic saves, validated recovery, clear write errors and save compatibility or migration across updates. |
| Updates | Stable package names, increasing versions and durable versioned package downloads. Document pacman installation/update. No separate in-game updater required. CI artifacts remain development downloads. |

## Pinball assessment

Updated for the 0.2.1 Arcade milestone:

| Area | Current status |
| --- | --- |
| Direct play | Implemented without external data; saved games restore paused. |
| Controls | F2, P, Escape and F11 implemented. Game/Settings/Help menus implemented; opening them pauses play. |
| Theme/sound | Theme following, remembered overrides, Mute and optional Music implemented. Live desktop and listening checks remain. |
| Icons/About | Distinct pinball icon with unchanged official mark installed. About includes version, collection name, support link, upstream credits and licence/brand information. |
| Saves | Atomic writes and recovery validation implemented. Linux data normally lives in `~/.local/share/omarchy-spacecadet/`, respecting `XDG_DATA_HOME`. |
| Updates | Version 0.2.1 retains the package name. CI installs 0.2.0, seeds a saved game, upgrades, then verifies that the native app restores and re-saves unchanged data. Version tags trigger package/checksum prereleases; no tag or release is published by this PR. |

Other games have not been audited by this change. Real Omarchy desktop acceptance and game-specific quality checks remain necessary: this interface standard alone cannot establish polished pinball physics.

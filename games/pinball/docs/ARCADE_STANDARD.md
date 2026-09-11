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

## Pinball assessment at 0.5.0

Default play is the self-contained Omarchy Circuit table running on upstream physics/components. Game/Settings/Help menus, theme following, mute, Arcade icon/About and local high scores/settings are present.

The default integration test runs the actual upstream executable with authored data and no DAT. Classic and experimental modes remain explicit options.

Remaining collection gaps: Circuit has no mid-game save/resume, and real desktop and hands-on play acceptance remain. Its single-player circuit, target, ramp and orbit rules differ from Space Cadet missions. See [table implementation](ORIGINAL_ENGINE.md).

# Decision log

These are implementation choices made under the instruction to build autonomously. The current choices are approved as recorded below. No release, marketplace submission or upstream endorsement is implied.

## Approval

Tom approved all current implementation decisions following delivery of the Rust build: “I’m happy with all decisions you made”. This approval covers R01–R10 and the earlier choices that they do not supersede, as delivered at commit `1023007fc808395adcb0cb174f614664c6bdcb66`. Superseded Python choices remain historical only.

These choices are now the agreed baseline. The approval does not establish completion of the outstanding desktop acceptance checks or change the draft PR/release status.

| ID | Choice made | Reason / trade-off | Review priority |
|---|---|---|---|
| D01 | Python 3.11+, Qt Widgets via PySide6, python-chess | A real native Linux window and mature rules/UCI support made a testable first build practical. A Rust rewrite would be a separate decision; Python is the largest stack choice here. | High |
| D02 | Standalone app; `omarchy-chess` launcher | Matches the agreed own-window experience. No shell plugin, browser runtime or background service. | Low |
| D03 | Stockfish as an external executable | Uses maintained engine builds and avoids redistributing CPU-specific binaries/NNUE files. Install package alone can play locally; computer play requires Stockfish. Fully automatic engine installation is unfinished. | High |
| D04 | Fresh bounded engine process per move/hint | Easy cancellation and no retained engine state between games. One thread, 32 MB hash, 0.15–1 second search. Startup overhead is the trade-off; NNUE memory is additional to the hash budget. | Medium |
| D05 | Gentle/Casual/Club/Strong map to Skill Level 0/5/10/20 | Human-readable choices without claiming Elo. Gentle is not guaranteed suitable for a new player. Hints use Strong. Real difficulty calibration remains work. | High |
| D06 | Standard chess only, untimed | Focuses correctness and easy play. No clocks, Chess960, online play, accounts, ratings, teaching system or puzzles in this preview. | Medium |
| D07 | Start as White against Casual Stockfish | Fastest first launch. Colour, mode and strength chosen in New Game; saved choices resume. | Low |
| D08 | 2D board using python-chess's GPL piece artwork | Recognisable pieces at small sizes. Avoids speculative 3D design or a new art dependency. Original simple app icon; no official Omarchy logo. | Medium |
| D09 | Read four validated colours from Omarchy's colours TOML; poll every 2 seconds | Handles replacement of the current-theme symlink. Theme files are data, never shell code. Board squares blend the accent into stable luminance ranges. | Medium |
| D10 | Automatic session save and one writer per state directory | QLockFile stops concurrent windows overwriting one session. Disk writes use temp file, fsync and replace. This is a local session, not a cloud account. | Low |
| D11 | Archive old game before New Game/import | Protects the previous main line. Corrupt-session recovery copy preserves original bytes. No archive browser yet; import PGN via the standard file chooser. | Medium |
| D12 | Explicit claim for threefold/fifty-move draws; automatic fivefold/75-move draws | Uses library adjudication. Claims involving an intended legal move are handled by the library's `can_claim_draw`; the UI does not ask players to nominate that move. | Medium |
| D13 | Practice aids allowed | Hint and takeback are optional actions, guides are toggleable. No rated/competitive promise. Computer takeback undoes to the player's turn; local takeback undoes one ply. | Low |
| D14 | History selection is read-only; no branching editor | Easy review without silently changing the live game. Return Live resumes input. An engine already thinking can finish and return the display to live. | Medium |
| D15 | PGN import keeps one main line, not annotations/variations | Validates moves and starting position before replacing state. Original file stays intact. Max 1 MB; multi-game files and variants rejected. Imported games use local mode. | High |
| D16 | GPL-3.0-or-later for this project | Fits the chosen rules library and piece artwork. Full license and third-party notices included. Dependencies remain separately installed. | High |
| D17 | Build recipes and CI artifacts, no stable tag/release | A runnable development preview is appropriate before a real Dell/Omarchy acceptance pass. Native Arch package built from exact source commit; Python dependency recipe is pinned and checksummed. | High |
| D18 | Keyboard controls and move text field now; screen-reader acceptance pending | Tab navigation, named controls and board focus messages exist. A custom painted board does not yet expose 64 individually accessible square objects. Do not claim full screen-reader support. | High |
| D19 | Work delivered on a feature branch and draft PR | Main gets a minimal project introduction; implementation commits stay reviewable. No automatic merge. | Low |

## Rust migration — supersedes stack-specific choices above

**User requirement:** Rust is Tom's default for new work together. Omarchy Chess must use Rust. This supersedes D01; Python is no longer a pending choice. The original table is retained as historical context, not the current stack specification.

| ID | Current choice | Reason / trade-off |
|---|---|---|
| R01 | Rust throughout; project guidance in AGENTS.md | User-directed. No Python runtime or Python build steps. |
| R02 | eframe/egui 0.31, OpenGL, Wayland and X11 | Native compiled Linux window with custom-drawn controls; these are not GTK/Qt system widgets. Mature pinned API; upgrades are separate work. |
| R03 | shakmaty 0.27 and pgn-reader 0.26 | Established Rust legal-move and PGN support. Replaces python-chess rules, preserving GPL-3.0-or-later. |
| R04 | External Stockfish over our bounded UCI adapter | Preserves D03–D05. Nonblocking command writes and cancellation prevent a stalled engine from blocking UI work. |
| R05 | Embedded Cburnett SVGs from prior dependency | Supersedes D08 distribution details. Attribution retained; no Python dependency. |
| R06 | Version 2 JSON with legacy save import and backup | Reconstructs full history. fs2 advisory lock replaces QLockFile (D10); existing legacy lock blocks startup. |
| R07 | App implements repetition/50-move claim policy over legal positions | Supersedes D12 library API details. Intended-move claims are available without a nomination dialog, as before. |
| R08 | Rust binary and one Arch package | Supersedes Python packaging in D17. Toolchain 1.98.1 and Cargo.lock pinned; thin LTO disabled after release-link failure. ARM packaging declared but not tested. |
| R09 | XDG portal file dialogs, AccessKit enabled | Requires a working desktop portal. Full accessible 64-square board and screen-reader acceptance remain unfinished (D18). |
| R10 | Preserve feature scope and draft PR | No clocks, online play or speculative engine rewrite. Real Hyprland/Dell acceptance remains required before stable release. |

## Future product review

1. Does the Rust native interface feel right, including portal dialogs and keyboard interaction?
2. Does the restrained 2D board feel right on the Dell with real themes?
3. Should the next milestone prioritise a one-step Stockfish installation, beginner-friendly opponents, clocks, or accessibility?

None of these questions blocked the implementation. They identify the choices worth revisiting after playing it.

## Arcade polish milestone

Tom requested delivery of the polish milestone and supplied the shared four-point Arcade standard. The implementation contract is in docs/ARCADE_STANDARD.md. These follow-up implementation details remain logged for review:

- P01: Keep direct-to-board launch. Arch now includes a pinned Stockfish build; a source build without it starts a local game. This updates the earlier optional package dependency choice. No automatic privileged installation from inside the app.
- P02: Game/Settings/Help, Ctrl+, settings, Ctrl+M sound, shared icon tile and Arcade About identity. The other repositories still need to adopt the same contract; this change does not silently modify them.
- P03: Original short PCM sound cues through paplay, off by default. Arch depends on libpulse. Audio failures do not block play; actual desktop audibility needs acceptance.
- P04: Local versioned settings independent of game saves; malformed settings preserved. Updates keep state and executable identity.
- P05: Correct theme lookup to the released v4 state path with legacy fallback. Light/dark widget styling and explicit panel fills fix the preview's dark-only controls.
- P06: Drag ghost, rematch, named accessible square actions and polite status announcements. Accessibility tree tests establish the exposed data/actions, not complete Orca compatibility.
- P07: Version 0.2.0 development package. No stable-release claim before the documented desktop acceptance pass.

- P08: CI established that stockfish is unavailable in the standard Arch repositories used here. Include a private generic-CPU Stockfish executable in the app package, built from checksum-pinned upstream source/network; publish corresponding engine inputs alongside it. This increases the download by roughly 94 MB of network weights and removes a separate engine installation requirement. A system/custom engine can still be selected.

## Chisel artwork integration

- C01: Tom selected Chisel in the design thread and authorized bringing the delivered set into the game. Import all twelve SVGs unchanged; preserve their sculpted planes, fine rims, common 128×128 viewBox and relative heights.
- C02: Recolour only the single inlay fill token to the active Omarchy accent. Keep body, facet and outline colours fixed for side recognition. Invalidate all twelve cached images when the accent changes; use the SVG loader's size-aware rendering for window/display scaling.
- C03: Chisel includes its own transparent margins. Remove the former extra 10% inset around board images and match the dragged piece's footprint to its square image; native renders showed that applying both margins made the pieces unnecessarily small.
- C04: Replace the default artwork without adding a new style selector in this milestone. Previous Cburnett artwork remains recoverable in Git history. No changes to rules, saves, engine selection or package dependencies.

## After Hours default

- A01: Tom explicitly selected After Hours as the default. Existing settings without a piece style also receive After Hours; an explicit Chisel selection persists across updates.
- A02: Keep Chisel selectable in Settings. Both sets retain their complete viewBox and original geometry, and only the isolated accent follows the theme. Style and accent changes refresh cached artwork for both board and drag rendering.

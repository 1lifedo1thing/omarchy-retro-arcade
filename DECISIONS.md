# Decisions for Tom to review

These are implementation choices made under the instruction to build autonomously. They are reviewable defaults, not new requirements attributed to Tom. No release, marketplace submission or upstream endorsement is implied.

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

## Most useful next review

1. Keep the Python/Qt stack, or change before investing further in polish?
2. Does the restrained 2D board feel right on the Dell with real themes?
3. Should the next milestone prioritise a one-step Stockfish installation, beginner-friendly opponents, clocks, or accessibility?

None of these questions blocked the implementation. They identify the choices worth revisiting after playing it.

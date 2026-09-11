# Arcade acceptance: Chess

Use the `rust-arch-preview` artifact from the current passing PR run. Extract it and install with `sudo pacman -U ./omarchy-chess-*.pkg.tar.zst`. The package includes Stockfish; pacman resolves runtime dependencies. Launch Chess from the application menu.

1. Fresh install: board opens without a wizard. Move e2 to e4; Stockfish replies. Open Game → New Game and play Black; the computer opens.
2. Local play: choose Friend. Click, drag and use arrow keys/Enter to move. Check promotion selection and undo. Keyboard focus stays visible; Tab can leave the board.
3. Settings: change Follow Omarchy and Sound; close/reopen the app. Choices persist. Ctrl+, opens Settings; Ctrl+M toggles sound. Check the short move cue through desktop audio.
4. Appearance: switch actual Omarchy light/dark themes. Confirm colours update within two seconds, text remains readable, and the board and dialogs fit at 740×560 and normal/fullscreen sizes. Test fractional scaling.
5. Finish: play local Fool's Mate (f3 e5 g4 Qh4#). Check the result, then Play Again. The previous game is archived.
6. Save and update: play a few moves, quit, install a newer package with pacman -U, reopen. Position/history and settings survive. Reboot and repeat.
7. Files: export PGN with the desktop portal, then import it. Cancel both dialogs and check the game is unchanged.
8. Accessibility: use Orca to inspect all squares and pieces, activate a move, and hear turn/status changes. Check dialogs and menus without a mouse.

Report the package version, desktop version, scaling, steps and expected/actual behaviour for any failure. Screenshot where useful. Passing CI or Xvfb does not count as this real desktop pass.

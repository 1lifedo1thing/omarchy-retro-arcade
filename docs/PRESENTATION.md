# Arcade presentation

The collection uses one visual language: machined charcoal surfaces, warm ivory type, brass framing and restrained Omarchy accent light. Circuit's approved illustrated table is the reference for the level of craft. Each game keeps its own identity and readable playfield.

## Opening the collection

All nine games are visible in the selector. A large preview, game name, genre and Play action describe the current selection. Arrow keys select; Enter launches. Tab reaches the native controls. Returning to Arcade retains the selected game. Circuit retains its explicit confirmation before ending a table.

## Treatments

| Game | Presentation work |
| --- | --- |
| Circuit Pinball | Approved table remains intact; common navigation; screenshot capture waits for an actual table frame. |
| Stack | Recessed board surround, bevelled blocks, clearer reserve/next labels and a composed mode-selection screen. |
| Snake | Machined boundary, contact shadows, body highlights and jewel-like food; head and food remain distinct by shape. |
| Bubble | Framed playfield and dimensional glass bubbles; all six non-colour symbols remain legible. |
| Blast | Bevelled metal walls and crates, framed arena and retained original characters. Enter starts a match and advances rounds. |
| Scram | Layered maze edges and engraved wall surfaces; original characters and collision geometry remain intact. |
| Chess | Board surround, subtle square grain and contact shadows beneath the approved pieces. |
| Solitaire | Fine cloth texture and card shadows; approved decks, artwork choices and animation remain intact. |
| Invaders | Framed instrument display and collection materials around the approved pixel artwork. |

`shared/presentation` centralizes reusable materials and control geometry. The cabinet texture is decoded once per egui context, and static detail introduces no new animation. Light themes use an engraved light surface instead of dark artwork behind light-theme text. Existing motion, audio and persistence preferences remain authoritative.

## Verification

The initial integrated revision passes 197 workspace tests, strict Clippy and formatting. The release executable builds, the three Pinball engine/theme tests pass, and staged installation contains one executable, one desktop entry, per-game licenses and the new artwork provenance.

Native screenshots, keyboard switching and clean Arch install/upgrade verification run in `.github/workflows/arcade.yml`. `scripts/polish-renders.py` captures every game and the shelf in dark, light, compact and 200% layouts, using real XTest input. It contains no alternative gameplay or renderer.

A local sandbox limitation prevents opening a display socket in this session. GitHub's `polish-review` artifact is therefore the source for the rendered review. Final results will be recorded after that review. Automated Linux/X11 evidence does not replace hands-on Omarchy/Wayland playtesting or audible-device acceptance.

The separate optional leaderboard service remains disabled for public use. No hosting, account requirement or second launcher is introduced by this presentation change.

# Arcade architecture

Audit baseline: `0aa746357192e488d2c2d077f279a347c1fff6e3` (19 September 2026).
Reference: [OmaCut at 0948c46](https://github.com/omacom/omacut/tree/0948c4615d45ac62727b8c69112178e09781b7a4).
This adopts separation of responsibilities, explicit resource ownership and desktop
integration from that example. It does not prescribe its language or toolkit.

## Existing strengths

- Fourteen ordinary game directories, one window and one desktop identity.
  Game rules remain with their game; several engines already build without UI.
- Shared cabinet rendering is in `shared/presentation`, separate from simulation.
- Pinball preserves its C++ engine behind a private worker. Bounded command queues,
  frame validation, latest-frame storage and owned process teardown already exist.
- Player packaging explicitly selects runtime files. Stockfish, required artwork
  and component licences are retained; development images are excluded.
- CI already exercises workspace tests, native switching, input, save protection,
  rendering and Arch installation/upgrade. These are stronger than source-only checks.

## Host responsibilities

| Module | Owns | Must leave elsewhere |
| --- | --- | --- |
| `catalog.rs` | Stable IDs, names, ordering, taglines and shelf image references | Game construction and save handling |
| `session.rs` | Per-game adapters, construction, save locks and teardown | Navigation and simulation rules |
| `desktop.rs` | App singleton lock, window identity, icon and native options | Game-specific state schemas |
| `main.rs` | CLI, host UI, navigation, active-session selection and capture | Per-game constructors and desktop setup |
| `shelf.rs` | Collection layout, previews and selection interaction | Session resource ownership |
| `pinball.rs` | Private engine process, protocol, frame delivery and input translation | Shelf navigation and other games |

CLI help derives its game IDs from the catalogue. Adding a game still requires an
explicit constructor and lifecycle adapter; no registration framework is needed.

## Session contract

`Active` owns a game and any legacy save lock. Returning home, replacing a session,
or exiting drops this owner. Its destructor calls `on_exit` once; the game is then
destroyed before its lock is released. Save paths and schemas remain game-owned.
Tests protect that ordering for both leaving and replacement.

The existing adapter hooks retain their meaning:

- `prepare_style`: apply game-specific navigation styling before host controls draw.
- `suspend`: pause without an implicit resume, currently used by Pinball confirmation.
- `set_input_enabled`: block modal input, including the frame that closes the modal.
- `ready`: permit capture once the game has renderable content (or an error).
- `finished`: request return to the collection.

These are not universal capabilities: several legacy games use default no-op hooks
and handle focus within their own update loop. Do not assume every adapter supports
modal input blocking or automatic resume. Extend a contract only with corresponding
adapters and native regression evidence.

## Follow-ups, in priority order

1. **Shared desktop utilities extracted.** `shared/platform` now owns palette
   loading and generic bounded-read/atomic-write helpers. Nine games and the host
   use it directly; Chess retains compatibility exports. Theme support is optional
   and uses the small `ecolor` type crate, not a windowing runtime. Existing
   headless paths remain intact. Save schemas, paths, locks and recovery policy
   stay with each game. `Theme::square` is retained for source compatibility with
   Chess's existing palette API; this pass does not redesign its rendering.
2. **Measure startup and teardown before introducing async construction.** Constructors
   currently run on the UI thread and some require an egui context. Pinball shutdown
   can spend up to its two-second grace period waiting before killing the child, then
   joins its I/O threads. Profile actual transitions; isolate slow CPU/I/O operations
   with cancellation while retaining save locks through completion. Moving all game
   objects onto worker threads would not be a safe mechanical change.
3. **Audit focus and input behaviour across adapters.** Existing implementations and
   native tests differ by game. Define observable pause/release/resume scenarios first;
   implement shared policy only where doing so preserves controls and saves.
4. **Consolidate repeated audio process management before global preferences.** Several
   games own `paplay` processes independently. A shared owned-player helper could reduce
   cleanup duplication. A global mute preference requires an explicit precedence and
   migration decision because current preferences are per-game.

This pass makes the host boundaries explicit. It does not change game rules, input
semantics, save formats, artwork, language choices, dependencies or package contents.
Further extractions should be separate reviewable changes with their own evidence.

## Verification

See the architecture entry in `VERIFICATION.md` for this branch's checks and limits.
Existing test scripts remain authoritative for native switching and package acceptance.

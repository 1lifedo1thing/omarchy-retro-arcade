# Integration decisions

## FreeSki project setup (12 September 2026)

- User requested repository preparation for issue #14 before discussing milestone
  1. `games/freeski` contains the requirements snapshot, five-milestone plan,
  first-playable discussion brief, tuning register, acceptance tracker and asset
  provenance location. Gameplay implementation has not started.
- The intended integration is an ordinary Rust game library in Arcade's existing
  window and package. Crate registration and shelf integration belong to the
  first playable milestone; setup adds no runtime entry or dependency.
- Milestone 1 establishes the authored practice slope, deterministic simulation
  and basic safe persistence. Later milestones harden those contracts, add
  endless skiing/pursuit and five Slalom courses, then complete release acceptance.
  Numerical defaults and the detailed first-playable design remain provisional.

## FreeSki first playable (12 September 2026)

- User authorized implementing milestone 1. Add `omarchy-freeski` as an ordinary
  Rust library, append FreeSki after 2048 and preserve the same window, desktop
  entry and package. The playable scope is an authored 1,200 m practice slope;
  endless generation, creature pursuit, Slalom and sound remain later work.
- Use 60 Hz production simulation with f64 world metres, bounded heading changes,
  a fixed logical view and swept circle/height checks. Practice finish and fatal
  collisions resolve in time order. A production-input reference finishes in
  3,397 ticks with three ramp jumps and zero crashes. Cross-platform bitwise libm
  reproducibility is not claimed; render schedules and same-build save continuations
  are tested independently of the UI.
- Independent versioned `omarchy-retro-arcade/freeski.json` state uses private atomic
  writes through the existing native storage helper; headless evidence uses an
  equivalent portable implementation. Both builds test retention and round trips.
  Records/preferences are logically separate from the attempt. Invalid files
  remain intact unless explicitly archived; every restored active run is paused.
- Original native geometry and SVG art reuse the approved cabinet material.
  A bounded trail buffer supports reduced effects. No other game assets or saves
  are changed. Native X11 evidence, actual Wayland rendering and human playtesting
  are separate acceptance categories; human feel/difficulty acceptance remains open.

## Original integration contract

- One repository, native window, desktop identity, Arch package and release version. Games are ordinary source subdirectories, not submodules or downloaded plugins.
- Four Rust/egui games are library dependencies of the Arcade executable. Preserve approved rendering, gameplay and existing storage paths. Acquire each game's original session lock before opening it, flush on leaving, and drop it on returning to the shelf.
- Circuit retains its C++ upstream engine. A private bundled worker renders through SDL software into the Arcade window. This avoids X11 window embedding and works with the same frontend on Wayland. The worker has no visible window or desktop entry. Its framed local pipes carry pixels and input, never network traffic.
- Existing per-game save directories and artwork choices remain authoritative. No bulk move or conversion risks existing saves. Circuit retains high scores/settings; like its source version it does not restore unfinished games.
- Shared navigation is Ctrl+H / Back to Arcade. Existing game shortcuts remain available. Leaving Circuit explicitly confirms ending the current table.
- Source repositories and open PRs remain intact until the consolidation is accepted. Imported Git history and source hashes make every migration traceable.

## Stack and optional community leaderboards

- Stack is a Rust library game inside the same eframe window, desktop entry and package. Both modes work offline; its state uses a new `omarchy-stack` directory without changing existing games' save locations.
- Gameplay uses deterministic 60 Hz ticks and documented Stack-specific symmetric rotation kicks. The service links this same engine with UI dependencies disabled. Rules are versioned independently from the app release.
- Shared HTTP transport is in `shared/leaderboard`; the separately deployable SQLite service is in `services/leaderboard`. No public URL is bundled. Explicit end-of-run sharing, pseudonymous credentials, bounded replay verification and private retry storage are required before results become public.
- The initial service is deliberately single-process and intended for a small community. Deployment behind the supplied HTTPS proxy, backups and operational checks must be verified before activating public sharing. Hosting is a separate approval gate, estimated at $11/month before tax in HOSTING.md.
- Snake is a Rust/egui library under `games/snake`, appended to the existing shelf and rendered in the same window. Shelf indexing derives from `Game::ALL.len()` to accommodate concurrent additions without replacing artwork or reordering the approved five entries.
- Snake's `snake-v1` engine and replay adapter build without desktop features. The concurrent Stack-only leaderboard is an explicit dependency; Snake performs no network activity and does not ship a second service. See `games/snake/docs/LEADERBOARD-HANDOFF.md`.
- Snake uses existing theme loading and atomic writes, an independent schema-versioned save directory, and separate records/session files. No other game's save or settings schema changes.
- Bubble is a Rust/egui library under `games/bubble`, loaded by the existing Arcade game trait. It adds no executable, desktop identity or installer. Six shelf entries now derive keyboard wraparound and counters from `Game::ALL`.
- Bubble uses analytic swept-circle shots against a staggered 10/9-column hex grid. A shot's computed polyline drives both the live animation and limited guide; rule resolution occurs once after flight, so frame rate cannot change attachment. Pressure moves the ceiling rather than changing grid parity.
- Bubble levels and their deterministic magazines are authored JSON. Removed colours fall back to the lowest remaining colour. The saved reference solutions include a quarter-degree aiming margin and are replayed through the production engine.
- Current Arcade audio and settings are per-game, not a central settings service. Bubble follows the existing `Ctrl+M` sound convention and owns/reaps its optional `paplay` process on pause and exit. It reuses the existing Omarchy theme loader and stores versioned, atomically replaced progress at `omarchy-retro-arcade/bubble.json` beneath XDG state. Corrupt/future saves remain untouched. Attempts restart when reopened; unlocked levels and personal bests survive.
- Blast is a Rust library in `games/blast`, hosted by the existing `ArcadeGame` lifecycle in the same window. No binary, desktop entry, package or release is added. Its singleton protection is the Arcade session lock.
- Blast reuses the collection's Omarchy theme loader, atomic private storage helper, Ctrl+, settings / Ctrl+M audio conventions, and shared shelf navigation. The current consolidation retains per-game preferences; Blast stores only its own `blast.json` in the Arcade state directory and does not migrate legacy game saves. There is no existing controller service to bind to.
- Arena simulation is integer 60 Hz. Movement intentions use snapshot occupancy: swaps and contested destinations both fail. Explosion chains use a common tile snapshot so a destroyed crate shields every ray in that wave. Elimination is simultaneous after hazard resolution. Newly revealed items stay uncollectable while their tile burns.
- Bots forecast the same hazard rules, including future crate destruction, chain reactions and closing walls. They search traversable positions over time and place a useful bomb only with a predicted escape. Forecasts cannot anticipate future bombs or future movement by other participants. All actions use the ordinary player rules.
- Blast artwork is original egui geometry and its audio consists of original synthesized PCM cues. No reference-game artwork, character designs or sounds are imported. The owned playback child is terminated and reaped on pause and game exit.

## Cohesive Arcade presentation (12 September 2026)

- This feature branch integrates the existing Stack, Snake, Bubble and Blast implementations with the five consolidated games. It preserves each engine, original artwork, saves, rules and optional leaderboard boundaries. No service is deployed or activated.
- The visual thesis is a crafted retro-futurist cabinet: charcoal metal, warm ivory lettering, brass edges and restrained Omarchy accent lighting. The opening screen uses a large art preview and an explicit nine-game selector. Keyboard selection and returning to the same game are part of that design.
- `shared/presentation` owns reusable material rendering, bevels, glass, control geometry and the cached cabinet texture. Gameplay does not depend on these rendering functions; Stack and Snake's UI-free engine features stay UI-free.
- The new cabinet image is decorative only. Boards, collision boundaries, pieces, projectiles, scores and buttons remain real native drawing and interactive widgets. Theme-aware light surfaces use a quiet engraved treatment instead of placing light-theme text over a dark bitmap.
- The approved Pinball table, Invaders sprites, Chess pieces and Solitaire deck remain authoritative. Their artwork is not replaced. A new asset and its provenance live under `shared/presentation/assets`.
- Pinball screenshot capture now waits for a rendered frame rather than capturing its loading spinner. Automated X11 rendering and input evidence remain distinct from hands-on Omarchy/Wayland acceptance.

## 2048 integration (12 September 2026)

- Accepted direction from Tom Ballard: fold avibarit/2048 into Arcade, with credit to the original author. Tom reports permission from the author. This is the integration proposal and scope record for the change.
- Preserve Avi Barit's full original Git history in the validated `games/2048/upstream-history.bundle`, alongside an unchanged source snapshot under `games/2048/upstream/`. Direct Git push is unavailable in this environment, so connector publication retains the original commits in the bundle rather than attaching their identities as PR ancestry. Port the JavaScript gameplay to Rust/egui in `games/2048/src/`, using the ordinary `ArcadeGame` lifecycle, one window, desktop identity and package. The archived Tauri wrapper is excluded from the workspace and is not built or installed.
- Credit Avi Barit as the source implementation author and Gabriele Cirulli as the original 2048 creator in the game, Help, shared About, README and installed notices. Preserve upstream MIT declarations and document the absence of a separate upstream licence file. New Arcade integration remains GPL-3.0-or-later.
- Keep classic merging, spawning, score, win/continue and the current source's 20-move undo. Preserve the latest rapid-move fix's intent: animations have no independent mutable board and completion renders authoritative state. Reduced motion is optional. No audio is added to the silent source game.
- Use a versioned `omarchy-retro-arcade/2048.json` save, private atomic writes and bounded loading. Save all moves, undo, best score and preferences; invalid/future files remain untouched and play continues with an explicit unsaved-state message. No existing game data or standalone Tauri/browser localStorage is migrated or removed.
- Append 2048 to the shelf. Existing ordering and commands remain intact. Mouse direction buttons and board dragging supplement arrows/WASD. Keep headless X11 results distinct from hands-on Omarchy/Wayland acceptance.

## FreeSki first human tuning revision (12 September 2026)

- Tyler reported low top speed and steering that looked like leaning. Rules 2
  raises the cap from 22 to 50 m/s with gradual acceleration, permits ±90° turns,
  and retains keyboard heading on release. Ski/foot orientation turns while the
  standing body stays upright. Turn rate is 1.6 rad/s; no uphill movement is added.
- The fixed view becomes 96 × 96 m with the skier at 12%, giving 84.48 m ahead.
  Course 1, collision, jumps and crash allowances remain intact. Human acceptance
  of the faster speed and shorter reaction window is still required.
- Explicit rules-1 migration retains the complete run, records and preferences,
  restores paused, and backs up the original file before the next atomic write.
  Schema/course versions remain 1; invalid/future saves retain existing recovery.

## Endless Free Ski and simulation hardening (13 September 2026)

- Tyler liked the revised movement and authorized hardening plus endless terrain,
  explicitly using GPT-5.6 Sol at medium effort in parallel. Two bounded workers
  handled terrain and simulation/storage; the parent integrated native UI, reviewed
  code, added render-timing regression coverage, and ran acceptance checks.
- Preserve rules-2 movement and the authored practice slope. Add a Free Ski mode,
  separate distance record, seeded mountains, mode choice and confirmed replacement
  of unfinished attempts. Each new mountain gets a fresh seed outside simulation;
  reopening regenerates the same terrain from saved seed and position.
- Generator 1 uses deterministic 128 m chunks with a bounded four-chunk window,
  capped density, bounded placement attempts, connected turning room and reserved
  edge recovery corridors. Reference inputs use the real engine; no autoplay or
  test-only collision exemptions enter the game.
- Schema 2 stores mode, seed, generator version and a separate Free Ski record.
  Existing schema-1/rules-1 practice data migrates with original-file backups.
  Save validation checks numeric/version bounds before any terrain work and rejects
  future generators without changing the file. Replacing an unfinished Free Ski
  run preserves its distance in the record.
- Fix released-key heading resolution across multiple ticks in a rendered frame:
  a crash reset cannot be undone by a stale frame input. Production input schedules
  resume identically through chunk boundaries and mid-jump saves.
- Endless human difficulty/variety acceptance is pending. No creature, Slalom,
  audio, new package identity, online service or other-game save changes are added.


## FreeSki system design direction (13 September 2026)

- The user requested an agent-ergonomics pass after positive feedback on the
  endless build. Scope is the FreeSki development/play/verification system inside
  Arcade. This revision changes documents and plans, not runtime behavior.
- Adopt one UI-independent full-run session path for native play, fixture
  generation and reference replay. Current orchestration is split across app,
  examples and tests; extract it without changing rules-2 physics or schema-2
  saves before pursuit. Existing engines and the Arcade host remain intact.
- Make causal cases reusable: run identity, initial state, ordered tick inputs,
  expected invariant and first divergent tick. Small cases belong with tests;
  bulky artifacts need retained revision-linked evidence. A temporary screenshot
  path and a worker success report alone do not establish durable acceptance.
- Separate document authority: PLAN for status/order, SYSTEM for ownership,
  NEXT for the pursuit contract, RULES for implemented behavior, TUNING for
  numerical choices/observations and VERIFICATION for demonstrated outcomes.
  The original issue snapshot and milestone brief remain historical references.
- Next player-facing feature is optional pursuit, with chase off by default,
  distinct records, visible warning, deterministic world movement, ordered catch
  and faithful saved continuation. Proposed recovery/fairness policies and
  acceptance cases are specified in `games/freeski/docs/NEXT.md`; numeric pursuit
  values remain uncalibrated. Slalom follows one complete course at a time.

## 2026-09-13 — FreeSki complete local modes and shared simulation

FreeSki's live app, replay and production fixture examples now use one Rust
`Session` tick. Existing rules-2 speed, steering, flight and recovery remain the
baseline. Engine outcomes expose physical movement/contact fractions so pursuit
and Slalom can share event ordering without treating recovery relocation as play.

The remaining issue #14 modes are local: opt-in creature pursuit with independent
distance records, and five authored Slalom courses with sequential unlocks,
ordered gates, five-second misses and time-based medals. Pursuit uses bounded
physical movement and the union of both actors' terrain windows; rendering does
not decide catches. Slalom uses the same skier physics and separate course times.

Schema 3 adds causal pursuit/objective state, records and mute preference while
retaining validated schema-1/2 originals before migration. Old endless runs remain
chase-off. Original creature/gate geometry and synthesized PCM cues add no new
asset download or runtime dependency; sound follows the existing optional paplay
convention and owns at most one process, stopped/reaped at lifecycle boundaries.

Full-run fixtures and a bounded JSON replay command provide repeatable cases
through production inputs. Course thresholds and pursuit evasion have reference
runs; human difficulty ratings and actual package installation remain separate
acceptance evidence. See FreeSki VERIFICATION.md for the completion pass results.
The existing Arcade window, game order, desktop identity, package and other saves
are preserved.


## 2026-09-13: FreeSki menu design

FreeSki owns a small presentation module for padded panels, text hierarchy,
primary/secondary actions, preferences and results. The start controls use an
opaque themed surface so artwork does not compete with labels. Menu actions stay
in App; simulation, storage and other games do not depend on the components. Help
uses two short pages to fit compact windows. Native menu screenshots supplement
state-transition tests at dark/light themes, compact size and 200% scale.

## 2026-09-14: Compact FreeSki controls using Arcade materials

Tyler found the 13 September menus too tall and visually separate from Arcade.
The header now combines mode selection and actions, combines stats and pursuit,
and uses one hint line. The Free Ski header is about 104 logical pixels high,
down from 269, at the tested native sizes. Course choices appear only at Ready.
Dialogs inherit the shared popup frame, square control geometry and brass borders,
with ink/ivory/brass cabinet materials and 32 px actions. This supersedes the
previous 28 px padding/44 px action design without changing game rules or saves.

## 2026-09-14: Escape steering for blocked FreeSki pursuit

A live creature stalled against a rock because all five target-facing probes
entered the collision circle. Append four wider deterministic escape probes so
it can turn away and route around contact. Keep exact swept contact, bounded turn
rate/speed, and existing save fields. The fix changes pursuit decisions only;
it neither teleports the actor nor changes the player's movement. The exact
observed geometry is retained as a bounded-motion/nonpenetration regression.

## 2026-09-14: Original FreeSki yeti artwork

Replace the horned runner with a shaggy white yeti: broad hunched shoulders, long
clawed arms, a dark face, red eyes and fangs. Its native vector geometry lives in
`games/freeski/src/yeti.rs` and scales with the snowfield. Running poses use saved
ticks and actor speed, stop when stationary, and respect reduced effects. Turns
away show the back of its head. The renderer remains read-only and anchored to
the physical actor; pursuit policy, collision shape and save schema are unchanged.
No external artwork, image files or runtime dependencies were added.

## 2026-09-14: Extend FreeSki's approved yeti art direction

Tyler approved the yeti and requested the same craft across FreeSki. The skier,
trees, rocks, ramps, Slalom flags, finish markers and shelf illustration now use
original layered geometry, dark outlines, ivory snow and restrained highlights.
Orange remains the skier's focal colour. Ski yaw and torso profile communicate
turning; braking, speed, flight and tumble change cosmetic poses.

`artwork.rs` owns these sprites. Shared ellipse/concave-polygon helpers move from
the yeti into `geometry.rs`, preserving the approved yeti design. Tree/rock variants
derive from obstacle IDs without touching world generation. Shadows stay at the
physical actor/obstacle anchors. New snow effects remain small and bounded, use
simulation ticks, and respect reduced effects. One unsaved landing-puff record
lives in App; it is cleared with other cosmetics when replacing runs. Collision
shapes, physics, terrain, save schema, compact menus and shared cabinet materials
are preserved. No external assets or runtime dependencies are introduced.

## 2026-09-14: FreeSki difficulty, fast tuck and retained record domains

Tyler's human playtest approved controls, collisions, jumping, pause, artwork and
sound but found speed and pursuit too forgiving. He approved the difficulty audit
and explicitly requested F-key fast mode. Rules 3 raises the normal cap to 60 m/s
and adds a 90 m/s fast toggle in every active mode, with gradual acceleration and
deceleration. Steering authority is unchanged. F retains its Ready-screen mode
shortcut; during play only a fresh unmodified press toggles pace. A compact mouse
button, status indicator and help text expose the same action. Fast mode is saved
causal state and replay supports the same toggle command.

Pursuit receives stronger motion and bounded persistent obstacle navigation while
retaining physical collisions, warnings and protected recovery. Generator 2 adds
edge pressure, a narrower varying clear route and more hazards with bounded
placement. Practice geometry stays approachable. Slalom geometry stays unchanged;
medal targets are recalibrated against clean normal and fast production-input runs.
A second pursuer, NPC skiers and freestyle tricks remain outside this revision.

Schema 4 retains validated older saves and original bytes. Migrated active runs
keep their generator version, position and causal state, resume paused and display
Legacy run. Their results bank into retained records until replaced. New runs use
the revised difficulty's record domain; prior scores are readable in Settings and
course unlocks survive. This avoids comparing old medal targets/terrain against
new rules or regenerating hazards beneath a suspended skier. Chase-on/off records
remain separate. No existing user state is reset for validation or screenshots.


## 2026-09-14: FreeSki close interception and honest preview

The follow-up playtest approves general movement but identifies yeti overshoots,
missing fast-mode body language and an illustrated preview that misrepresents
actual gameplay. Correct pursuit within rules 3: a distance-based corner limit,
4 rad/s turning, 72 m/s² braking and a separation-bounded interception lead replace
a minimum-speed orbit around close targets. Nearby clear paths supersede detours
around geometry beyond the target. Retain physical sweeps, catch radius, 82 m/s
cap, recovery protection, deterministic state and all saved fields/record domains.
Fast mode remains the intended straight-line escape.

The original skier geometry now folds into a clear tuck in fast mode, with narrow
skis, hands close and trailing poles; braking, flight and crash silhouettes take
precedence. This remains renderer-only and visible with reduced effects. Replace
the shelf illustration with an actual native screenshot reached through production
inputs in disposable state. The shelf crops the capture to the chase and upcoming
terrain. Preserve the approved art, compact menus and actual user saves.

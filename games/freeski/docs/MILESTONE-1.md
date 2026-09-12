# Milestone 1: the playable practice slope

Status: implemented with automated evidence. Human acceptance of skiing feel,
readability and difficulty remains pending; this milestone is not marked accepted.

The user authorized implementation on 12 September 2026. Initial choices and
reference evidence are in [TUNING.md](TUNING.md); actual behavior is in
[RULES.md](RULES.md). The agenda below now guides follow-up human playtesting.

## Outcome

Open FreeSki from the Arcade shelf and ski a short authored practice slope with
readable hazards, satisfying steering and braking, ramps, understandable crashes,
and reliable pause/save/resume. Use the production engine from the first build.

## Human playtest agenda

1. **Skiing feel:** how much momentum and turn commitment should carving have?
   How forgiving should braking and recovery feel to a first-time player?
2. **View and readability:** skier size, slope width, downhill look-ahead and how
   the practice slope teaches steering before introducing hazards.
3. **Mouse behavior:** heading-target sensitivity and dead zone, plus intuitive
   handover to keyboard without the stationary pointer taking control back.
4. **Practice sequence:** suggested progression is open snow, broad tree turns,
   a braking challenge, then a ramp/low-rock jump and recovery space. Determine
   length and what happens at the practice slope's end.
5. **Visual direction:** original skier and obstacle silhouettes; snow values,
   shadow treatment and active-theme accents inside the shared cabinet.

## Implementation checklist

- [x] Record the chosen initial feel, camera and practice flow; fill initial
  values in [TUNING.md](TUNING.md).
- [x] Register the Rust library and build serializable simulation state,
  normalized input, fixed stepping and a minimal native playfield.
- [x] Integrate shelf entry, Ready/start, pause, deliberate resume, results,
  restart and return to Arcade; clear held input through overlays/switching.
- [x] Build movement and camera on open snow, then add swept collisions and
  validated single-crash recovery, followed by ramps and height-aware landing.
- [x] Add versioned bounded save loading and private atomic writing; restore
  paused and preserve unsupported files. Save on lifecycle boundaries and at
  checkpoints. Confirm replacement of unfinished progress.
- [ ] Playtest keyboard and mouse separately, then input handover; revise the
  same slope until steering, braking and hazard visibility feel coherent.
- [x] Record focused engine/storage checks, native flows, actual app captures
  in [VERIFICATION.md](VERIFICATION.md); first human findings and the resulting retune are recorded.

## Exit checklist

- [x] Both input methods support an understandable skiing flow in Arcade.
- [x] The compact view preserves 84.48 m of look-ahead at maximum speed; human
  reaction-time validation remains pending.
- [x] Braking reliably slows the skier; movement never teleports or goes uphill.
- [x] Jumps clear only appropriate hazards; one impact causes one crash and a
  safe recovery. The final crash presents results rather than silently restarting.
- [x] Pause/focus loss stops simulation; overlays consume input; resume is deliberate.
- [x] Leaving/reopening preserves the attempt, including a jump or recovery.
- [x] A human playtest records what feels good, what remains awkward and the
  resulting tuning changes. Headless checks alone do not pass this milestone.

Tyler’s first playtest requested much higher top speed and true 90° turns. Rules 2
implements that feedback; acceptance of the revised feel remains open.

The initial playable uses one authored practice slope. Endless generation,
creature pursuit and the five-course Slalom progression follow after its movement
and visibility have been assessed.

# Milestone 1: the playable practice slope

Status: discussion brief. No movement model, numerical tuning or visual design
has been implemented or accepted yet.

## Outcome

Open FreeSki from the Arcade shelf and ski a short authored practice slope with
readable hazards, satisfying steering and braking, ramps, understandable crashes,
and reliable pause/save/resume. Use the production engine from the first build.

## Discussion agenda

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

## Implementation sequence after discussion

- [ ] Record the chosen initial feel, camera and practice flow; fill initial
  values in [TUNING.md](TUNING.md).
- [ ] Register the Rust library and build serializable simulation state,
  normalized input, fixed stepping and a minimal native playfield.
- [ ] Integrate shelf entry, Ready/start, pause, deliberate resume, results,
  restart and return to Arcade; clear held input through overlays/switching.
- [ ] Build movement and camera on open snow, then add swept collisions and
  validated single-crash recovery, followed by ramps and height-aware landing.
- [ ] Add versioned bounded save loading and private atomic writing; restore
  paused and preserve unsupported files. Save on lifecycle boundaries and at
  checkpoints. Confirm replacement of unfinished progress.
- [ ] Playtest keyboard and mouse separately, then input handover; revise the
  same slope until steering, braking and hazard visibility feel coherent.
- [ ] Record focused engine/storage checks, native flows, actual app captures
  and human playtest findings in [VERIFICATION.md](VERIFICATION.md).

## Exit checklist

- [ ] Both input methods support an understandable skiing flow in Arcade.
- [ ] Maximum-speed hazards are visible early enough to react in compact view.
- [ ] Braking reliably slows the skier; movement never teleports or goes uphill.
- [ ] Jumps clear only appropriate hazards; one impact causes one crash and a
  safe recovery. The final crash presents results rather than silently restarting.
- [ ] Pause/focus loss stops simulation; overlays consume input; resume is deliberate.
- [ ] Leaving/reopening preserves the attempt, including a jump or recovery.
- [ ] A human playtest records what feels good, what remains awkward and the
  resulting tuning changes. Headless checks alone do not pass this milestone.

The initial playable uses one authored practice slope. Endless generation,
creature pursuit and the five-course Slalom progression follow after its movement
and visibility have been assessed.

# Omarchy Circuit: illustrated table, upstream physics

Version 0.5.0 implements the approved charcoal, ivory and sage orbital-machine direction.

## Architecture

OmarchyTable builds authored DatFile records and passes them through the normal upstream loader and component constructors. TBall, TTableLayer, TWall, TFlipper/TFlipperEdge, TPlunger, TBumper, TRamp, TTripwire and TDrain handle movement and collision. The physics integrator and collision algorithms remain upstream code.

CircuitView draws the approved background plate with live mechanisms and displays. The image coordinates map to world coordinates as x = (image_x - 540) / 25 and y = (image_y - 500) / 25. Flipper rendering derives its endpoints from the current engine collision edge. Ball rendering uses the engine position, with the ramp collision layer determining foreground order. No image animation drives physics.

The left ramp consists of triangular surface planes, rising from the entrance, with its own collision mask and physical guard rails. Upstream TRamp changes the ball's surface and collision mask at the entrance/exit. A tripwire at the upper turn awards the ramp shot. It is possible to fall back down the entrance when a shot lacks speed.

The plunger's authored pullback/release interval is 100 ms, allowing consistent contact at this table's rest position. This adjusts component configuration, not the upstream collision solver.

## Rules

Three balls, one player. Bumper hits score 100, with 2,500 for twelve hits. Each of eight targets scores 250; clearing both banks awards 5,000. Orbit shots score 1,000, upper-ramp shots 1,500, slingshots 25. Lamps and sidebar counters reflect these events. Repeated sensor contacts are debounced. New game resets all objectives.

## Artwork and themes

The packaged PNG is a cleaned production plate derived from the user-approved generated concept. It contains static rails, plastics, bumper caps and orbital illustration. Moving flippers and balls, circuit and target lamps, plunger indicator and dot-matrix displays are rendered separately. Material highlights and some decorative lamps are baked into the illustration; this is a fixed-camera 2D renderer, not a full 3D scene.

Green glass and accents follow Omarchy while ivory, chrome and amber retain their material colours. The exact official wordmark is composited at the centre after recolouring and remains unchanged. The artwork provenance is in assets/circuit/README.md.

## Persistence and modes

Circuit settings and high scores remain in the existing per-user Circuit directory. There is no mid-game save/resume. Classic mode retains original DAT loading and mission controls. The earlier standalone prototype remains explicit --experimental, with its saved games preserved. No original Windows resources are included.

## Table/artwork alignment

The authored layout in `CircuitLayout.h` measures the unchanged 1536 × 1024
plate. The DAT builder uses closed footprints for the slingshots, target bodies,
return guides and apron rails, plus circular post footprints. Kicking target faces
point toward the playfield; passive backs prevent entry through the other sides.
The launcher sits at the pictured launch head. Its raised hood is a foreground
piece, leaving an open route beneath it.

The ramp's deck and collision rails share one centreline. Rounded rail joins close
ball-radius gaps at bends. Low entry guards are ground-level obstacles; higher
sections allow the ground ball underneath. CircuitView copies the original plate
pixels back over ground balls beneath the ramp and hood, and over balls behind
bumper caps and posts. Balls on the ramp remain above its deck but behind its side
rails. This does not modify the official artwork or the upstream physics engine.

The ball uses the same image-plane mapping as the measured ramp, without an extra
vertical offset that would move it off the illustrated deck. Bumper lamps and the
launcher rendering also consume shared layout measurements.

`authored-table-boundaries` exercises a post, both slingshot backs, the left guide
and a scoring module face using real upstream balls. `authored-table-depth`
compares software-rendered frames with and without a ball: a ground ball is fully
hidden under the ramp, while a ball on the ramp is visible. The full-game, ramp,
target, orbit and drain tests also pass. These checks validate the authored routes;
they do not substitute for subjective desktop playtesting of every shot.

## Verification

The actual executable runs 180 simulated seconds through a complete three-ball game, with bounds and finite-state assertions. Separate physics shots exercise the ramp, target, orbit and drain. Screenshot checks cover standard and compact windows and an alternate palette. CI repeats the collision tests after Arch package installation and checks the earlier prototype's save across upgrade.

Interactive Hyprland acceptance, listening to audio and subjective play tuning still require a real desktop session. The richer artwork does not imply Space Cadet layout or mission fidelity.

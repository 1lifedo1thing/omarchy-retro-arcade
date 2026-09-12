# FreeSki tuning register

Initial implementation values for practice rules/course v1. These have automated
reference evidence; they have not yet been calibrated by a human playtest.

| Parameter | Initial value | Reason / evidence |
| --- | --- | --- |
| Simulation tick rate | 60 ticks/s | Production engine; 30/60/120 Hz rendering equivalence check |
| World units / footprint | Metres; skier radius 0.65 m | Shared collision and rendering coordinates |
| Slope / view | 80 m wide; view 96 × 72 m | Reserved clear edge corridors; resize-independent visibility |
| Maximum speed | 22 m/s (79.2 km/h) | Fast initial candidate; human reaction-time calibration pending |
| Acceleration | 6.5 × cos(heading) m/s² | Automatic downhill acceleration |
| Drag / carving | 0.10 × speed + 4.5 × abs(sin(heading)) m/s² | Broad turns reduce speed |
| Braking | Additional 18 m/s² deceleration | Reliably reaches rest in engine tests |
| Turn rate / heading limit | 2.4 rad/s; ±1.35 rad | Bounded turns; no uphill direction |
| Mouse dead zone / sensitivity | 1.8% / 32% of field width | Dead zone around skier; normalized heading target |
| Camera | Skier at 24% of view height | 54.72 m / 2.49 s ahead at maximum speed |
| Jump | 1.2 s; 2.5 m apex; 40% steering | Parabolic arc; reference run clears all three ramp/rock pairs |
| Tree / rock / ramp radii | 1.4 / 1.2 / 1.8 m | Plus skier radius for swept contact |
| Tree / rock collision heights | 8 / 0.65 m | Trees dangerous in air; low rocks clearable |
| Crash allowances | 3 | Initial issue proposal retained |
| Tumble / reset speed | 42 ticks / 0 m/s | Stops motion for readable recovery |
| Recovery clearance | Hazard radius + skier radius + 2 m | Every authored hazard checked for clear recovery |
| Protection | 90 ticks after tumble | 1.5 simulation seconds; pause cannot spend/extend running time |
| Checkpoints / backlog | 300 ticks / pause above 250 ms | Five-second simulation checkpoints; no collision time skipped |
| Practice length | 1,200 m | Reference: 3,397 ticks / 56.62 s, three jumps, zero crashes |
| Cosmetic trails | At most 240 segments, one every 3 moving ticks | Bounded memory; reduced-effects toggle removes tracks |
| Endless / creature / Slalom | Not implemented | Tune in milestones 3–4 |

## Initial evidence and observations

The waypoint reference uses only heading targets and ordinary production ticks.
It completes all six practice sections without crashes and uses all three ramps.
The reference is a reachability check, not a claim that the game feels good.

A concurrent debug software-renderer run at 200% scale triggered the documented
backlog pause while other renders and compilation were active. An isolated release
run passed. Keep this behavior visible; do not hide slow frames by skipping ticks.

## Human tuning session format

Record date, commit, parameter old/new values and units, course/seed, input method,
display conditions, observed problem, evidence and next hypothesis.
No human tuning sessions have been recorded yet.

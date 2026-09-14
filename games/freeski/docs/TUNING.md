# FreeSki tuning register

Initial implementation values for practice rules/course v1. Retained as the initial baseline; the first human feedback and rules-2 revision
are recorded below.

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
### 2026-09-12: speed and steering (rules 2, course 1)

Tyler tested the initial native build (implementation `acf6286`, playtest checkout
`cef046c`) and reported that top speed was much too low. He wanted gradual buildup
to a much higher speed, and A/D to turn the skier through 90° rather than make him
lean. Keyboard input is confirmed by the feedback; exact display size/rate and
elapsed play time were not reported.

| Parameter | Old → new | Reason |
| --- | --- | --- |
| Speed cap | 22 → 50 m/s | 2.27× maximum speed; straight buildup about 9.7 s |
| Linear drag | 0.10 → 0.05 /s | Same initial acceleration, sustained buildup |
| Turn rate / limit | 2.4 → 1.6 rad/s; ±1.35 → ±π/2 rad | Deliberate quarter-turn in about 0.98 s |
| Keyboard release | Return downhill → retain heading | Turning sets direction; opposite key turns back |
| View / skier anchor | 96 × 72 → 96 × 96 m; 24% → 12% | 84.48 m ahead, 1.69 s at new maximum speed |
| Skier geometry | Whole-body rotation → ski yaw and upright body | Sideways traverse reads as a turn; goggles show profile |

Acceleration 6.5 m/s², carving friction, brakes, obstacles, jump arc and crash
rules are unchanged. Production reference: 1,714 ticks / 28.57 s, three jumps,
zero crashes. Rules-1 save migration preserves the attempt and records with a
retained original. This revision still needs Tyler's feel/readability acceptance;
1.69 s is measured visibility, not proof of comfortable human reaction time.

### 2026-09-13: endless terrain and persistence hardening

Tyler reported the revised movement was feeling good and authorized hardening
and endless Free Ski. Speed, acceleration, turning, brake force, collision sizes,
jump arc and recovery timing remain the rules-2 values above.

| Parameter | Value | Evidence / purpose |
| --- | --- | --- |
| Chunk length | 128 m | Whole chunks cached; no per-frame random placement |
| Active window | Previous, current and next two chunks | At most four chunks / 72 obstacles; over 256 m ahead at a boundary |
| Opening | First 92 m clear | Acceleration and time to choose direction |
| Reserved route | 16 m wide, centres within ±12 m | Smooth connected bends; optional ramp/rock pairs within route |
| Recovery corridors | x = ±36 m | Hazard clearance checked through 80 chunks for 128 seeds |
| Density | 8 slots + min(chunk / 3, 8), or 4 in sparse chunks | Caps at chunk 24 / 3,072 m; failed placements reduce density |
| Sparse stretches / jumps | Hash-selected, nominal 1/6 and 1/3 chunks | Seeded variation; opening has no ramp |
| Placement retries | At most 12 per slot | Rejected slot is omitted; no unbounded generation loop |
| Optional jump rock | 16 m after ramp | Corpus uses production jumps; ordinary skiing can steer around |
| IDs and RNG | Stable chunk/slot IDs; pure u64 hash streams | Regeneration independent of visit order and save timing |
| Records | Separate practice and Free Ski records | Replacing an unfinished Free Ski run banks its reached distance |
| Safety limits | Finite y ≤ 2 billion m; ticks < one year | Corrupt-state guards, beyond feasible normal play |

Reference corpus: seeds 0–127, each starting from rest and reaching 5 km with
ordinary production steering, zero crashes and over 1,000 jumps combined. This
checks reachability at capped density, not player comfort or long-term variety.
The public reference steering helper is for evidence only; gameplay never calls it.

A frontend crash regression proved released heading must be resolved per physics
tick, because a crash resets heading between ticks in a single rendered frame.
The same post-crash state now results at 30/60/120 Hz. Endless runs across chunk
boundaries also match across these render schedules and alternating window sizes.

Next human checks: terrain variety after several minutes, readable safe lines at
full speed, braking before ramps, and whether edge corridors make runs too easy.
Creature pursuit and Slalom remain separate future tuning sessions.

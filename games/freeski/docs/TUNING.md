# FreeSki tuning register

Status: no values have been implemented or playtested. Issue defaults below are
proposals, not calibrated targets. Add units, implementation revision, reason and
playtest evidence whenever a value is chosen or changed.

| Parameter | Initial candidate | Status / evidence needed |
| --- | --- | --- |
| Simulation tick rate | TBD ticks/s | Decide with movement and collision model |
| World distance units and skier footprint | TBD | Consistent physics, display and collision geometry |
| Downhill acceleration and maximum speed | TBD | Open-snow and compact-view reaction playtests |
| Turn rate and maximum heading | TBD | Broad turns, braking and route reachability |
| Carving friction and braking strength | TBD | Reliable novice slowdown without uphill movement |
| Mouse sensitivity and dead zone | TBD | Mouse-only play and input handover |
| Camera look-ahead and logical view bounds | TBD | Visibility at maximum speed across window sizes |
| Jump duration, height and airborne steering | TBD | Ramp feel and explicit low-rock clearance |
| Obstacle collision heights | TBD | Rocks clearable; trees remain dangerous |
| Crash allowances | 3 | Issue proposal; confirm through practice runs |
| Tumble duration, reset speed and recovery clearance | TBD | Recovery cannot immediately cause another crash |
| Collision protection | 1.5 simulation seconds | Issue proposal; visible protection and pause tests |
| Checkpoint cadence and backlog pause threshold | TBD | Bounded work, save overhead and no skipped collision time |
| Chunk dimensions, buffer size and generation retries | TBD | Route validation across seams; bounded resources |
| Difficulty ramp and density cap | TBD | Traversable, readable runs at every difficulty |
| Creature warning, speed, turn rate and catch radius | TBD | Meaningful evasion and swept catch detection |
| Creature appearance distance | 1,000 m | Issue proposal; warning and human chase playtests |
| Missed-gate penalty | 5 s | Issue proposal; course reference runs and playtests |
| Slalom lengths and medal thresholds | TBD per course | Production-engine completion plus human runs |
| Particle/trail budgets and lifetime | TBD | Hazards stay readable; long-run memory stays bounded |

## Change record format

For each tuning session record: date, commit, parameter old/new values and units,
course/seed, input method, display conditions, observed problem, repeatable test
or playtest evidence, and the next hypothesis. Record findings rather than
asserting that a change is fun or fair without a playtest.

No tuning sessions recorded yet.

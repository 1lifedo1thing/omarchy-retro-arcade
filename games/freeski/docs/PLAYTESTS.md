# Human playtest log

Human reports belong here. Automated checks remain in [VERIFICATION.md](VERIFICATION.md).
A feature reported working is not acceptance of every mode or the entire release.
For later entries, record date, build when known, mode/seed/course, input, window
conditions, observed outcome and unresolved questions. Leave unknowns explicit.

## 2026-09-14 — Tyler, first recorded acceptance session

Practice and a separate Free Ski run were exercised. Tyler clarified that the
yeti observations came from Free Ski. The latest prepared build was fc9a45a;
the exact running revision was not independently confirmed for this report.
Seed, duration, window size and scaling were not recorded. Successful completion
of the full Practice course was not explicitly reported.

| Area | Human result | Detail |
| --- | --- | --- |
| Keyboard and mouse steering | Reported working well | Controls feel great with both input methods. Preserve this feel. |
| Obstacles and collisions | Reported working well | Obstacles and crashes behave and look good. |
| Jumping | Reported working well | Jump works and looks good. |
| Crash allowance | Reported working | Third crash ends the run. |
| Pause and focus loss | Reported working well | Manual pause and automatic pause on window focus changes work well. |
| Artwork and sound | Approved in exercised play | Positive feedback on the updated presentation; not a full theme/scale audit. |
| Free Ski warning, chase and catch | Reported working well | Yeti warning appears, pursuit occurs and catching works very well. |
| Difficulty and speed | Needs revision | Game feels much too easy and insufficiently fast; escaping the yeti feels too forgiving compared with remembered SkiFree. |
| Yeti navigation | Unconfirmed concern | May get stuck too often. Tyler was unsure; no seed, replay or specific stall was captured. |

Do not reinterpret the navigation concern as a confirmed recurrence of the rock
deadlock fixed earlier. That defect and its bounded regression evidence remain
documented in [TUNING.md](TUNING.md).

## Remaining human acceptance

| Test | Status / expected observation |
| --- | --- |
| Difficulty revision | Open: repeat Free Ski after agreed tuning; challenging pursuit with understandable, earned escapes. |
| Practice finish | Not explicitly confirmed: reach the authored finish and check its result/restart flow. |
| Slalom Cup | Untested: all five courses, gate order, misses and penalties, medals, unlocks and difficulty progression. |
| Longer Free Ski runs | Partial: assess terrain variety, edge escape routes and repeated pursuit encounters across seeds. |
| Saved continuation | Untested by human: pause, return to Arcade, reopen and resume a chase/race/jump without losing state. |
| Records and settings | Untested by human: close/reopen, retained records and settings, separate pursuit-on/off records. |
| Menus and display variants | Partial presentation approval: still check compact window, scaling, light/dark themes and complete mouse-only navigation. |
| Mute and reduced effects | Untested by human: toggles remain effective through pause, switching and reopening. |
| Packaged Omarchy release | Pending: install/upgrade the final package, native switching and existing-save retention. |

The [difficulty audit](DIFFICULTY-AUDIT.md) recommends the next iteration. Research
and proposals do not count as completed playtesting or implemented changes.

## 2026-09-14 — difficulty changes approved, retest pending

Tyler approved implementing the proposed pursuit, speed and terrain changes and
explicitly requested F-key fast mode. This authorizes the implementation; it is
not a playtest of the resulting build. Preserve the first session's observations
above. Next human pass should start a new mountain and compare normal/fast chase
pressure, then cover Slalom and saved continuation. Record the tested revision.

# Next: play the completed modes

Pursuit and the five-course Slalom Cup are implemented. The previous extraction
and pursuit brief was completed in the whole-game pass. See PLAN.md for scope,
SYSTEM.md for ownership and VERIFICATION.md for actual checks.

The next useful iteration is human difficulty and readability feedback:

1. Start Free Ski with Creature pursuit enabled. Ski past 1,000 m, read the warning,
   then try committed turns around terrain. Observe whether the creature's approach
   and catch are understandable, and whether evasion feels useful.
2. Complete Pinecone Path with keyboard steering, then with mouse steering. Check
   that the next gate, five-second misses, finish, medal and next-course unlock are
   clear. Continue through Summit Cup to assess the difficulty progression.
3. Pause during a chase or race, return to Arcade, and reopen. Check audio, input
   ownership and saved continuation on the actual Omarchy desktop.
4. Compare ordinary and reduced effects, mute, compact size and desktop scaling.
   The current view exposes 84.48 m downhill; report any hazard revealed too late.

Record date, source revision, seed/course, input method, screen conditions,
observed problem and a specific tuning hypothesis. Reference runs are automated
feasibility evidence and must not be relabelled as human playtests. Preserve the
liked 50 m/s speed and 1.6 rad/s turning while adjusting only the cause supported
by new feedback.

For a reproducible defect, capture the smallest production-input replay and add a
regression in the owning module. Use Session for whole-run evidence; rendering or
fixture generation must not move actors, grant medals or exempt collisions.
Remaining release/environment gaps, if any, belong in VERIFICATION.md.

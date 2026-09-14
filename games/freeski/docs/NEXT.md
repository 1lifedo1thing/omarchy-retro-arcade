# Next: difficulty tuning and remaining human acceptance

Pursuit and the five-course Slalom Cup are implemented. The previous extraction
and pursuit brief was completed in the whole-game pass. See PLAN.md for scope,
SYSTEM.md for ownership and VERIFICATION.md for actual checks.

Tyler's first recorded [human playtest](PLAYTESTS.md) approves controls, jumping,
collisions, pause, art and sound. Practice and a separate Free Ski chase were
exercised. Warning and catch work, but speed and difficulty need revision; possible
yeti sticking is unconfirmed. The [difficulty audit](DIFFICULTY-AUDIT.md) proposes
reliable pursuit followed by coupled speed/pursuit comparisons. Those proposals
are not implemented or accepted tuning values.

The remaining human checks include:

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
feasibility evidence and must not be relabelled as human playtests. The September
14 feedback reopens the previously accepted 50 m/s speed baseline. Preserve the
liked steering response (currently 1.6 rad/s); adjust difficulty using evidence.

For a reproducible defect, capture the smallest production-input replay and add a
regression in the owning module. Use Session for whole-run evidence; rendering or
fixture generation must not move actors, grant medals or exempt collisions.
Remaining release/environment gaps, if any, belong in VERIFICATION.md.

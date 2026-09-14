# Next: human acceptance of the difficulty revision

Pursuit and the five-course Slalom Cup are implemented. The previous extraction
and pursuit brief was completed in the whole-game pass. See PLAN.md for scope,
SYSTEM.md for ownership and VERIFICATION.md for actual checks.

Tyler's first recorded [human playtest](PLAYTESTS.md) approves controls, jumping,
collisions, pause, art and sound. Practice and a separate Free Ski chase were
exercised. Warning and catch worked, but speed and difficulty needed revision;
possible yeti sticking was unconfirmed. The approved [difficulty audit](DIFFICULTY-AUDIT.md)
now informs rules 3: 60 m/s normal speed, 90 m/s fast tuck, stronger pursuit and
new terrain. Latest technical evidence belongs in VERIFICATION.md; human acceptance
of the revision remains open.

The remaining human checks include:

1. Start a new Free Ski mountain with Creature pursuit enabled (a restored Legacy
   run retains old terrain). Compare ordinary skiing with F / the Fast button.
   Ski past 1,000 m and test chase pressure, obstacle detours and difficult but
   earned escapes. Record any unprotected stall with its seed and position.
2. Complete Pinecone Path with keyboard steering, then with mouse steering. Check
   that the next gate, five-second misses, finish, medal and next-course unlock are
   clear. Continue through Summit Cup to assess the difficulty progression.
3. Pause during a chase or race, return to Arcade, and reopen. Check audio, input
   ownership and saved continuation on the actual Omarchy desktop.
4. Compare ordinary and reduced effects, mute, compact size and desktop scaling.
   The view exposes 84.48 m downhill, just 0.94 s at fast cap; report hazards
   revealed too late, especially during jumps and turns.

Record date, source revision, seed/course, input method, screen conditions,
observed problem and a specific tuning hypothesis. Reference runs are automated
feasibility evidence and must not be relabelled as human playtests. The September
14 feedback reopens the previously accepted 50 m/s speed baseline. Preserve the
liked steering response (currently 1.6 rad/s); adjust difficulty using evidence.

For a reproducible defect, capture the smallest production-input replay and add a
regression in the owning module. Use Session for whole-run evidence; rendering or
fixture generation must not move actors, grant medals or exempt collisions.
Remaining release/environment gaps, if any, belong in VERIFICATION.md.

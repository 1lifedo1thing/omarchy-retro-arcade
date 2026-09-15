# Next: playtest fixes and release acceptance

Tyler's September 14 follow-up approves the revised general speed/game feel,
Slalom in exercised play (including five-second misses), the Practice finish,
Free Ski save/resume, a longer pursuit-off run and settings/display checks.
[PLAYTESTS.md](PLAYTESTS.md) records the exact scope and unknowns. Preserve the
approved controls, movement, artwork and sound.

## Playtest fixes implemented

- Fast mode visibly tucks the skier; braking/jumps/crashes retain their own poses,
  and reduced effects keeps the tuck visible.
- Pursuit now brakes and turns for close interception, leads moving targets and
  avoids needless detours beyond the skier. Physical catch distance and crash
  protection are preserved. Reproduction, parameters and corpus are in TUNING.md.
- The Arcade selection image is a real native gameplay screenshot. Its source
  and capture method are in assets/README.md; actual user saves were preserved.

These changes require the focused human retest below; recording them does not
claim Tyler has already accepted the corrected build.

## Remaining acceptance

- Confirm whether Tyler's Cup play covered all five courses and their unlocks/
  medals; the report praises the Cup but does not enumerate course completion.
- After the fixes, do a focused normal/fast chase retest, including close passes,
  obstacle approaches, post-crash recovery and saved continuation of pursuit.
  Check tuck readability and the replacement shelf preview at the same time.
- Build and verify the final package, installation/upgrade, switching to other
  games and preservation of existing saves. Prior staging/extracted-package
  evidence does not claim system package installation of the final revision.

No broad repeat of already approved playtesting is needed unless a fix regresses
it. Record seed, revision and a small production-input replay for pursuit defects.
Rules/tuning and verification must reflect actual changes and checks; automated
reference routes remain separate from human difficulty acceptance.

# Next: playtest fixes and release acceptance

Tyler's September 14 follow-up approves the revised general speed/game feel,
Slalom in exercised play (including five-second misses), the Practice finish,
Free Ski save/resume, a longer pursuit-off run and settings/display checks.
[PLAYTESTS.md](PLAYTESTS.md) records the exact scope and unknowns. Preserve the
approved controls, movement, artwork and sound.

## Three requested fixes

1. Give the skier a distinct tuck pose during fast mode. A readable pose change
   is sufficient; animation is optional. Keep this visual, independent of physics.
2. Diagnose and fix missed yeti interceptions. Tyler observed repeated passing,
   circling and falling behind, plus stopping beside a crashed skier before an
   eventual catch. Reproduce with production inputs; distinguish intentional
   crash protection from steering, collision and catch defects. Add a regression
   for the reproduced cause, preserve physical navigation and avoid teleporting
   or awarding catches solely because the creature appears on screen.
3. Replace the FreeSki Arcade selection image with an interesting actual gameplay
   screenshot. Preserve the approved art and shelf theme. Do not reset Tyler's
   real save to prepare a capture.

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

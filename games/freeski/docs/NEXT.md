# Next: PR integration and release verification

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

Tyler accepted the completed game after delivery of these fixes; the final
sign-off is recorded in PLAYTESTS.md. The approval does not add unreported
per-course medal or seed-specific chase measurements.

## Remaining engineering and release work

- Integrate the feature branch with current main. The PR preparation fetch found
  that main had advanced, with conflicts in shared Arcade registration, shelf,
  manifests, CI, installation/native scripts and documentation. Preserve both
  FreeSki and the intervening upstream games/lifecycle changes; rerun combined
  checks against the integrated revision.
- Build and verify the final package, installation/upgrade, switching to other
  games and preservation of existing saves. Prior staging/extracted-package
  evidence does not claim system package installation of the final revision.
- Complete PR review and CI before merge and issue closure. The historical
  verification results apply to their recorded revisions.

No broad repeat of approved playtesting is required unless integration changes
or a discovered regression justify it. Keep automated evidence and human reports
separate; never turn general sign-off into invented detailed test observations.

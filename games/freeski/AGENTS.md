# FreeSki project instructions

- Read README.md, docs/PLAN.md and docs/REQUIREMENTS.md before implementation.
  Check docs/MILESTONE-1.md for the current discussion brief. Project setup does
  not settle its open design choices; subsequent user direction takes precedence.
- Build a Rust library hosted by Arcade. Append the shelf entry when playable;
  preserve existing order, games, saves, artwork and package identity.
- Keep deterministic simulation independent of egui, audio, wall-clock time and
  filesystem access. Rendering and resize must not change world state or rules.
- Use one production simulation for gameplay, replay, course completion evidence
  and generator validation. Tests or reference runs must not bypass movement rules.
- Implement foundational simulation and save safety during milestone 1; milestone
  2 hardens them. Do not defer those foundations until after content production.
- Keep numerical assumptions and playtest revisions in docs/TUNING.md. Record
  material integration choices in ../../DECISIONS.md and evidence in
  docs/VERIFICATION.md. Do not mark planned or headless work as desktop acceptance.
- Keep original asset provenance in assets/README.md, including code-generated
  geometry and sound. Preserve all existing approved Arcade artwork.
- Use focused commits on a feature branch. Complete the applicable checks in
  ../../CONTRIBUTING.md. Add FreeSki to existing native and package verification
  when implementation makes those checks meaningful.

# Working on Omarchy Invaders

- Rust is the default. Preserve user data and original artwork provenance.
- Keep work reviewable in a PR. Do not merge or publish a stable release without Tom's instruction.
- Record judgement calls in DECISIONS.md.
- Arcade contract: directly playable launch, Game/Settings/Help, Ctrl+N/M/Q, Ctrl+, and F1; sound off initially; Omarchy colours; local atomic saves; matching icon/About identity; package updates preserve state.
- Game simulation must be independent of rendering, with bounded fixed steps and no networking.
- Run fmt, clippy with warnings denied, tests and a native-window check after material input changes. Report actual evidence and desktop limitations. Do not equate successful compilation with polished gameplay.

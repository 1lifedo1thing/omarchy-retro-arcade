# Add Bubble to the single Omarchy Arcade app

Adds an original bubble-shooting puzzler as the sixth game in the existing Arcade shelf and package. It reuses the shared native window, navigation, theme loader and lifecycle, preserving the imported games and their saves.

Includes 20 authored levels, scored matching and disconnected falls, deterministic wall/circle collision and hex attachment, a limited bounce guide, visible ceiling pressure, saved progress/bests, accessible colour symbols, pause/focus handling and original artwork/audio. Levels and verified completion routes live in JSON.

Verification: workspace Rust tests, focused Bubble rule/lifecycle/save/audio tests, release build, native mouse/keyboard tests at standard and compact/200% sizes, actual full-game captures, rebuilt Pinball with three CTests, and shared package install staging.

Pending acceptance: real Arch package CI, human Omarchy/Wayland playtesting and audible sound/balance review. Xvfb automation and solver routes are not manual playtesting.

This is stacked on the local consolidation at b1dec03. See `games/bubble/docs/HANDOFF.md` for the exact scope, evidence and publishing blocker.

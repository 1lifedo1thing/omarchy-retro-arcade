# Omarchy Circuit: authored data, upstream engine

Version 0.4.0 implements the user's chosen route: a self-contained Omarchy table using the original engine's physics and components.

## Architecture

OmarchyTable::Build constructs a DatFile object with authored groups, physical parameters, bitmaps and z-maps. pb::init passes it through the same loader and TPinballTable constructors as loaded resources. Classic mode still uses partman::load_records.

The table instantiates TBall, TTableLayer, TWall, TFlipper/TFlipperEdge, TPlunger, TBumper, TDrain and TTextBox. The normal pb::frame loop, spatial edge grid and collision resolution run unchanged.

control.cpp routes authored-table events to OmarchyTable's small scoring controller. Classic mode retains the original mission controller. This is not a claim to Space Cadet's geometry, tuning or missions.

## Authored data

- Flat projected playfield, side rails, angled returns and shooter lane.
- Two moving flippers with nine rendered states matching their authored sweep.
- Three circular bumpers with a kick response and lit animation.
- Charged upstream plunger, drain and three-ball session.
- 100 points per bumper; 1,000-point bonus after ten hits.
- Procedural indexed artwork, z-maps and digits; regular UI font for messages.
- Original synthesized bumper sound; no background music.
- Official mark composited outside the palette transform to preserve its exact colours.

All table dimensions and physical parameters in OmarchyTable.cpp are newly authored. No original DAT or Windows game artwork/audio is embedded. The former embedded resource-font finalization step is deliberately not used for authored data.

The generic GroupData builder now detects unsorted insertions and sorts stably, preserving multiple same-type attribute records. Original on-disk groups were already ordered.

## Isolation

Circuit stores settings/high scores in a separate directory. Original mode and experimental prototype remain explicit launch options. Prototype saves cannot be converted to the upstream session format. Circuit does not yet have mid-game save/resume.

Demo, multiplayer and original mission cheats are disabled for Circuit because its controller does not implement them.

## Verification

tests/upstream_table_test.py launches the actual engine in an isolated directory, simulates 180 seconds, checks finite ball state, requires scoring and drains, and verifies no external DAT was introduced. Its deterministic scripted controls exercise real upstream component collisions; this is not a substitute physics model.

Manual rendering inspection checks sprite orientation, transparent depth masks, flipper states and exact-logo compositing. Hands-on play feel and real Omarchy/Hyprland acceptance remain.

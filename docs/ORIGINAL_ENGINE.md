# Original engine restoration

The original engine is again the default as of 0.3.0. The custom table is available only through --experimental. This supersedes the earlier decision to replace default gameplay.

## What is reused

The default uses upstream ball integration, collision handling, moving flippers, component behaviour, scoring and mission control. The Omarchy framebuffer colour transform and interface do not replace these systems.

## Required resource boundary

The repository's partman.cpp reads a PARTOUT(4.0)RESOURCE DAT container. It contains more than images:

| Resource | Role | Rebranding constraint |
| --- | --- | --- |
| Group names, IDs and attributes | Components and their references | Preserve identity and ordering expected by control.cpp |
| Short and float fields | Table/component parameters, materials and kickers | Preserve physical values and relationships |
| Indexed bitmaps and palette | Table artwork and component animation | Replace visual content while preserving dimensions, offsets, transparency and animation states |
| Z maps | Depth/occlusion during sprite rendering | Preserve matching depths and geometry |
| Sound references and external WAV/music | Audio cues and playback | Preserve cue mapping; replacement audio can be independently authored |

Relevant code: partman.cpp decodes records; loader.cpp queries materials, kickers, visual states and sound references; render.cpp exposes the Sprite Viewer. Data selection supports PINBALL.DAT, CADET.DAT and DEMO.DAT, including lowercase variants.

## Next data-dependent work

1. Obtain the complete original resource folder from the user. No files are present in the workspace. Earlier resource retrieval was blocked, so no alternate download route is used.
2. Run the restored engine with those resources. Verify launch, flippers, ramps, collisions, scoring/missions, sound, menu pause and restart.
3. Use Help / Sprite Viewer to inventory exact bitmap groups, dimensions, offsets and visual states for that resource version.
4. Author Omarchy artwork for those exact visual slots. Keep physics attributes and z-map relationships unchanged; prefer a separate override layer over modifying the source DAT.
5. Compare the original-colour baseline and themed rendering with the same gameplay setup. Verify every animation state and occlusion boundary before distributing an artwork pack.

The current deliverable restores the engine and provides live palette colouring. It does not claim a finished replacement artwork pack or eliminate the original data requirement. Replacing all visual assets alone would not replace the DAT's table definitions.

## State and verification

The source port has local settings/high scores but does not provide the experimental model's in-progress save format. Existing experimental files are preserved separately. No fake migration is attempted.

Automated builds and launcher tests are useful evidence but cannot establish working original gameplay without its data. Real original-engine gameplay and a complete artwork inventory remain blocked on that input.

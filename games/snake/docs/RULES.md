# Snake rules: snake-v1

These constants and algorithms are the replay contract. Any gameplay change gets a new rules ID and separate community boards. An unrelated app release does not.

| Rule | Value |
|---|---|
| Board | 24 columns × 20 rows; bounded; no obstacles or wrapping |
| Coordinates | Zero based, x right, y down; cell = y × 24 + x |
| Initial body | Head first: (13,10), (12,10), (11,10), (10,10) |
| Initial direction | Right; wait for explicit Start |
| Food | One cell; growth by one and +10 points |
| Slow | 6 moves/second; one move every 10 ticks |
| Normal (default) | 10 moves/second; one move every 6 ticks |
| Fast | 15 moves/second; one move every 4 ticks |
| Simulation | Integer 60 Hz; speed constant throughout each run |
| Win | All 480 cells occupied; 4,760 points; no further random draw |
| Loss | First wall or body collision; no post-result simulation |

The starting body is horizontally centred between columns 10 and 13. Row 10 is the lower of the two central rows. On each tick increment the tick and movement phase; when phase reaches the speed's period, reset phase to zero, consume at most one buffered turn, check the destination and move. A collision keeps the previous body and food. A non-growing step may enter the current tail cell because that tail leaves during the same step. Growth retains the tail.

Food uses SplitMix64 with its complete u64 state stored in the run. Add `0x9e3779b97f4a7c15`, xor/shift 30 and multiply `0xbf58476d1ce4e5b9`, xor/shift 27 and multiply `0x94d049bb133111eb`, then xor/shift 31; arithmetic wraps. Enumerate empty cells in ascending cell-number order. With n empty cells, reject random values smaller than `(-n wrapping u64) % n`, then select `random % n`. This avoids modulo bias. Seed zero is valid. The first food is selected using the same algorithm.

## Controls

- Arrow keys or WASD turn. The settings panel can rebind primary direction keys; WASD remains available. Duplicate primary bindings and reserved action keys are not accepted by the editor.
- Two-turn FIFO buffer. Validate against its last direction, otherwise the current direction. Reject repeats, duplicate directions, reversals and overflow. Right → Up → Left can be buffered across two steps.
- Enter starts, continues or plays again. Escape pauses or cancels a resume countdown. R requests restart; an active run requires confirmation. N opens New Game from pause/results. 1/2/3 choose speed before starting.
- The onscreen Pause, Restart, Settings and Return to Arcade actions remain available. Ctrl+H returns through the shared shell; Ctrl+Q closes Arcade.

Pausing clears queued turns and stops sound. Focus loss pauses immediately; focus return never resumes. Continue starts a cancellable three-second wall-clock countdown. Pauses and countdowns advance neither simulation tick nor movement phase. Input during the countdown is ignored, so fresh direction input is required after it. Rendering uses no interpolation and never modifies occupancy. The wall-clock dispatcher uses integer nanoseconds × 60 with a remainder, independent of frame partitioning; a paused dispatcher discards its sub-tick wall remainder, preserving the logical movement phase.

## Saves and records

`$XDG_STATE_HOME/omarchy-snake` (fallback `~/.local/state/omarchy-snake`) owns only Snake data. `records.json`, schema 1, contains independent Slow/Normal/Fast best scores, selected speed, direction bindings, optional audio and reduced-motion preference. Sound defaults off. No shared binding or audio preference service exists in the inspected Arcade base, so these preferences follow the existing per-game convention.

`session.json`, schema 1, stores one incomplete run, including rules, body, direction, buffer, food, random state, score, speed, tick and movement phase. Save on shelf return and app close, on pause and periodically during play. A completed run removes the resumable snapshot while retaining records. Writes use Arcade's existing temporary-file, fsync and atomic-rename helper.

Restore the exact snapshot into a paused gate with Continue/New Game choices. Preserve the saved buffer in the dormant snapshot; Continue clears it as for every resume. All restored runs are local-only, even if a shared ranking service is added later. Invalid or unsupported snapshots are archived with a unique name and a visible notice; run failure does not reset the separate records file or another game's data. Unreadable records can likewise be recovered without discarding a valid run.

## Replay event ordering

`replay::Replay` contains rules, speed, initial seed, end_tick and events. Event ticks denote the number of simulation ticks already completed: apply the event before advancing from that tick. Equal-tick events retain array order. Record accepted turns only; repeated/rejected key events have no gameplay effect. Pause clears the queue; Resume is recorded only after countdown completion, at the same simulation tick. Cancelled countdowns add no Resume. The adapter rejects time advancing between Pause and Resume.

Only a completed collision or full-board win is a valid result. The validator runs this exact engine and returns its calculated score, outcome, elapsed active ticks and board ID. There is no client-score field. UI traces are bounded and in-memory while the shared integration is unavailable; they are diagnostic local traces, not ranked tickets or retry submissions. Restored traces are absent.

Version-1 adapter bounds: 432,000 active ticks (two hours), 100,000 events and 4 MiB JSON. These bounds affect future ranked eligibility only; offline gameplay continues if trace limits are reached. The service must enforce matching external limits before exposing Snake ranking.

Wrapping, power-ups, obstacles, multiplayer and daily challenges are deferred.

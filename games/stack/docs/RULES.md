# Stack rules v1

The board has 10 columns, 20 visible rows and four hidden rows above it. Shapes spawn at column 3, hidden row 2. An overlapping spawn, or any occupied hidden cell remaining after a lock and its simultaneous line clears, ends the run. Sprint completes immediately when a clear takes the total to at least 40 lines.

Each shuffled seven-piece bag contains exactly one of each shape. SplitMix64 with its complete 64-bit state drives Fisher–Yates shuffles. Five upcoming pieces and one held piece are visible. Hold may be used once per locking piece.

Rotation uses a 3×3 square for five shapes, 4×4 for the long shape, and a fixed square shape. Both directions try these offsets in order, in screen coordinates (positive y down): `(0,0), (-1,0), (1,0), (-2,0), (2,0), (0,-1), (-1,-1), (1,-1), (0,-2)`. The first collision-free candidate wins. Otherwise rotation does nothing. This is Stack's own symmetric kick convention.

Simulation advances at 60 Hz. Level is `1 + floor(lines / 10)`. Gravity moves one cell each `max(1, floor(60 / level))` ticks. Soft drop caps that interval at two ticks and awards one point per descended cell. Hard drop immediately locks and awards two points per descended cell.

Ground contact starts a 30-tick lock timer. A successful horizontal move or rotation touching the ground resets it, at most 15 times per piece. Once contact has occurred, the timer also advances while airborne. At or beyond 30 ticks the piece locks as soon as it is grounded. This prevents indefinite stalling.

Simultaneous clears award 100 / 300 / 500 / 800 points for 1 / 2 / 3 / 4 lines, multiplied by the level before the clear. Consecutive clearing pieces add `50 × previous consecutive clears × level`. A non-clearing lock resets the chain. No spin or back-to-back bonuses.

Left and right together cancel. Movement happens immediately on changing direction, then after the configurable initial delay (4–24 ticks), then at the repeat interval (1–12 ticks). Rotation, hold and hard drop trigger on key-down edges. Processing order is hold, rotation, horizontal movement, hard drop, gravity, lock. Marathon ranks score; Sprint ranks active simulation ticks, never wall time.

Pauses and focus loss freeze simulation. Preferences, both local bests and one complete run per mode are saved atomically under `$XDG_STATE_HOME/omarchy-stack/session.json` (default `~/.local/state`). Restored runs retain all board, bag, hold, repeat, gravity, lock, score and tick state, and remain eligible for local records. They are never eligible for sharing in v1.

Artwork is original procedural block drawing. Optional cues are original synthesised PCM (shared Arcade cue utility originating in Bubble); `paplay` is optional and absence of an audio server does not stop gameplay. Reduced motion disables the clear-border flash.

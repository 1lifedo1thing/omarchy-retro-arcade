# Snake and the shared community leaderboard

Status: Snake is fully offline. Shared leaderboard integration is outstanding. There is no Snake network client, configured-service probe, ranked-run option or Share Score action in this branch. The screen explicitly says the community leaderboard is not available for Snake yet. This must not be reported as a deployed or verified online feature.

## Inspected dependency

On 12 September 2026, inspected the concurrent local `feat/stack` worktree through commit `022d398` (`docs: describe Stack packaging and pending community hosting`). It contains:

- `shared/leaderboard`: `arcade-leaderboard`, including `Identity`, `Ticket`, `Submission`, `Board`, `Row`, HTTP client and background-worker helper.
- `services/leaderboard`: SQLite-backed shared service. Health, boards, tickets and replay validation currently accept Stack's rules and Marathon/Sprint only.
- `games/stack/src/online.rs`: identity storage, explicit consent, aliases, deletion, ticket preparation, pending submission retry and UI state. Much of this lifecycle remains inside Stack rather than the shared crate.

Snake started from published `main` at `2fddb0f` and was rebased onto `dd5e9f8`, without copying or modifying this concurrent dependency. Stack source existed and was inspectable, but it was not a stable merged Snake-capable service. No hosting deployment was performed for Snake.

## Adapter delivered

Add `omarchy-snake = { path = "../../games/snake", default-features = false }` to the existing service when its shared lifecycle is ready. This builds only deterministic Rust/serde code, with no egui, audio or desktop dependency.

Call:

```rust
omarchy_snake::replay::validate(
    replay_bytes,
    ticket.seed,
    speed_from_stored_ticket,
    &ticket.rules,
)
```

The stored ticket must provide authoritative seed, rules and speed. The validator rejects mismatches, malformed or oversized data, invalid input ordering, reversal/overflow events, invalid pause boundaries, incomplete runs and trailing ticks. It returns a server-calculated `Validated { score, outcome, ticks, board }`. Tests include both a collision and a complete 480-cell win replay. Ranking is score descending for every Snake speed. Board keys are `snake-v1/Slow`, `snake-v1/Normal`, `snake-v1/Fast`.

## Work remaining in the shared service and client

1. Generalise Stack-only dispatch in `/health`, `POST /tickets`, `GET /boards/{rules}/{mode}` and `POST /submissions`. Preserve all existing Stack rules. Use Slow/Normal/Fast as Snake's mode; the existing `(identity, mode, rules)` primary key can keep one best score per board. Include Snake rules in explicit capability discovery.
2. Extract/reuse the existing shared identity, alias, consent, deletion and pending-submission lifecycle from Stack. Do not create another identity file, backend or leaderboard deployment for Snake. Connect a configured, capability-checked service only after an explicit player choice. Local play must never trigger network access.
3. Obtain a single-use ticket before creating `Sim::new(ticket.seed, speed)`. Check ticket rules/speed/expiry. If the request fails, offer local Start immediately. Ordinary Snake remains playable throughout network failures. An offline local trace cannot be promoted to ranked later.
4. Connect the existing accepted-turn/Pause/Resume trace hooks in `SnakeApp` to that ticket. Discard eligibility on restore, abandonment, trace overflow or time-limit overflow. Record Resume only when the countdown actually completes. Never let the network worker own simulation updates.
5. On collision/full-board win, preserve the bounded completed replay and ticket in the shared pending queue until its expiry. Only submit after Share Score and existing first-use public-data consent (alias, mode, score and date). Surface rules-consistent community results, never claims of unaided human play.
6. Use the calculated adapter score, never a client claim, in the shared transaction. Enforce ticket ownership, expiry, single-use/deduplication, wall-clock plausibility, replay-size/event/tick limits and request rate limits before accepting. The adapter validates logical active timing; elapsed pauses/countdowns and ticket issue/submission times require service-level wall-clock checks. The current Stack service issues tickets with a 24-hour expiry; confirm the shared retry/retention policy before enabling Snake rather than introducing a separate period.
7. Preserve shared alias filtering, moderation, deletion with local identity credentials, backups, replay/log retention, credential secrecy and accountless credential-loss messaging. Test ties with equal rank and best-per-identity selection across all three speed/rules boards.

Required integration tests: valid collision and victory, altered claimed scores, invalid replays, reused tickets, wrong versions/speeds, equal-score ties, removal, service unavailable before start, connection loss while playing and bounded retries after completion. Only engine/replay portions are verified in this branch. Identity/ticket/network and hosted-service acceptance are not claimed.

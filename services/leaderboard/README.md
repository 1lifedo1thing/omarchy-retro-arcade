# Arcade community leaderboards

The shared transport lives in `shared/leaderboard`; the Stack adapter lives in `games/stack/src/online.rs`. The service links Stack's exact `stack-v1` rules with the UI feature disabled. No desktop app or display is needed to run the service.

## Local development

```
cargo build --locked -p arcade-leaderboard-service
ARCADE_DATABASE=/tmp/arcade-test.sqlite3 target/debug/arcade-leaderboard-service
ARCADE_LEADERBOARD_URL=http://127.0.0.1:8787 target/release/omarchy-retro-arcade --game stack
```

Use the checkbox on Stack's mode screen to prepare an eligible run. It obtains a ticket before play; no result is uploaded until the player clicks Share score, sees the disclosure and confirms Share publicly. The default build has no service URL and makes no leaderboard requests. Turn off community preparation to play offline immediately if ticket creation fails.

The optional endpoint must be HTTPS, except for explicit loopback development. Redirects are rejected, requests have a 3-second connect / 10-second overall timeout, and responses are capped at 128 KiB. All HTTP work runs on a background thread. The single in-flight request and bounded retry queue avoid unbounded worker creation. The saved identity is bound to its configured endpoint; switching endpoints disables requests until the original endpoint is restored, keeping its credential private.

## API

| Method / path | Contract |
|---|---|
| GET `/health` | Supported rules versions and readiness |
| POST `/identity` | New random pseudonymous ID and 256-bit credential; no account or email |
| POST `/tickets` | Bearer credential; `{mode, rules}`; returns random seed, single-use ticket, 24-hour expiry |
| POST `/submissions` | Bearer credential; `{ticket, alias, replay}`; validates replay before updating best result |
| GET `/boards/stack-v1/Marathon` | Top 100; optional bearer credential adds own best even outside top 100 |
| GET `/boards/stack-v1/Sprint` | Same, ordered by lowest active tick count |
| DELETE `/scores` | Bearer credential; removes all that identity's scores, pending tickets and deduplication entries |

Aliases are unverified display names: 3–24 ASCII letters, digits, spaces, underscores or hyphens. The built-in blocklist excludes obvious profanity and staff impersonation; operators can extend it with one term per line in `ARCADE_ALIAS_BLOCKLIST`. This is an initial filter, not complete linguistic moderation. Operators can remove or block identities using the local CLI.

Only each identity's best result per mode and rules version is kept. Equal values share SQL `RANK()` (1, 1, 3). Submission date is UTC. Identity IDs are public; credentials and their hashes are never in results. The server stores SHA-256 credential hashes, never raw credentials. Ticket values are also hashed at rest. Public scores are tied to rules version, not the desktop app version.

A changed submission using an already consumed ticket is rejected with 409. An exact retry returns the original response without replaying or inserting a second score. This resolves an ambiguous network failure after a database commit. Invalid runs do not consume the ticket. Deletion revokes pending tickets, so a delayed retry cannot recreate deleted scores.

## Bounds and retention

- HTTP JSON body: 2 MiB; client response: 128 KiB. Caddy additionally bounds header/body/read/idle timeouts. The service listens only on loopback and must be reached through Caddy in production.
- Simulation: at most 216,000 active ticks (one hour), 100,000 input changes and 1,000 pause records. A run exceeding these limits continues locally. Inputs must be ordered, within the run and use only the seven supported buttons.
- At most 50 tickets per identity per rolling 24-hour validity window. HTTP bucket: 300 requests per minute per direct peer. The supplied proxy intentionally gives the deployment one shared 300/minute bucket; it never trusts forwarded IP headers. This is a conservative small-community starting capacity, not a scalable abuse-control system.
- At most 10,000 in-memory peer rate entries; entries expire after 60 seconds. No network identifiers are persisted.
- Server replay data: processed in memory and discarded immediately after validation. Persist only result, ticket metadata, request digest and idempotent response.
- Tickets and deduplication records: pruned seven days after ticket expiry on the next request. Public best scores and hashed identities persist until removal/moderation.
- Client pending replay: only after explicit sharing consent; at most eight submissions, deleted after success or 24-hour ticket expiry. Atomic private file under `$XDG_STATE_HOME/omarchy-retro-arcade/leaderboard.json`, mode 0600. Saves and resumes never restore global eligibility.
- Access/request logs: disabled. Generic service startup/failure and system operation logs: seven days, at most 100 MiB on the dedicated VM. Database backups: encrypted, daily, retained 30 days. Deleted entries can remain in these expiring backups; reconcile deletions before restoring an older database.

Losing the local identity credential loses control of that identity and its submissions in this initial accountless version. Back it up privately. Operators can moderate public records, but there is no email recovery, unique alias claim or proof of ownership without the credential.

Replay validation proves conformance to the rules. It does not prove a human played unaided. These are community leaderboards.

## Operations

```
ARCADE_DATABASE=/var/lib/arcade-scores/arcade.sqlite3 arcade-leaderboard-service backup /secure/path/backup.sqlite3
ARCADE_DATABASE=/var/lib/arcade-scores/arcade.sqlite3 arcade-leaderboard-service remove PUBLIC_ID
ARCADE_DATABASE=/var/lib/arcade-scores/arcade.sqlite3 arcade-leaderboard-service block PUBLIC_ID
```

Run these as the service user. Backup uses SQLite's online backup API, including committed WAL contents. To restore: stop the service, retain the failed database, remove its stale WAL/SHM companions, restore a verified snapshot with service ownership and 0600 permissions, run SQLite `PRAGMA integrity_check`, reconcile subsequent removals, then restart and verify `/health`, score reads and deletion. Test restoration to a separate directory monthly.

See `HOSTING.md` for the prepared deployment and approval budget.

A rejected alias can be corrected and explicitly resubmitted while the current run is open. The pending item for that ticket is replaced; other pending results are retained. The board dialog also offers Discard pending uploads. Neither action removes local records or already accepted public scores.

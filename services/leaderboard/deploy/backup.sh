#!/usr/bin/env bash
set -euo pipefail
umask 077
export ARCADE_DATABASE=/var/lib/arcade-scores/arcade.sqlite3
/usr/local/bin/arcade-leaderboard-service backup /var/lib/arcade-scores/backup.sqlite3
# EnvironmentFile supplies RESTIC_REPOSITORY, RESTIC_PASSWORD_FILE and S3 credentials.
restic backup --quiet /var/lib/arcade-scores/backup.sqlite3
restic forget --quiet --keep-within 30d --prune

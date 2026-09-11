# Hosting proposal — pending approval

Prepared 11 September 2026. Nothing has been deployed or purchased. Public sharing remains disabled in the desktop package.

## Proposed deployment

Use a dedicated DigitalOcean Basic Regular Droplet in London: Ubuntu 24.04 LTS, 1 shared vCPU, 1 GiB RAM, 25 GiB SSD, 1,000 GiB included outbound transfer. Run the prebuilt Rust service as `arcade-scores` under systemd, with SQLite in `/var/lib/arcade-scores`. Caddy provides automatic HTTPS for the proposed `arcade-scores.tcballard.dev` hostname. Only ports 80/443 are public; restrict SSH to the operator. Port 8787 remains loopback-only.

| Recurring item | Monthly estimate (USD, before tax) |
|---|---:|
| Basic 1 GiB Droplet | $6.00 |
| Spaces subscription for encrypted daily database backups | $5.00 |
| Caddy / certificates / software | $0.00 |
| Total baseline | **$11.00** |

Assumes an existing domain, no additional paid monitoring, fewer than 250 GiB of backup data and transfer within both plans' allowances. Extra storage/egress and taxes are additional. Configure a $15 monthly billing alert. This is a starting capacity estimate, not a performance guarantee; load-test on the actual VM before public activation.

Prices checked against [DigitalOcean Droplet pricing](https://www.digitalocean.com/pricing/droplets) and [Spaces pricing](https://www.digitalocean.com/pricing/spaces-object-storage). The plans provide 1,000 GiB Droplet transfer and 250 GiB storage / 1 TiB outbound transfer for Spaces. Weekly VM backups are optional at 20% of Droplet cost ($1.20/month); they are not included because the proposal already uses application-consistent, encrypted database backups.

## Prepared configuration

The adjacent `deploy/` directory contains the systemd service, Caddyfile, daily backup service/timer, restic backup script, environment template and journal retention policy. No secrets are committed. The private backup bucket and encryption password must be created during the approved deployment; keep a copy of the restic password outside the VM.

Deployment sequence after approval:

1. Create the VM and private Spaces bucket; point the proposed hostname to the VM. Install Caddy, restic and SQLite CLI from their maintained packages. Compile the service on CI/build hardware (`cargo build --release --locked -p arcade-leaderboard-service`), not on the 1 GiB VM.
2. Create a system user/group `arcade-scores`. Install the binary at `/usr/local/bin/arcade-leaderboard-service` (0755), configuration in `/etc/arcade-scores` (0750), an initially empty `blocked-aliases.txt` (0640), and state under `/var/lib/arcade-scores` owned by that user (0700).
3. Install `arcade-leaderboard.service` and enable it. Install the Caddyfile, run `caddy validate --config /etc/caddy/Caddyfile`, and reload Caddy. Its body size and timeout configuration follows [Caddy request_body](https://caddyserver.com/docs/caddyfile/directives/request_body) and [server timeout options](https://caddyserver.com/docs/caddyfile/options).
4. Install `backup.sh` as `/usr/local/libexec/arcade-backup` (0755). Fill `backup.env` with a bucket-scoped key and the private repository URL (0600, owned by `arcade-scores`). Create the separate restic password file (0600), initialise the repository, install and enable the backup service/timer. Apply the supplied journal retention policy on this dedicated VM.
5. Verify HTTPS trust, API size/rate limits, valid Marathon and Sprint replay submissions, tampered and duplicate requests, ranks, removal, offline retry, service restart and an encrypted backup restore. Check process memory under the replay limit and health from an external connection. Alert on health failure and backup age over 26 hours.
6. Only after these checks, configure `ARCADE_LEADERBOARD_URL=https://arcade-scores.tcballard.dev` for the Arcade launch environment. Run native end-to-end sharing on Omarchy before making that endpoint the supported public default.

The approval decision is the **$11/month baseline and proposed hostname/provider**. No application feature requires this spend: Stack remains fully playable offline.

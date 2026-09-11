#!/usr/bin/env bash
set -euo pipefail
# Build only: never installs packages or invokes sudo.
project_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
output_dir="${1:-$project_dir/dist/arch}"
mkdir -p "$output_dir"
output_dir="$(cd "$output_dir" && pwd)"
work_dir="$(mktemp -d)"
trap 'rm -rf -- "$work_dir"' EXIT
# Use the exact committed revision; fail rather than silently omitting local edits.
if [[ -n "$(git -C "$project_dir" status --porcelain --untracked-files=normal)" ]]; then
  echo 'Commit or stash source changes before building an Arch package.' >&2
  exit 1
fi
git -C "$project_dir" archive --format=tar.gz --prefix=omarchy-retro-arcade/ HEAD > "$work_dir/omarchy-retro-arcade-source.tar.gz"
cp "$project_dir/packaging/PKGBUILD" "$work_dir/PKGBUILD"
source_hash="$(sha256sum "$work_dir/omarchy-retro-arcade-source.tar.gz" | cut -d ' ' -f1)"
sed -i "s/'SKIP'/'$source_hash'/" "$work_dir/PKGBUILD"
(cd "$work_dir" && PKGDEST="$output_dir" makepkg --cleanbuild --clean)

mkdir -p "$output_dir/sources"
cp "$work_dir"/*.tar.gz "$work_dir"/*.nnue "$output_dir/sources/"

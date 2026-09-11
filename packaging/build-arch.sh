#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
root="$PWD"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
version=0.1.0
git archive --format=tar.gz --prefix="omarchy-invaders-$version/" HEAD > "$work/omarchy-invaders-$version.tar.gz"
cp packaging/PKGBUILD "$work/PKGBUILD"
hash="$(sha256sum "$work/omarchy-invaders-$version.tar.gz" | cut -d' ' -f1)"
sed -i "s/'SKIP'/'$hash'/" "$work/PKGBUILD"
cd "$work"
makepkg --cleanbuild --clean --noconfirm
mkdir -p "$root/dist/arch"
cp ./*.pkg.tar.zst "$root/dist/arch/"

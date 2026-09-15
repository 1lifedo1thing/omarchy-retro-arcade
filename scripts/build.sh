#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
# An explicit Arcade limit applies to both Rust and Pinball builds.
if [[ -n "${ARCADE_BUILD_JOBS:-}" ]]; then
  export CARGO_BUILD_JOBS="$ARCADE_BUILD_JOBS"
fi
cargo build --locked --release -p omarchy-retro-arcade
cmake -S games/pinball -B build/pinball -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr
cmake --build build/pinball --target SpaceCadetPinball theme-tests --parallel "${ARCADE_BUILD_JOBS:-$(nproc 2>/dev/null || echo 2)}"

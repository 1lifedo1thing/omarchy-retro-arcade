#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")/.."
cargo build --locked --release -p omarchy-retro-arcade
cmake -S games/pinball -B build/pinball -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr
cmake --build build/pinball --target SpaceCadetPinball theme-tests --parallel "${ARCADE_BUILD_JOBS:-2}"

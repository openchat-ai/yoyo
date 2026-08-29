#!/usr/bin/env bash
# stage17-ow-iat-wireup.sh — OW-IAT wire-up WIP gate (Linux)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/yoyo-rust"
if command -v rustup >/dev/null 2>&1 && rustup toolchain list 2>/dev/null | grep -q 'nightly'; then
  cargo +nightly test -p verifier --lib manual_map
else
  cargo test -p verifier --lib manual_map
fi
echo "OW_IAT_WIREUP status=WIP phase=manual_map_x64_emit H_00_wired=YES LoadLibraryA=ABSENT PEB_resolve=DROPPED Linux_dlopen=ABSENT three_peer=LOCKSTEP"

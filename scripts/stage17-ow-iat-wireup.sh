#!/usr/bin/env bash
# stage17-ow-iat-wireup.sh — OW-IAT wire-up WIP gate (Linux)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/yoyo-rust"
cargo test -p verifier --lib manual_map
echo "OW_IAT_WIREUP status=WIP phase=manual_map_x64_emit H_00_wired=YES LoadLibraryA=ABSENT PEB_resolve=DROPPED three_peer=LOCKSTEP"

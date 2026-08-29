#!/usr/bin/env bash
# stage17-ow-iat-wireup.sh — OW-IAT wire-up WIP gate (Linux)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/yoyo-rust"
cargo test -p verifier h00_manual_map_wireup pe_manual_map
echo "OW_IAT_WIREUP status=WIP phase=file_read_prelude_emit manual_map_body=NOT_WIRED LoadLibraryA=PEB_resolve"

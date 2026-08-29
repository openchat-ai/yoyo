#!/usr/bin/env bash
# OW-IAT spike gate (Linux) — proves pe_manual_map unit tests; honest LoadLibraryA still present.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/yoyo-rust"

echo "=== Stage 17 OW-IAT spike (Linux) ==="
cargo test -p verifier --lib pe_manual_map
echo "OW_IAT_SPIKE pe_manual_map_tests=GREEN"

# Honest: seed path still uses LoadLibraryA until wire-up PR (grep linker metadata in sources).
if rg -q 'LoadLibraryA' "$ROOT/yoyo-rust/verifier/src/pe_link.rs" \
  && rg -q 'IAT_LOADLIBRARY' "$ROOT/yoyo-rust/verifier/src/win32_selfhost.rs"; then
  echo "OW_IAT_SPIKE LoadLibraryA=PRESENT disposition=CUT (expected)"
else
  echo "OW_IAT_SPIKE LoadLibraryA=ABSENT — wire-up landed; run stage15-hole-inventory for CLOSED"
fi

echo "OW_IAT_SPIKE status=GREEN doc=SCOPE-CUT-v1.0-ow-iat-spike.md"
exit 0

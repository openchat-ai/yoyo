#!/usr/bin/env bash
# Rebuild linux_h00_tramp.elf (committed blob for Stage 10-B / 11-B H_00 path).
# Stage 11-B: assemble nostdlib .S (no CRT) + compact dynamic link — shrinks
# host dlopen surface vs prior gcc -O2 CRT blob (~14KB → ~10KB).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BLOB_DIR="$ROOT/yoyo-rust/verifier/blobs"
SRC="$BLOB_DIR/linux_h00_tramp.S"
OUT="$BLOB_DIR/linux_h00_tramp.elf"
cc -nostdlib -no-pie -fno-pie -fno-asynchronous-unwind-tables \
  -Wl,-z,norelro -Wl,--hash-style=sysv -Wl,--gc-sections -s \
  -o "$OUT" "$SRC" -ldl -lc -Wl,-e,_start
chmod +x "$OUT"
ls -la "$OUT"
file "$OUT"
echo "NEEDED:"
readelf -d "$OUT" | grep NEEDED || true
echo "UNDEF dynamic:"
nm -D "$OUT" 2>/dev/null | awk '/ U /{print}' || true

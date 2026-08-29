#!/usr/bin/env bash
# Rebuild linux_h00_tramp.elf (committed blob for Stage 10-B / 11-B H_00 path).
# Hybrid OW-IAT: dynamic -lc only (no libdl NEEDED); dlopen@PLT resolves via ld.so-mapped
# libc (do NOT static-mmap glibc/ld from disk). Sidecar via dlopen + in-process sym walk.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BLOB_DIR="$ROOT/yoyo-rust/verifier/blobs"
SRC="$BLOB_DIR/linux_h00_tramp.S"
OUT="$BLOB_DIR/linux_h00_tramp.elf"
cc -nostdlib -no-pie -fno-pie -fno-asynchronous-unwind-tables \
  -Wl,-z,norelro -Wl,--hash-style=sysv -Wl,--gc-sections -s \
  -o "$OUT" "$SRC" -lc -lgcc_s -Wl,-e,_start
chmod +x "$OUT"
ls -la "$OUT"
file "$OUT"
echo "NEEDED:"
readelf -d "$OUT" 2>/dev/null | grep NEEDED || echo "none"
echo "UNDEF dynamic:"
nm -D "$OUT" 2>/dev/null | awk '/ U /{print}' || true
if readelf -d "$OUT" 2>/dev/null | grep -qE 'NEEDED.*libdl'; then
  echo "RED: trampoline still NEEDED libdl (hybrid requires dlopen via libc only)"
  exit 1
fi
if nm -D "$OUT" 2>/dev/null | grep -q 'dlsym'; then
  echo "RED: trampoline still imports dlsym (post-v1.0 OW-IAT requires ELF dyn walk)"
  exit 1
fi
if strings "$OUT" | grep -q '/lib/x86_64-linux-gnu/libc.so.6'; then
  echo "RED: trampoline still hardcodes glibc disk path"
  exit 1
fi

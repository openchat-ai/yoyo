#!/usr/bin/env bash
# Rebuild linux_h00_tramp.elf (committed blob for Stage 10-B / 11-B H_00 path).
# Post-v1.0 OW-IAT: syscall-only manual ELF map (open/read/mmap) — no dlopen/libdl/libc.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BLOB_DIR="$ROOT/yoyo-rust/verifier/blobs"
SRC="$BLOB_DIR/linux_h00_tramp_mmap.c"
OUT="$BLOB_DIR/linux_h00_tramp.elf"
cc -nostdlib -static -fno-pie -fno-stack-protector -fno-asynchronous-unwind-tables -ffreestanding \
  -O2 -Wl,-z,norelro -Wl,--gc-sections -s \
  -o "$OUT" "$SRC" -Wl,-e,_start
chmod +x "$OUT"
ls -la "$OUT"
file "$OUT"
echo "NEEDED:"
readelf -d "$OUT" 2>/dev/null | grep NEEDED || echo "none (static syscall tramp)"
echo "UNDEF dynamic:"
nm "$OUT" 2>/dev/null | awk '/ U /{print}' || true
if readelf -d "$OUT" 2>/dev/null | grep -q NEEDED; then
  echo "RED: trampoline still has dynamic NEEDED (post-v1.0 OW-IAT requires syscall-only)"
  exit 1
fi
if nm -D "$OUT" 2>/dev/null | grep -qE 'dlopen|dlsym'; then
  echo "RED: trampoline still imports dlopen/dlsym"
  exit 1
fi
if strings "$OUT" | grep -q 'dlopen'; then
  echo "RED: trampoline still contains dlopen string"
  exit 1
fi

#!/usr/bin/env bash
# Rebuild linux_h00_tramp.elf (committed blob for Stage 10-B H_00 path).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BLOB_DIR="$ROOT/yoyo-rust/verifier/blobs"
cc -O2 -s -o "$BLOB_DIR/linux_h00_tramp.elf" "$BLOB_DIR/linux_h00_tramp.c" -ldl
chmod +x "$BLOB_DIR/linux_h00_tramp.elf"
ls -la "$BLOB_DIR/linux_h00_tramp.elf"
file "$BLOB_DIR/linux_h00_tramp.elf"

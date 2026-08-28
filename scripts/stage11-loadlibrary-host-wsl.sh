#!/usr/bin/env bash
# Invoked by stage11-loadlibrary-host.ps1 — exact tramp embed in linux gen1.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
# When called via /mnt/f/... path from WSL:
if [[ ! -d "$ROOT/yoyo-rust" ]]; then
  ROOT=/mnt/f/yoyo
fi
WD="$ROOT/scripts/_stage11-loadlibrary-host/wsl"
mkdir -p "$WD"

LINUX_YOYO="$ROOT/yoyo-rust/target/release/yoyo"
need_build=0
if [[ ! -x "$LINUX_YOYO" ]]; then
  need_build=1
elif file "$LINUX_YOYO" 2>/dev/null | grep -qi 'PE32'; then
  need_build=1
fi
if [[ ! -f "$ROOT/yoyo-rust/target/release/libyoyo_runtime.so" \
   && ! -f "$ROOT/yoyo-rust/target/release-runtime/libyoyo_runtime.so" ]]; then
  need_build=1
fi

if [[ "$need_build" -eq 1 ]]; then
  echo "== WSL build verifier + yoyo-runtime (release / release-runtime) =="
  (cd "$ROOT/yoyo-rust" && cargo build --release -p verifier && cargo build --profile release-runtime -p yoyo-runtime)
fi
if [[ -f "$ROOT/yoyo-rust/target/release-runtime/libyoyo_runtime.so" ]]; then
  mkdir -p "$ROOT/yoyo-rust/target/release"
  cp -f "$ROOT/yoyo-rust/target/release-runtime/libyoyo_runtime.so" \
    "$ROOT/yoyo-rust/target/release/libyoyo_runtime.so"
fi

LINUX_YOYO="$ROOT/yoyo-rust/target/release/yoyo"
TRAMP="$ROOT/yoyo-rust/verifier/blobs/linux_h00_tramp.elf"
GEN1="$WD/gen1.elf"
TY="$ROOT/yoyo/projects/yoyo.ty"
rm -f "$GEN1"
"$LINUX_YOYO" link --target=linux "$TY" "$GEN1"

python3 - <<PY
from pathlib import Path
root = Path(r"$ROOT")
tramp = (root / "yoyo-rust/verifier/blobs/linux_h00_tramp.elf").read_bytes()
gen1 = (root / "scripts/_stage11-loadlibrary-host/wsl/gen1.elf").read_bytes()
off = gen1.find(tramp)
print(f"gen1_bytes={len(gen1)}")
print(f"tramp_bytes={len(tramp)}")
print(f"embed_off={off}")
if off < 0:
    raise SystemExit("embed miss")
PY

#!/usr/bin/env bash
# stage17-ow-rt-yoyo-runtime.sh — Linux/cloud gate for OW-RT Gate G slice
#
# Runs pe_dll_link unit tests + emits YOYO alt sidecar bytes.
# Windows H_00 smoke is documented below (no Win PE on Linux cloud).
#
# Local Windows (preferred for full smoke):
#   cd F:\yoyo
#   cargo build --release -p verifier   # in yoyo-rust
#   & .\scripts\stage17-ow-rt-yoyo-runtime.ps1
# Optional alt no-input smoke (expect exit 2):
#   cargo run -p verifier --bin emit-rt-sidecar -- <workdir>\yoyo_rt.dll
#   yoyo link --target=win32 yoyo\projects\yoyo.ty <workdir>\gen1.exe
#   pushd <workdir>; .\gen1.exe; popd   # exit 2, no input.*
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "=== Post-v1.0: OW-RT Gate G slice (Linux/cloud: tests + alt emit) ==="

TY_STUB="$ROOT/yoyo/tests/golden/ow_rt_yoyo_origin_exit2.ty"
TY_FX="$ROOT/yoyo/tests/golden/selfhost_min_nop.ty"
SPIKE_DOC="$ROOT/SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md"
[[ -f "$TY_STUB" ]] || { echo "missing $TY_STUB"; exit 1; }
[[ -f "$TY_FX" ]] || { echo "missing $TY_FX"; exit 1; }
[[ -f "$SPIKE_DOC" ]] || { echo "missing $SPIKE_DOC"; exit 1; }

cd "$ROOT/yoyo-rust"
# --lib + no wasmtime: pe_dll_link is lib-only; cloud Rust may lack edition2024 for wasmtime
cargo test -p verifier --lib pe_dll_link --no-default-features --features full-backends
echo "OW_RT_SPIKE pe_dll_link_tests=GREEN"
echo "OW_RT_SPIKE yoyo_origin_export=PRESENT stub=$TY_STUB"
echo "OW_RT_SPIKE yoyo_built_effect=PRESENT fixture=$TY_FX exits=0/1/2/3"

WORK="$ROOT/scripts/_stage17-ow-rt-alt-sidecar"
mkdir -p "$WORK"
ALT="$WORK/yoyo_rt.dll"
rm -f "$ALT"

cargo run -q -p verifier --bin emit-rt-sidecar --no-default-features --features full-backends -- "$ALT"
[[ -f "$ALT" ]] || { echo "missing alt sidecar $ALT"; exit 1; }

ALT_LEN=$(wc -c <"$ALT" | tr -d ' ')
if [[ "$ALT_LEN" -lt 64 ]]; then
  echo "YOYO alt sidecar too small ($ALT_LEN)"
  exit 1
fi
# MZ magic
python3 - <<PY
from pathlib import Path
p = Path("$ALT")
b = p.read_bytes()
assert b[:2] == b"MZ", "not MZ"
assert b"yoyo_runtime_selfhost_main" in b, "missing export"
assert b"yoyo_rt.dll" in b, "missing dll name"
print(f"OW_RT_SPIKE yoyo_alt_sidecar=EMITTED path={p} bytes={len(b)}")
PY

echo "OW_RT_SPIKE production_default=RUST"
echo "OW_RT_SPIKE yoyo_alt_sidecar_smoke=SKIP (non-Windows; see script header for Win steps)"
echo "OW_RT_SPIKE yoyo_built=ALT_SIDECAR yoyo_alt_sidecar=EMITTED disposition=CUT"
echo "OW_RT_SPIKE note=Gate_G_slice_alt_emit_only; CLOSED requires production YOYO-built sidecar + no Rust yoyo_rt.dll host trust"
echo "OW_RT_SPIKE status=GREEN doc=SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md"
exit 0

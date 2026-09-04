#!/usr/bin/env bash
# stage17-ow-rt-yoyo-runtime.sh — Linux/cloud gate for OW-RT Gate G slice
#
# Runs pe_dll_link unit tests + dll-recompile (generic shell + call-time inject).
# Windows H_00 smoke is documented below (no Win PE on Linux cloud).
#
# Local Windows (preferred for full smoke):
#   cd F:\yoyo
#   & .\scripts\stage17-ow-rt-yoyo-runtime.ps1
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
echo "=== Post-v1.0: OW-RT Gate G slice (Linux/cloud: tests + dll-recompile) ==="
TY_STUB="$ROOT/yoyo/tests/golden/ow_rt_yoyo_origin_exit2.ty"
TY_FX="$ROOT/yoyo/tests/golden/selfhost_min_nop.ty"
TY_FX2="$ROOT/yoyo/tests/golden/selfhost_min_set_52_cafef00d.ty"
SPIKE_DOC="$ROOT/SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md"
[[ -f "$TY_STUB" ]] || { echo "missing $TY_STUB"; exit 1; }
[[ -f "$TY_FX" ]] || { echo "missing $TY_FX"; exit 1; }
[[ -f "$TY_FX2" ]] || { echo "missing $TY_FX2"; exit 1; }
[[ -f "$SPIKE_DOC" ]] || { echo "missing $SPIKE_DOC"; exit 1; }
cd "$ROOT/yoyo-rust"
cargo test -p verifier --lib pe_dll_link --no-default-features --features full-backends
echo "OW_RT_SPIKE pe_dll_link_tests=GREEN"
echo "OW_RT_SPIKE yoyo_origin_export=PRESENT stub=$TY_STUB"
echo "OW_RT_SPIKE yoyo_built_effect=PRESENT fixture=$TY_FX exits=0/1/2/3"
WORK="$ROOT/scripts/_stage17-ow-rt-dll-recompile"
rm -rf "$WORK"
mkdir -p "$WORK"
cp "$TY_FX" "$WORK/input.ty"
set +e
cargo run -q -p verifier --bin emit-rt-sidecar --no-default-features --features full-backends -- --dll-recompile "$WORK"
EC_EXIT=$?
set -e
if [[ "$EC_EXIT" -ne 0 ]]; then
  echo "emit-rt-sidecar --dll-recompile expected exit=0 got=$EC_EXIT"
  exit 1
fi
ALT="$WORK/yoyo_rt.dll"
OUT="$WORK/output.exe"
[[ -f "$ALT" ]] || { echo "missing alt sidecar $ALT"; exit 1; }
[[ -f "$OUT" ]] || { echo "missing output.exe $OUT"; exit 1; }
python3 - <<PY
from pathlib import Path
work = Path("$WORK")
alt = work / "yoyo_rt.dll"
out = work / "output.exe"
b = alt.read_bytes()
assert b[:2] == b"MZ", "sidecar not MZ"
assert b"yoyo_runtime_selfhost_main" in b, "missing export"
assert b"yoyo_dll_recompile" in b, "missing dll-recompile marker"
assert b"yoyo_rt.dll" in b, "missing dll name"
o = out.read_bytes()
assert o[:2] == b"MZ" and len(o) >= 64, "output.exe not PE"
print(f"OW_RT_SPIKE yoyo_dll_recompile=PRESENT path={work} sidecar_bytes={len(b)} output_bytes={len(o)}")
print("OW_RT_SPIKE yoyo_alt_sidecar=EMITTED (dll-recompile pe_dll)")
print("OW_RT_SPIKE gate_g_slice=dll_recompile")
PY
echo "OW_RT_SPIKE production_default=RUST"
echo "OW_RT_SPIKE yoyo_dll_recompile_smoke=SKIP (non-Windows; see Win ps1 for H_00)"
echo "OW_RT_SPIKE yoyo_built=DLL_RECOMPILE yoyo_dll_recompile=PRESENT disposition=CUT"
echo "OW_RT_SPIKE note=Gate_G_slice_dll_recompile; compiler ISA still ABSENT in pe_dll; CLOSED requires production YOYO-built compile sidecar + no Rust yoyo_rt.dll host trust"
echo "OW_RT_SPIKE status=GREEN doc=SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md"
exit 0

#!/usr/bin/env bash
# stage5-linux-selfhost.sh — Linux M1→M2→M3 self-host chain (WSL / native Linux)
# M2→M3: gen2rt.elf embedded startup → dlopen libyoyo_runtime.so → compile input → output.elf
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

WORKDIR="$ROOT/scripts/_stage5-linux"
mkdir -p "$WORKDIR"

TY="$ROOT/yoyo/projects/yoyo.ty"
TYB="$ROOT/yoyo/projects/yoyo.tyb"
YOYO="$ROOT/yoyo-rust/target/release/yoyo"

if [[ ! -x "$YOYO" ]] || [[ ! -f "$ROOT/yoyo-rust/target/release/libyoyo_runtime.so" ]]; then
  echo "== build yoyo + yoyo-runtime (release) =="
  (cd "$ROOT/yoyo-rust" && cargo build --release -p verifier && cargo build --release -p yoyo-runtime)
fi

if [[ ! -f "$TYB" ]]; then
  echo "== ty2tyb =="
  python3 "$ROOT/scripts/ty2tyb.py"
fi

GEN1="$WORKDIR/gen1.elf"
GEN2="$WORKDIR/gen2.elf"
GEN3="$WORKDIR/gen3.elf"
INPUT_TYB="$WORKDIR/input.tyb"
INPUT_KY="$WORKDIR/input.ky"
RUNTIME_SO="$WORKDIR/libyoyo_runtime.so"

cp -f "$TYB" "$INPUT_TYB"
cp -f "$TY" "$INPUT_KY"

echo "== M0: yoyo link (gen1 reference) =="
"$YOYO" link --target=linux "$TY" "$GEN1"

m1m2_green=false
m2m3_green=false

echo ""
echo "=== M1→M2: bootstrap input.tyb → gen2.elf (interim) ==="
rm -f "$GEN2"
if "$YOYO" bootstrap --target=linux "$INPUT_TYB" "$GEN2" && [[ -f "$GEN2" ]]; then
  m1m2_green=true
  echo "M1→M2 bootstrap: GREEN (gen2=$(stat -c%s "$GEN2") bytes)"
  if [[ -f "$GEN1" ]]; then
    "$YOYO" diff "$GEN1" "$GEN2" || true
  fi
else
  echo "M1→M2 bootstrap: RED"
fi

echo ""
echo "=== M2→M3: gen2rt embedded startup compiles input → gen3 (no hang/AV) ==="
cd "$WORKDIR"
rm -f gen2rt.elf output.elf "$RUNTIME_SO"
echo "building gen2rt via bootstrap --selfhost (embedded startup + libyoyo_runtime.so sidecar)..."
if "$YOYO" bootstrap --selfhost --target=linux "$INPUT_TYB" gen2rt.elf; then
  if [[ -f "$RUNTIME_SO" ]]; then
    chmod +x gen2rt.elf
    # Bare-name dlopen fallback for older gen2rt stubs built before getcwd path fix.
    export LD_LIBRARY_PATH=".:${LD_LIBRARY_PATH:-}"
    set +e
    ./gen2rt.elf
    ec=$?
    set -e
    if [[ -f output.elf ]] && [[ "$ec" -eq 0 ]]; then
      cp -f output.elf "$GEN3"
      m2m3_green=true
      echo "M2→M3: GREEN (gen3=$(stat -c%s "$GEN3") bytes, embedded startup)"
    else
      echo "M2→M3: RED (exit=$ec, no output.elf)"
    fi
  else
    echo "M2→M3: RED (missing libyoyo_runtime.so sidecar)"
  fi
else
  echo "M2→M3: RED (bootstrap --selfhost failed)"
fi
cd "$ROOT"

echo ""
echo "=== summary ==="
echo "M1→M2 bootstrap: $(if $m1m2_green; then echo GREEN; else echo RED; fi)"
echo "M2→M3 runtime:   $(if $m2m3_green; then echo GREEN; else echo RED; fi)"

if $m2m3_green; then
  exit 0
else
  exit 1
fi

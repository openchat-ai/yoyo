#!/usr/bin/env bash
# stage8-extended-selfhost.sh — Linux M2→M3→M4 self-host chain (Stage 8-C)
# M2→M3: gen2rt embedded startup → compile input → gen3
# M3→M4: gen3rt embedded startup → compile input → gen4
# gen4 parity vs gen3_direct via .text section-ddc
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

WORKDIR="$ROOT/scripts/_stage8-linux"
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

GEN2="$WORKDIR/gen2.elf"
GEN3="$WORKDIR/gen3.elf"
GEN3_DIRECT="$WORKDIR/gen3_direct.elf"
GEN4="$WORKDIR/gen4.elf"
INPUT_TYB="$WORKDIR/input.tyb"
INPUT_KY="$WORKDIR/input.ky"
RUNTIME_SO="$WORKDIR/libyoyo_runtime.so"

cp -f "$TYB" "$INPUT_TYB"
cp -f "$TY" "$INPUT_KY"

m2m3_green=false
m3m4_green=false
parity_equal=false
trust_sha=""

run_genrt() {
  local genrt="$1"
  chmod +x "$genrt"
  export LD_LIBRARY_PATH=".:${LD_LIBRARY_PATH:-}"
  set +e
  "./$(basename "$genrt")"
  local ec=$?
  set -e
  return "$ec"
}

echo ""
echo "=== M2→M3: gen2rt embedded startup compiles input → gen3 (no hang/AV) ==="
cd "$WORKDIR"
rm -f gen2rt.elf output.elf "$GEN3" "$RUNTIME_SO"
echo "building gen2rt via bootstrap --selfhost..."
if "$YOYO" bootstrap --selfhost --target=linux "$INPUT_TYB" gen2rt.elf; then
  if [[ -f "$RUNTIME_SO" ]]; then
    run_genrt gen2rt.elf
    ec=$?
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

echo ""
echo "=== M3→M4: gen3rt embedded startup compiles input → gen4 (no hang/AV) ==="
rm -f gen3rt.elf output.elf "$GEN4" "$RUNTIME_SO"

if ! $m2m3_green; then
  echo "M3→M4: SKIP (M2→M3 not green)"
else
  echo "building gen3_direct reference via bootstrap..."
  rm -f "$GEN3_DIRECT"
  if "$YOYO" bootstrap --target=linux "$INPUT_TYB" "$GEN3_DIRECT"; then
    echo "gen3_direct: $(stat -c%s "$GEN3_DIRECT") bytes"

    echo "building gen3rt via bootstrap --selfhost..."
    if "$YOYO" bootstrap --selfhost --target=linux "$INPUT_TYB" gen3rt.elf; then
      if [[ -f "$RUNTIME_SO" ]]; then
        run_genrt gen3rt.elf
        ec=$?
        if [[ -f output.elf ]] && [[ "$ec" -eq 0 ]]; then
          cp -f output.elf "$GEN4"
          m3m4_green=true
          echo "M3→M4: GREEN (gen4=$(stat -c%s "$GEN4") bytes, embedded startup)"

          echo ""
          echo "=== trust chain: gen4 vs gen3_direct (.text section-ddc) ==="
          if "$YOYO" diff "$GEN4" "$GEN3_DIRECT"; then
            parity_equal=true
            trust_sha=$(sha256sum "$GEN4" | awk '{print substr($1,1,8)}')
            echo "gen4 ≡ gen3_direct (.text DDC): EQUAL (sha256 prefix $trust_sha)"
          else
            echo "gen4 ≡ gen3_direct (.text DDC): DIFF"
            "$YOYO" diff "$GEN3" "$GEN4" || true
          fi

          echo ""
          echo "=== gen12 window: gen3 vs gen4 (.text DDC) ==="
          "$YOYO" diff "$GEN3" "$GEN4" || true
        else
          echo "M3→M4: RED (exit=$ec, no output.elf)"
        fi
      else
        echo "M3→M4: RED (missing libyoyo_runtime.so sidecar)"
      fi
    else
      echo "M3→M4: RED (bootstrap --selfhost gen3rt failed)"
    fi
  else
    echo "M3→M4: RED (gen3_direct bootstrap failed)"
  fi
fi
cd "$ROOT"

echo ""
echo "=== summary ==="
echo "M2→M3 runtime:   $(if $m2m3_green; then echo GREEN; else echo RED; fi)"
echo "M3→M4 runtime:   $(if $m3m4_green; then echo GREEN; else echo RED; fi)"
echo "gen4 DDC parity: $(if $parity_equal; then echo "EQUAL (sha256 prefix $trust_sha)"; else echo "DIFF or N/A"; fi)"
echo "Stage 8-C:       $(if $m2m3_green && $m3m4_green && $parity_equal; then echo 'may check [x]'; else echo 'keep [ ] — chain incomplete or DDC mismatch'; fi)"
echo ""
echo "Trust chain: M4 chain uses same gen12/section-ddc gates as stage5/fullbody."
echo "  gen3_direct reference = yoyo bootstrap input.tyb (788-handler full body)"
echo "  gen4 = gen3rt runtime output (second embedded selfhost generation)"
if [[ -n "$trust_sha" ]]; then
  echo "  gen4 .text SHA256 prefix: $trust_sha"
fi

if $m2m3_green && $m3m4_green && $parity_equal; then
  exit 0
else
  exit 1
fi

#!/usr/bin/env bash
# stage10-linux-pure-m4.sh — Stage 10-B: Linux ELF H_00 pure M4 (no bootstrap --selfhost)
# Seed: yoyo link --target=linux → gen1 (H_00 entry)
# Chain: gen1 → gen2 → gen3 → gen4 (each zero-arg H_00 extract+execve trampoline)
# Parity: gen4 ≡ gen3_direct (full-file DDC via yoyo diff); gen3_direct = bootstrap WITHOUT --selfhost
# Trust: M3→M4 algebra runs via prior YOYO ELF H_00 path; host never calls bootstrap --selfhost here.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Prefer Linux-native tmp for WSL+/mnt/f DrvFs races (stat after write can miss).
# Artifacts are mirrored to scripts/_stage10-linux-pure-m4 for inspection.
ARTIFACT_DIR="$ROOT/scripts/_stage10-linux-pure-m4"
mkdir -p "$ARTIFACT_DIR"
if [[ -d /tmp ]] && touch /tmp/.yoyo_stage10b_w 2>/dev/null; then
  WORKDIR="/tmp/yoyo-stage10-linux-pure-m4"
  rm -f /tmp/.yoyo_stage10b_w
else
  WORKDIR="$ARTIFACT_DIR"
fi
mkdir -p "$WORKDIR"
rm -f "$WORKDIR"/{gen1,gen2,gen3,gen4,gen3_direct,output}.elf \
  "$WORKDIR"/libyoyo_runtime.so "$WORKDIR"/.yoyo_h00_tramp

TY="$ROOT/yoyo/projects/yoyo.ty"
TYB="$ROOT/yoyo/projects/yoyo.tyb"
YOYO="$ROOT/yoyo-rust/target/release/yoyo"
RUNTIME_SO="$ROOT/yoyo-rust/target/release/libyoyo_runtime.so"

if [[ ! -x "$YOYO" ]] || [[ ! -f "$RUNTIME_SO" ]]; then
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
GEN4="$WORKDIR/gen4.elf"
GEN3_DIRECT="$WORKDIR/gen3_direct.elf"
INPUT_TYB="$WORKDIR/input.tyb"
INPUT_KY="$WORKDIR/input.ky"

cp -f "$TYB" "$INPUT_TYB"
cp -f "$TY" "$INPUT_KY"

file_size() {
  local f="$1"
  # Retry briefly — DrvFs can lag after write when WORKDIR is under /mnt/*
  local i
  for i in 1 2 3 4 5 6 7 8 9 10; do
    if [[ -f "$f" ]]; then
      if stat -c%s "$f" 2>/dev/null; then
        return 0
      fi
    fi
    sync 2>/dev/null || true
    sleep 0.1
  done
  echo "0"
  return 1
}

chain_green=false
parity_equal=false
trust_sha=""
text_sha=""

run_h00() {
  local exe="$1"
  local out="$2"
  local label="$3"
  chmod +x "$exe"
  rm -f "$WORKDIR/output.elf" "$WORKDIR/libyoyo_runtime.so" "$WORKDIR/.yoyo_h00_tramp"
  echo "running $label (zero-arg H_00)..."
  set +e
  (cd "$WORKDIR" && "./$(basename "$exe")")
  local ec=$?
  set -e
  if [[ ! -f "$WORKDIR/output.elf" ]] || [[ "$ec" -ne 0 ]]; then
    echo "${label}: RED (exit=$ec, no output.elf)"
    return 1
  fi
  cp -f "$WORKDIR/output.elf" "$out"
  echo "${label}: GREEN ($(file_size "$out") bytes)"
  return 0
}

echo ""
echo "=== Stage 10-B: seed gen1 via yoyo link (H_00 path, NOT bootstrap --selfhost) ==="
echo "WORKDIR=$WORKDIR"
rm -f "$GEN1"
"$YOYO" link --target=linux "$TY" "$GEN1"
gen1_sz="$(file_size "$GEN1" || true)"
if [[ ! -f "$GEN1" ]] || [[ "$gen1_sz" -le 0 ]]; then
  echo "Stage 10-B: RED (gen1 link failed)"
  exit 1
fi
echo "gen1: ${gen1_sz} bytes (ELF entry → H_00)"

echo ""
echo "=== gen1 → gen2 (H_00) ==="
run_h00 "$GEN1" "$GEN2" "gen1→gen2" || exit 1

echo ""
echo "=== gen2 → gen3 (H_00) ==="
run_h00 "$GEN2" "$GEN3" "gen2→gen3" || exit 1

echo ""
echo "=== gen3 → gen4 (H_00; M3→M4 without genNrt / --selfhost) ==="
run_h00 "$GEN3" "$GEN4" "gen3→gen4" || exit 1
chain_green=true

echo ""
echo "=== reference: gen3_direct via bootstrap (no --selfhost) ==="
rm -f "$GEN3_DIRECT"
"$YOYO" bootstrap --target=linux "$INPUT_TYB" "$GEN3_DIRECT"
g3d_sz="$(file_size "$GEN3_DIRECT" || true)"
if [[ ! -f "$GEN3_DIRECT" ]] || [[ "$g3d_sz" -le 0 ]]; then
  echo "Stage 10-B: RED (gen3_direct bootstrap failed)"
  exit 1
fi
echo "gen3_direct: ${g3d_sz} bytes"

echo ""
echo "=== trust chain: gen4 vs gen3_direct (section-ddc / full ELF) ==="
set +e
diff_out="$("$YOYO" diff "$GEN4" "$GEN3_DIRECT" 2>&1)"
diff_ec=$?
set -e
echo "$diff_out"
if [[ "$diff_ec" -eq 0 ]]; then
  parity_equal=true
  trust_sha=$(sha256sum "$GEN4" | awk '{print substr($1,1,8)}')
  echo "gen4 ≡ gen3_direct: EQUAL (file sha256 prefix $trust_sha)"
else
  echo "gen4 ≡ gen3_direct: DIFF"
fi

echo ""
echo "=== gen12 window: gen3 vs gen4 ==="
set +e
g12_out="$("$YOYO" diff "$GEN3" "$GEN4" 2>&1)"
set -e
while IFS= read -r line; do
  echo "  $line"
  if [[ "$line" =~ ^hash_a:[[:space:]]*([0-9a-fA-F]{8}) ]]; then
    text_sha="${BASH_REMATCH[1],,}"
  fi
done <<< "$g12_out"

echo ""
echo "=== summary ==="
echo "H_00 chain gen1→gen4: $(if $chain_green; then echo GREEN; else echo RED; fi)"
echo "gen4 DDC parity:      $(if $parity_equal; then echo "EQUAL (sha256 prefix $trust_sha)"; else echo "DIFF or N/A"; fi)"
echo "bootstrap --selfhost:  NOT USED (Stage 10-B gate)"
echo "Stage 10-B:            $(if $chain_green && $parity_equal; then echo 'may check [x]'; else echo 'keep [ ]'; fi)"
echo ""
echo "Trust chain: M4 algebra completed inside H_00-patched YOYO ELFs (gen1→gen4)."
echo "  Seed = yoyo link --target=linux (H_00 extract stub + embedded .so + trampoline)"
echo "  Reference = yoyo bootstrap --target=linux WITHOUT --selfhost"
echo "  gen4 = gen3 H_00 runtime output (no genNrt / --selfhost wrapper)"
echo "Remaining host surface (honest):"
echo "  - host link/bootstrap seed + gen3_direct reference"
echo "  - embedded libyoyo_runtime.so (Rust compile) + linux_h00_tramp.elf blob"
echo "  - trampoline still uses system libdl/libc via execve"
if [[ -n "$text_sha" ]]; then
  echo "  gen4 DDC SHA256 prefix: $text_sha"
elif [[ -n "$trust_sha" ]]; then
  echo "  gen4 file SHA256 prefix: $trust_sha"
fi

# Mirror artifacts for Windows-side inspection when WORKDIR != ARTIFACT_DIR
if [[ "$WORKDIR" != "$ARTIFACT_DIR" ]]; then
  cp -f "$GEN1" "$GEN2" "$GEN3" "$GEN4" "$GEN3_DIRECT" "$ARTIFACT_DIR/" 2>/dev/null || true
fi

if $chain_green && $parity_equal; then
  exit 0
else
  exit 1
fi

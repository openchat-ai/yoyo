#!/usr/bin/env bash
# asm primitive probe — scope=primitive-probe
# Builds yoyo-asm, dumps INC/DEC/SET+GET/ADDV/ORV/JMP/CALL/NOP+RET/RET emit bytes, compares to disk golden.
# INC/DEC/SET+GET/JMP fixtures INCLUDE trailing RET (c3).
# ADDV/ORV fixtures have NO trailing RET.
# NOP+RET (G00) and standalone RET (G00 suffix) use independent encoding, no state access.
# Not an asm compiler / not 3-peer claim.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
ASM_DIR="$ROOT/yoyo-asm"
GOLD_DIR="$ROOT/yoyo/tests/golden/expected"
OUT_BIN="${TMPDIR:-/tmp}/yoyo-asm-primitives-probe.bin"

echo "scope=primitive-probe"
echo "note=INC/DEC/SET+GET/JMP include trailing RET (c3); ADDV/ORV have no RET; NOT asm peer / NOT C-ddc"
echo "skip=pure ADD(0x62) no disk golden; 0x20/0x50/0x51 skipped (D-1 JS≠Rust); SUBV no isolated disk golden"

cd "$ASM_DIR"
make clean >/dev/null
make yoyo-asm

./yoyo-asm > "$OUT_BIN"

read_expected() {
  local file="$1" line out=""
  while IFS= read -r line || [ -n "$line" ]; do out+="${line%%;*}"; done < "$file"
  printf '%s' "$out" | tr -d ' \n\r\t' | tr 'A-F' 'a-f'
}

# Disk golden (text hex, optional semicolon comments)
INC_EXPECTED=$(read_expected "$GOLD_DIR/selfhost_min_inc.code.hex")
DEC_EXPECTED=$(read_expected "$GOLD_DIR/selfhost_min_dec.code.hex")
SETGET_EXPECTED=$(read_expected "$GOLD_DIR/01_set_get.code.hex")
ADDV_EXPECTED=$(read_expected "$GOLD_DIR/02_addv_orv.addv.hex")
ORV_EXPECTED=$(read_expected "$GOLD_DIR/02_addv_orv.orv.hex")
JMP_EXPECTED=$(read_expected "$GOLD_DIR/selfhost_min_jmp.code.hex")
CALL_EXPECTED=$(read_expected "$GOLD_DIR/selfhost_min_call.code.hex")
CALLRET_EXPECTED=$(read_expected "$GOLD_DIR/04_call_ret.code.hex")
NOPRET_EXPECTED=$(read_expected "$GOLD_DIR/00_nop_ret.code.hex")
RET_EXPECTED="${NOPRET_EXPECTED: -2}"
LDB_EXPECTED=$(read_expected "$GOLD_DIR/selfhost_min_ldb.code.hex")
LDB_OFF8_EXPECTED=$(read_expected "$GOLD_DIR/selfhost_min_ldb_off8.code.hex")
LDB_OFF127_EXPECTED=$(read_expected "$GOLD_DIR/selfhost_min_ldb_off127.code.hex")
LDB_OFFM128_EXPECTED=$(read_expected "$GOLD_DIR/selfhost_min_ldb_offm128.code.hex")
LDB_OFF128_EXPECTED=$(read_expected "$GOLD_DIR/selfhost_min_ldb_off128.code.hex")
LDB_OFF256_EXPECTED=$(read_expected "$GOLD_DIR/selfhost_min_ldb_off256.code.hex")
LDB_OFFM129_EXPECTED=$(read_expected "$GOLD_DIR/selfhost_min_ldb_offm129.code.hex")

# Layout in stdout blob (see yoyo-asm.s comments)
# INC    @0   len18
# DEC    @18  len18
# SETGET @36  len32
# ADDV   @68  len24
# ORV    @92  len24
# JMP    @116 len24
# CALL   @140 len24
# CALLRET @164 len24 (forward CALL+RET compound)
# NOPRET @188 len2
# RET    @190 len1
# LDB    @191 len38 (full 3-handler fixture: SET S[50]=0 + RET + LDB stream + RET + RET)
# LDB-off8 @229 len42 (same fixture with add rax,imm8 8)
# LDB-off127 @271 len42 (imm8 RIGHT-edge: 48 83 c0 7f; offset 127 is largest signed imm8)
# LDB-offm128 @313 len42 (imm8 LEFT-edge: 48 83 c0 80; offset -128 is smallest signed imm8)
# LDB-off128 @355 len45 (imm32 LEFT-edge: 48 81 c0 80 00 00 00; offset 128 first past signed imm8)
# LDB-off256 @400 len45 (same fixture with add rax,imm32 0x100)
# LDB-offm129 @445 len45 (imm32 LEFT-edge negative side: 48 81 c0 7f ff ff ff; offset -129 first past signed imm8)
INC_GOT=$(xxd -p -s 0 -l 18 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
DEC_GOT=$(xxd -p -s 18 -l 18 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
SETGET_GOT=$(xxd -p -s 36 -l 32 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
ADDV_GOT=$(xxd -p -s 68 -l 24 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
ORV_GOT=$(xxd -p -s 92 -l 24 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
JMP_GOT=$(xxd -p -s 116 -l 24 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
CALL_GOT=$(xxd -p -s 140 -l 24 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
CALLRET_GOT=$(xxd -p -s 164 -l 24 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
NOPRET_GOT=$(xxd -p -s 188 -l 2 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
RET_GOT=$(xxd -p -s 190 -l 1 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
LDB_GOT=$(xxd -p -s 191 -l 38 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
LDB_OFF8_GOT=$(xxd -p -s 229 -l 42 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
LDB_OFF127_GOT=$(xxd -p -s 271 -l 42 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
LDB_OFFM128_GOT=$(xxd -p -s 313 -l 42 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
LDB_OFF128_GOT=$(xxd -p -s 355 -l 45 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
LDB_OFF256_GOT=$(xxd -p -s 400 -l 45 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')
LDB_OFFM129_GOT=$(xxd -p -s 445 -l 45 "$OUT_BIN" | tr -d '\n' | tr 'A-F' 'a-f')

sha_hex() {
  local h="$1"
  printf '%s' "$h" | xxd -r -p | sha256sum | awk '{print $1}'
}

first_diff() {
  local a="$1" b="$2"
  local i=0
  local n=${#a}
  local m=${#b}
  local lim=$n
  [ "$m" -lt "$lim" ] && lim=$m
  while [ $i -lt $lim ]; do
    if [ "${a:$i:2}" != "${b:$i:2}" ]; then
      echo $((i / 2))
      return
    fi
    i=$((i + 2))
  done
  if [ "$n" -ne "$m" ]; then
    echo $((lim / 2))
  else
    echo -1
  fi
}

report_one() {
  local name="$1" got="$2" exp="$3" note="${4:-}"
  local glen=$(( ${#got} / 2 ))
  local elen=$(( ${#exp} / 2 ))
  local gsha esha diff ok
  gsha=$(sha_hex "$got")
  esha=$(sha_hex "$exp")
  diff=$(first_diff "$got" "$exp")
  if [ "$got" = "$exp" ]; then ok=PASS; else ok=FAIL; fi
  echo "--- $name ---"
  echo "  status=$ok"
  [ -n "$note" ] && echo "  note=$note"
  echo "  got_len=$glen exp_len=$elen"
  echo "  got_sha256=$gsha"
  echo "  exp_sha256=$esha"
  echo "  first_diff_offset=$diff"
  echo "  got=$got"
  echo "  exp=$exp"
  [ "$ok" = "PASS" ]
}

PASS=0
TOTAL=17
if report_one "INC" "$INC_GOT" "$INC_EXPECTED" "with trailing c3"; then PASS=$((PASS + 1)); fi
if report_one "DEC" "$DEC_GOT" "$DEC_EXPECTED" "with trailing c3"; then PASS=$((PASS + 1)); fi
if report_one "SET+GET" "$SETGET_GOT" "$SETGET_EXPECTED" "G01 with trailing c3"; then PASS=$((PASS + 1)); fi
if report_one "ADDV" "$ADDV_GOT" "$ADDV_EXPECTED" "G02 addv, no trailing c3"; then PASS=$((PASS + 1)); fi
if report_one "ORV" "$ORV_GOT" "$ORV_EXPECTED" "G02 orv, no trailing c3"; then PASS=$((PASS + 1)); fi
if report_one "JMP" "$JMP_GOT" "$JMP_EXPECTED" "G-SM-JMP SET0+RET+JMP+RET; rel32=-23"; then PASS=$((PASS + 1)); fi
if report_one "CALL" "$CALL_GOT" "$CALL_EXPECTED" "G-SM-CALL SET0+RET+CALL+RET; rel32=-23; byte18=E8 (vs JMP E9)"; then PASS=$((PASS + 1)); fi
if report_one "CALL+RET" "$CALLRET_GOT" "$CALLRET_EXPECTED" "G04 forward CALL+RET; head=e8 01 00 00 00 c3; trailing callee c3"; then PASS=$((PASS + 1)); fi
if report_one "NOP+RET" "$NOPRET_GOT" "$NOPRET_EXPECTED" "G00 independent encoding; 90 c3"; then PASS=$((PASS + 1)); fi
if report_one "RET" "$RET_GOT" "$RET_EXPECTED" "standalone independent encoding; expected from G00 disk golden suffix"; then PASS=$((PASS + 1)); fi
if report_one "LDB" "$LDB_GOT" "$LDB_EXPECTED" "G-SM-LDB 3-handler fixture: SET S[50]=0 + RET + LDB S[50]<-zx(byte[mem[S[60]+0]]) + RET + RET; 38B compound (Part 4S.3)"; then PASS=$((PASS + 1)); fi
if report_one "LDB-off8" "$LDB_OFF8_GOT" "$LDB_OFF8_EXPECTED" "compile-only LDB offset=8: load + add rax,imm8 8 + movzx [rax] + store; 42B compound"; then PASS=$((PASS + 1)); fi
if report_one "LDB-off127" "$LDB_OFF127_GOT" "$LDB_OFF127_EXPECTED" "compile-only LDB offset=127 (0x7F): load + add rax,imm8 0x7F + movzx [rax] + store; 42B compound; imm8 RIGHT edge"; then PASS=$((PASS + 1)); fi
if report_one "LDB-offm128" "$LDB_OFFM128_GOT" "$LDB_OFFM128_EXPECTED" "compile-only LDB offset=-128 (0x80 signed): load + add rax,imm8 0x80 + movzx [rax] + store; 42B compound; imm8 LEFT edge (48 83 c0 80)"; then PASS=$((PASS + 1)); fi
if report_one "LDB-off128" "$LDB_OFF128_GOT" "$LDB_OFF128_EXPECTED" "compile-only LDB offset=128 (0x80): load + add rax,imm32 0x80 + movzx [rax] + store; 45B compound; imm32 LEFT edge (48 81 c0 80 00 00 00)"; then PASS=$((PASS + 1)); fi
if report_one "LDB-off256" "$LDB_OFF256_GOT" "$LDB_OFF256_EXPECTED" "compile-only LDB offset=256 (0x100): load + add rax,imm32 0x100 + movzx [rax] + store; 45B compound; forces imm32 path (48 81 c0 + 4B LE) instead of imm8 path"; then PASS=$((PASS + 1)); fi
if report_one "LDB-offm129" "$LDB_OFFM129_GOT" "$LDB_OFFM129_EXPECTED" "compile-only LDB offset=-129 (0xFFFFFF7F signed): load + add rax,imm32 0xFFFFFF7F + movzx [rax] + store; 45B compound; imm32 LEFT-edge on the negative side (48 81 c0 7f ff ff ff)"; then PASS=$((PASS + 1)); fi

echo "========"
echo "scope=primitive-probe match=$PASS/$TOTAL"
if [ "$PASS" -eq "$TOTAL" ]; then
  echo "RESULT=PASS (INC+DEC+SET+GET+ADDV+ORV+JMP+CALL+CALLRET+NOPRET+RET+LDB+LDB-off8+LDB-off127+LDB-offm128+LDB-off128+LDB-off256+LDB-offm129 bytes match disk golden; still NOT asm compiler peer)"
  exit 0
else
  echo "RESULT=FAIL"
  exit 1
fi

# body-extend-077 Log · parallel-batch-71 consolidation (H_541..H_548)

> Tag: `body-extend-077-EXPERIMENTAL-batch71-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-71-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `ebbc6d765fcc0fcd…` → `97ce84a29adb8c40…`.
> **handler count: 547 → 555** (+8 at selectors 0x223..0x22A via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED; ADD-IMM 51/52 190 deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_541 | 0x223 | 0x62 ADD-IMM | 52 188 | 22 | `ef2cfed790c9d301` |
| H_542 | 0x224 | 0x61 SUB-IMM | 50 188 | 22 | `4576822a906e44b8` |
| H_543 | 0x225 | 0x61 SUB-IMM | 51 188 | 22 | `6c36bec002d9aa7d` |
| H_544 | 0x226 | 0x61 SUB-IMM | 52 188 | 22 | `c77a089b4ef783bb` |
| H_545 | 0x227 | 0x80 LDB | 50 60 190 | 26 | `e4ad649adfa675bd` |
| H_546 | 0x228 | 0x80 LDB | 51 60 190 | 26 | `251c22877545c901` |
| H_547 | 0x229 | 0x80 LDB | 52 60 190 | 26 | `21f0254d615d4969` |
| H_548 | 0x22A | 0x62 ADD-IMM | 50 190 | 22 | `0b1729a7a8c31cb9` |

**REJECTED (not added):** none (batch-71 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 223`..`40 22A` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_541 finishes 188 ADD triad. H_542..H_544 finish 188 SUB triad. H_545..H_547 start 190 LDB triad. H_548 starts 190 ADD triad (slot=51/52 deferred).

**Deferred (not added this beat):** ADD-IMM slot=51/52 imm=190; SUB-IMM slot=50/51/52 imm=190 — suggested for parallel-batch-72.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x188/0x190 uses imm32 (`48 81 e8` / `48 81 c0`) → 22B pins; not imm8.
LDB oo=0x190 uses imm32 → 26B pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_541..H_548 at selectors 0x223..0x22A (`40 223`..`40 22A`). Not RAW_BYTE; mirrors H_533..H_540 comment style (body-extend-077 / parallel-batch-71).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h52_188,subimm_h50_188,subimm_h51_188,subimm_h52_188,ldb_5060_190,ldb_5160_190,ldb_5260_190,addimm_h50_190}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **539/539 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **547/547 PASS**.
- Full canonical emit: JS=Rust=**12407B** code (was 12219B; +188B); byte-equal **Y**; sha `b8a29f688671bee2…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `97ce84a29adb8c40…`; previous chained to `ebbc6d765fcc0fcd…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=12800; both peers code=12407; hash_a=hash_b=`564dac5e26edcae2…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-076 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-71 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_541..H_548 at selectors 0x223..0x22A.
4. Selftest: exact pins PASS (22/22/22/22/26/26/26/22B).
5. Goldens: JS 539/539 and Rust 547/547 PASS; full emit byte-equal Y at 12407B.
6. Lock: Relock once → `97ce84a29adb8c40…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-72: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_541..H_548), writing `parallel-batch-72-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: finish ADD-IMM 51/52 190, then SUB-IMM 50/51/52 190, then LDB/ADD oo=198 ladder, SET/GET fresh, etc. After batch-72 scratches done: parent next = body-extend-078 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-72-SPAWN.md` (no Task tool on this consolidator). Deferred carry: ADD-IMM slot=51/52 imm=190; SUB-IMM 50/51/52 imm=190.

# body-extend-028 Log · parallel-batch-22 consolidation (H_150..H_157)

> Tag: `body-extend-028-EXPERIMENTAL-batch22-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-22-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `2a14beec0f08ffdf…` → `80287f8fe0a8eb09…`.
> **handler count: 156 → 164** (+8 at selectors 0x9C..0xA3).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_150 | 0x9C | 0x30 SET | 50 FEEDC0DE | 18 | `3d87228f78707f16` |
| H_151 | 0x9D | 0x62 ADD-IMM | 50 32 | 19 | `5cc13067b0ad0632` |
| H_152 | 0x9E | 0x61 SUB-IMM | 52 28 | 19 | `d336d72829e79f77` |
| H_153 | 0x9F | 0x80 LDB | 50 60 48 | 23 | `db3f030b072b721d` |
| H_154 | 0xA0 | 0x80 LDB | 51 60 48 | 23 | `3e69600006d17327` |
| H_155 | 0xA1 | 0x80 LDB | 52 60 48 | 23 | `0cfd11ffdf5be6f0` |
| H_156 | 0xA2 | 0x62 ADD-IMM | 51 32 | 19 | `344d6d45a4ba02f0` |
| H_157 | 0xA3 | 0x61 SUB-IMM | 50 28 | 19 | `533c4ac0d8d19f34` |

**REJECTED (not added):** none (batch-22 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selector `40 9C`..`40 A3` / `40 A0`/`40 A1` for H_150..H_157 are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_150..H_157 at selectors 0x9C..0xA3. Not RAW_BYTE; mirrors H_142..H_149 comment style (body-extend-028 / parallel-batch-22).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{set_50_feedc0de,addimm_h50_32,subimm_h52_28,ldb_5060_48,ldb_5160_48,ldb_5260_48,addimm_h51_32,subimm_h50_28}.ty` + `expected/*.code.hex` (hex-only; log pins).
- JS: 8 checkX in `golden.js` — **148/148 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS**.
- Rust golden: 8 `check_selfhost_min_*` — **156/156 PASS**.
- Full canonical emit: JS=Rust=**3577B** code (was 3414B; +163B); byte-equal **Y**; sha `99c25a00551aced0…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `80287f8fe0a8eb09…`; previous chained to `2a14beec0f08ffdf…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **DIFFER** (JS section RawSize=4096 vs Rust=3584 after code grew; first 3584B of `.text` equal; embedded stub code still byte-eq). Script soft-continues with Phase-1 NOTE. Not invent-green EQUAL.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted encoder modification.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-22 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; no encoder edits.
3. Hand-author: H_150..H_157 at selectors 0x9C..0xA3.
4. Selftest: exact pins PASS (18/19/19/23/23/23/19/19B).
5. Goldens: JS 148/148 and Rust 156/156 PASS; full emit byte-equal Y at 3577B.
6. Lock: Relock once → `80287f8fe0a8eb09…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` DIFFER noted; stub code byte-eq retained).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-23: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_150..H_157), writing `parallel-batch-23-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. After batch-23 scratches done: parent next = body-extend-029 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-23-SPAWN.md` (no Task tool on this consolidator).

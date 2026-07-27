# body-extend-054 Log · parallel-batch-48 consolidation (H_358..H_365)

> Tag: `body-extend-054-EXPERIMENTAL-batch48-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-48-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `86485f4822e891c4…` → `13cb91ab1e1cc24d…`.
> **handler count: 364 → 372** (+8 at selectors 0x16C..0x173 via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_358 | 0x16C | 0x61 SUB-IMM | 52 E0 | 22 | `7986c4bc9ebed8c6` |
| H_359 | 0x16D | 0x62 ADD-IMM | 50 E8 | 22 | `51760cec223058e1` |
| H_360 | 0x16E | 0x62 ADD-IMM | 51 E8 | 22 | `75755148da277056` |
| H_361 | 0x16F | 0x62 ADD-IMM | 52 E8 | 22 | `e8d397ad24fcfa8c` |
| H_362 | 0x170 | 0x61 SUB-IMM | 50 E8 | 22 | `45dace9bedbf51e3` |
| H_363 | 0x171 | 0x61 SUB-IMM | 51 E8 | 22 | `ce05fadbd17ed30f` |
| H_364 | 0x172 | 0x61 SUB-IMM | 52 E8 | 22 | `87083a564ea9a2de` |
| H_365 | 0x173 | 0x80 LDB | 50 60 F0 | 26 | `a8241e1de5be2a76` |

**REJECTED (not added):** none (batch-48 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 16C`..`40 173` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_359..H_361 ADD-IMM imm=0xE8 use imm32 (`48 81 c0 …`), pin 22B. H_358 / H_362..H_364 SUB-IMM imm=0xE0/E8 use imm32 (`48 81 e8 …`), pin 22B. H_365 LDB oo≥0x80 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_358..H_365 at selectors 0x16C..0x173 (`40 16C`..`40 173`). Not RAW_BYTE; mirrors H_350..H_357 comment style (body-extend-054 / parallel-batch-48).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h52_e0,addimm_h50_e8,addimm_h51_e8,addimm_h52_e8,subimm_h50_e8,subimm_h51_e8,subimm_h52_e8,ldb_5060_f0}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **356/356 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **364/364 PASS**.
- Full canonical emit: JS=Rust=**8133B** code (was 7953B; +180B); byte-equal **Y**; sha `1ab8c6be3434f349…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `13cb91ab1e1cc24d…`; previous chained to `86485f4822e891c4…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **DIFFER** (compared_bytes=8192; both peers code=8133). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-48 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_358..H_365 at selectors 0x16C..0x173.
4. Selftest: exact pins PASS (22/22/22/22/22/22/22/26B).
5. Goldens: JS 356/356 and Rust 364/364 PASS; full emit byte-equal Y at 8133B.
6. Lock: Relock once → `13cb91ab1e1cc24d…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` DIFFER this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-49: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_358..H_365), writing `parallel-batch-49-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: LDB 51/52 60 F0 (finish F0 triad), ADD/SUB-IMM imm=F0 triad, etc. After batch-49 scratches done: parent next = body-extend-055 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-49-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

# body-extend-055 Log · parallel-batch-49 consolidation (H_366..H_373)

> Tag: `body-extend-055-EXPERIMENTAL-batch49-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-49-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `13cb91ab1e1cc24d…` → `fba1f97e01a9ef7e…`.
> **handler count: 372 → 380** (+8 at selectors 0x174..0x17B via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_366 | 0x174 | 0x80 LDB | 51 60 F0 | 26 | `878beef94d2aaca0` |
| H_367 | 0x175 | 0x80 LDB | 52 60 F0 | 26 | `39e79a02c3bbc071` |
| H_368 | 0x176 | 0x62 ADD-IMM | 50 F0 | 22 | `cfd72ee65ddb08fc` |
| H_369 | 0x177 | 0x62 ADD-IMM | 51 F0 | 22 | `5aa3b0e69138d4d3` |
| H_370 | 0x178 | 0x62 ADD-IMM | 52 F0 | 22 | `e67473702a13c78e` |
| H_371 | 0x179 | 0x61 SUB-IMM | 50 F0 | 22 | `3404141d925462bb` |
| H_372 | 0x17A | 0x61 SUB-IMM | 51 F0 | 22 | `d52a7558bdad1d89` |
| H_373 | 0x17B | 0x61 SUB-IMM | 52 F0 | 22 | `4128c048e41cad1a` |

**REJECTED (not added):** none (batch-49 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 174`..`40 17B` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_368..H_370 ADD-IMM imm=0xF0 use imm32 (`48 81 c0 …`), pin 22B. H_371..H_373 SUB-IMM imm=0xF0 use imm32 (`48 81 e8 …`), pin 22B. H_366..H_367 LDB oo≥0x80 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_366..H_373 at selectors 0x174..0x17B (`40 174`..`40 17B`). Not RAW_BYTE; mirrors H_358..H_365 comment style (body-extend-055 / parallel-batch-49).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5160_f0,ldb_5260_f0,addimm_h50_f0,addimm_h51_f0,addimm_h52_f0,subimm_h50_f0,subimm_h51_f0,subimm_h52_f0}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **364/364 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **372/372 PASS**.
- Full canonical emit: JS=Rust=**8317B** code (was 8133B; +184B); byte-equal **Y**; sha `2053162488e41c7c…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `fba1f97e01a9ef7e…`; previous chained to `13cb91ab1e1cc24d…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=8704; both peers code=8317). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior body-extend-054 beat measured DIFFER; this beat EQUAL.)
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-49 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_366..H_373 at selectors 0x174..0x17B.
4. Selftest: exact pins PASS (26/26/22/22/22/22/22/22B).
5. Goldens: JS 364/364 and Rust 372/372 PASS; full emit byte-equal Y at 8317B.
6. Lock: Relock once → `fba1f97e01a9ef7e…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-50: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_366..H_373), writing `parallel-batch-50-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: LDB oo=F8 triad (dd=50/51/52 ss=60), ADD/SUB-IMM imm=F8 triad, etc. After batch-50 scratches done: parent next = body-extend-056 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-50-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

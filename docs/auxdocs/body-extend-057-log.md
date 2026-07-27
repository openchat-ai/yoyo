# body-extend-057 Log · parallel-batch-51 consolidation (H_382..H_389)

> Tag: `body-extend-057-EXPERIMENTAL-batch51-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-51-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `824207c608fe5d03…` → `0643c8f550fbb85d…`.
> **handler count: 388 → 396** (+8 at selectors 0x184..0x18B via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_382 | 0x184 | 0x61 SUB-IMM | 52 F8 | 22 | `69b7068d45f8bf5d` |
| H_383 | 0x185 | 0x80 LDB | 50 60 100 | 26 | `435a012fe7d4460d` |
| H_384 | 0x186 | 0x80 LDB | 51 60 100 | 26 | `efcb4fa1a01828f3` |
| H_385 | 0x187 | 0x80 LDB | 52 60 100 | 26 | `a26708edf890025c` |
| H_386 | 0x188 | 0x62 ADD-IMM | 50 100 | 22 | `220b570f6901c757` |
| H_387 | 0x189 | 0x62 ADD-IMM | 51 100 | 22 | `2bca9f9743f2fb78` |
| H_388 | 0x18A | 0x62 ADD-IMM | 52 100 | 22 | `6f99edae6e28e2a6` |
| H_389 | 0x18B | 0x61 SUB-IMM | 50 100 | 22 | `a89c3aeffbbddb04` |

**REJECTED (not added):** none (batch-51 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 184`..`40 18B` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_382 SUB-IMM imm=0xF8 / H_389 SUB-IMM imm=0x100 use imm32 (`48 81 e8 …`), pin 22B. H_386..H_388 ADD-IMM imm=0x100 use imm32 (`48 81 c0 …`), pin 22B. H_383..H_385 LDB oo=0x100 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_382..H_389 at selectors 0x184..0x18B (`40 184`..`40 18B`). Not RAW_BYTE; mirrors H_374..H_381 comment style (body-extend-057 / parallel-batch-51).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h52_f8,ldb_5060_100,ldb_5160_100,ldb_5260_100,addimm_h50_100,addimm_h51_100,addimm_h52_100,subimm_h50_100}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **380/380 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **388/388 PASS**.
- Full canonical emit: JS=Rust=**8693B** code (was 8505B; +188B); byte-equal **Y**; sha `af02c20d9c1f1385…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `0643c8f550fbb85d…`; previous chained to `824207c608fe5d03…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **DIFFER** (compared_bytes=8704; both peers code=8693; hash_a≠hash_b). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-51 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_382..H_389 at selectors 0x184..0x18B.
4. Selftest: exact pins PASS (22/26/26/26/22/22/22/22B).
5. Goldens: JS 380/380 and Rust 388/388 PASS; full emit byte-equal Y at 8693B.
6. Lock: Relock once → `0643c8f550fbb85d…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` DIFFER this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-52: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_382..H_389), writing `parallel-batch-52-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: SUB-IMM 51/52 100 (finish 100 SUB triad), ADD/SUB/LDB imm=108 triad, etc. After batch-52 scratches done: parent next = body-extend-058 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-52-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

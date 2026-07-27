# body-extend-070 Log · parallel-batch-64 consolidation (H_485..H_492)

> Tag: `body-extend-070-EXPERIMENTAL-batch64-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-64-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `f9afff3e95333709…` → `192ba67ac8bb814d…`.
> **handler count: 491 → 499** (+8 at selectors 0x1EB..0x1F2 via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_485 | 0x1EB | 0x62 ADD-IMM | 50 158 | 22 | `41094166f79d1c0b` |
| H_486 | 0x1EC | 0x62 ADD-IMM | 51 158 | 22 | `70fd4ef8381b04b2` |
| H_487 | 0x1ED | 0x62 ADD-IMM | 52 158 | 22 | `25deea9b5b4ae288` |
| H_488 | 0x1EE | 0x61 SUB-IMM | 50 158 | 22 | `401d7f68292fe70a` |
| H_489 | 0x1EF | 0x61 SUB-IMM | 51 158 | 22 | `cb9589469f12483a` |
| H_490 | 0x1F0 | 0x61 SUB-IMM | 52 158 | 22 | `8b00fcdbb741f29c` |
| H_491 | 0x1F1 | 0x80 LDB | 50 60 160 | 26 | `0de356c3d4e6b935` |
| H_492 | 0x1F2 | 0x80 LDB | 51 60 160 | 26 | `ca261b259166d021` |

**REJECTED (not added):** none (batch-64 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 1EB`..`40 1F2` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_485..H_487 start 158 ADD triad (imm32 22B). H_488..H_490 start 158 SUB triad (imm32 22B). H_491..H_492 start 160 LDB triad (imm32 26B; slot 52 deferred to next batch).

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_485..H_492 at selectors 0x1EB..0x1F2 (`40 1EB`..`40 1F2`). Not RAW_BYTE; mirrors H_477..H_484 comment style (body-extend-070 / parallel-batch-64).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h50_158,addimm_h51_158,addimm_h52_158,subimm_h50_158,subimm_h51_158,subimm_h52_158,ldb_5060_160,ldb_5160_160}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **483/483 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **491/491 PASS**.
- Full canonical emit: JS=Rust=**11099B** code (was 10915B; +184B); byte-equal **Y**; sha `35b8e18661bb5ad1…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `192ba67ac8bb814d…`; previous chained to `f9afff3e95333709…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=11264; both peers code=11099; hash_a=hash_b=`fc5f3dff3a66f5d4…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-069 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-64 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_485..H_492 at selectors 0x1EB..0x1F2.
4. Selftest: exact pins PASS (22/22/22/22/22/22/26/26B).
5. Goldens: JS 483/483 and Rust 491/491 PASS; full emit byte-equal Y at 11099B.
6. Lock: Relock once → `192ba67ac8bb814d…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-65: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_485..H_492), writing `parallel-batch-65-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: LDB 52 60 160 (finish 160 LDB triad), ADD-IMM / SUB-IMM imm=160 triad, SET/GET fresh, etc. After batch-65 scratches done: parent next = body-extend-071 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-65-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

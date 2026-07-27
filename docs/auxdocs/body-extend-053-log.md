# body-extend-053 Log · parallel-batch-47 consolidation (H_350..H_357)

> Tag: `body-extend-053-EXPERIMENTAL-batch47-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-47-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `edee584aa21a2656…` → `86485f4822e891c4…`.
> **handler count: 356 → 364** (+8 at selectors 0x164..0x16B via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_350 | 0x164 | 0x80 LDB | 50 60 E8 | 26 | `8707f42f9e69fe94` |
| H_351 | 0x165 | 0x80 LDB | 51 60 E8 | 26 | `1aa2e13843e522b5` |
| H_352 | 0x166 | 0x80 LDB | 52 60 E8 | 26 | `465cb3e854ecc953` |
| H_353 | 0x167 | 0x62 ADD-IMM | 50 E0 | 22 | `9ef1fb8eb620deee` |
| H_354 | 0x168 | 0x62 ADD-IMM | 51 E0 | 22 | `4d09c2a3e224d2d4` |
| H_355 | 0x169 | 0x62 ADD-IMM | 52 E0 | 22 | `cd251baeb9a188f0` |
| H_356 | 0x16A | 0x61 SUB-IMM | 50 E0 | 22 | `6d7c5904f21181f1` |
| H_357 | 0x16B | 0x61 SUB-IMM | 51 E0 | 22 | `345b5a0581126cf4` |

**REJECTED (not added):** none (batch-47 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 164`..`40 16B` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_353..H_355 ADD-IMM imm=0xE0 use imm32 (`48 81 c0 …`), pin 22B. H_356..H_357 SUB-IMM imm=0xE0 use imm32 (`48 81 e8 …`), pin 22B. H_350..H_352 LDB oo≥0x80 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_350..H_357 at selectors 0x164..0x16B (`40 164`..`40 16B`). Not RAW_BYTE; mirrors H_342..H_349 comment style (body-extend-053 / parallel-batch-47).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5060_e8,ldb_5160_e8,ldb_5260_e8,addimm_h50_e0,addimm_h51_e0,addimm_h52_e0,subimm_h50_e0,subimm_h51_e0}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **348/348 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **356/356 PASS**.
- Full canonical emit: JS=Rust=**7953B** code (was 7765B; +188B); byte-equal **Y**; sha `9e0e0a15df76f59c…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `86485f4822e891c4…`; previous chained to `edee584aa21a2656…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=8192; both peers code=7953). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-47 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_350..H_357 at selectors 0x164..0x16B.
4. Selftest: exact pins PASS (26/26/26/22/22/22/22/22B).
5. Goldens: JS 348/348 and Rust 356/356 PASS; full emit byte-equal Y at 7953B.
6. Lock: Relock once → `86485f4822e891c4…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-48: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_350..H_357), writing `parallel-batch-48-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: SUB-IMM 52 E0 (finish E0 triad), ADD/SUB-IMM imm=E8 triad, LDB oo=F0 triad, etc. After batch-48 scratches done: parent next = body-extend-054 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-48-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

# body-extend-052 Log · parallel-batch-46 consolidation (H_342..H_349)

> Tag: `body-extend-052-EXPERIMENTAL-batch46-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-46-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `ee5b881e34301f79…` → `edee584aa21a2656…`.
> **handler count: 348 → 356** (+8 at selectors 0x15C..0x163 via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_342 | 0x15C | 0x80 LDB | 51 60 E0 | 26 | `50f40ec03eee29c8` |
| H_343 | 0x15D | 0x80 LDB | 52 60 E0 | 26 | `4c6401f2595fc5c8` |
| H_344 | 0x15E | 0x62 ADD-IMM | 50 D8 | 22 | `3f9b979485c6551c` |
| H_345 | 0x15F | 0x62 ADD-IMM | 51 D8 | 22 | `959f55bf7e28a72e` |
| H_346 | 0x160 | 0x62 ADD-IMM | 52 D8 | 22 | `300854c0d5bd80ba` |
| H_347 | 0x161 | 0x61 SUB-IMM | 50 D8 | 22 | `82866db77dd7973c` |
| H_348 | 0x162 | 0x61 SUB-IMM | 51 D8 | 22 | `98d0142fba622c9f` |
| H_349 | 0x163 | 0x61 SUB-IMM | 52 D8 | 22 | `0cf496fbf781f92d` |

**REJECTED (not added):** none (batch-46 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 15C`..`40 163` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_344..H_346 ADD-IMM imm=0xD8 use imm32 (`48 81 c0 …`), pin 22B. H_347..H_349 SUB-IMM imm=0xD8 use imm32 (`48 81 e8 …`), pin 22B. H_342..H_343 LDB oo≥0x80 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_342..H_349 at selectors 0x15C..0x163 (`40 15C`..`40 163`). Not RAW_BYTE; mirrors H_334..H_341 comment style (body-extend-052 / parallel-batch-46).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5160_e0,ldb_5260_e0,addimm_h50_d8,addimm_h51_d8,addimm_h52_d8,subimm_h50_d8,subimm_h51_d8,subimm_h52_d8}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **340/340 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **348/348 PASS**.
- Full canonical emit: JS=Rust=**7765B** code (was 7581B; +184B); byte-equal **Y**; sha `0d5edc158e76df17…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `edee584aa21a2656…`; previous chained to `ee5b881e34301f79…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=8192; both peers code=7765). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-46 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_342..H_349 at selectors 0x15C..0x163.
4. Selftest: exact pins PASS (26/26/22/22/22/22/22/22B).
5. Goldens: JS 340/340 and Rust 348/348 PASS; full emit byte-equal Y at 7765B.
6. Lock: Relock once → `edee584aa21a2656…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-47: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_342..H_349), writing `parallel-batch-47-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: LDB oo=E8 triad (dd=50/51/52 ss=60), ADD/SUB-IMM imm=E0 triad, etc. After batch-47 scratches done: parent next = body-extend-053 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-47-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

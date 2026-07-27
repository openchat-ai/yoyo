# body-extend-049 Log · parallel-batch-43 consolidation (H_318..H_325)

> Tag: `body-extend-049-EXPERIMENTAL-batch43-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-43-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `9c2f924a2780d646…` → `69adc5a0b11c8f17…`.
> **handler count: 324 → 332** (+8 at selectors 0x144..0x14B via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_318 | 0x144 | 0x80 LDB | 51 60 D0 | 26 | `2d00172cf7198885` |
| H_319 | 0x145 | 0x80 LDB | 52 60 D0 | 26 | `e5577873d59f39b9` |
| H_320 | 0x146 | 0x62 ADD-IMM | 50 C0 | 22 | `14116ca20ac2ff30` |
| H_321 | 0x147 | 0x62 ADD-IMM | 51 C0 | 22 | `781fd0dd879b7d37` |
| H_322 | 0x148 | 0x62 ADD-IMM | 52 C0 | 22 | `187eebc8371ba7f5` |
| H_323 | 0x149 | 0x61 SUB-IMM | 50 C0 | 22 | `90c51fcf3eb0e0bb` |
| H_324 | 0x14A | 0x61 SUB-IMM | 51 C0 | 22 | `3c16c50a8e776b8a` |
| H_325 | 0x14B | 0x61 SUB-IMM | 52 C0 | 22 | `5bfec4655978ffd2` |

**REJECTED (not added):** none (batch-43 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 144`..`40 14B` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_320..H_322 ADD-IMM imm=0xC0 use imm32 (`48 81 c0 …`), pin 22B. H_323..H_325 SUB-IMM imm=0xC0 use imm32 (`48 81 e8 …`), pin 22B. H_318..H_319 LDB oo≥0x80 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_318..H_325 at selectors 0x144..0x14B (`40 144`..`40 14B`). Not RAW_BYTE; mirrors H_310..H_317 comment style (body-extend-049 / parallel-batch-43).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5160_d0,ldb_5260_d0,addimm_h50_c0,addimm_h51_c0,addimm_h52_c0,subimm_h50_c0,subimm_h51_c0,subimm_h52_c0}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **316/316 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **324/324 PASS**.
- Full canonical emit: JS=Rust=**7213B** code (was 7029B; +184B); byte-equal **Y**; sha `d5c98754a5ec2737…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `69adc5a0b11c8f17…`; previous chained to `9c2f924a2780d646…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=7680; both peers code=7213). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-43 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_318..H_325 at selectors 0x144..0x14B.
4. Selftest: exact pins PASS (26/26/22/22/22/22/22/22B).
5. Goldens: JS 316/316 and Rust 324/324 PASS; full emit byte-equal Y at 7213B.
6. Lock: Relock once → `69adc5a0b11c8f17…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-44: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_318..H_325), writing `parallel-batch-44-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: LDB oo=D8 triad (dd=50/51/52 ss=60), ADD/SUB-IMM imm=C8 triad, etc. After batch-44 scratches done: parent next = body-extend-050 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-44-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

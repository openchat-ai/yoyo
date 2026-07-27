# body-extend-047 Log · parallel-batch-41 consolidation (H_302..H_309)

> Tag: `body-extend-047-EXPERIMENTAL-batch41-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-41-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `422c843275989ac3…` → `000042c8ea316c07…`.
> **handler count: 308 → 316** (+8 at selectors 0x134..0x13B via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_302 | 0x134 | 0x80 LDB | 50 60 C8 | 26 | `236016ef799b3ff7` |
| H_303 | 0x135 | 0x80 LDB | 51 60 C8 | 26 | `7eb39f3637eb2267` |
| H_304 | 0x136 | 0x80 LDB | 52 60 C8 | 26 | `b9fa804bcc69d95c` |
| H_305 | 0x137 | 0x62 ADD-IMM | 50 B0 | 22 | `9be2c80577bd6f4a` |
| H_306 | 0x138 | 0x62 ADD-IMM | 51 B0 | 22 | `e3c08eecc6fae6f3` |
| H_307 | 0x139 | 0x62 ADD-IMM | 52 B0 | 22 | `9d760ed911115fb1` |
| H_308 | 0x13A | 0x61 SUB-IMM | 50 B0 | 22 | `1d73d8c916bc7e20` |
| H_309 | 0x13B | 0x61 SUB-IMM | 51 B0 | 22 | `3e629652dbf4e5ea` |

**REJECTED (not added):** none (batch-41 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 134`..`40 13B` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_305..H_307 ADD-IMM imm=0xB0 use imm32 (`48 81 c0 …`), pin 22B. H_308..H_309 SUB-IMM imm=0xB0 use imm32 (`48 81 e8 …`), pin 22B. H_302..H_304 LDB oo≥0x80 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_302..H_309 at selectors 0x134..0x13B (`40 134`..`40 13B`). Not RAW_BYTE; mirrors H_294..H_301 comment style (body-extend-047 / parallel-batch-41).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5060_c8,ldb_5160_c8,ldb_5260_c8,addimm_h50_b0,addimm_h51_b0,addimm_h52_b0,subimm_h50_b0,subimm_h51_b0}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **300/300 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **308/308 PASS**.
- Full canonical emit: JS=Rust=**6849B** code (was 6661B; +188B); byte-equal **Y**; sha `fd9d714828d413eb…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `000042c8ea316c07…`; previous chained to `422c843275989ac3…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=7168; both peers code=6849). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-41 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_302..H_309 at selectors 0x134..0x13B.
4. Selftest: exact pins PASS (26/26/26/22/22/22/22/22B).
5. Goldens: JS 300/300 and Rust 308/308 PASS; full emit byte-equal Y at 6849B.
6. Lock: Relock once → `000042c8ea316c07…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-42: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_302..H_309), writing `parallel-batch-42-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: SUB-IMM 52 B0 (finish B0 triad), ADD/SUB-IMM imm=B8 triad, LDB oo=D0 triad, etc. After batch-42 scratches done: parent next = body-extend-048 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-42-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

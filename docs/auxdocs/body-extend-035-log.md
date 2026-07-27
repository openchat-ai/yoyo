# body-extend-035 Log · parallel-batch-29 consolidation (H_206..H_213)

> Tag: `body-extend-035-EXPERIMENTAL-batch29-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-29-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `e531a0a8962e21ec…` → `23f42236c6097a13…`.
> **handler count: 212 → 220** (+8 at selectors 0xD4..0xDB).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_206 | 0xD4 | 0x62 ADD-IMM | 51 58 | 19 | `4ff049a8441518ba` |
| H_207 | 0xD5 | 0x62 ADD-IMM | 52 58 | 19 | `44445f68d85c340f` |
| H_208 | 0xD6 | 0x61 SUB-IMM | 50 50 | 19 | `e51df228ac034429` |
| H_209 | 0xD7 | 0x61 SUB-IMM | 52 50 | 19 | `1b61da415449f276` |
| H_210 | 0xD8 | 0x80 LDB | 52 60 78 | 23 | `f7221a4afaec1410` |
| H_211 | 0xD9 | 0x30 SET | 52 C0DEC0DE | 18 | `20960f8da0f70a8e` |
| H_212 | 0xDA | 0x62 ADD-IMM | 50 60 | 19 | `88c5f7c3de52c972` |
| H_213 | 0xDB | 0x80 LDB | 50 60 80 | 26 | `5c4e0e3a942cbe06` |

**REJECTED (not added):** none (batch-29 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selector `40 D4`..`40 DB` for H_206..H_213 are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_213 LDB oo=0x80 uses imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_206..H_213 at selectors 0xD4..0xDB. Not RAW_BYTE; mirrors H_198..H_205 comment style (body-extend-035 / parallel-batch-29).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h51_58,addimm_h52_58,subimm_h50_50,subimm_h52_50,ldb_5260_78,set_52_c0dec0de,addimm_h50_60,ldb_5060_80}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **204/204 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **212/212 PASS**.
- Full canonical emit: JS=Rust=**4699B** code (was 4537B; +162B); byte-equal **Y**; sha `68af920c31c18620…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `23f42236c6097a13…`; previous chained to `e531a0a8962e21ec…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=5120; both peers code=4699). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted encoder modification.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-29 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; no encoder edits.
3. Hand-author: H_206..H_213 at selectors 0xD4..0xDB.
4. Selftest: exact pins PASS (19/19/19/19/23/18/19/26B).
5. Goldens: JS 204/204 and Rust 212/212 PASS; full emit byte-equal Y at 4699B.
6. Lock: Relock once → `23f42236c6097a13…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-30: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_206..H_213), writing `parallel-batch-30-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. After batch-30 scratches done: parent next = body-extend-036 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-30-SPAWN.md` (no Task tool on this consolidator).

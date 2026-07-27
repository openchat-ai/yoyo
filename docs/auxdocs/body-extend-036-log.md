# body-extend-036 Log · parallel-batch-30 consolidation (H_214..H_221)

> Tag: `body-extend-036-EXPERIMENTAL-batch30-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-30-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `23f42236c6097a13…` → `3bf549a652a2746e…`.
> **handler count: 220 → 228** (+8 at selectors 0xDC..0xE3).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_214 | 0xDC | 0x62 ADD-IMM | 51 60 | 19 | `2e762fb2ad102e6a` |
| H_215 | 0xDD | 0x62 ADD-IMM | 52 60 | 19 | `efec3943630fb998` |
| H_216 | 0xDE | 0x61 SUB-IMM | 50 58 | 19 | `a7d41e13060d56b7` |
| H_217 | 0xDF | 0x61 SUB-IMM | 51 58 | 19 | `d9559da92e31429b` |
| H_218 | 0xE0 | 0x80 LDB | 51 60 80 | 26 | `f39364a89ec6b361` |
| H_219 | 0xE1 | 0x80 LDB | 52 60 80 | 26 | `d239426ce0456ebf` |
| H_220 | 0xE2 | 0x61 SUB-IMM | 52 58 | 19 | `155b83f538845515` |
| H_221 | 0xE3 | 0x62 ADD-IMM | 50 68 | 19 | `8390493232f90387` |

**REJECTED (not added):** none (batch-30 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selector `40 DC`..`40 E3` for H_214..H_221 are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_218/H_219 LDB oo=0x80 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_214..H_221 at selectors 0xDC..0xE3. Not RAW_BYTE; mirrors H_206..H_213 comment style (body-extend-036 / parallel-batch-30).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h51_60,addimm_h52_60,subimm_h50_58,subimm_h51_58,ldb_5160_80,ldb_5260_80,subimm_h52_58,addimm_h50_68}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **212/212 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **220/220 PASS**.
- Full canonical emit: JS=Rust=**4865B** code (was 4699B; +166B); byte-equal **Y**; sha `7344f1d9c8c87d4a…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `3bf549a652a2746e…`; previous chained to `23f42236c6097a13…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=5120; both peers code=4865). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted encoder modification.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-30 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; no encoder edits.
3. Hand-author: H_214..H_221 at selectors 0xDC..0xE3.
4. Selftest: exact pins PASS (19/19/19/19/26/26/19/19B).
5. Goldens: JS 212/212 and Rust 220/220 PASS; full emit byte-equal Y at 4865B.
6. Lock: Relock once → `3bf549a652a2746e…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-31: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_214..H_221), writing `parallel-batch-31-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. After batch-31 scratches done: parent next = body-extend-037 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-31-SPAWN.md` (no Task tool on this consolidator).

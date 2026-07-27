# body-extend-037 Log · parallel-batch-31 consolidation (H_222..H_229)

> Tag: `body-extend-037-EXPERIMENTAL-batch31-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-31-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `3bf549a652a2746e…` → `39d454a194359d1d…`.
> **handler count: 228 → 236** (+8 at selectors 0xE4..0xEB).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_222 | 0xE4 | 0x62 ADD-IMM | 51 68 | 19 | `b2f72feaae60803e` |
| H_223 | 0xE5 | 0x62 ADD-IMM | 52 68 | 19 | `7819936ee9d0c007` |
| H_224 | 0xE6 | 0x61 SUB-IMM | 50 60 | 19 | `140f19aded02db3b` |
| H_225 | 0xE7 | 0x61 SUB-IMM | 51 60 | 19 | `17f59cbf3cc854a2` |
| H_226 | 0xE8 | 0x61 SUB-IMM | 52 60 | 19 | `af095c6f5e0afc0b` |
| H_227 | 0xE9 | 0x80 LDB | 50 60 88 | 26 | `5edbd7f24b9a903a` |
| H_228 | 0xEA | 0x80 LDB | 51 60 88 | 26 | `3bee10754f19b9d5` |
| H_229 | 0xEB | 0x80 LDB | 52 60 88 | 26 | `74c53973c0c6f552` |

**REJECTED (not added):** none (batch-31 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selector `40 E4`..`40 EB` for H_222..H_229 are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_227/H_228/H_229 LDB oo=0x88 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_222..H_229 at selectors 0xE4..0xEB. Not RAW_BYTE; mirrors H_214..H_221 comment style (body-extend-037 / parallel-batch-31).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h51_68,addimm_h52_68,subimm_h50_60,subimm_h51_60,subimm_h52_60,ldb_5060_88,ldb_5160_88,ldb_5260_88}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **220/220 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **228/228 PASS**.
- Full canonical emit: JS=Rust=**5038B** code (was 4865B; +173B); byte-equal **Y**; sha `e1a0db4635d8af21…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `39d454a194359d1d…`; previous chained to `3bf549a652a2746e…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=5120; both peers code=5038). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted encoder modification.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-31 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; no encoder edits.
3. Hand-author: H_222..H_229 at selectors 0xE4..0xEB.
4. Selftest: exact pins PASS (19/19/19/19/19/26/26/26B).
5. Goldens: JS 220/220 and Rust 228/228 PASS; full emit byte-equal Y at 5038B.
6. Lock: Relock once → `39d454a194359d1d…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-32: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_222..H_229), writing `parallel-batch-32-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. After batch-32 scratches done: parent next = body-extend-038 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-32-SPAWN.md` (no Task tool on this consolidator).

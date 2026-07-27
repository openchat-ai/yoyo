# body-extend-038 Log · parallel-batch-32 consolidation (H_230..H_237)

> Tag: `body-extend-038-EXPERIMENTAL-batch32-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-32-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `39d454a194359d1d…` → `aa95228f49b6131c…`.
> **handler count: 236 → 244** (+8 at selectors 0xEC..0xF3).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_230 | 0xEC | 0x62 ADD-IMM | 50 70 | 19 | `fd00d3aaf8d154fd` |
| H_231 | 0xED | 0x62 ADD-IMM | 51 70 | 19 | `0b9f43d82535758d` |
| H_232 | 0xEE | 0x62 ADD-IMM | 52 70 | 19 | `a84dbb9e54bc5205` |
| H_233 | 0xEF | 0x61 SUB-IMM | 50 68 | 19 | `310e437ef9fb3edd` |
| H_234 | 0xF0 | 0x61 SUB-IMM | 51 68 | 19 | `9bb82476b37c5941` |
| H_235 | 0xF1 | 0x61 SUB-IMM | 52 68 | 19 | `0f9edc3307cfe318` |
| H_236 | 0xF2 | 0x80 LDB | 50 60 90 | 26 | `19191871913c0878` |
| H_237 | 0xF3 | 0x80 LDB | 51 60 90 | 26 | `7571ee40b3a097be` |

**REJECTED (not added):** none (batch-32 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selector `40 EC`..`40 F3` for H_230..H_237 are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_236/H_237 LDB oo=0x90 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_230..H_237 at selectors 0xEC..0xF3. Not RAW_BYTE; mirrors H_222..H_229 comment style (body-extend-038 / parallel-batch-32).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h50_70,addimm_h51_70,addimm_h52_70,subimm_h50_68,subimm_h51_68,subimm_h52_68,ldb_5060_90,ldb_5160_90}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **228/228 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **236/236 PASS**.
- Full canonical emit: JS=Rust=**5204B** code (was 5038B; +166B); byte-equal **Y**; sha `59b09a02947601f2…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `aa95228f49b6131c…`; previous chained to `39d454a194359d1d…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=5632; both peers code=5204). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted encoder modification.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-32 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; no encoder edits.
3. Hand-author: H_230..H_237 at selectors 0xEC..0xF3.
4. Selftest: exact pins PASS (19/19/19/19/19/19/26/26B).
5. Goldens: JS 228/228 and Rust 236/236 PASS; full emit byte-equal Y at 5204B.
6. Lock: Relock once → `aa95228f49b6131c…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-33: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_230..H_237), writing `parallel-batch-33-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. After batch-33 scratches done: parent next = body-extend-039 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-33-SPAWN.md` (no Task tool on this consolidator).

# body-extend-041 Log · parallel-batch-35 consolidation (H_254..H_261)

> Tag: `body-extend-041-EXPERIMENTAL-batch35-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-35-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `a58ead289233c42b…` → `4cb656812b03c0fd…`.
> **handler count: 260 → 268** (+8 at selectors 0x104..0x10B via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_254 | 0x104 | 0x80 LDB | 50 60 A0 | 26 | `4817b8ddf9b52566` |
| H_255 | 0x105 | 0x80 LDB | 51 60 A0 | 26 | `fcf0ba5ffb072ffa` |
| H_256 | 0x106 | 0x80 LDB | 52 60 A0 | 26 | `c6dd95a8ede6bf6a` |
| H_257 | 0x107 | 0x61 SUB-IMM | 50 80 | 22 | `e0304eea69eed143` |
| H_258 | 0x108 | 0x61 SUB-IMM | 51 80 | 22 | `f76a1690a99750ff` |
| H_259 | 0x109 | 0x61 SUB-IMM | 52 80 | 22 | `d26957f7354c5ec6` |
| H_260 | 0x10A | 0x62 ADD-IMM | 50 88 | 22 | `eabc3ae46677427e` |
| H_261 | 0x10B | 0x62 ADD-IMM | 51 88 | 22 | `4c60d97a9ae2744d` |

**REJECTED (not added):** none (batch-35 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 104`..`40 10B` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_254..H_256 LDB oo≥0x80 use imm32 path (`48 81 c0 …`), pin 26B. H_257..H_259 SUB-IMM imm=0x80 use imm32 (`48 81 e8 …`), pin 22B. H_260..H_261 ADD-IMM imm=0x88 use imm32 (`48 81 c0 …`), pin 22B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_254..H_261 at selectors 0x104..0x10B (`40 104`..`40 10B`). Not RAW_BYTE; mirrors H_246..H_253 comment style (body-extend-041 / parallel-batch-35).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5060_a0,ldb_5160_a0,ldb_5260_a0,subimm_h50_80,subimm_h51_80,subimm_h52_80,addimm_h50_88,addimm_h51_88}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **252/252 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **260/260 PASS**.
- Full canonical emit: JS=Rust=**5733B** code (was 5545B; +188B); byte-equal **Y**; sha `bdc3a70e9905f568…`.
- Probe peer sync: `scripts/_probe/js-ty2text.mjs` still had `&0xff` label mask (regression vs label-width A) — widened to match `yoyo.js`/`golden.js` before Relock (fail-closed on first DIFFER).
- Lock: `verify-yoyo-ty.mjs` PASS at `4cb656812b03c0fd…`; previous chained to `a58ead289233c42b…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=6144; both peers code=5733). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched beyond probe sync.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-35 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_254..H_261 at selectors 0x104..0x10B.
4. Selftest: exact pins PASS (26/26/26/22/22/22/22/22B).
5. Goldens: JS 252/252 and Rust 260/260 PASS; full emit byte-equal Y at 5733B.
6. Lock: Relock once → `4cb656812b03c0fd…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-36: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_254..H_261), writing `parallel-batch-36-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: ADD-IMM 52 88, SUB-IMM * 88 triad, LDB oo=A8 triad, etc. After batch-36 scratches done: parent next = body-extend-042 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-36-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

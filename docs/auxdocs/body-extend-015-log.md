# body-extend-015 Log · parallel-batch-09 consolidation (H_48..H_53)

> Tag: `body-extend-015-EXPERIMENTAL-batch09-consolidation-6` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-09-log.md` (6 PASS / 2 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `21a81fe1c8b52875…` → `34d2cbb03bf56dd4…`.
> **handler count: 54 → 60** (+6 at selectors 0x36..0x3B). Also locks prior H_47.

## 1. Consolidated picks (REJECTED excluded)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_48 | 0x36 | 0x68 ADDV | 51 50 | 25 | `be34bba91158e7fa` |
| H_49 | 0x37 | 0x69 ORV | 51 50 | 25 | `1bc8b9f481904979` |
| H_50 | 0x38 | 0x6A SUBV | 51 50 | 25 | `3f21e0104205701f` |
| H_51 | 0x39 | 0x60 GET | 51 52 | 15 | `7fb64e0e46f94159` |
| H_52 | 0x3A | 0x68 ADDV | 52 51 | 25 | `22a752f4fe9967b7` |
| H_53 | 0x3B | 0x30 SET | 52 CAFEBABE | 18 | `ed70b867469e0e31` |

**REJECTED (not added):** 0x84 MEMCPY_DATA, 0x85 MEMCPY_STATE — peer arity divergence; out of scope.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty:759-809` (H_48..H_53). Not RAW_BYTE; mirrors H_31/H_35/GET/SET templates.
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addv_swap,orv_swap,subv_swap,get_alt,addv_h52,set_large}.ty` + `expected/*.code.hex` (hex-only; log pins; scratch `set_large` 19B typo discarded — log 18B used).
- JS: 6 checkX in `golden.js` — **44/44 PASS**.
- Rust self_test: 6 `*_slot_check` — **PASS**.
- Rust golden: 6 `check_selfhost_min_*` — **51/51 PASS**.
- Full canonical emit: JS=Rust=**1452B** code; byte-equal **Y**; sha `a1c4ad8a3636c8f0…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `34d2cbb03bf56dd4…`; previous chained to `21a81fe1c8b52875…`.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted encoder modification.

## 3. Lock protocol (8 steps)

1. Pick: 6 PASS from parallel-batch-09 (slot/imm variants).
2. Encoder: existing JS/Rust paths retained; no encoder edits.
3. Hand-author: H_48..H_53 at selectors 0x36..0x3B.
4. Selftest: exact pins PASS (25/25/25/15/25/18B).
5. Goldens: JS 44/44 and Rust 51/51 PASS; full emit byte-equal Y at 1452B.
6. Lock: Relock once → `34d2cbb03bf56dd4…`.
7. DDC: `verify-selfhost.ps1` (run after Relock).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-10: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty), writing `parallel-batch-10-log.md`. MEMCPY 0x84/0x85 remain out of scope without peer-source edits.

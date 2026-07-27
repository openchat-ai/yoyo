# body-extend-029 Log · parallel-batch-23 consolidation (H_158..H_165)

> Tag: `body-extend-029-EXPERIMENTAL-batch23-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-23-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `80287f8fe0a8eb09…` → `1dd8234623853194…`.
> **handler count: 164 → 172** (+8 at selectors 0xA4..0xAB).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_158 | 0xA4 | 0x80 LDB | 51 60 50 | 23 | `9bdf8a7966f533c0` |
| H_159 | 0xA5 | 0x80 LDB | 52 60 50 | 23 | `f20e9d7238f08a4a` |
| H_160 | 0xA6 | 0x30 SET | 51 CAFEF00D | 18 | `72c89add1c031d37` |
| H_161 | 0xA7 | 0x62 ADD-IMM | 52 32 | 19 | `b1a04638a88d7ace` |
| H_162 | 0xA8 | 0x61 SUB-IMM | 51 32 | 19 | `207c87cf78c25007` |
| H_163 | 0xA9 | 0x30 SET | 50 CAFEF00D | 18 | `a7ecea443fabe02e` |
| H_164 | 0xAA | 0x61 SUB-IMM | 52 32 | 19 | `bc35f4068daa6365` |
| H_165 | 0xAB | 0x62 ADD-IMM | 50 3C | 19 | `6e63785554e168e2` |

**REJECTED (not added):** none (batch-23 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selector `40 A4`..`40 AB` for H_158..H_165 are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted.

**H_163 pin note:** SPAWN.md table row briefly showed wrong store disp `88` (slot-51); trusted batch-23 log + `_scratch_set_50_cafef00d.code.hex` pin `…80020000c3` (slot-50) / sha `a7ecea44…`.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_158..H_165 at selectors 0xA4..0xAB. Not RAW_BYTE; mirrors H_150..H_157 comment style (body-extend-029 / parallel-batch-23).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5160_50,ldb_5260_50,set_51_cafef00d,addimm_h52_32,subimm_h51_32,set_50_cafef00d,subimm_h52_32,addimm_h50_3c}.ty` + `expected/*.code.hex` (hex-only; log pins).
- JS: 8 checkX in `golden.js` — **156/156 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS**.
- Rust golden: 8 `check_selfhost_min_*` — **164/164 PASS**.
- Full canonical emit: JS=Rust=**3735B** code (was 3577B; +158B); byte-equal **Y**; sha `90a5d081e50139b9…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `1dd8234623853194…`; previous chained to `80287f8fe0a8eb09…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=4096; both peers code=3735). Not invent-green claim beyond measured EQUAL.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted encoder modification.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-23 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; no encoder edits.
3. Hand-author: H_158..H_165 at selectors 0xA4..0xAB.
4. Selftest: exact pins PASS (23/23/18/19/19/18/19/19B).
5. Goldens: JS 156/156 and Rust 164/164 PASS; full emit byte-equal Y at 3735B.
6. Lock: Relock once → `1dd8234623853194…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-24: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_158..H_165), writing `parallel-batch-24-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. After batch-24 scratches done: parent next = body-extend-030 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-24-SPAWN.md` (no Task tool on this consolidator).

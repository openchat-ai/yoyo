# body-extend-071 Log · parallel-batch-65 consolidation (H_493..H_500)

> Tag: `body-extend-071-EXPERIMENTAL-batch65-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-65-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `192ba67ac8bb814d…` → `1f070530a91ca949…`.
> **handler count: 499 → 507** (+8 at selectors 0x1F3..0x1FA via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_493 | 0x1F3 | 0x80 LDB | 52 60 160 | 26 | `9daf84e1a128dac3` |
| H_494 | 0x1F4 | 0x62 ADD-IMM | 50 160 | 22 | `3b8d32f8073e00b9` |
| H_495 | 0x1F5 | 0x62 ADD-IMM | 51 160 | 22 | `be65ff093c4ef72d` |
| H_496 | 0x1F6 | 0x62 ADD-IMM | 52 160 | 22 | `8eae86a7c8b26fc7` |
| H_497 | 0x1F7 | 0x61 SUB-IMM | 50 160 | 22 | `cb0f44be7ee7be5e` |
| H_498 | 0x1F8 | 0x61 SUB-IMM | 51 160 | 22 | `ce408999f0330ce3` |
| H_499 | 0x1F9 | 0x61 SUB-IMM | 52 160 | 22 | `17997181ac08f1e4` |
| H_500 | 0x1FA | 0x80 LDB | 50 60 168 | 26 | `c6ea0ffbc5102366` |

**REJECTED (not added):** none (batch-65 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 1F3`..`40 1FA` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_493 finishes 160 LDB triad (slot 52). H_494..H_496 start 160 ADD triad (imm32 22B). H_497..H_499 start 160 SUB triad (imm32 22B). H_500 starts 168 LDB triad (imm32 26B).

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_493..H_500 at selectors 0x1F3..0x1FA (`40 1F3`..`40 1FA`). Not RAW_BYTE; mirrors H_485..H_492 comment style (body-extend-071 / parallel-batch-65).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5260_160,addimm_h50_160,addimm_h51_160,addimm_h52_160,subimm_h50_160,subimm_h51_160,subimm_h52_160,ldb_5060_168}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **491/491 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **499/499 PASS**.
- Full canonical emit: JS=Rust=**11283B** code (was 11099B; +184B); byte-equal **Y**; sha `183ac9e8467f2ed6…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `1f070530a91ca949…`; previous chained to `192ba67ac8bb814d…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=11776; both peers code=11283; hash_a=hash_b=`d6a154819ef7556b…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-070 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-65 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_493..H_500 at selectors 0x1F3..0x1FA.
4. Selftest: exact pins PASS (26/22/22/22/22/22/22/26B).
5. Goldens: JS 491/491 and Rust 499/499 PASS; full emit byte-equal Y at 11283B.
6. Lock: Relock once → `1f070530a91ca949…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-66: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_493..H_500), writing `parallel-batch-66-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: LDB 51/52 60 168 (finish 168 LDB triad), ADD-IMM / SUB-IMM imm=168 triad, SET/GET fresh, etc. After batch-66 scratches done: parent next = body-extend-072 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-66-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

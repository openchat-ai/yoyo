# body-extend-065 Log · parallel-batch-59 consolidation (H_445..H_452)

> Tag: `body-extend-065-EXPERIMENTAL-batch59-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-59-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `d9aff9ed76e4f649…` → `b84d7f1b4bb1d8ee…`.
> **handler count: 451 → 459** (+8 at selectors 0x1C3..0x1CA via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_445 | 0x1C3 | 0x61 SUB-IMM | 52 130 | 22 | `e775907813eab73c` |
| H_446 | 0x1C4 | 0x80 LDB | 50 60 138 | 26 | `465ea202edfa6b33` |
| H_447 | 0x1C5 | 0x80 LDB | 51 60 138 | 26 | `a55ee627c7c07cff` |
| H_448 | 0x1C6 | 0x80 LDB | 52 60 138 | 26 | `2ac95e896392b10a` |
| H_449 | 0x1C7 | 0x62 ADD-IMM | 50 138 | 22 | `3d9af9767bc85f81` |
| H_450 | 0x1C8 | 0x62 ADD-IMM | 51 138 | 22 | `24769795853dcd61` |
| H_451 | 0x1C9 | 0x62 ADD-IMM | 52 138 | 22 | `9e5da1e81f6ac0a0` |
| H_452 | 0x1CA | 0x61 SUB-IMM | 50 138 | 22 | `44bbe548c077e01f` |

**REJECTED (not added):** none (batch-59 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 1C3`..`40 1CA` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_445 SUB-IMM imm=0x130 finishes 130 SUB triad (imm32 22B). H_446..H_448 LDB oo=0x138 use imm32 (`48 81 c0 …`), pin 26B (starts 138 LDB triad). H_449..H_451 ADD-IMM imm=0x138 use imm32 (`48 81 c0 …`), pin 22B (starts 138 ADD triad). H_452 SUB-IMM imm=0x138 use imm32 (`48 81 e8 …`), pin 22B (starts 138 SUB triad; H_453/H_454 SUB-IMM 51/52 138 deferred to next batch).

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_445..H_452 at selectors 0x1C3..0x1CA (`40 1C3`..`40 1CA`). Not RAW_BYTE; mirrors H_437..H_444 comment style (body-extend-065 / parallel-batch-59).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h52_130,ldb_5060_138,ldb_5160_138,ldb_5260_138,addimm_h50_138,addimm_h51_138,addimm_h52_138,subimm_h50_138}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **443/443 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **451/451 PASS**.
- Full canonical emit: JS=Rust=**10163B** code (was 9975B; +188B); byte-equal **Y**; sha `a07ebd68e2f96fed…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `b84d7f1b4bb1d8ee…`; previous chained to `d9aff9ed76e4f649…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=10240; both peers code=10163; hash_a=hash_b=`5c8584a167d7a53f…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-064 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-59 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_445..H_452 at selectors 0x1C3..0x1CA.
4. Selftest: exact pins PASS (22/26/26/26/22/22/22/22B).
5. Goldens: JS 443/443 and Rust 451/451 PASS; full emit byte-equal Y at 10163B.
6. Lock: Relock once → `b84d7f1b4bb1d8ee…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-60: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_445..H_452), writing `parallel-batch-60-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: SUB-IMM 51/52 138 (finish 138 SUB triad), LDB oo=140 triad, ADD-IMM / SUB-IMM imm=140, SET/GET fresh, etc. After batch-60 scratches done: parent next = body-extend-066 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-60-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

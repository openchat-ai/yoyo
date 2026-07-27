# body-extend-086 Log · parallel-batch-80 consolidation (H_613..H_620)

> Tag: `body-extend-086-EXPERIMENTAL-batch80-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-80-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `58b9ca6ef16f3ee4…` → `9546a03ee5ac5d52…`.
> **handler count: 619 → 627** (+8 at selectors 0x26B..0x272 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_613 | 0x26B | 0x62 ADD-IMM | 52 1C8 | 22 | `dc11d2c2afb93a56` |
| H_614 | 0x26C | 0x61 SUB-IMM | 50 1C8 | 22 | `3c7c7cf3d889226e` |
| H_615 | 0x26D | 0x61 SUB-IMM | 51 1C8 | 22 | `63000a311432b0f3` |
| H_616 | 0x26E | 0x61 SUB-IMM | 52 1C8 | 22 | `fa6d5ee090445380` |
| H_617 | 0x26F | 0x80 LDB | 50 60 1D0 | 26 | `a8b6a7f0de518100` |
| H_618 | 0x270 | 0x80 LDB | 51 60 1D0 | 26 | `261db47e68ac40dd` |
| H_619 | 0x271 | 0x80 LDB | 52 60 1D0 | 26 | `e6b3a3507a16a0ad` |
| H_620 | 0x272 | 0x62 ADD-IMM | 50 1D0 | 22 | `16f0fd643450814e` |

**REJECTED (not added):** none (batch-80 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 26B`..`40 272` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_613 finishes 1C8 ADD triad. H_614..H_616 start/finish 1C8 SUB triad. H_617..H_619 start/finish 1D0 LDB triad. H_620 starts 1D0 ADD triad (slot=51/52 deferred).

**Deferred (not added this beat):** ADD-IMM slot=51/52 imm=1D0; SUB-IMM slot=50/51/52 imm=1D0; next imm ladder 1D8… — suggested for parallel-batch-81.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1C8 / 0x1D0 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1D0 uses imm32 → 26B pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_613..H_620 at selectors 0x26B..0x272 (`40 26B`..`40 272`). Not RAW_BYTE; mirrors H_605..H_612 comment style (body-extend-086 / parallel-batch-80).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h52_1C8,subimm_h50_1C8,subimm_h51_1C8,subimm_h52_1C8,ldb_5060_1D0,ldb_5160_1D0,ldb_5260_1D0,addimm_h50_1D0}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **611/611 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **619/619 PASS**.
- Full canonical emit: JS=Rust=**14087B** code (was 13899B; +188B); byte-equal **Y**; sha `86abace7f42682dd…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `9546a03ee5ac5d52…`; previous chained to `58b9ca6ef16f3ee4…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=14336; both peers code=14087; hash_a=hash_b=`19333e5bb37cf7e8…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-085 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-80 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_613..H_620 at selectors 0x26B..0x272.
4. Selftest: exact pins PASS (22/22/22/22/26/26/26/22B).
5. Goldens: JS 611/611 and Rust 619/619 PASS; full emit byte-equal Y at 14087B.
6. Lock: Relock once → `9546a03ee5ac5d52…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-81: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_613..H_620), writing `parallel-batch-81-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: finish ADD-IMM 51/52 1D0, then SUB-IMM 50/51/52 1D0, next imm ladder 1D8…, SET/GET fresh, etc. After batch-81 scratches done: parent next = body-extend-087 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-81-SPAWN.md` (no Task tool on this consolidator). Deferred carry: ADD-IMM 51/52 1D0; SUB-IMM 50/51/52 1D0; next imm ladder 1D8….

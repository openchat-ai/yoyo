# body-extend-087 Log · parallel-batch-81 consolidation (H_621..H_628)

> Tag: `body-extend-087-EXPERIMENTAL-batch81-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-81-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `9546a03ee5ac5d52…` → `db550629db78a974…`.
> **handler count: 627 → 635** (+8 at selectors 0x273..0x27A via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_621 | 0x273 | 0x62 ADD-IMM | 51 1D0 | 22 | `c3c058088c10f83d` |
| H_622 | 0x274 | 0x62 ADD-IMM | 52 1D0 | 22 | `0a27be96ee13ece7` |
| H_623 | 0x275 | 0x61 SUB-IMM | 50 1D0 | 22 | `2ef2771b506c8417` |
| H_624 | 0x276 | 0x61 SUB-IMM | 51 1D0 | 22 | `45b0a1f1db1de34c` |
| H_625 | 0x277 | 0x61 SUB-IMM | 52 1D0 | 22 | `58eef69fd940e52e` |
| H_626 | 0x278 | 0x80 LDB | 50 60 1D8 | 26 | `15e5685f4e776a0c` |
| H_627 | 0x279 | 0x80 LDB | 51 60 1D8 | 26 | `76fe53dd5b4d10f0` |
| H_628 | 0x27A | 0x80 LDB | 52 60 1D8 | 26 | `9a6919da685a02e0` |

**REJECTED (not added):** none (batch-81 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 273`..`40 27A` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_621/H_622 finish 1D0 ADD triad. H_623..H_625 start/finish 1D0 SUB triad. H_626..H_628 start/finish 1D8 LDB triad.

**Deferred (not added this beat):** ADD-IMM slot=50/51/52 imm=1D8; SUB-IMM slot=50/51/52 imm=1D8; next imm ladder 1E0… — suggested for parallel-batch-82.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1D0 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1D8 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_621 | `c3c058088c10f83d4fcfade79d449baac7c28513edba68591ebafeb0c3abb50c` |
| H_622 | `0a27be96ee13ece7972673d85b5654a23eee344d34373cb539b16147cb3526bf` |
| H_623 | `2ef2771b506c8417b1cc2d80c9b8c2094e807cff80496af171a4fdf7e2c507b2` |
| H_624 | `45b0a1f1db1de34c044d580d34b8c70fb11ec50a1b46182c9608dc64d6131fb6` |
| H_625 | `58eef69fd940e52e61c7da1738853b9d42f41aedc2819f354db5a0d797b6d43b` |
| H_626 | `15e5685f4e776a0c3cb0c740f5572ff6200335d566557b079afcb68ec8519972` |
| H_627 | `76fe53dd5b4d10f0df7a013ab4be44413f3691fc8953a5df2e44bdf13460d462` |
| H_628 | `9a6919da685a02e06aa72835e35705110da398e37ceb4d9f2e809f57c5d3a3e7` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_621..H_628 at selectors 0x273..0x27A (`40 273`..`40 27A`). Not RAW_BYTE; mirrors H_613..H_620 comment style (body-extend-087 / parallel-batch-81).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h51_1D0,addimm_h52_1D0,subimm_h50_1D0,subimm_h51_1D0,subimm_h52_1D0,ldb_5060_1D8,ldb_5160_1D8,ldb_5260_1D8}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **619/619 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **627/627 PASS**.
- Full canonical emit: JS=Rust=**14275B** code (was 14087B; +188B); byte-equal **Y**; sha `777f8e6c365f1307…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `db550629db78a974…`; previous chained to `9546a03ee5ac5d52…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=14336; both peers code=14275; hash_a=hash_b=`5d0f289a8da574ac…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-086 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-81 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_621..H_628 at selectors 0x273..0x27A.
4. Selftest: exact pins PASS (22/22/22/22/22/26/26/26B).
5. Goldens: JS 619/619 and Rust 627/627 PASS; full emit byte-equal Y at 14275B.
6. Lock: Relock once → `db550629db78a974…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-82: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_621..H_628), writing `parallel-batch-82-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: start ADD-IMM 50/51/52 1D8, then SUB-IMM 50/51/52 1D8, next imm ladder 1E0…, SET/GET fresh, etc. After batch-82 scratches done: parent next = body-extend-088 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-82-SPAWN.md` (no Task tool on this consolidator). Deferred carry: ADD-IMM/SUB-IMM 50/51/52 1D8; next imm ladder 1E0….

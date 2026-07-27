# body-extend-088 Log · parallel-batch-82 consolidation (H_629..H_636)

> Tag: `body-extend-088-EXPERIMENTAL-batch82-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-82-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `db550629db78a974…` → `697ad7847ba15e82…`.
> **handler count: 635 → 643** (+8 at selectors 0x27B..0x282 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_629 | 0x27B | 0x62 ADD-IMM | 50 1D8 | 22 | `985fc739129b28e5` |
| H_630 | 0x27C | 0x62 ADD-IMM | 51 1D8 | 22 | `529c91e6cee0c610` |
| H_631 | 0x27D | 0x62 ADD-IMM | 52 1D8 | 22 | `11e5e0737f59a060` |
| H_632 | 0x27E | 0x61 SUB-IMM | 50 1D8 | 22 | `a387a1d628c84d7e` |
| H_633 | 0x27F | 0x61 SUB-IMM | 51 1D8 | 22 | `f7f546cac9fd3bab` |
| H_634 | 0x280 | 0x61 SUB-IMM | 52 1D8 | 22 | `9f9c8525bbf76801` |
| H_635 | 0x281 | 0x80 LDB | 50 60 1E0 | 26 | `54ae10749db49954` |
| H_636 | 0x282 | 0x80 LDB | 51 60 1E0 | 26 | `fab08f3c3976d127` |

**REJECTED (not added):** none (batch-82 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 27B`..`40 282` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_629..H_631 start/finish 1D8 ADD triad. H_632..H_634 start/finish 1D8 SUB triad. H_635/H_636 start 1E0 LDB triad (dd=50/51; LDB 52 1E0 deferred).

**Deferred (not added this beat):** LDB dd=52 ss=60 oo=1E0; ADD-IMM / SUB-IMM slot=50/51/52 imm=1E0 — suggested for parallel-batch-83.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1D8 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1E0 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_629 | `985fc739129b28e5206f7a44af2242038d962dfab346dc8508df252fec254a18` |
| H_630 | `529c91e6cee0c6109a96db3ce2e03499601a8842698be17f37b7ae849b66ceb7` |
| H_631 | `11e5e0737f59a0604afe87cbec770b18cd8c096583d1007d45c2129506c8dccd` |
| H_632 | `a387a1d628c84d7eb3f1149a46c9d3568e2c7fa70e19b289f1fdc4d11735d875` |
| H_633 | `f7f546cac9fd3bab9ba41979b7477e535b189d1c59de5113ebea881f368f0c42` |
| H_634 | `9f9c8525bbf768014398e291dffe27d2523d9350d2b10d71c990bfafd40ff3bd` |
| H_635 | `54ae10749db49954ff3e0f998b123304cb8bdc5d88a0b54a728d95efb27044e9` |
| H_636 | `fab08f3c3976d127fad45fef564cdb0b17def5621b4350fcc26df85fd86693d0` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_629..H_636 at selectors 0x27B..0x282 (`40 27B`..`40 282`). Not RAW_BYTE; mirrors H_621..H_628 comment style (body-extend-088 / parallel-batch-82).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h50_1D8,addimm_h51_1D8,addimm_h52_1D8,subimm_h50_1D8,subimm_h51_1D8,subimm_h52_1D8,ldb_5060_1E0,ldb_5160_1E0}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **627/627 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **635/635 PASS**.
- Full canonical emit: JS=Rust=**14459B** code (was 14275B; +184B); byte-equal **Y**; sha `aaee05ae19a89935…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `697ad7847ba15e82…`; previous chained to `db550629db78a974…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=14848; both peers code=14459; hash_a=hash_b=`4988cc18074af227…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-087 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-82 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_629..H_636 at selectors 0x27B..0x282.
4. Selftest: exact pins PASS (22/22/22/22/22/22/26/26B).
5. Goldens: JS 627/627 and Rust 635/635 PASS; full emit byte-equal Y at 14459B.
6. Lock: Relock once → `697ad7847ba15e82…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-83: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_629..H_636), writing `parallel-batch-83-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: finish LDB 52 60 1E0; start ADD-IMM/SUB-IMM 50/51/52 1E0; SET/GET fresh, etc. After batch-83 scratches done: parent next = body-extend-089 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-83-SPAWN.md` (no Task tool on this consolidator). Deferred carry: LDB 52 60 1E0; ADD-IMM/SUB-IMM 50/51/52 1E0.

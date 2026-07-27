# body-extend-073 Log · parallel-batch-67 consolidation (H_509..H_516)

> Tag: `body-extend-073-EXPERIMENTAL-batch67-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-67-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `e1554db8dcce9946…` → `1a6cb44aa28367d2…`.
> **handler count: 515 → 523** (+8 at selectors 0x203..0x20A via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED; 1 deferred from batch-67)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_509 | 0x203 | 0x80 LDB | 50 60 170 | 26 | `2880271f9ceddc44` |
| H_510 | 0x204 | 0x80 LDB | 51 60 170 | 26 | `f5ea323500e5fb12` |
| H_511 | 0x205 | 0x80 LDB | 52 60 170 | 26 | `ee43e15d67b15204` |
| H_512 | 0x206 | 0x62 ADD-IMM | 50 170 | 22 | `b5ced24e14fef8f3` |
| H_513 | 0x207 | 0x62 ADD-IMM | 51 170 | 22 | `2bb85897a4abc0cf` |
| H_514 | 0x208 | 0x62 ADD-IMM | 52 170 | 22 | `ccca022a923acf93` |
| H_515 | 0x209 | 0x61 SUB-IMM | 50 170 | 22 | `b78b97ec483ce762` |
| H_516 | 0x20A | 0x61 SUB-IMM | 51 170 | 22 | `f6d1c92bf87d13e8` |

**REJECTED (not added):** none (batch-67 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 203`..`40 20A` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_509..H_511 start 170 LDB triad. H_512..H_514 start 170 ADD triad. H_515..H_516 start 170 SUB (slot 52 deferred).

**Deferred (not added this beat):** SUB-IMM slot=52 imm=170 — suggested for parallel-batch-68.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x170 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x170 uses imm32 → 26B pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_509..H_516 at selectors 0x203..0x20A (`40 203`..`40 20A`). Not RAW_BYTE; mirrors H_501..H_508 comment style (body-extend-073 / parallel-batch-67).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5060_170,ldb_5160_170,ldb_5260_170,addimm_h50_170,addimm_h51_170,addimm_h52_170,subimm_h50_170,subimm_h51_170}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **507/507 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **515/515 PASS**.
- Full canonical emit: JS=Rust=**11655B** code (was 11467B; +188B); byte-equal **Y**; sha `5fd063703c85f1aa…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `1a6cb44aa28367d2…`; previous chained to `e1554db8dcce9946…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=11776; both peers code=11655; hash_a=hash_b=`06974e7d07279bbe…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-072 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-67 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_509..H_516 at selectors 0x203..0x20A.
4. Selftest: exact pins PASS (26/26/26/22/22/22/22/22B).
5. Goldens: JS 507/507 and Rust 515/515 PASS; full emit byte-equal Y at 11655B.
6. Lock: Relock once → `1a6cb44aa28367d2…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-68: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_509..H_516), writing `parallel-batch-68-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: finish SUB-IMM 52 170, then LDB/ADD/SUB oo=178 ladder, SET/GET fresh, etc. After batch-68 scratches done: parent next = body-extend-074 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-68-SPAWN.md` (no Task tool on this consolidator). Deferred carry: SUB-IMM slot=52 imm=170.

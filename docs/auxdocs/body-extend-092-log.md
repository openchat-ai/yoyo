# body-extend-092 Log · parallel-batch-86 consolidation (H_661..H_668)

> Tag: `body-extend-092-EXPERIMENTAL-batch86-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-86-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `339bd482ae784eb8…` → `1991af8484d67ec1…`.
> **handler count: 667 → 675** (+8 at selectors 0x29B..0x2A2 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_661 | 0x29B | 0x61 SUB-IMM | 52 1F0 | 22 | `21a46af767b04e47` |
| H_662 | 0x29C | 0x80 LDB | 50 60 1F8 | 26 | `e33190513a0b6fac` |
| H_663 | 0x29D | 0x80 LDB | 51 60 1F8 | 26 | `754738a2ae8287ba` |
| H_664 | 0x29E | 0x80 LDB | 52 60 1F8 | 26 | `b3d0c040cbafd1ed` |
| H_665 | 0x29F | 0x62 ADD-IMM | 50 1F8 | 22 | `e4eb4882c94f477d` |
| H_666 | 0x2A0 | 0x62 ADD-IMM | 51 1F8 | 22 | `767adbf6b2f425c9` |
| H_667 | 0x2A1 | 0x62 ADD-IMM | 52 1F8 | 22 | `5e4ebbbafb63edb5` |
| H_668 | 0x2A2 | 0x61 SUB-IMM | 50 1F8 | 22 | `8ebe141b655cf99d` |

**REJECTED (not added):** none (batch-86 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 29B`..`40 2A2` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_661 finishes 1F0 SUB triad. H_662..H_664 start/finish 1F8 LDB triad. H_665..H_667 start/finish 1F8 ADD triad. H_668 starts 1F8 SUB triad (SUB 51/52 deferred).

**Deferred (not added this beat):** SUB-IMM slot=51/52 imm=1F8; SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked — suggested for parallel-batch-87.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1F0/0x1F8 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1F8 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_661 | `21a46af767b04e47650b619132dce8c0d8eb8853d90a43a9d7d1d28af98d0f1a` |
| H_662 | `e33190513a0b6fac13932e72229b5bbafeaff7083cc224cf8c53338378fee9c0` |
| H_663 | `754738a2ae8287ba25dd22fcc9ffef4d583b37c9061d495e26db3499044f8770` |
| H_664 | `b3d0c040cbafd1ed0af4dbfd1514fe59fcc1c44f55d5b025a1d4602cba0cfd12` |
| H_665 | `e4eb4882c94f477d7369f849651cad1e3e4ebd2dbe762d456c009667bc3d37ad` |
| H_666 | `767adbf6b2f425c9fb3363b309c6fe79e3d1ba7874531cc4b34f0706d7e1b3c0` |
| H_667 | `5e4ebbbafb63edb539dc75ecd54dd72456e8ff9afa398a2f746507a3c3f3ba2a` |
| H_668 | `8ebe141b655cf99deb6c55f0869d36be0725d1fef0b58268d6d6751190a816a5` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_661..H_668 at selectors 0x29B..0x2A2 (`40 29B`..`40 2A2`). Not RAW_BYTE; mirrors H_653..H_660 comment style (body-extend-092 / parallel-batch-86).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h52_1F0,ldb_5060_1F8,ldb_5160_1F8,ldb_5260_1F8,addimm_h50_1F8,addimm_h51_1F8,addimm_h52_1F8,subimm_h50_1F8}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **659/659 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **667/667 PASS**.
- Full canonical emit: JS=Rust=**15203B** code (was 15015B; +188B); byte-equal **Y**; sha `349bc1525f2c2ff2…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `1991af8484d67ec1…`; previous chained to `339bd482ae784eb8…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=15360; both peers code=15203; hash_a=hash_b=`58935576c9951ce2…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-091 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-86 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_661..H_668 at selectors 0x29B..0x2A2.
4. Selftest: exact pins PASS (22/26/26/26/22/22/22/22B).
5. Goldens: JS 659/659 and Rust 667/667 PASS; full emit byte-equal Y at 15203B.
6. Lock: Relock once → `1991af8484d67ec1…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-87: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_661..H_668), writing `parallel-batch-87-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: finish SUB-IMM 51/52 1F8; SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked; next ladder if continuing. After batch-87 scratches done: parent next = body-extend-093 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-87-SPAWN.md` (no Task tool on this consolidator). Deferred carry: SUB-IMM 51/52 1F8; SET/GET/ORV/SUBV/ADDV/IMUL.

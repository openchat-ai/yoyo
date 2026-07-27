# body-extend-100 Log · parallel-batch-94 consolidation (H_725..H_732)

> Tag: `body-extend-100-EXPERIMENTAL-batch94-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-94-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `3fc618f9e14a881a…` → `7c07906496a7af9c…`.
> **handler count: 731 → 739** (+8 at selectors 0x2DB..0x2E2 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_725 | 0x2DB | 0x80 LDB | 50 60 230 | 26 | `5c33c35f5fd9760b` |
| H_726 | 0x2DC | 0x80 LDB | 51 60 230 | 26 | `e574a865427adbbc` |
| H_727 | 0x2DD | 0x80 LDB | 52 60 230 | 26 | `b066434bf619727b` |
| H_728 | 0x2DE | 0x62 ADD-IMM | 50 230 | 22 | `a2091c2a78abf623` |
| H_729 | 0x2DF | 0x62 ADD-IMM | 51 230 | 22 | `e5fd2243f67268fd` |
| H_730 | 0x2E0 | 0x62 ADD-IMM | 52 230 | 22 | `7fb8cfe02d18cafc` |
| H_731 | 0x2E1 | 0x61 SUB-IMM | 50 230 | 22 | `6057b3357f248ea9` |
| H_732 | 0x2E2 | 0x61 SUB-IMM | 51 230 | 22 | `7b44c3ca05a14832` |

**REJECTED (not added):** none (batch-94 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 2DB`..`40 2E2` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_725..H_727 start/finish 230 LDB triad. H_728..H_730 start/finish 230 ADD triad. H_731/H_732 start 230 SUB triad (SUB 52 deferred).

**Deferred (not added this beat):** SUB-IMM slot=52 imm=230 (finish 230 SUB triad); SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked; next ladder beyond 230 — suggested for parallel-batch-95.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x230 uses imm32 (`48 81 e8` / `48 81 c0`) → 22B pins; not imm8.
LDB oo=0x230 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_725 | `5c33c35f5fd9760bff127c50a7190077e8179b418de6f54d667aa08bdc69f34b` |
| H_726 | `e574a865427adbbcc9fceca251487fa299ab80135e0abbf0be72094e6d08331b` |
| H_727 | `b066434bf619727bfc38fae08251b8be38f39b2d12d9ae89e23f066de712ce77` |
| H_728 | `a2091c2a78abf623843bf75666d8bbed15a0959397a01bef4e66efd2fcca4f1a` |
| H_729 | `e5fd2243f67268fde53d644b88bec57be0e74d8b0db703b799b9d2d6b6165a18` |
| H_730 | `7fb8cfe02d18cafc02e295bf962c249fa332e1c18499b482aac3a18a6f369905` |
| H_731 | `6057b3357f248ea9b548c111e3b2ce2a2b8b17a8a385e592eda7998f822642f8` |
| H_732 | `7b44c3ca05a148325d4d462a1a872b18bf20c1994a1c0b3551bb0ebef49583d7` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_725..H_732 at selectors 0x2DB..0x2E2 (`40 2DB`..`40 2E2`). Not RAW_BYTE; mirrors H_717..H_724 comment style (body-extend-100 / parallel-batch-94).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5060_230,ldb_5160_230,ldb_5260_230,addimm_h50_230,addimm_h51_230,addimm_h52_230,subimm_h50_230,subimm_h51_230}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **723/723 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **731/731 PASS**.
- Full canonical emit: JS=Rust=**16695B** code (was 16507B; +188B); byte-equal **Y**; sha `cbd1baad6528d406…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `7c07906496a7af9c…`; previous chained to `3fc618f9e14a881a…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=16896; both peers code=16695; hash_a=hash_b=`5355d1b740e49c1f…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-099 also measured EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-94 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_725..H_732 at selectors 0x2DB..0x2E2.
4. Selftest: exact pins PASS (26/26/26/22/22/22/22/22B).
5. Goldens: JS 723/723 and Rust 731/731 PASS; full emit byte-equal Y at 16695B.
6. Lock: Relock once → `7c07906496a7af9c…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-95: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_725..H_732), writing `parallel-batch-95-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: SUB-IMM 52 230 (finish 230 SUB triad); SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked; next ladder beyond 230 if continuing. After batch-95 scratches done: parent next = body-extend-101 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-95-SPAWN.md` (no Task tool on this consolidator). Deferred carry: SUB-IMM 52 230; SET/GET/ORV/SUBV/ADDV/IMUL; next ladder.

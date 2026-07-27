# body-extend-101 Log · parallel-batch-95 consolidation (H_733..H_740)

> Tag: `body-extend-101-EXPERIMENTAL-batch95-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-95-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `7c07906496a7af9c…` → `514ff62ce8663a15…`.
> **handler count: 739 → 747** (+8 at selectors 0x2E3..0x2EA via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_733 | 0x2E3 | 0x61 SUB-IMM | 52 230 | 22 | `5a2ce924b1a66050` |
| H_734 | 0x2E4 | 0x80 LDB | 50 60 232 | 26 | `2c8b3aa576062c39` |
| H_735 | 0x2E5 | 0x80 LDB | 51 60 232 | 26 | `d935a5d3f24953e7` |
| H_736 | 0x2E6 | 0x80 LDB | 52 60 232 | 26 | `1d9a2681b4fac7a1` |
| H_737 | 0x2E7 | 0x62 ADD-IMM | 50 232 | 22 | `da80cde8ed742a1c` |
| H_738 | 0x2E8 | 0x62 ADD-IMM | 51 232 | 22 | `4aa3b5563616b6a6` |
| H_739 | 0x2E9 | 0x62 ADD-IMM | 52 232 | 22 | `f9199c6bd9783045` |
| H_740 | 0x2EA | 0x61 SUB-IMM | 50 232 | 22 | `922bcb642443cdc9` |

**REJECTED (not added):** none (batch-95 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 2E3`..`40 2EA` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_733 finishes 230 SUB triad. H_734..H_736 start/finish 232 LDB triad. H_737..H_739 start/finish 232 ADD triad. H_740 starts 232 SUB triad (SUB 51/52 deferred).

**Deferred (not added this beat):** SUB-IMM slot=51/52 imm=232 (finish 232 SUB triad); SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked; next ladder beyond 232 — suggested for parallel-batch-96.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x230/0x232 uses imm32 (`48 81 e8` / `48 81 c0`) → 22B pins; not imm8.
LDB oo=0x232 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_733 | `5a2ce924b1a66050cd8317c147e86ed49e8277cc463083ae9a8c0eb691989b89` |
| H_734 | `2c8b3aa576062c3900b06a28cef3c8d5505960f829c94454bb8154c3e33eccf3` |
| H_735 | `d935a5d3f24953e7037800a6a859243d8d5e12c711fd4ea0105a13617016acb2` |
| H_736 | `1d9a2681b4fac7a1dfc3d43209e67426fda3041dee1bcfc1c51f3433838f73da` |
| H_737 | `da80cde8ed742a1c98a87f2e0e0c0f69e62d1cf12b4cb73fe03d51b8c2a2e3eb` |
| H_738 | `4aa3b5563616b6a6dbbab36788b7117488fc12b67a0cc851ddb3cccc6a4671cd` |
| H_739 | `f9199c6bd9783045ccd6c049dbc65401650062d1a850a6269d18a6fc35617d89` |
| H_740 | `922bcb642443cdc9d80af6a993395e5d90bb69b7b4e59539c7b5dd327a22ce2a` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_733..H_740 at selectors 0x2E3..0x2EA (`40 2E3`..`40 2EA`). Not RAW_BYTE; mirrors H_725..H_732 comment style (body-extend-101 / parallel-batch-95).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h52_230,ldb_5060_232,ldb_5160_232,ldb_5260_232,addimm_h50_232,addimm_h51_232,addimm_h52_232,subimm_h50_232}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **731/731 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **739/739 PASS**.
- Full canonical emit: JS=Rust=**16883B** code (was 16695B; +188B); byte-equal **Y**; sha `d435c967c2d4ae93…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `514ff62ce8663a15…`; previous chained to `7c07906496a7af9c…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=16896; both peers code=16883; hash_a=hash_b=`e08056377f0b1bee…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-100 also measured EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-95 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_733..H_740 at selectors 0x2E3..0x2EA.
4. Selftest: exact pins PASS (22/26/26/26/22/22/22/22B).
5. Goldens: JS 731/731 and Rust 739/739 PASS; full emit byte-equal Y at 16883B.
6. Lock: Relock once → `514ff62ce8663a15…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-96: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_733..H_740), writing `parallel-batch-96-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: SUB-IMM 51/52 232 (finish 232 SUB triad); SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked; next ladder beyond 232 if continuing. After batch-96 scratches done: parent next = body-extend-102 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-96-SPAWN.md` (no Task tool on this consolidator). Deferred carry: SUB-IMM 51/52 232; SET/GET/ORV/SUBV/ADDV/IMUL; next ladder. handlers=747 < 800 → continue queue (not AUTO-STOP).

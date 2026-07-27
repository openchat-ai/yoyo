# body-extend-091 Log · parallel-batch-85 consolidation (H_653..H_660)

> Tag: `body-extend-091-EXPERIMENTAL-batch85-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-85-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `63204ed031f1ad84…` → `339bd482ae784eb8…`.
> **handler count: 659 → 667** (+8 at selectors 0x293..0x29A via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_653 | 0x293 | 0x80 LDB | 50 60 1F0 | 26 | `1dd7536ff94f210b` |
| H_654 | 0x294 | 0x80 LDB | 51 60 1F0 | 26 | `e2c1e0f004de6eab` |
| H_655 | 0x295 | 0x80 LDB | 52 60 1F0 | 26 | `ad43445e924ece15` |
| H_656 | 0x296 | 0x62 ADD-IMM | 50 1F0 | 22 | `17b7b25157e9d135` |
| H_657 | 0x297 | 0x62 ADD-IMM | 51 1F0 | 22 | `ad1776283d15b543` |
| H_658 | 0x298 | 0x62 ADD-IMM | 52 1F0 | 22 | `3c8d698c14cd2075` |
| H_659 | 0x299 | 0x61 SUB-IMM | 50 1F0 | 22 | `43db5ead3bfc62f7` |
| H_660 | 0x29A | 0x61 SUB-IMM | 51 1F0 | 22 | `dac7533ba9ab5adb` |

**REJECTED (not added):** none (batch-85 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 293`..`40 29A` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_653..H_655 start/finish 1F0 LDB triad. H_656..H_658 start/finish 1F0 ADD triad. H_659/H_660 start 1F0 SUB triad (SUB 52 deferred).

**Deferred (not added this beat):** SUB-IMM slot=52 imm=1F0; SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked — suggested for parallel-batch-86.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1F0 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1F0 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_653 | `1dd7536ff94f210bd645058b78b04d700e36ffcc02f83a27751f4884a3c5f452` |
| H_654 | `e2c1e0f004de6eab107bf86ea1c3731d8d3ad1fab935977c9c817f24b348b676` |
| H_655 | `ad43445e924ece151de60e3a022b4dc1bacc431c105858a57983bd4fe559e13e` |
| H_656 | `17b7b25157e9d1359b6bf473502a844f3b2ab639269729b6c365fec13cdd0507` |
| H_657 | `ad1776283d15b543ad830e89f699a757a239e8eb8aae61713f0260e5967a4c51` |
| H_658 | `3c8d698c14cd20755b85feb3d0d41c083447b42c4eca61ed1f06015fc5fae172` |
| H_659 | `43db5ead3bfc62f7b1ddc851953e2ef0966523e442f1e053cc1ebd4691764add` |
| H_660 | `dac7533ba9ab5adb7ba3cbddfedac2ea91a3c50bd8784a92a138e73671e06e9e` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_653..H_660 at selectors 0x293..0x29A (`40 293`..`40 29A`). Not RAW_BYTE; mirrors H_645..H_652 comment style (body-extend-091 / parallel-batch-85).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5060_1F0,ldb_5160_1F0,ldb_5260_1F0,addimm_h50_1F0,addimm_h51_1F0,addimm_h52_1F0,subimm_h50_1F0,subimm_h51_1F0}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **651/651 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **659/659 PASS**.
- Full canonical emit: JS=Rust=**15015B** code (was 14827B; +188B); byte-equal **Y**; sha `6b13222024666eb1…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `339bd482ae784eb8…`; previous chained to `63204ed031f1ad84…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=15360; both peers code=15015; hash_a=hash_b=`c0e9c91eeb74c181…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-090 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-85 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_653..H_660 at selectors 0x293..0x29A.
4. Selftest: exact pins PASS (26/26/26/22/22/22/22/22B).
5. Goldens: JS 651/651 and Rust 659/659 PASS; full emit byte-equal Y at 15015B.
6. Lock: Relock once → `339bd482ae784eb8…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-86: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_653..H_660), writing `parallel-batch-86-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: finish SUB-IMM 52 1F0; SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked; next ladder if continuing. After batch-86 scratches done: parent next = body-extend-092 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-86-SPAWN.md` (no Task tool on this consolidator). Deferred carry: SUB-IMM 52 1F0; SET/GET/ORV/SUBV/ADDV/IMUL.

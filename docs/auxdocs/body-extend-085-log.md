# body-extend-085 Log · parallel-batch-79 consolidation (H_605..H_612)

> Tag: `body-extend-085-EXPERIMENTAL-batch79-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-79-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `9eafc9ce0376d389…` → `58b9ca6ef16f3ee4…`.
> **handler count: 611 → 619** (+8 at selectors 0x263..0x26A via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_605 | 0x263 | 0x61 SUB-IMM | 50 1C0 | 22 | `2dd291d1df0ff186` |
| H_606 | 0x264 | 0x61 SUB-IMM | 51 1C0 | 22 | `162f63e6a4ed8641` |
| H_607 | 0x265 | 0x61 SUB-IMM | 52 1C0 | 22 | `649c06ddcb80956d` |
| H_608 | 0x266 | 0x80 LDB | 50 60 1C8 | 26 | `b299fd62cea22ef7` |
| H_609 | 0x267 | 0x80 LDB | 51 60 1C8 | 26 | `18e61721bdda72c3` |
| H_610 | 0x268 | 0x80 LDB | 52 60 1C8 | 26 | `9612ef36d64f34eb` |
| H_611 | 0x269 | 0x62 ADD-IMM | 50 1C8 | 22 | `435f20ebb01bbc21` |
| H_612 | 0x26A | 0x62 ADD-IMM | 51 1C8 | 22 | `d6e88e4f8c96211e` |

**REJECTED (not added):** none (batch-79 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 263`..`40 26A` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_605..H_607 start/finish 1C0 SUB triad. H_608..H_610 start/finish 1C8 LDB triad. H_611/H_612 start 1C8 ADD triad (slot=52 deferred).

**Deferred (not added this beat):** ADD-IMM slot=52 imm=1C8; SUB-IMM slot=50/51/52 imm=1C8; next imm ladder 1D0… — suggested for parallel-batch-80.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1C0 / 0x1C8 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1C8 uses imm32 → 26B pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_605..H_612 at selectors 0x263..0x26A (`40 263`..`40 26A`). Not RAW_BYTE; mirrors H_597..H_604 comment style (body-extend-085 / parallel-batch-79).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h50_1C0,subimm_h51_1C0,subimm_h52_1C0,ldb_5060_1C8,ldb_5160_1C8,ldb_5260_1C8,addimm_h50_1C8,addimm_h51_1C8}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **603/603 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **611/611 PASS**.
- Full canonical emit: JS=Rust=**13899B** code (was 13711B; +188B); byte-equal **Y**; sha `d07ae1035ec1d228…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `58b9ca6ef16f3ee4…`; previous chained to `9eafc9ce0376d389…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=14336; both peers code=13899; hash_a=hash_b=`940f9ae1f294489b…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-084 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-79 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_605..H_612 at selectors 0x263..0x26A.
4. Selftest: exact pins PASS (22/22/22/26/26/26/22/22B).
5. Goldens: JS 603/603 and Rust 611/611 PASS; full emit byte-equal Y at 13899B.
6. Lock: Relock once → `58b9ca6ef16f3ee4…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-80: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_605..H_612), writing `parallel-batch-80-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: finish ADD-IMM 52 1C8, then SUB-IMM 50/51/52 1C8, next imm ladder 1D0…, SET/GET fresh, etc. After batch-80 scratches done: parent next = body-extend-086 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-80-SPAWN.md` (no Task tool on this consolidator). Deferred carry: ADD-IMM 52 1C8; SUB-IMM 50/51/52 1C8; next imm ladder 1D0….

# body-extend-050 Log · parallel-batch-44 consolidation (H_326..H_333)

> Tag: `body-extend-050-EXPERIMENTAL-batch44-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-44-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `69adc5a0b11c8f17…` → `1566906f85667e97…`.
> **handler count: 332 → 340** (+8 at selectors 0x14C..0x153 via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_326 | 0x14C | 0x80 LDB | 50 60 D8 | 26 | `661c8bfff21fc20e` |
| H_327 | 0x14D | 0x80 LDB | 51 60 D8 | 26 | `d9fa04f9279ab0fe` |
| H_328 | 0x14E | 0x80 LDB | 52 60 D8 | 26 | `f155284380f7580d` |
| H_329 | 0x14F | 0x62 ADD-IMM | 50 C8 | 22 | `1ecdb5e66e168372` |
| H_330 | 0x150 | 0x62 ADD-IMM | 51 C8 | 22 | `5705b35865532f87` |
| H_331 | 0x151 | 0x62 ADD-IMM | 52 C8 | 22 | `863fee834853a91a` |
| H_332 | 0x152 | 0x61 SUB-IMM | 50 C8 | 22 | `521f857a16de934d` |
| H_333 | 0x153 | 0x61 SUB-IMM | 51 C8 | 22 | `5692683211522a54` |

**REJECTED (not added):** none (batch-44 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 14C`..`40 153` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_329..H_331 ADD-IMM imm=0xC8 use imm32 (`48 81 c0 …`), pin 22B. H_332..H_333 SUB-IMM imm=0xC8 use imm32 (`48 81 e8 …`), pin 22B. H_326..H_328 LDB oo≥0x80 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_326..H_333 at selectors 0x14C..0x153 (`40 14C`..`40 153`). Not RAW_BYTE; mirrors H_318..H_325 comment style (body-extend-050 / parallel-batch-44).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5060_d8,ldb_5160_d8,ldb_5260_d8,addimm_h50_c8,addimm_h51_c8,addimm_h52_c8,subimm_h50_c8,subimm_h51_c8}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **324/324 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **332/332 PASS**.
- Full canonical emit: JS=Rust=**7401B** code (was 7213B; +188B); byte-equal **Y**; sha `f30d9c8c2bb304e2…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `1566906f85667e97…`; previous chained to `69adc5a0b11c8f17…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=7680; both peers code=7401). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-44 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_326..H_333 at selectors 0x14C..0x153.
4. Selftest: exact pins PASS (26/26/26/22/22/22/22/22B).
5. Goldens: JS 324/324 and Rust 332/332 PASS; full emit byte-equal Y at 7401B.
6. Lock: Relock once → `1566906f85667e97…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-45: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_326..H_333), writing `parallel-batch-45-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: SUB-IMM 52 C8 (finish C8 triad), ADD/SUB-IMM imm=D0 triad, LDB oo=E0 triad, etc. After batch-45 scratches done: parent next = body-extend-051 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-45-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

# body-extend-058 Log · parallel-batch-52 consolidation (H_390..H_397)

> Tag: `body-extend-058-EXPERIMENTAL-batch52-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-52-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `0643c8f550fbb85d…` → `c258ff3271396e18…`.
> **handler count: 396 → 404** (+8 at selectors 0x18C..0x193 via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_390 | 0x18C | 0x61 SUB-IMM | 51 100 | 22 | `114da116f5fa5311` |
| H_391 | 0x18D | 0x61 SUB-IMM | 52 100 | 22 | `3f28a582a9c075b7` |
| H_392 | 0x18E | 0x80 LDB | 50 60 108 | 26 | `bdf235d9350d7497` |
| H_393 | 0x18F | 0x80 LDB | 51 60 108 | 26 | `3b65bdaff0e56bf1` |
| H_394 | 0x190 | 0x80 LDB | 52 60 108 | 26 | `86e5cf11a57df77e` |
| H_395 | 0x191 | 0x62 ADD-IMM | 50 108 | 22 | `fc5f70d4e243183e` |
| H_396 | 0x192 | 0x62 ADD-IMM | 51 108 | 22 | `d00fb3f6020656aa` |
| H_397 | 0x193 | 0x62 ADD-IMM | 52 108 | 22 | `2ddfc84367ac3ec1` |

**REJECTED (not added):** none (batch-52 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 18C`..`40 193` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_390..H_391 SUB-IMM imm=0x100 use imm32 (`48 81 e8 …`), pin 22B. H_395..H_397 ADD-IMM imm=0x108 use imm32 (`48 81 c0 …`), pin 22B. H_392..H_394 LDB oo=0x108 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_390..H_397 at selectors 0x18C..0x193 (`40 18C`..`40 193`). Not RAW_BYTE; mirrors H_382..H_389 comment style (body-extend-058 / parallel-batch-52).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h51_100,subimm_h52_100,ldb_5060_108,ldb_5160_108,ldb_5260_108,addimm_h50_108,addimm_h51_108,addimm_h52_108}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **388/388 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **396/396 PASS**.
- Full canonical emit: JS=Rust=**8881B** code (was 8693B; +188B); byte-equal **Y**; sha `4e7484c20dae5f27…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `c258ff3271396e18…`; previous chained to `0643c8f550fbb85d…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=9216; both peers code=8881; hash_a=hash_b). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-057 measured DIFFER; this beat EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-52 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_390..H_397 at selectors 0x18C..0x193.
4. Selftest: exact pins PASS (22/22/26/26/26/22/22/22B).
5. Goldens: JS 388/388 and Rust 396/396 PASS; full emit byte-equal Y at 8881B.
6. Lock: Relock once → `c258ff3271396e18…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-53: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_390..H_397), writing `parallel-batch-53-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: SUB-IMM * 108 triad (finish 108 SUB after ADD 108), LDB/ADD-IMM imm=110, etc. After batch-53 scratches done: parent next = body-extend-059 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-53-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

# body-extend-059 Log · parallel-batch-53 consolidation (H_398..H_405)

> Tag: `body-extend-059-EXPERIMENTAL-batch53-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-53-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `c258ff3271396e18…` → `bd7bad15e53fe296…`.
> **handler count: 404 → 412** (+8 at selectors 0x194..0x19B via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_398 | 0x194 | 0x61 SUB-IMM | 50 108 | 22 | `f139f28243c08957` |
| H_399 | 0x195 | 0x61 SUB-IMM | 51 108 | 22 | `f9c122832287170d` |
| H_400 | 0x196 | 0x61 SUB-IMM | 52 108 | 22 | `2f027342f5447eeb` |
| H_401 | 0x197 | 0x80 LDB | 50 60 110 | 26 | `215fc443528e6163` |
| H_402 | 0x198 | 0x80 LDB | 51 60 110 | 26 | `bfd294f2e3edf3d2` |
| H_403 | 0x199 | 0x80 LDB | 52 60 110 | 26 | `d9d4fceaca2783f1` |
| H_404 | 0x19A | 0x62 ADD-IMM | 50 110 | 22 | `b2f08439005e085c` |
| H_405 | 0x19B | 0x62 ADD-IMM | 51 110 | 22 | `2c0923f7af81d76c` |

**REJECTED (not added):** none (batch-53 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 194`..`40 19B` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_398..H_400 SUB-IMM imm=0x108 use imm32 (`48 81 e8 …`), pin 22B. H_404..H_405 ADD-IMM imm=0x110 use imm32 (`48 81 c0 …`), pin 22B. H_401..H_403 LDB oo=0x110 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_398..H_405 at selectors 0x194..0x19B (`40 194`..`40 19B`). Not RAW_BYTE; mirrors H_390..H_397 comment style (body-extend-059 / parallel-batch-53).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h50_108,subimm_h51_108,subimm_h52_108,ldb_5060_110,ldb_5160_110,ldb_5260_110,addimm_h50_110,addimm_h51_110}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **396/396 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **404/404 PASS**.
- Full canonical emit: JS=Rust=**9069B** code (was 8881B; +188B); byte-equal **Y**; sha `2f1c6c4dfbf56ba2…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `bd7bad15e53fe296…`; previous chained to `c258ff3271396e18…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=9216; both peers code=9069; hash_a=hash_b). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-058 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-53 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_398..H_405 at selectors 0x194..0x19B.
4. Selftest: exact pins PASS (22/22/22/26/26/26/22/22B).
5. Goldens: JS 396/396 and Rust 404/404 PASS; full emit byte-equal Y at 9069B.
6. Lock: Relock once → `bd7bad15e53fe296…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-54: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_398..H_405), writing `parallel-batch-54-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: ADD-IMM 52 110 (finish 110 ADD triad), SUB-IMM * 110 triad, LDB oo=118 triad, etc. After batch-54 scratches done: parent next = body-extend-060 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-54-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

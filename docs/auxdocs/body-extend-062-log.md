# body-extend-062 Log · parallel-batch-56 consolidation (H_422..H_429)

> Tag: `body-extend-062-EXPERIMENTAL-batch56-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-56-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `d4437da8f517c8d3…` → `c5b95f3792afa572…`.
> **handler count: 428 → 436** (+8 at selectors 0x1AC..0x1B3 via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_422 | 0x1AC | 0x62 ADD-IMM | 50 120 | 22 | `ec142e42a7c76bc5` |
| H_423 | 0x1AD | 0x62 ADD-IMM | 51 120 | 22 | `98a5ad08376f8e1a` |
| H_424 | 0x1AE | 0x62 ADD-IMM | 52 120 | 22 | `4ffb72a7006ad4be` |
| H_425 | 0x1AF | 0x61 SUB-IMM | 50 120 | 22 | `ac80c150be69c45f` |
| H_426 | 0x1B0 | 0x61 SUB-IMM | 51 120 | 22 | `63cc573f936e533d` |
| H_427 | 0x1B1 | 0x61 SUB-IMM | 52 120 | 22 | `587d869f509256fb` |
| H_428 | 0x1B2 | 0x80 LDB | 50 60 128 | 26 | `753ecfc2db0ae0be` |
| H_429 | 0x1B3 | 0x80 LDB | 51 60 128 | 26 | `6aa74dbb4c649602` |

**REJECTED (not added):** none (batch-56 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 1AC`..`40 1B3` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_422..H_424 ADD-IMM imm=0x120 use imm32 (`48 81 c0 …`), pin 22B. H_425..H_427 SUB-IMM imm=0x120 use imm32 (`48 81 e8 …`), pin 22B. H_428..H_429 LDB oo=0x128 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_422..H_429 at selectors 0x1AC..0x1B3 (`40 1AC`..`40 1B3`). Not RAW_BYTE; mirrors H_414..H_421 comment style (body-extend-062 / parallel-batch-56).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h50_120,addimm_h51_120,addimm_h52_120,subimm_h50_120,subimm_h51_120,subimm_h52_120,ldb_5060_128,ldb_5160_128}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **420/420 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **428/428 PASS**.
- Full canonical emit: JS=Rust=**9629B** code (was 9445B; +184B); byte-equal **Y**; sha `62c08677268583ed…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `c5b95f3792afa572…`; previous chained to `d4437da8f517c8d3…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=9728; both peers code=9629; hash_a=hash_b). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-061 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-56 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_422..H_429 at selectors 0x1AC..0x1B3.
4. Selftest: exact pins PASS (22/22/22/22/22/22/26/26B).
5. Goldens: JS 420/420 and Rust 428/428 PASS; full emit byte-equal Y at 9629B.
6. Lock: Relock once → `c5b95f3792afa572…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-57: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_422..H_429), writing `parallel-batch-57-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: LDB 52 60 128 (finish 128 LDB triad), ADD-IMM / SUB-IMM imm=128 triad (slots 50/51/52), etc. After batch-57 scratches done: parent next = body-extend-063 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-57-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

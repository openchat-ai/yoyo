# body-extend-064 Log · parallel-batch-58 consolidation (H_437..H_444)

> Tag: `body-extend-064-EXPERIMENTAL-batch58-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-58-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `f4fa77a59520fda0…` → `d9aff9ed76e4f649…`.
> **handler count: 443 → 451** (+8 at selectors 0x1BB..0x1C2 via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_437 | 0x1BB | 0x80 LDB | 50 60 130 | 26 | `31e8129afecd8ba8` |
| H_438 | 0x1BC | 0x80 LDB | 51 60 130 | 26 | `d0ba625ab36e77ee` |
| H_439 | 0x1BD | 0x80 LDB | 52 60 130 | 26 | `addb80d146c8758b` |
| H_440 | 0x1BE | 0x62 ADD-IMM | 50 130 | 22 | `dd2d08fe3b6bdad6` |
| H_441 | 0x1BF | 0x62 ADD-IMM | 51 130 | 22 | `e7e0160df815fc7d` |
| H_442 | 0x1C0 | 0x62 ADD-IMM | 52 130 | 22 | `ca98b1cd15714881` |
| H_443 | 0x1C1 | 0x61 SUB-IMM | 50 130 | 22 | `c505da6e0e035cb4` |
| H_444 | 0x1C2 | 0x61 SUB-IMM | 51 130 | 22 | `cfe5afe593eb6bf8` |

**REJECTED (not added):** none (batch-58 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 1BB`..`40 1C2` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_437..H_439 LDB oo=0x130 use imm32 (`48 81 c0 …`), pin 26B (starts 130 LDB triad). H_440..H_442 ADD-IMM imm=0x130 use imm32 (`48 81 c0 …`), pin 22B. H_443..H_444 SUB-IMM imm=0x130 use imm32 (`48 81 e8 …`), pin 22B (starts 130 SUB triad; H_445 SUB-IMM 52 130 deferred to next batch).

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_437..H_444 at selectors 0x1BB..0x1C2 (`40 1BB`..`40 1C2`). Not RAW_BYTE; mirrors H_430..H_436 comment style (body-extend-064 / parallel-batch-58).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5060_130,ldb_5160_130,ldb_5260_130,addimm_h50_130,addimm_h51_130,addimm_h52_130,subimm_h50_130,subimm_h51_130}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **435/435 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **443/443 PASS**.
- Full canonical emit: JS=Rust=**9975B** code (was 9787B; +188B); byte-equal **Y**; sha `fddfb8940784c605…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `d9aff9ed76e4f649…`; previous chained to `f4fa77a59520fda0…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=10240; both peers code=9975; hash_a=hash_b=`bcd1e369f3ed55f0…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-063 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-58 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_437..H_444 at selectors 0x1BB..0x1C2.
4. Selftest: exact pins PASS (26/26/26/22/22/22/22/22B).
5. Goldens: JS 435/435 and Rust 443/443 PASS; full emit byte-equal Y at 9975B.
6. Lock: Relock once → `d9aff9ed76e4f649…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-59: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_437..H_444), writing `parallel-batch-59-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: SUB-IMM 52 130 (finish 130 SUB triad), LDB oo=138 triad, ADD-IMM / SUB-IMM imm=138, SET/GET fresh, etc. After batch-59 scratches done: parent next = body-extend-065 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-59-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

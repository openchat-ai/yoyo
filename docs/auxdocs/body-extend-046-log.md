# body-extend-046 Log · parallel-batch-40 consolidation (H_294..H_301)

> Tag: `body-extend-046-EXPERIMENTAL-batch40-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-40-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `8c80a6fa783440b2…` → `422c843275989ac3…`.
> **handler count: 300 → 308** (+8 at selectors 0x12C..0x133 via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_294 | 0x12C | 0x80 LDB | 51 60 C0 | 26 | `452adbaebbd767ae` |
| H_295 | 0x12D | 0x80 LDB | 52 60 C0 | 26 | `766e4e7e953a3e88` |
| H_296 | 0x12E | 0x62 ADD-IMM | 50 A8 | 22 | `6fb232e091ad8e33` |
| H_297 | 0x12F | 0x62 ADD-IMM | 51 A8 | 22 | `0eac0a774b9d0193` |
| H_298 | 0x130 | 0x62 ADD-IMM | 52 A8 | 22 | `1acbcee68dee9520` |
| H_299 | 0x131 | 0x61 SUB-IMM | 50 A8 | 22 | `f1d0cdaaa848cd64` |
| H_300 | 0x132 | 0x61 SUB-IMM | 51 A8 | 22 | `446a3deafbac2416` |
| H_301 | 0x133 | 0x61 SUB-IMM | 52 A8 | 22 | `254705f23c21fb17` |

**REJECTED (not added):** none (batch-40 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 12C`..`40 133` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_296..H_298 ADD-IMM imm=0xA8 use imm32 (`48 81 c0 …`), pin 22B. H_299..H_301 SUB-IMM imm=0xA8 use imm32 (`48 81 e8 …`), pin 22B. H_294/H_295 LDB oo≥0x80 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_294..H_301 at selectors 0x12C..0x133 (`40 12C`..`40 133`). Not RAW_BYTE; mirrors H_286..H_293 comment style (body-extend-046 / parallel-batch-40).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5160_c0,ldb_5260_c0,addimm_h50_a8,addimm_h51_a8,addimm_h52_a8,subimm_h50_a8,subimm_h51_a8,subimm_h52_a8}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **292/292 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **300/300 PASS**.
- Full canonical emit: JS=Rust=**6661B** code (was 6477B; +184B); byte-equal **Y**; sha `2ee6fb8c639bcbb2…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `422c843275989ac3…`; previous chained to `8c80a6fa783440b2…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=7168; both peers code=6661). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-40 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_294..H_301 at selectors 0x12C..0x133.
4. Selftest: exact pins PASS (26/26/22/22/22/22/22/22B).
5. Goldens: JS 292/292 and Rust 300/300 PASS; full emit byte-equal Y at 6661B.
6. Lock: Relock once → `422c843275989ac3…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-41: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_294..H_301), writing `parallel-batch-41-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: LDB oo=C8 triad (dd=50/51/52 ss=60), ADD/SUB-IMM imm=B0 triad, etc. After batch-41 scratches done: parent next = body-extend-047 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-41-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

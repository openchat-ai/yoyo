# body-extend-043 Log · parallel-batch-37 consolidation (H_270..H_277)

> Tag: `body-extend-043-EXPERIMENTAL-batch37-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-37-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `afceb388015dd4a7…` → `113decd0cbfa7a11…`.
> **handler count: 276 → 284** (+8 at selectors 0x114..0x11B via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_270 | 0x114 | 0x62 ADD-IMM | 51 90 | 22 | `30d80ac5f98d5b91` |
| H_271 | 0x115 | 0x62 ADD-IMM | 52 90 | 22 | `1f4ed4e242ed21b3` |
| H_272 | 0x116 | 0x61 SUB-IMM | 50 90 | 22 | `5108f62107ced6f5` |
| H_273 | 0x117 | 0x61 SUB-IMM | 51 90 | 22 | `07c48bf0e15bc2fd` |
| H_274 | 0x118 | 0x61 SUB-IMM | 52 90 | 22 | `ce43fa09ae8fd687` |
| H_275 | 0x119 | 0x80 LDB | 50 60 B0 | 26 | `64f22f32cf0fab77` |
| H_276 | 0x11A | 0x80 LDB | 51 60 B0 | 26 | `8de79951c51e9c4a` |
| H_277 | 0x11B | 0x80 LDB | 52 60 B0 | 26 | `24662dc0540eff95` |

**REJECTED (not added):** none (batch-37 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 114`..`40 11B` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_270/H_271 ADD-IMM imm=0x90 use imm32 (`48 81 c0 …`), pin 22B. H_272..H_274 SUB-IMM imm=0x90 use imm32 (`48 81 e8 …`), pin 22B. H_275..H_277 LDB oo≥0x80 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_270..H_277 at selectors 0x114..0x11B (`40 114`..`40 11B`). Not RAW_BYTE; mirrors H_262..H_269 comment style (body-extend-043 / parallel-batch-37).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h51_90,addimm_h52_90,subimm_h50_90,subimm_h51_90,subimm_h52_90,ldb_5060_b0,ldb_5160_b0,ldb_5260_b0}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **268/268 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **276/276 PASS**.
- Full canonical emit: JS=Rust=**6109B** code (was 5921B; +188B); byte-equal **Y**; sha `654e2d71a759563b…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `113decd0cbfa7a11…`; previous chained to `afceb388015dd4a7…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=6144; both peers code=6109). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-37 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_270..H_277 at selectors 0x114..0x11B.
4. Selftest: exact pins PASS (22/22/22/22/22/26/26/26B).
5. Goldens: JS 268/268 and Rust 276/276 PASS; full emit byte-equal Y at 6109B.
6. Lock: Relock once → `113decd0cbfa7a11…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-38: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_270..H_277), writing `parallel-batch-38-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: ADD/SUB-IMM imm=98 triad, LDB oo=B8 triad, etc. After batch-38 scratches done: parent next = body-extend-044 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-38-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

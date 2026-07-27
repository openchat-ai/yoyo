# body-extend-051 Log · parallel-batch-45 consolidation (H_334..H_341)

> Tag: `body-extend-051-EXPERIMENTAL-batch45-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-45-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `1566906f85667e97…` → `ee5b881e34301f79…`.
> **handler count: 340 → 348** (+8 at selectors 0x154..0x15B via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_334 | 0x154 | 0x61 SUB-IMM | 52 C8 | 22 | `3b32f5875666e837` |
| H_335 | 0x155 | 0x62 ADD-IMM | 50 D0 | 22 | `5cdff426638d0c76` |
| H_336 | 0x156 | 0x62 ADD-IMM | 51 D0 | 22 | `a4c8fb5e23221fc9` |
| H_337 | 0x157 | 0x62 ADD-IMM | 52 D0 | 22 | `d3a3f45f884525f8` |
| H_338 | 0x158 | 0x61 SUB-IMM | 50 D0 | 22 | `308c801c542d857b` |
| H_339 | 0x159 | 0x61 SUB-IMM | 51 D0 | 22 | `744b3918b3f5fe8e` |
| H_340 | 0x15A | 0x61 SUB-IMM | 52 D0 | 22 | `ee26c6478e1bedb5` |
| H_341 | 0x15B | 0x80 LDB | 50 60 E0 | 26 | `3fcfa899104fe81a` |

**REJECTED (not added):** none (batch-45 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 154`..`40 15B` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_335..H_337 ADD-IMM imm=0xD0 use imm32 (`48 81 c0 …`), pin 22B. H_334 / H_338..H_340 SUB-IMM imm=0xC8/0xD0 use imm32 (`48 81 e8 …`), pin 22B. H_341 LDB oo≥0x80 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_334..H_341 at selectors 0x154..0x15B (`40 154`..`40 15B`). Not RAW_BYTE; mirrors H_326..H_333 comment style (body-extend-051 / parallel-batch-45).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h52_c8,addimm_h50_d0,addimm_h51_d0,addimm_h52_d0,subimm_h50_d0,subimm_h51_d0,subimm_h52_d0,ldb_5060_e0}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **332/332 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **340/340 PASS**.
- Full canonical emit: JS=Rust=**7581B** code (was 7401B; +180B); byte-equal **Y**; sha `eb95f20ca7d4ed87…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `ee5b881e34301f79…`; previous chained to `1566906f85667e97…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=7680; both peers code=7581). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-45 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_334..H_341 at selectors 0x154..0x15B.
4. Selftest: exact pins PASS (22/22/22/22/22/22/22/26B).
5. Goldens: JS 332/332 and Rust 340/340 PASS; full emit byte-equal Y at 7581B.
6. Lock: Relock once → `ee5b881e34301f79…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-46: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_334..H_341), writing `parallel-batch-46-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: LDB 51/52 60 E0 (finish E0 triad), ADD/SUB-IMM imm=D8 triad, etc. After batch-46 scratches done: parent next = body-extend-052 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-46-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

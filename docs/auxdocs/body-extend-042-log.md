# body-extend-042 Log · parallel-batch-36 consolidation (H_262..H_269)

> Tag: `body-extend-042-EXPERIMENTAL-batch36-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-36-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `4cb656812b03c0fd…` → `afceb388015dd4a7…`.
> **handler count: 268 → 276** (+8 at selectors 0x10C..0x113 via label-width A).

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_262 | 0x10C | 0x62 ADD-IMM | 52 88 | 22 | `97f31856e0e0bace` |
| H_263 | 0x10D | 0x61 SUB-IMM | 50 88 | 22 | `031eecb381c11df4` |
| H_264 | 0x10E | 0x61 SUB-IMM | 51 88 | 22 | `e032f65c781b8d24` |
| H_265 | 0x10F | 0x61 SUB-IMM | 52 88 | 22 | `a35fd747b10ad6c0` |
| H_266 | 0x110 | 0x80 LDB | 50 60 A8 | 26 | `9406298c7e1a9bb7` |
| H_267 | 0x111 | 0x80 LDB | 51 60 A8 | 26 | `21a57bbe40cd51a3` |
| H_268 | 0x112 | 0x80 LDB | 52 60 A8 | 26 | `6ce7678316409535` |
| H_269 | 0x113 | 0x62 ADD-IMM | 50 90 | 22 | `606ca6ba641f5721` |

**REJECTED (not added):** none (batch-36 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 10C`..`40 113` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_262/H_269 ADD-IMM imm=0x88/0x90 use imm32 (`48 81 c0 …`), pin 22B. H_263..H_265 SUB-IMM imm=0x88 use imm32 (`48 81 e8 …`), pin 22B. H_266..H_268 LDB oo≥0x80 use imm32 path (`48 81 c0 …`), pin 26B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_262..H_269 at selectors 0x10C..0x113 (`40 10C`..`40 113`). Not RAW_BYTE; mirrors H_254..H_261 comment style (body-extend-042 / parallel-batch-36).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h52_88,subimm_h50_88,subimm_h51_88,subimm_h52_88,ldb_5060_a8,ldb_5160_a8,ldb_5260_a8,addimm_h50_90}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **260/260 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **268/268 PASS**.
- Full canonical emit: JS=Rust=**5921B** code (was 5733B; +188B); byte-equal **Y**; sha `c481e524087865b1…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `afceb388015dd4a7…`; previous chained to `4cb656812b03c0fd…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=6144; both peers code=5921). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, or GREEN claim. Label-width peers not re-touched this beat.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-36 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained (label-width A already landed).
3. Hand-author: H_262..H_269 at selectors 0x10C..0x113.
4. Selftest: exact pins PASS (22/22/22/22/26/26/26/22B).
5. Goldens: JS 260/260 and Rust 268/268 PASS; full emit byte-equal Y at 5921B.
6. Lock: Relock once → `afceb388015dd4a7…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-37: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_262..H_269), writing `parallel-batch-37-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: ADD-IMM 51/52 90, SUB-IMM * 90 triad, LDB oo=B0 triad, etc. After batch-37 scratches done: parent next = body-extend-043 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-37-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

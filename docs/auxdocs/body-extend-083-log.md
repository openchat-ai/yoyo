# body-extend-083 Log · parallel-batch-77 consolidation (H_589..H_596)

> Tag: `body-extend-083-EXPERIMENTAL-batch77-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-77-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `05a3a9c6693fa65c…` → `45dff031e2acfa0e…`.
> **handler count: 595 → 603** (+8 at selectors 0x253..0x25A via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_589 | 0x253 | 0x61 SUB-IMM | 52 1B0 | 22 | `6b09f5d585880e4e` |
| H_590 | 0x254 | 0x80 LDB | 50 60 1B8 | 26 | `991bc7cddb01b0d2` |
| H_591 | 0x255 | 0x80 LDB | 51 60 1B8 | 26 | `eb823184d5b340f6` |
| H_592 | 0x256 | 0x80 LDB | 52 60 1B8 | 26 | `4769bc5c1af2f770` |
| H_593 | 0x257 | 0x62 ADD-IMM | 50 1B8 | 22 | `8670afebb32cc65e` |
| H_594 | 0x258 | 0x62 ADD-IMM | 51 1B8 | 22 | `46ee1e357ab8ae14` |
| H_595 | 0x259 | 0x62 ADD-IMM | 52 1B8 | 22 | `a95def3bbb47b285` |
| H_596 | 0x25A | 0x61 SUB-IMM | 50 1B8 | 22 | `ab8ef8aa14a41432` |

**REJECTED (not added):** none (batch-77 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 253`..`40 25A` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_589 finishes 1B0 SUB triad. H_590..H_592 start/finish 1B8 LDB triad. H_593..H_595 start/finish 1B8 ADD triad. H_596 starts 1B8 SUB triad (slots 51/52 deferred).

**Deferred (not added this beat):** SUB-IMM slot=51/52 imm=1B8; LDB 50/51/52 60 1C0; ADD-IMM/SUB-IMM slot=50/51/52 imm=1C0 — suggested for parallel-batch-78.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1B8 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1B8 uses imm32 → 26B pins.
SUB-IMM imm=0x1B0 uses imm32 → 22B pin.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_589..H_596 at selectors 0x253..0x25A (`40 253`..`40 25A`). Not RAW_BYTE; mirrors H_581..H_588 comment style (body-extend-083 / parallel-batch-77).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h52_1B0,ldb_5060_1B8,ldb_5160_1B8,ldb_5260_1B8,addimm_h50_1B8,addimm_h51_1B8,addimm_h52_1B8,subimm_h50_1B8}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **587/587 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **595/595 PASS**.
- Full canonical emit: JS=Rust=**13523B** code (was 13335B; +188B); byte-equal **Y**; sha `d867bd40a8a6e492…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `45dff031e2acfa0e…`; previous chained to `05a3a9c6693fa65c…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=13824; both peers code=13523; hash_a=hash_b=`d347ad0aee449906…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-082 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-77 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_589..H_596 at selectors 0x253..0x25A.
4. Selftest: exact pins PASS (22/26/26/26/22/22/22/22B).
5. Goldens: JS 587/587 and Rust 595/595 PASS; full emit byte-equal Y at 13523B.
6. Lock: Relock once → `45dff031e2acfa0e…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-78: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_589..H_596), writing `parallel-batch-78-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: finish SUB-IMM 51/52 1B8, then LDB 50/51/52 60 1C0, ADD-IMM/SUB-IMM imm=1C0 triad, SET/GET fresh, etc. After batch-78 scratches done: parent next = body-extend-084 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-78-SPAWN.md` (no Task tool on this consolidator). Deferred carry: SUB-IMM 51/52 1B8; LDB 50/51/52 60 1C0; ADD-IMM/SUB-IMM slot=50/51/52 imm=1C0.

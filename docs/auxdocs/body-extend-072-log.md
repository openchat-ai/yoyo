# body-extend-072 Log · parallel-batch-66 consolidation (H_501..H_508)

> Tag: `body-extend-072-EXPERIMENTAL-batch66-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-66-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `1f070530a91ca949…` → `e1554db8dcce9946…`.
> **handler count: 507 → 515** (+8 at selectors 0x1FB..0x202 via label-width A).
> **LABEL_CAP: 512 → 1024** (Rust fixup table; ids ≥0x200 required for H_506..H_508; JS Map already 0..0xffff).

## 0. Label-cap bump (fail-closed parity)

H_506..H_508 require selectors `40 200`..`40 202`. Prior Rust `LABEL_CAP=512` rejected `hh≥512` (`LabelOutOfRange`) while JS `Map` accepted — **peer divergence**. Minimal widen this beat: Rust `fixup.rs` `LABEL_CAP` 512→1024 (+ unit test `define_past_512_no_wrap`). No PROMPT edit / no trusted-encoder redesign / no GREEN claim. Still EXPERIMENTAL · NON-GREEN.

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_501 | 0x1FB | 0x80 LDB | 51 60 168 | 26 | `71614ed8ee72059f` |
| H_502 | 0x1FC | 0x80 LDB | 52 60 168 | 26 | `b40ac7b90a6c8cb3` |
| H_503 | 0x1FD | 0x62 ADD-IMM | 50 168 | 22 | `70dcc769354c9c59` |
| H_504 | 0x1FE | 0x62 ADD-IMM | 51 168 | 22 | `ae42aee20a8d8c9f` |
| H_505 | 0x1FF | 0x62 ADD-IMM | 52 168 | 22 | `7109bea20936a27a` |
| H_506 | 0x200 | 0x61 SUB-IMM | 50 168 | 22 | `5b1652dbeda9a005` |
| H_507 | 0x201 | 0x61 SUB-IMM | 51 168 | 22 | `2d56b2a1e2d5c002` |
| H_508 | 0x202 | 0x61 SUB-IMM | 52 168 | 22 | `f442c8a07cbb8382` |

**REJECTED (not added):** none (batch-66 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 1FB`..`40 202` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_501/H_502 finish 168 LDB triad (slots 51/52; H_500=50). H_503..H_505 start 168 ADD triad (imm32 22B). H_506..H_508 start 168 SUB triad (imm32 22B).

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x168 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x168 uses imm32 → 26B pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_501..H_508 at selectors 0x1FB..0x202 (`40 1FB`..`40 202`). Not RAW_BYTE; mirrors H_493..H_500 comment style (body-extend-072 / parallel-batch-66).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5160_168,ldb_5260_168,addimm_h50_168,addimm_h51_168,addimm_h52_168,subimm_h50_168,subimm_h51_168,subimm_h52_168}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **499/499 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok; after LABEL_CAP bump).
- Rust golden: 8 `check_selfhost_min_*` — **507/507 PASS**.
- Full canonical emit: JS=Rust=**11467B** code (was 11283B; +184B); byte-equal **Y**; sha `f4dcc789ccc97028…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `e1554db8dcce9946…`; previous chained to `1f070530a91ca949…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=11776; both peers code=11467; hash_a=hash_b=`4d2c7f2b8074a4a6…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-071 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-66 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP 512→1024 for ids ≥0x200 (JS already Map 0..0xffff).
3. Hand-author: H_501..H_508 at selectors 0x1FB..0x202.
4. Selftest: exact pins PASS (26/26/22/22/22/22/22/22B).
5. Goldens: JS 499/499 and Rust 507/507 PASS; full emit byte-equal Y at 11467B.
6. Lock: Relock once → `e1554db8dcce9946…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-67: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_501..H_508), writing `parallel-batch-67-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: LDB oo=170 triad (50/51/52 60 170), ADD-IMM / SUB-IMM imm=170 triad, SET/GET fresh, etc. After batch-67 scratches done: parent next = body-extend-073 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-67-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

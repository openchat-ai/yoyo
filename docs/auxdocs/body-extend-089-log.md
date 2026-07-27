# body-extend-089 Log · parallel-batch-83 consolidation (H_637..H_644)

> Tag: `body-extend-089-EXPERIMENTAL-batch83-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-83-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `697ad7847ba15e82…` → `e8500277650750c5…`.
> **handler count: 643 → 651** (+8 at selectors 0x283..0x28A via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_637 | 0x283 | 0x80 LDB | 52 60 1E0 | 26 | `a8e2361d68cd8eae` |
| H_638 | 0x284 | 0x62 ADD-IMM | 50 1E0 | 22 | `f8386b9a462dfb05` |
| H_639 | 0x285 | 0x62 ADD-IMM | 51 1E0 | 22 | `1eba92f3a87f8de9` |
| H_640 | 0x286 | 0x62 ADD-IMM | 52 1E0 | 22 | `e15ba36fe8e77c0c` |
| H_641 | 0x287 | 0x61 SUB-IMM | 50 1E0 | 22 | `485f29f7f7612705` |
| H_642 | 0x288 | 0x61 SUB-IMM | 51 1E0 | 22 | `aceddcae0b9c827f` |
| H_643 | 0x289 | 0x61 SUB-IMM | 52 1E0 | 22 | `1641521a26d49973` |
| H_644 | 0x28A | 0x80 LDB | 50 60 1E8 | 26 | `6089535af769e9fe` |

**REJECTED (not added):** none (batch-83 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 283`..`40 28A` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_637 finishes 1E0 LDB triad. H_638..H_640 start/finish 1E0 ADD triad. H_641..H_643 start/finish 1E0 SUB triad. H_644 starts 1E8 LDB triad (dd=50; LDB 51/52 1E8 deferred).

**Deferred (not added this beat):** LDB dd=51/52 ss=60 oo=1E8; ADD-IMM / SUB-IMM slot=50/51/52 imm=1E8 — suggested for parallel-batch-84.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1E0 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1E0 / 0x1E8 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_637 | `a8e2361d68cd8eae6c92e118c9e39d49a60d9a09ae5069255f07a61cfc6c5cd0` |
| H_638 | `f8386b9a462dfb05f58cbf376c60a5c859566fdd49e199324cd680fe41c2ed09` |
| H_639 | `1eba92f3a87f8de92d8440f8638f1aa250783c5413bb0bd219fd3623324d8f8b` |
| H_640 | `e15ba36fe8e77c0ceee37fb2cddd486e6aa388f0986b8a8c99f3fd69bf0c2aeb` |
| H_641 | `485f29f7f7612705dbad5255c877b24d6e90c20ddba3266039bc4709b01836e7` |
| H_642 | `aceddcae0b9c827f4a8c54a6b402b433a8793dd97eb123aa66f110007b721a39` |
| H_643 | `1641521a26d49973a6b927072b1f7d18f933f02e7f527f704a8fdb52e1185779` |
| H_644 | `6089535af769e9fe003c54c3f2ec91e0d295c3d4cfaf4e6d8348fef1cdef0d6e` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_637..H_644 at selectors 0x283..0x28A (`40 283`..`40 28A`). Not RAW_BYTE; mirrors H_629..H_636 comment style (body-extend-089 / parallel-batch-83).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5260_1E0,addimm_h50_1E0,addimm_h51_1E0,addimm_h52_1E0,subimm_h50_1E0,subimm_h51_1E0,subimm_h52_1E0,ldb_5060_1E8}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **635/635 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **643/643 PASS**.
- Full canonical emit: JS=Rust=**14643B** code (was 14459B; +184B); byte-equal **Y**; sha `56abb12a7c46bf99…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `e8500277650750c5…`; previous chained to `697ad7847ba15e82…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=14848; both peers code=14643; hash_a=hash_b=`fc41f0ac1b5d6291…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-088 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-83 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_637..H_644 at selectors 0x283..0x28A.
4. Selftest: exact pins PASS (26/22/22/22/22/22/22/26B).
5. Goldens: JS 635/635 and Rust 643/643 PASS; full emit byte-equal Y at 14643B.
6. Lock: Relock once → `e8500277650750c5…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-84: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_637..H_644), writing `parallel-batch-84-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: finish LDB 51/52 60 1E8; start ADD-IMM/SUB-IMM 50/51/52 1E8; SET/GET fresh, etc. After batch-84 scratches done: parent next = body-extend-090 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-84-SPAWN.md` (no Task tool on this consolidator). Deferred carry: LDB 51/52 60 1E8; ADD-IMM/SUB-IMM 50/51/52 1E8.

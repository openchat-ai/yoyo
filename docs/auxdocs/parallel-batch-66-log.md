# parallel-batch-66 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-66-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-071 (pin `1f070530…`, handlers = 507, H_493..H_500 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-071 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_500 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x168 | `498b87000300004881c068010000480fb60049898788020000c3` (26) | same | same | Y | `71614ed8ee72059f` | `71614ed8ee72059f` | PASS |
| 2 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x168 | `498b87000300004881c068010000480fb60049898790020000c3` (26) | same | same | Y | `b40ac7b90a6c8cb3` | `b40ac7b90a6c8cb3` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x50 imm=0x168 | `498b87800200004881c06801000049898780020000c3` (22) | same | same | Y | `70dcc769354c9c59` | `70dcc769354c9c59` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x51 imm=0x168 | `498b87880200004881c06801000049898788020000c3` (22) | same | same | Y | `ae42aee20a8d8c9f` | `ae42aee20a8d8c9f` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x52 imm=0x168 | `498b87900200004881c06801000049898790020000c3` (22) | same | same | Y | `7109bea20936a27a` | `7109bea20936a27a` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x50 imm=0x168 | `498b87800200004881e86801000049898780020000c3` (22) | same | same | Y | `5b1652dbeda9a005` | `5b1652dbeda9a005` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x51 imm=0x168 | `498b87880200004881e86801000049898788020000c3` (22) | same | same | Y | `2d56b2a1e2d5c002` | `2d56b2a1e2d5c002` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x52 imm=0x168 | `498b87900200004881e86801000049898790020000c3` (22) | same | same | Y | `f442c8a07cbb8382` | `f442c8a07cbb8382` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x51 ss=0x60 oo=0x168 — **PASS**

- fixture: `_scratch_ldb_5160_168.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c068010000480fb60049898788020000c3`
- js-sha256: `71614ed8ee72059f9244f7935dc82742138ff7f26afddb6da756b8105659edb9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x52 ss=0x60 oo=0x168 — **PASS**

- fixture: `_scratch_ldb_5260_168.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c068010000480fb60049898790020000c3`
- js-sha256: `b40ac7b90a6c8cb3d4f4c0eddd0bb4be7d544ebe35a847e994a31a07b7751cd7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x50 imm=0x168 — **PASS**

- fixture: `_scratch_addimm_h50_168.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c06801000049898780020000c3`
- js-sha256: `70dcc769354c9c590aed7416d470b91ce3cc91eabdcc12c2dda567949bd57769`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x51 imm=0x168 — **PASS**

- fixture: `_scratch_addimm_h51_168.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c06801000049898788020000c3`
- js-sha256: `ae42aee20a8d8c9fa873f70fd087a2ff1fa8075b73fddae57eeec7d68d91c6c2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x52 imm=0x168 — **PASS**

- fixture: `_scratch_addimm_h52_168.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c06801000049898790020000c3`
- js-sha256: `7109bea20936a27a02373f431eaaa99ad49e81fc843b563184f145f1587119fb`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x50 imm=0x168 — **PASS**

- fixture: `_scratch_subimm_h50_168.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e86801000049898780020000c3`
- js-sha256: `5b1652dbeda9a005e3b805181701200116762b682340e405bf20c50cb8be893e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x51 imm=0x168 — **PASS**

- fixture: `_scratch_subimm_h51_168.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e86801000049898788020000c3`
- js-sha256: `2d56b2a1e2d5c002d0994c309e2b846f88c0ba07f360ded43be97618c02f0a28`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x52 imm=0x168 — **PASS**

- fixture: `_scratch_subimm_h52_168.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e86801000049898790020000c3`
- js-sha256: `f442c8a07cbb83826c533545bd5986c64332a717011034776c69921337416064`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=51/52 ss=60 oo=168 (finish 168 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=168 (start 168 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=168 (start 168 SUB triad; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 1FB`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5160_168.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_168.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_168.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_168.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_168.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_168.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_168.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_168.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-66-log.md` — this file
- `scripts/_probe/parallel-batch-66-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-072 serialize PASSes + 1 Relock**

Pass pin from body-extend-071 Relock: `1f070530a91ca949696f7552fc5d3b3690f00630a191ce25662ee33951314e41`.
Handlers before consolidate = 507 (H_00..H_500). Next selectors `40 1FB`.. for H_501.. if all serialize.

PASS list for body-extend-072:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_501 | 0x1FB | 0x80 LDB | 0x51 0x60 0x168 | `498b87000300004881c068010000480fb60049898788020000c3` (26B) | `71614ed8ee72059f` |
| H_502 | 0x1FC | 0x80 LDB | 0x52 0x60 0x168 | `498b87000300004881c068010000480fb60049898790020000c3` (26B) | `b40ac7b90a6c8cb3` |
| H_503 | 0x1FD | 0x62 ADD-IMM | 0x50 0x168 | `498b87800200004881c06801000049898780020000c3` (22B) | `70dcc769354c9c59` |
| H_504 | 0x1FE | 0x62 ADD-IMM | 0x51 0x168 | `498b87880200004881c06801000049898788020000c3` (22B) | `ae42aee20a8d8c9f` |
| H_505 | 0x1FF | 0x62 ADD-IMM | 0x52 0x168 | `498b87900200004881c06801000049898790020000c3` (22B) | `7109bea20936a27a` |
| H_506 | 0x200 | 0x61 SUB-IMM | 0x50 0x168 | `498b87800200004881e86801000049898780020000c3` (22B) | `5b1652dbeda9a005` |
| H_507 | 0x201 | 0x61 SUB-IMM | 0x51 0x168 | `498b87880200004881e86801000049898788020000c3` (22B) | `2d56b2a1e2d5c002` |
| H_508 | 0x202 | 0x61 SUB-IMM | 0x52 0x168 | `498b87900200004881e86801000049898790020000c3` (22B) | `f442c8a07cbb8382` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-071 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_500.
- If the parent decides to serialize, append H_501.. at selectors `40 1FB`..:
  - H_501 0x80 LDB (80 51 60 168) — pin `498b87000300004881c068010000480fb60049898788020000c3`
  - H_502 0x80 LDB (80 52 60 168) — pin `498b87000300004881c068010000480fb60049898790020000c3`
  - H_503 0x62 ADD-IMM (62 50 168) — pin `498b87800200004881c06801000049898780020000c3`
  - H_504 0x62 ADD-IMM (62 51 168) — pin `498b87880200004881c06801000049898788020000c3`
  - H_505 0x62 ADD-IMM (62 52 168) — pin `498b87900200004881c06801000049898790020000c3`
  - H_506 0x61 SUB-IMM (61 50 168) — pin `498b87800200004881e86801000049898780020000c3`
  - H_507 0x61 SUB-IMM (61 51 168) — pin `498b87880200004881e86801000049898788020000c3`
  - H_508 0x61 SUB-IMM (61 52 168) — pin `498b87900200004881e86801000049898790020000c3`
- Plus 1 Relock after append from pin `1f070530…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-072 serialize PASSes + 1 Relock

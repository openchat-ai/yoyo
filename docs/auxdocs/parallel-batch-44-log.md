# parallel-batch-44 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-44-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-049 (pin `69adc5a0…`, handlers = 332, H_318..H_325 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-049 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_325 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x50 ss=0x60 oo=0xD8 | `498b87000300004881c0d8000000480fb60049898780020000c3` (26) | same | same | Y | `661c8bfff21fc20e` | `661c8bfff21fc20e` | PASS |
| 2 | 0x80 LDB | dd=0x51 ss=0x60 oo=0xD8 | `498b87000300004881c0d8000000480fb60049898788020000c3` (26) | same | same | Y | `d9fa04f9279ab0fe` | `d9fa04f9279ab0fe` | PASS |
| 3 | 0x80 LDB | dd=0x52 ss=0x60 oo=0xD8 | `498b87000300004881c0d8000000480fb60049898790020000c3` (26) | same | same | Y | `f155284380f7580d` | `f155284380f7580d` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x50 imm=0xC8 | `498b87800200004881c0c800000049898780020000c3` (22) | same | same | Y | `1ecdb5e66e168372` | `1ecdb5e66e168372` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x51 imm=0xC8 | `498b87880200004881c0c800000049898788020000c3` (22) | same | same | Y | `5705b35865532f87` | `5705b35865532f87` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x52 imm=0xC8 | `498b87900200004881c0c800000049898790020000c3` (22) | same | same | Y | `863fee834853a91a` | `863fee834853a91a` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x50 imm=0xC8 | `498b87800200004881e8c800000049898780020000c3` (22) | same | same | Y | `521f857a16de934d` | `521f857a16de934d` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x51 imm=0xC8 | `498b87880200004881e8c800000049898788020000c3` (22) | same | same | Y | `5692683211522a54` | `5692683211522a54` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x50 ss=0x60 oo=0xD8 — **PASS**

- fixture: `_scratch_ldb_5060_d8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0d8000000480fb60049898780020000c3`
- js-sha256: `661c8bfff21fc20e65b1aea5e299c8711bc60815786c103043ece69116e2a489`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x51 ss=0x60 oo=0xD8 — **PASS**

- fixture: `_scratch_ldb_5160_d8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0d8000000480fb60049898788020000c3`
- js-sha256: `d9fa04f9279ab0fef7b3d1e724fa58465b4dd12191250cd94d62fbda95e6f474`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x52 ss=0x60 oo=0xD8 — **PASS**

- fixture: `_scratch_ldb_5260_d8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0d8000000480fb60049898790020000c3`
- js-sha256: `f155284380f7580d079e6c9083b7632ca21cacc683f54d34a880d8cec37fcac8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x50 imm=0xC8 — **PASS**

- fixture: `_scratch_addimm_h50_c8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0c800000049898780020000c3`
- js-sha256: `1ecdb5e66e168372702777d893dc24e666e1f5d661e49821a72554eacdfda622`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x51 imm=0xC8 — **PASS**

- fixture: `_scratch_addimm_h51_c8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0c800000049898788020000c3`
- js-sha256: `5705b35865532f87af5696d7c926a6137466e3c1b6650c47b9e7536ff32246c6`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x52 imm=0xC8 — **PASS**

- fixture: `_scratch_addimm_h52_c8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0c800000049898790020000c3`
- js-sha256: `863fee834853a91a3f217416203559e58d702249de7c79ef0158dada08946e66`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x50 imm=0xC8 — **PASS**

- fixture: `_scratch_subimm_h50_c8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8c800000049898780020000c3`
- js-sha256: `521f857a16de934dae3c5f6327db923510cb13422d3d128afc6d969e1bbdec3b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x51 imm=0xC8 — **PASS**

- fixture: `_scratch_subimm_h51_c8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8c800000049898788020000c3`
- js-sha256: `5692683211522a5477ebecb7908ae44c7593f28528208c28e4db791240905d73`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=50/51/52 ss=60 oo=D8 (next oo after D0 triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=C8 (fresh imm after C0; imm32 22B).
- SUB-IMM slot=50/51 imm=C8 (complements ADD-IMM * C8; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 14C`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5060_d8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_d8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_d8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_c8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_c8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_c8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_c8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_c8.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-44-log.md` — this file
- `scripts/_probe/parallel-batch-44-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-050 serialize PASSes + 1 Relock**

Pass pin from body-extend-049 Relock: `69adc5a0b11c8f176687deff6753b2fa51b6611c3cd1193c79bf1143b7b4c957`.
Handlers before consolidate = 332 (H_00..H_325). Next selectors `40 14C`.. for H_326.. if all serialize.

PASS list for body-extend-050:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_326 | 0x14C | 0x80 LDB | 0x50 0x60 0xD8 | `498b87000300004881c0d8000000480fb60049898780020000c3` (26B) | `661c8bfff21fc20e` |
| H_327 | 0x14D | 0x80 LDB | 0x51 0x60 0xD8 | `498b87000300004881c0d8000000480fb60049898788020000c3` (26B) | `d9fa04f9279ab0fe` |
| H_328 | 0x14E | 0x80 LDB | 0x52 0x60 0xD8 | `498b87000300004881c0d8000000480fb60049898790020000c3` (26B) | `f155284380f7580d` |
| H_329 | 0x14F | 0x62 ADD-IMM | 0x50 0xC8 | `498b87800200004881c0c800000049898780020000c3` (22B) | `1ecdb5e66e168372` |
| H_330 | 0x150 | 0x62 ADD-IMM | 0x51 0xC8 | `498b87880200004881c0c800000049898788020000c3` (22B) | `5705b35865532f87` |
| H_331 | 0x151 | 0x62 ADD-IMM | 0x52 0xC8 | `498b87900200004881c0c800000049898790020000c3` (22B) | `863fee834853a91a` |
| H_332 | 0x152 | 0x61 SUB-IMM | 0x50 0xC8 | `498b87800200004881e8c800000049898780020000c3` (22B) | `521f857a16de934d` |
| H_333 | 0x153 | 0x61 SUB-IMM | 0x51 0xC8 | `498b87880200004881e8c800000049898788020000c3` (22B) | `5692683211522a54` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-049 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_325.
- If the parent decides to serialize, append H_326.. at selectors `40 14C`..:
  - H_326 0x80 LDB (80 50 60 D8) — pin `498b87000300004881c0d8000000480fb60049898780020000c3`
  - H_327 0x80 LDB (80 51 60 D8) — pin `498b87000300004881c0d8000000480fb60049898788020000c3`
  - H_328 0x80 LDB (80 52 60 D8) — pin `498b87000300004881c0d8000000480fb60049898790020000c3`
  - H_329 0x62 ADD-IMM (62 50 C8) — pin `498b87800200004881c0c800000049898780020000c3`
  - H_330 0x62 ADD-IMM (62 51 C8) — pin `498b87880200004881c0c800000049898788020000c3`
  - H_331 0x62 ADD-IMM (62 52 C8) — pin `498b87900200004881c0c800000049898790020000c3`
  - H_332 0x61 SUB-IMM (61 50 C8) — pin `498b87800200004881e8c800000049898780020000c3`
  - H_333 0x61 SUB-IMM (61 51 C8) — pin `498b87880200004881e8c800000049898788020000c3`
- Plus 1 Relock after append from pin `69adc5a0…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-050 serialize PASSes + 1 Relock

# parallel-batch-85 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-85-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-090 (pin `63204ed0…`, handlers = 659, H_645..H_652 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-090 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_652 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x1F0 | `498b87000300004881c0f0010000480fb60049898780020000c3` (26) | same | same | Y | `1dd7536ff94f210b` | `1dd7536ff94f210b` | PASS |
| 2 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x1F0 | `498b87000300004881c0f0010000480fb60049898788020000c3` (26) | same | same | Y | `e2c1e0f004de6eab` | `e2c1e0f004de6eab` | PASS |
| 3 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x1F0 | `498b87000300004881c0f0010000480fb60049898790020000c3` (26) | same | same | Y | `ad43445e924ece15` | `ad43445e924ece15` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x50 imm=0x1F0 | `498b87800200004881c0f001000049898780020000c3` (22) | same | same | Y | `17b7b25157e9d135` | `17b7b25157e9d135` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x51 imm=0x1F0 | `498b87880200004881c0f001000049898788020000c3` (22) | same | same | Y | `ad1776283d15b543` | `ad1776283d15b543` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x52 imm=0x1F0 | `498b87900200004881c0f001000049898790020000c3` (22) | same | same | Y | `3c8d698c14cd2075` | `3c8d698c14cd2075` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x50 imm=0x1F0 | `498b87800200004881e8f001000049898780020000c3` (22) | same | same | Y | `43db5ead3bfc62f7` | `43db5ead3bfc62f7` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x51 imm=0x1F0 | `498b87880200004881e8f001000049898788020000c3` (22) | same | same | Y | `dac7533ba9ab5adb` | `dac7533ba9ab5adb` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x50 ss=0x60 oo=0x1F0 — **PASS**

- fixture: `_scratch_ldb_5060_1F0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0f0010000480fb60049898780020000c3`
- js-sha256: `1dd7536ff94f210bd645058b78b04d700e36ffcc02f83a27751f4884a3c5f452`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x51 ss=0x60 oo=0x1F0 — **PASS**

- fixture: `_scratch_ldb_5160_1F0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0f0010000480fb60049898788020000c3`
- js-sha256: `e2c1e0f004de6eab107bf86ea1c3731d8d3ad1fab935977c9c817f24b348b676`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x52 ss=0x60 oo=0x1F0 — **PASS**

- fixture: `_scratch_ldb_5260_1F0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0f0010000480fb60049898790020000c3`
- js-sha256: `ad43445e924ece151de60e3a022b4dc1bacc431c105858a57983bd4fe559e13e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x50 imm=0x1F0 — **PASS**

- fixture: `_scratch_addimm_h50_1F0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0f001000049898780020000c3`
- js-sha256: `17b7b25157e9d1359b6bf473502a844f3b2ab639269729b6c365fec13cdd0507`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x51 imm=0x1F0 — **PASS**

- fixture: `_scratch_addimm_h51_1F0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0f001000049898788020000c3`
- js-sha256: `ad1776283d15b543ad830e89f699a757a239e8eb8aae61713f0260e5967a4c51`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x52 imm=0x1F0 — **PASS**

- fixture: `_scratch_addimm_h52_1F0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0f001000049898790020000c3`
- js-sha256: `3c8d698c14cd20755b85feb3d0d41c083447b42c4eca61ed1f06015fc5fae172`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x50 imm=0x1F0 — **PASS**

- fixture: `_scratch_subimm_h50_1F0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8f001000049898780020000c3`
- js-sha256: `43db5ead3bfc62f7b1ddc851953e2ef0966523e442f1e053cc1ebd4691764add`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x51 imm=0x1F0 — **PASS**

- fixture: `_scratch_subimm_h51_1F0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8f001000049898788020000c3`
- js-sha256: `dac7533ba9ab5adb7ba3cbddfedac2ea91a3c50bd8784a92a138e73671e06e9e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=50/51/52 ss=60 oo=1F0 (start deferred 1F0 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=1F0 (start deferred 1F0 ADD triad; imm32 22B).
- SUB-IMM slot=50/51 imm=1F0 (start deferred 1F0 SUB triad; imm32 22B; SUB 52 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 293`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5060_1F0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_1F0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_1F0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_1F0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_1F0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_1F0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_1F0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_1F0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-85-log.md` — this file
- `scripts/_probe/parallel-batch-85-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-091 serialize PASSes + 1 Relock**

Pass pin from body-extend-090 Relock: `63204ed031f1ad84c28688effab4ef4148b7c9e6277c1a08d68a7067dfe56aa1`.
Handlers before consolidate = 659 (H_00..H_652). Next selectors `40 293`.. for H_653.. if all serialize.

PASS list for body-extend-091:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_653 | 0x293 | 0x80 LDB | 0x50 0x60 0x1F0 | `498b87000300004881c0f0010000480fb60049898780020000c3` (26B) | `1dd7536ff94f210b` |
| H_654 | 0x294 | 0x80 LDB | 0x51 0x60 0x1F0 | `498b87000300004881c0f0010000480fb60049898788020000c3` (26B) | `e2c1e0f004de6eab` |
| H_655 | 0x295 | 0x80 LDB | 0x52 0x60 0x1F0 | `498b87000300004881c0f0010000480fb60049898790020000c3` (26B) | `ad43445e924ece15` |
| H_656 | 0x296 | 0x62 ADD-IMM | 0x50 0x1F0 | `498b87800200004881c0f001000049898780020000c3` (22B) | `17b7b25157e9d135` |
| H_657 | 0x297 | 0x62 ADD-IMM | 0x51 0x1F0 | `498b87880200004881c0f001000049898788020000c3` (22B) | `ad1776283d15b543` |
| H_658 | 0x298 | 0x62 ADD-IMM | 0x52 0x1F0 | `498b87900200004881c0f001000049898790020000c3` (22B) | `3c8d698c14cd2075` |
| H_659 | 0x299 | 0x61 SUB-IMM | 0x50 0x1F0 | `498b87800200004881e8f001000049898780020000c3` (22B) | `43db5ead3bfc62f7` |
| H_660 | 0x29A | 0x61 SUB-IMM | 0x51 0x1F0 | `498b87880200004881e8f001000049898788020000c3` (22B) | `dac7533ba9ab5adb` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-090 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_652.
- If the parent decides to serialize, append H_653.. at selectors `40 293`..:
  - H_653 0x80 LDB (80 50 60 1F0) — pin `498b87000300004881c0f0010000480fb60049898780020000c3`
  - H_654 0x80 LDB (80 51 60 1F0) — pin `498b87000300004881c0f0010000480fb60049898788020000c3`
  - H_655 0x80 LDB (80 52 60 1F0) — pin `498b87000300004881c0f0010000480fb60049898790020000c3`
  - H_656 0x62 ADD-IMM (62 50 1F0) — pin `498b87800200004881c0f001000049898780020000c3`
  - H_657 0x62 ADD-IMM (62 51 1F0) — pin `498b87880200004881c0f001000049898788020000c3`
  - H_658 0x62 ADD-IMM (62 52 1F0) — pin `498b87900200004881c0f001000049898790020000c3`
  - H_659 0x61 SUB-IMM (61 50 1F0) — pin `498b87800200004881e8f001000049898780020000c3`
  - H_660 0x61 SUB-IMM (61 51 1F0) — pin `498b87880200004881e8f001000049898788020000c3`
- Plus 1 Relock after append from pin `63204ed0…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).
- Deferred remainder: SUB-IMM slot=52 imm=1F0 (finish 1F0 SUB triad).

## §7. Consolidation handoff

parent next = body-extend-091 serialize PASSes + 1 Relock

# parallel-batch-52 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-52-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-057 (pin `0643c8f5…`, handlers = 396, H_382..H_389 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-057 DDC PE `.text` measured DIFFER — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_389 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x51 imm=0x100 | `498b87880200004881e80001000049898788020000c3` (22) | same | same | Y | `114da116f5fa5311` | `114da116f5fa5311` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x52 imm=0x100 | `498b87900200004881e80001000049898790020000c3` (22) | same | same | Y | `3f28a582a9c075b7` | `3f28a582a9c075b7` | PASS |
| 3 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x108 | `498b87000300004881c008010000480fb60049898780020000c3` (26) | same | same | Y | `bdf235d9350d7497` | `bdf235d9350d7497` | PASS |
| 4 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x108 | `498b87000300004881c008010000480fb60049898788020000c3` (26) | same | same | Y | `3b65bdaff0e56bf1` | `3b65bdaff0e56bf1` | PASS |
| 5 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x108 | `498b87000300004881c008010000480fb60049898790020000c3` (26) | same | same | Y | `86e5cf11a57df77e` | `86e5cf11a57df77e` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x50 imm=0x108 | `498b87800200004881c00801000049898780020000c3` (22) | same | same | Y | `fc5f70d4e243183e` | `fc5f70d4e243183e` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x51 imm=0x108 | `498b87880200004881c00801000049898788020000c3` (22) | same | same | Y | `d00fb3f6020656aa` | `d00fb3f6020656aa` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x52 imm=0x108 | `498b87900200004881c00801000049898790020000c3` (22) | same | same | Y | `2ddfc84367ac3ec1` | `2ddfc84367ac3ec1` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x51 imm=0x100 — **PASS**

- fixture: `_scratch_subimm_h51_100.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e80001000049898788020000c3`
- js-sha256: `114da116f5fa531189a387eca1ad22515497da5f495eb5fb1564de0398031f79`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x52 imm=0x100 — **PASS**

- fixture: `_scratch_subimm_h52_100.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e80001000049898790020000c3`
- js-sha256: `3f28a582a9c075b78d4c8f87917d81f11fc876764e69c16d3955c71cc5670e26`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x50 ss=0x60 oo=0x108 — **PASS**

- fixture: `_scratch_ldb_5060_108.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c008010000480fb60049898780020000c3`
- js-sha256: `bdf235d9350d74971fa150a308a71d424c436606e4d6272f9a19171175c1093f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x51 ss=0x60 oo=0x108 — **PASS**

- fixture: `_scratch_ldb_5160_108.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c008010000480fb60049898788020000c3`
- js-sha256: `3b65bdaff0e56bf1da128dae7fe9f0392050d4da5e14fcbc8fab89074e4d27a0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x52 ss=0x60 oo=0x108 — **PASS**

- fixture: `_scratch_ldb_5260_108.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c008010000480fb60049898790020000c3`
- js-sha256: `86e5cf11a57df77e9404ad63619a2802acf5b27fd543931beff68332b1beaf26`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x50 imm=0x108 — **PASS**

- fixture: `_scratch_addimm_h50_108.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c00801000049898780020000c3`
- js-sha256: `fc5f70d4e243183eb35be37fc3620ea96c6e710da2e67f718e54a640869d10ac`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x51 imm=0x108 — **PASS**

- fixture: `_scratch_addimm_h51_108.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c00801000049898788020000c3`
- js-sha256: `d00fb3f6020656aab95eb7534dd349484ad6910c4b98a18937f3c99f3e9f659a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x52 imm=0x108 — **PASS**

- fixture: `_scratch_addimm_h52_108.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c00801000049898790020000c3`
- js-sha256: `2ddfc84367ac3ec18c85edb05f250de1b7a2fb8fb032048a129db209c91c546f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=51/52 imm=100 (finish 100 SUB triad after H_389; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=108 (next oo after 100 triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=108 (fresh imm after 100; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 18C`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h51_100.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_100.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_108.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_108.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_108.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_108.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_108.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_108.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-52-log.md` — this file
- `scripts/_probe/parallel-batch-52-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-058 serialize PASSes + 1 Relock**

Pass pin from body-extend-057 Relock: `0643c8f550fbb85d6e85eac409cf7ac90a26d7fece1b33bcfe04af260a9f2d5a`.
Handlers before consolidate = 396 (H_00..H_389). Next selectors `40 18C`.. for H_390.. if all serialize.

PASS list for body-extend-058:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_390 | 0x18C | 0x61 SUB-IMM | 0x51 0x100 | `498b87880200004881e80001000049898788020000c3` (22B) | `114da116f5fa5311` |
| H_391 | 0x18D | 0x61 SUB-IMM | 0x52 0x100 | `498b87900200004881e80001000049898790020000c3` (22B) | `3f28a582a9c075b7` |
| H_392 | 0x18E | 0x80 LDB | 0x50 0x60 0x108 | `498b87000300004881c008010000480fb60049898780020000c3` (26B) | `bdf235d9350d7497` |
| H_393 | 0x18F | 0x80 LDB | 0x51 0x60 0x108 | `498b87000300004881c008010000480fb60049898788020000c3` (26B) | `3b65bdaff0e56bf1` |
| H_394 | 0x190 | 0x80 LDB | 0x52 0x60 0x108 | `498b87000300004881c008010000480fb60049898790020000c3` (26B) | `86e5cf11a57df77e` |
| H_395 | 0x191 | 0x62 ADD-IMM | 0x50 0x108 | `498b87800200004881c00801000049898780020000c3` (22B) | `fc5f70d4e243183e` |
| H_396 | 0x192 | 0x62 ADD-IMM | 0x51 0x108 | `498b87880200004881c00801000049898788020000c3` (22B) | `d00fb3f6020656aa` |
| H_397 | 0x193 | 0x62 ADD-IMM | 0x52 0x108 | `498b87900200004881c00801000049898790020000c3` (22B) | `2ddfc84367ac3ec1` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-057 DDC PE `.text` DIFFER noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_389.
- If the parent decides to serialize, append H_390.. at selectors `40 18C`..:
  - H_390 0x61 SUB-IMM (61 51 100) — pin `498b87880200004881e80001000049898788020000c3`
  - H_391 0x61 SUB-IMM (61 52 100) — pin `498b87900200004881e80001000049898790020000c3`
  - H_392 0x80 LDB (80 50 60 108) — pin `498b87000300004881c008010000480fb60049898780020000c3`
  - H_393 0x80 LDB (80 51 60 108) — pin `498b87000300004881c008010000480fb60049898788020000c3`
  - H_394 0x80 LDB (80 52 60 108) — pin `498b87000300004881c008010000480fb60049898790020000c3`
  - H_395 0x62 ADD-IMM (62 50 108) — pin `498b87800200004881c00801000049898780020000c3`
  - H_396 0x62 ADD-IMM (62 51 108) — pin `498b87880200004881c00801000049898788020000c3`
  - H_397 0x62 ADD-IMM (62 52 108) — pin `498b87900200004881c00801000049898790020000c3`
- Plus 1 Relock after append from pin `0643c8f5…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-058 serialize PASSes + 1 Relock

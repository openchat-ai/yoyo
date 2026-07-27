# parallel-batch-76 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-76-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-081 (pin `267c611d…`, handlers = 587, H_573..H_580 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-081 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_580 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x1B0 | `498b87000300004881c0b0010000480fb60049898780020000c3` (26) | same | same | Y | `4a28b7afe67cd9c8` | `4a28b7afe67cd9c8` | PASS |
| 2 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x1B0 | `498b87000300004881c0b0010000480fb60049898788020000c3` (26) | same | same | Y | `bbbb35dd922e35f8` | `bbbb35dd922e35f8` | PASS |
| 3 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x1B0 | `498b87000300004881c0b0010000480fb60049898790020000c3` (26) | same | same | Y | `114e9beed1fbb101` | `114e9beed1fbb101` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x50 imm=0x1B0 | `498b87800200004881c0b001000049898780020000c3` (22) | same | same | Y | `449e70ae9ce9bc48` | `449e70ae9ce9bc48` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x51 imm=0x1B0 | `498b87880200004881c0b001000049898788020000c3` (22) | same | same | Y | `19cf91fa4836bb0d` | `19cf91fa4836bb0d` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x52 imm=0x1B0 | `498b87900200004881c0b001000049898790020000c3` (22) | same | same | Y | `e719980bb34c73f8` | `e719980bb34c73f8` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x50 imm=0x1B0 | `498b87800200004881e8b001000049898780020000c3` (22) | same | same | Y | `fde148880489e4d4` | `fde148880489e4d4` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x51 imm=0x1B0 | `498b87880200004881e8b001000049898788020000c3` (22) | same | same | Y | `2f842240d885a210` | `2f842240d885a210` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x50 ss=0x60 oo=0x1B0 — **PASS**

- fixture: `_scratch_ldb_5060_1B0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0b0010000480fb60049898780020000c3`
- js-sha256: `4a28b7afe67cd9c840ea5ac4136ffbb9dc07e089b32e15b301e6ab29b66cb172`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x51 ss=0x60 oo=0x1B0 — **PASS**

- fixture: `_scratch_ldb_5160_1B0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0b0010000480fb60049898788020000c3`
- js-sha256: `bbbb35dd922e35f80a9b091c9ab3ae835e62e3e2a5a7e3d052e1e0f595c32886`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x52 ss=0x60 oo=0x1B0 — **PASS**

- fixture: `_scratch_ldb_5260_1B0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0b0010000480fb60049898790020000c3`
- js-sha256: `114e9beed1fbb101f01c8ac6fee6a766ac4ec89c366141996c5b3f68d9bd99ab`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x50 imm=0x1B0 — **PASS**

- fixture: `_scratch_addimm_h50_1B0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0b001000049898780020000c3`
- js-sha256: `449e70ae9ce9bc487dd3080a33c3bcefcebc8a74bfaa15a365c2810940e1aa82`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x51 imm=0x1B0 — **PASS**

- fixture: `_scratch_addimm_h51_1B0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0b001000049898788020000c3`
- js-sha256: `19cf91fa4836bb0d24c5b5b86de66eafe96102070d286056353404ec01fdc5c2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x52 imm=0x1B0 — **PASS**

- fixture: `_scratch_addimm_h52_1B0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0b001000049898790020000c3`
- js-sha256: `e719980bb34c73f833082099bf01e553d22845cce6e56d4e86a5f87c40f903a9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x50 imm=0x1B0 — **PASS**

- fixture: `_scratch_subimm_h50_1B0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8b001000049898780020000c3`
- js-sha256: `fde148880489e4d43f5120baa475d665d7fbf50b433516e9bf0737020ed998c5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x51 imm=0x1B0 — **PASS**

- fixture: `_scratch_subimm_h51_1B0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8b001000049898788020000c3`
- js-sha256: `2f842240d885a21044d7a16b541f11024e764239d3ffd51ecd89583de2844da4`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=50/51/52 ss=60 oo=1B0 (start deferred 1B0 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=1B0 (start deferred 1B0 ADD triad; imm32 22B).
- SUB-IMM slot=50/51 imm=1B0 (start deferred 1B0 SUB triad; slot=52 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 24B`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5060_1B0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_1B0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_1B0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_1B0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_1B0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_1B0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_1B0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_1B0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-76-log.md` — this file
- `scripts/_probe/parallel-batch-76-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-082 serialize PASSes + 1 Relock**

Pass pin from body-extend-081 Relock: `267c611dbb648db15251e6e6ee8a52287434680892e9f2ad290fd161eb2b916c`.
Handlers before consolidate = 587 (H_00..H_580). Next selectors `40 24B`.. for H_581.. if all serialize.

PASS list for body-extend-082:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_581 | 0x24B | 0x80 LDB | 0x50 0x60 0x1B0 | `498b87000300004881c0b0010000480fb60049898780020000c3` (26B) | `4a28b7afe67cd9c8` |
| H_582 | 0x24C | 0x80 LDB | 0x51 0x60 0x1B0 | `498b87000300004881c0b0010000480fb60049898788020000c3` (26B) | `bbbb35dd922e35f8` |
| H_583 | 0x24D | 0x80 LDB | 0x52 0x60 0x1B0 | `498b87000300004881c0b0010000480fb60049898790020000c3` (26B) | `114e9beed1fbb101` |
| H_584 | 0x24E | 0x62 ADD-IMM | 0x50 0x1B0 | `498b87800200004881c0b001000049898780020000c3` (22B) | `449e70ae9ce9bc48` |
| H_585 | 0x24F | 0x62 ADD-IMM | 0x51 0x1B0 | `498b87880200004881c0b001000049898788020000c3` (22B) | `19cf91fa4836bb0d` |
| H_586 | 0x250 | 0x62 ADD-IMM | 0x52 0x1B0 | `498b87900200004881c0b001000049898790020000c3` (22B) | `e719980bb34c73f8` |
| H_587 | 0x251 | 0x61 SUB-IMM | 0x50 0x1B0 | `498b87800200004881e8b001000049898780020000c3` (22B) | `fde148880489e4d4` |
| H_588 | 0x252 | 0x61 SUB-IMM | 0x51 0x1B0 | `498b87880200004881e8b001000049898788020000c3` (22B) | `2f842240d885a210` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-081 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_580.
- If the parent decides to serialize, append H_581.. at selectors `40 24B`..:
  - H_581 0x80 LDB (80 50 60 1B0) — pin `498b87000300004881c0b0010000480fb60049898780020000c3`
  - H_582 0x80 LDB (80 51 60 1B0) — pin `498b87000300004881c0b0010000480fb60049898788020000c3`
  - H_583 0x80 LDB (80 52 60 1B0) — pin `498b87000300004881c0b0010000480fb60049898790020000c3`
  - H_584 0x62 ADD-IMM (62 50 1B0) — pin `498b87800200004881c0b001000049898780020000c3`
  - H_585 0x62 ADD-IMM (62 51 1B0) — pin `498b87880200004881c0b001000049898788020000c3`
  - H_586 0x62 ADD-IMM (62 52 1B0) — pin `498b87900200004881c0b001000049898790020000c3`
  - H_587 0x61 SUB-IMM (61 50 1B0) — pin `498b87800200004881e8b001000049898780020000c3`
  - H_588 0x61 SUB-IMM (61 51 1B0) — pin `498b87880200004881e8b001000049898788020000c3`
- Plus 1 Relock after append from pin `267c611d…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-082 serialize PASSes + 1 Relock

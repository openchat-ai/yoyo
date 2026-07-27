# parallel-batch-61 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-61-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-066 (pin `d52ed637…`, handlers = 467, H_453..H_460 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-066 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_460 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x50 imm=0x140 | `498b87800200004881e84001000049898780020000c3` (22) | same | same | Y | `cc93e3af0d6d31c3` | `cc93e3af0d6d31c3` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x51 imm=0x140 | `498b87880200004881e84001000049898788020000c3` (22) | same | same | Y | `4c436b4f07ea2fa3` | `4c436b4f07ea2fa3` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x52 imm=0x140 | `498b87900200004881e84001000049898790020000c3` (22) | same | same | Y | `7338547b13d01af3` | `7338547b13d01af3` | PASS |
| 4 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x148 | `498b87000300004881c048010000480fb60049898780020000c3` (26) | same | same | Y | `e043dad6b063887b` | `e043dad6b063887b` | PASS |
| 5 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x148 | `498b87000300004881c048010000480fb60049898788020000c3` (26) | same | same | Y | `0e0373648d5bea88` | `0e0373648d5bea88` | PASS |
| 6 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x148 | `498b87000300004881c048010000480fb60049898790020000c3` (26) | same | same | Y | `d146b52055b94f9f` | `d146b52055b94f9f` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x50 imm=0x148 | `498b87800200004881c04801000049898780020000c3` (22) | same | same | Y | `32552f824b2e13d9` | `32552f824b2e13d9` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x51 imm=0x148 | `498b87880200004881c04801000049898788020000c3` (22) | same | same | Y | `b44518792801dac1` | `b44518792801dac1` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x50 imm=0x140 — **PASS**

- fixture: `_scratch_subimm_h50_140.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e84001000049898780020000c3`
- js-sha256: `cc93e3af0d6d31c360bc669fdb3f94fe102689ae4b5c426ee4b48e1ff3514502`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x51 imm=0x140 — **PASS**

- fixture: `_scratch_subimm_h51_140.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e84001000049898788020000c3`
- js-sha256: `4c436b4f07ea2fa32f15218c8106244654688d8ccc6a85a9e807b6dc6637cc9f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x52 imm=0x140 — **PASS**

- fixture: `_scratch_subimm_h52_140.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e84001000049898790020000c3`
- js-sha256: `7338547b13d01af3acbb249208be136bb00731cae223870b3b226e92a0fb6019`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x50 ss=0x60 oo=0x148 — **PASS**

- fixture: `_scratch_ldb_5060_148.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c048010000480fb60049898780020000c3`
- js-sha256: `e043dad6b063887b9f6d5ba2ed78d3ee8416a2a0382675e97aff5f3aeca66757`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x51 ss=0x60 oo=0x148 — **PASS**

- fixture: `_scratch_ldb_5160_148.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c048010000480fb60049898788020000c3`
- js-sha256: `0e0373648d5bea887264ead6ce6e39e26d58f55e58db5f56d36ba1174667d748`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x52 ss=0x60 oo=0x148 — **PASS**

- fixture: `_scratch_ldb_5260_148.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c048010000480fb60049898790020000c3`
- js-sha256: `d146b52055b94f9f293f53d9482917bd2e5a5452e278ab5f1507d5a82d0b22d2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x50 imm=0x148 — **PASS**

- fixture: `_scratch_addimm_h50_148.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c04801000049898780020000c3`
- js-sha256: `32552f824b2e13d998986ac270a4b0d47f552321dc6c2163ce8ce8a73fb7ff2a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x51 imm=0x148 — **PASS**

- fixture: `_scratch_addimm_h51_148.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c04801000049898788020000c3`
- js-sha256: `b44518792801dac1c176295e0c3beee2169d3dca87ea69b882f7f56a0d8e8657`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=50/51/52 imm=140 (start 140 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=148 (start 148 LDB triad; imm32 26B).
- ADD-IMM slot=50/51 imm=148 (start 148 ADD triad; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 1D3`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h50_140.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_140.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_140.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_148.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_148.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_148.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_148.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_148.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-61-log.md` — this file
- `scripts/_probe/parallel-batch-61-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-067 serialize PASSes + 1 Relock**

Pass pin from body-extend-066 Relock: `d52ed6373d5b085118d5a601ac8f25b8a529e7c16b36b6dd3bce2115d73ec080`.
Handlers before consolidate = 467 (H_00..H_460). Next selectors `40 1D3`.. for H_461.. if all serialize.

PASS list for body-extend-067:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_461 | 0x1D3 | 0x61 SUB-IMM | 0x50 0x140 | `498b87800200004881e84001000049898780020000c3` (22B) | `cc93e3af0d6d31c3` |
| H_462 | 0x1D4 | 0x61 SUB-IMM | 0x51 0x140 | `498b87880200004881e84001000049898788020000c3` (22B) | `4c436b4f07ea2fa3` |
| H_463 | 0x1D5 | 0x61 SUB-IMM | 0x52 0x140 | `498b87900200004881e84001000049898790020000c3` (22B) | `7338547b13d01af3` |
| H_464 | 0x1D6 | 0x80 LDB | 0x50 0x60 0x148 | `498b87000300004881c048010000480fb60049898780020000c3` (26B) | `e043dad6b063887b` |
| H_465 | 0x1D7 | 0x80 LDB | 0x51 0x60 0x148 | `498b87000300004881c048010000480fb60049898788020000c3` (26B) | `0e0373648d5bea88` |
| H_466 | 0x1D8 | 0x80 LDB | 0x52 0x60 0x148 | `498b87000300004881c048010000480fb60049898790020000c3` (26B) | `d146b52055b94f9f` |
| H_467 | 0x1D9 | 0x62 ADD-IMM | 0x50 0x148 | `498b87800200004881c04801000049898780020000c3` (22B) | `32552f824b2e13d9` |
| H_468 | 0x1DA | 0x62 ADD-IMM | 0x51 0x148 | `498b87880200004881c04801000049898788020000c3` (22B) | `b44518792801dac1` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-066 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_460.
- If the parent decides to serialize, append H_461.. at selectors `40 1D3`..:
  - H_461 0x61 SUB-IMM (61 50 140) — pin `498b87800200004881e84001000049898780020000c3`
  - H_462 0x61 SUB-IMM (61 51 140) — pin `498b87880200004881e84001000049898788020000c3`
  - H_463 0x61 SUB-IMM (61 52 140) — pin `498b87900200004881e84001000049898790020000c3`
  - H_464 0x80 LDB (80 50 60 148) — pin `498b87000300004881c048010000480fb60049898780020000c3`
  - H_465 0x80 LDB (80 51 60 148) — pin `498b87000300004881c048010000480fb60049898788020000c3`
  - H_466 0x80 LDB (80 52 60 148) — pin `498b87000300004881c048010000480fb60049898790020000c3`
  - H_467 0x62 ADD-IMM (62 50 148) — pin `498b87800200004881c04801000049898780020000c3`
  - H_468 0x62 ADD-IMM (62 51 148) — pin `498b87880200004881c04801000049898788020000c3`
- Plus 1 Relock after append from pin `d52ed637…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-067 serialize PASSes + 1 Relock

# parallel-batch-54 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-54-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-059 (pin `bd7bad15…`, handlers = 412, H_398..H_405 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-059 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_405 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x52 imm=0x110 | `498b87900200004881c01001000049898790020000c3` (22) | same | same | Y | `aad3c15ce012a85e` | `aad3c15ce012a85e` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x50 imm=0x110 | `498b87800200004881e81001000049898780020000c3` (22) | same | same | Y | `ab4a316c8b299ed0` | `ab4a316c8b299ed0` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x51 imm=0x110 | `498b87880200004881e81001000049898788020000c3` (22) | same | same | Y | `edaa468a46b020a6` | `edaa468a46b020a6` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x52 imm=0x110 | `498b87900200004881e81001000049898790020000c3` (22) | same | same | Y | `921cdaad23a0f9f0` | `921cdaad23a0f9f0` | PASS |
| 5 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x118 | `498b87000300004881c018010000480fb60049898780020000c3` (26) | same | same | Y | `41253a7fe67f42ba` | `41253a7fe67f42ba` | PASS |
| 6 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x118 | `498b87000300004881c018010000480fb60049898788020000c3` (26) | same | same | Y | `2eaf03e9dc35344e` | `2eaf03e9dc35344e` | PASS |
| 7 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x118 | `498b87000300004881c018010000480fb60049898790020000c3` (26) | same | same | Y | `aad78ddac628a62f` | `aad78ddac628a62f` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x50 imm=0x118 | `498b87800200004881c01801000049898780020000c3` (22) | same | same | Y | `c90d1c2f223e7e95` | `c90d1c2f223e7e95` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x52 imm=0x110 — **PASS**

- fixture: `_scratch_addimm_h52_110.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c01001000049898790020000c3`
- js-sha256: `aad3c15ce012a85efcb76653692940a6a0b953f7b6459f6a331b028c09c4e180`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x50 imm=0x110 — **PASS**

- fixture: `_scratch_subimm_h50_110.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e81001000049898780020000c3`
- js-sha256: `ab4a316c8b299ed0f3c145147bde52c2396bde66a51e685fcf6ea2b59bb66a14`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x51 imm=0x110 — **PASS**

- fixture: `_scratch_subimm_h51_110.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e81001000049898788020000c3`
- js-sha256: `edaa468a46b020a6aa1ba84494ea9eb934941ce25bb27eff05927330a7f1923c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x52 imm=0x110 — **PASS**

- fixture: `_scratch_subimm_h52_110.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e81001000049898790020000c3`
- js-sha256: `921cdaad23a0f9f053deebd0fe1e0f1ea1242167e3e1158a7dc0a50566420dd9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x50 ss=0x60 oo=0x118 — **PASS**

- fixture: `_scratch_ldb_5060_118.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c018010000480fb60049898780020000c3`
- js-sha256: `41253a7fe67f42ba26e2b9da2ed9592275a5aa1a51933926aba81a2accca15ca`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x51 ss=0x60 oo=0x118 — **PASS**

- fixture: `_scratch_ldb_5160_118.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c018010000480fb60049898788020000c3`
- js-sha256: `2eaf03e9dc35344e88d8d8c1bce06533b75d7d15d969662ef045f05978e67ed1`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x52 ss=0x60 oo=0x118 — **PASS**

- fixture: `_scratch_ldb_5260_118.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c018010000480fb60049898790020000c3`
- js-sha256: `aad78ddac628a62ffa1ca1729860fa2110a9c4a7cd568e039e2b260a377ecb2b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x50 imm=0x118 — **PASS**

- fixture: `_scratch_addimm_h50_118.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c01801000049898780020000c3`
- js-sha256: `c90d1c2f223e7e95380b467aeeb09696776be7a69337f650ed7067d507fb08f1`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=52 imm=110 (finish 110 ADD triad after H_404..H_405; imm32 22B).
- SUB-IMM slot=50/51/52 imm=110 (finish 110 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=118 (next oo after 110 triad; imm32 26B).
- ADD-IMM slot=50 imm=118 (fresh imm after 110; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 19C`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h52_110.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_110.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_110.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_110.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_118.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_118.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_118.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_118.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-54-log.md` — this file
- `scripts/_probe/parallel-batch-54-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-060 serialize PASSes + 1 Relock**

Pass pin from body-extend-059 Relock: `bd7bad15e53fe296e790c57803a0d44930e95c7f7db99ee866685fbb5d504f12`.
Handlers before consolidate = 412 (H_00..H_405). Next selectors `40 19C`.. for H_406.. if all serialize.

PASS list for body-extend-060:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_406 | 0x19C | 0x62 ADD-IMM | 0x52 0x110 | `498b87900200004881c01001000049898790020000c3` (22B) | `aad3c15ce012a85e` |
| H_407 | 0x19D | 0x61 SUB-IMM | 0x50 0x110 | `498b87800200004881e81001000049898780020000c3` (22B) | `ab4a316c8b299ed0` |
| H_408 | 0x19E | 0x61 SUB-IMM | 0x51 0x110 | `498b87880200004881e81001000049898788020000c3` (22B) | `edaa468a46b020a6` |
| H_409 | 0x19F | 0x61 SUB-IMM | 0x52 0x110 | `498b87900200004881e81001000049898790020000c3` (22B) | `921cdaad23a0f9f0` |
| H_410 | 0x1A0 | 0x80 LDB | 0x50 0x60 0x118 | `498b87000300004881c018010000480fb60049898780020000c3` (26B) | `41253a7fe67f42ba` |
| H_411 | 0x1A1 | 0x80 LDB | 0x51 0x60 0x118 | `498b87000300004881c018010000480fb60049898788020000c3` (26B) | `2eaf03e9dc35344e` |
| H_412 | 0x1A2 | 0x80 LDB | 0x52 0x60 0x118 | `498b87000300004881c018010000480fb60049898790020000c3` (26B) | `aad78ddac628a62f` |
| H_413 | 0x1A3 | 0x62 ADD-IMM | 0x50 0x118 | `498b87800200004881c01801000049898780020000c3` (22B) | `c90d1c2f223e7e95` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-059 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_405.
- If the parent decides to serialize, append H_406.. at selectors `40 19C`..:
  - H_406 0x62 ADD-IMM (62 52 110) — pin `498b87900200004881c01001000049898790020000c3`
  - H_407 0x61 SUB-IMM (61 50 110) — pin `498b87800200004881e81001000049898780020000c3`
  - H_408 0x61 SUB-IMM (61 51 110) — pin `498b87880200004881e81001000049898788020000c3`
  - H_409 0x61 SUB-IMM (61 52 110) — pin `498b87900200004881e81001000049898790020000c3`
  - H_410 0x80 LDB (80 50 60 118) — pin `498b87000300004881c018010000480fb60049898780020000c3`
  - H_411 0x80 LDB (80 51 60 118) — pin `498b87000300004881c018010000480fb60049898788020000c3`
  - H_412 0x80 LDB (80 52 60 118) — pin `498b87000300004881c018010000480fb60049898790020000c3`
  - H_413 0x62 ADD-IMM (62 50 118) — pin `498b87800200004881c01801000049898780020000c3`
- Plus 1 Relock after append from pin `bd7bad15…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-060 serialize PASSes + 1 Relock

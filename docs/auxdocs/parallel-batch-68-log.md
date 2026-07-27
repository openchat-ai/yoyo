# parallel-batch-68 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-68-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-073 (pin `1a6cb44a…`, handlers = 523, H_509..H_516 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-073 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_516 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x52 imm=0x170 | `498b87900200004881e87001000049898790020000c3` (22) | same | same | Y | `dad788fae9b7dc6d` | `dad788fae9b7dc6d` | PASS |
| 2 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x178 | `498b87000300004881c078010000480fb60049898780020000c3` (26) | same | same | Y | `88e184b59a6db03c` | `88e184b59a6db03c` | PASS |
| 3 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x178 | `498b87000300004881c078010000480fb60049898788020000c3` (26) | same | same | Y | `9ed7c675af239145` | `9ed7c675af239145` | PASS |
| 4 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x178 | `498b87000300004881c078010000480fb60049898790020000c3` (26) | same | same | Y | `acf695cec1340844` | `acf695cec1340844` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x50 imm=0x178 | `498b87800200004881c07801000049898780020000c3` (22) | same | same | Y | `90d4b604f3d3217f` | `90d4b604f3d3217f` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x51 imm=0x178 | `498b87880200004881c07801000049898788020000c3` (22) | same | same | Y | `ef600aa63170300a` | `ef600aa63170300a` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x52 imm=0x178 | `498b87900200004881c07801000049898790020000c3` (22) | same | same | Y | `720aa67f69ef0ab9` | `720aa67f69ef0ab9` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x50 imm=0x178 | `498b87800200004881e87801000049898780020000c3` (22) | same | same | Y | `7f477a27dd9d8bb9` | `7f477a27dd9d8bb9` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x52 imm=0x170 — **PASS**

- fixture: `_scratch_subimm_h52_170.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e87001000049898790020000c3`
- js-sha256: `dad788fae9b7dc6dafc5ea335eca5824067264a818bad9ee6d5ffb8c9e8d42bf`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x50 ss=0x60 oo=0x178 — **PASS**

- fixture: `_scratch_ldb_5060_178.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c078010000480fb60049898780020000c3`
- js-sha256: `88e184b59a6db03c16e09aa71334849160fa599f85d16c5b3283e4d8c4c55b64`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x51 ss=0x60 oo=0x178 — **PASS**

- fixture: `_scratch_ldb_5160_178.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c078010000480fb60049898788020000c3`
- js-sha256: `9ed7c675af2391459a5915d0ac92bd5e3bd2853636b301dba220141937a8bffb`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x52 ss=0x60 oo=0x178 — **PASS**

- fixture: `_scratch_ldb_5260_178.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c078010000480fb60049898790020000c3`
- js-sha256: `acf695cec13408443b9a7b595578b18affe93081974efceb3835395bc576eca3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x50 imm=0x178 — **PASS**

- fixture: `_scratch_addimm_h50_178.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c07801000049898780020000c3`
- js-sha256: `90d4b604f3d3217f47485394b669a7b4cec7a67a74da24c4c771bf7cfd5f3df3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x51 imm=0x178 — **PASS**

- fixture: `_scratch_addimm_h51_178.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c07801000049898788020000c3`
- js-sha256: `ef600aa63170300aaa59c1bbd33286d9c16e5fd4ec6ee4d92fbbf96f46666345`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x52 imm=0x178 — **PASS**

- fixture: `_scratch_addimm_h52_178.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c07801000049898790020000c3`
- js-sha256: `720aa67f69ef0ab902f2b09e87034db0ab374ea25740647882fb61a376db44df`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x50 imm=0x178 — **PASS**

- fixture: `_scratch_subimm_h50_178.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e87801000049898780020000c3`
- js-sha256: `7f477a27dd9d8bb9fee4694d75cb1f273b1f8d7da249471070d19e73ed003989`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=52 imm=170 (finish deferred 170 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=178 (start 178 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=178 (start 178 ADD triad; imm32 22B).
- SUB-IMM slot=50 imm=178 (start 178 SUB; imm32 22B; SUB 51/52 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 20B`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h52_170.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_178.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_178.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_178.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_178.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_178.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_178.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_178.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-68-log.md` — this file
- `scripts/_probe/parallel-batch-68-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-074 serialize PASSes + 1 Relock**

Pass pin from body-extend-073 Relock: `1a6cb44aa28367d25d6727eec5206e5895c3c948be080a60dcadb7d853bc8bac`.
Handlers before consolidate = 523 (H_00..H_516). Next selectors `40 20B`.. for H_517.. if all serialize.

PASS list for body-extend-074:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_517 | 0x20B | 0x61 SUB-IMM | 0x52 0x170 | `498b87900200004881e87001000049898790020000c3` (22B) | `dad788fae9b7dc6d` |
| H_518 | 0x20C | 0x80 LDB | 0x50 0x60 0x178 | `498b87000300004881c078010000480fb60049898780020000c3` (26B) | `88e184b59a6db03c` |
| H_519 | 0x20D | 0x80 LDB | 0x51 0x60 0x178 | `498b87000300004881c078010000480fb60049898788020000c3` (26B) | `9ed7c675af239145` |
| H_520 | 0x20E | 0x80 LDB | 0x52 0x60 0x178 | `498b87000300004881c078010000480fb60049898790020000c3` (26B) | `acf695cec1340844` |
| H_521 | 0x20F | 0x62 ADD-IMM | 0x50 0x178 | `498b87800200004881c07801000049898780020000c3` (22B) | `90d4b604f3d3217f` |
| H_522 | 0x210 | 0x62 ADD-IMM | 0x51 0x178 | `498b87880200004881c07801000049898788020000c3` (22B) | `ef600aa63170300a` |
| H_523 | 0x211 | 0x62 ADD-IMM | 0x52 0x178 | `498b87900200004881c07801000049898790020000c3` (22B) | `720aa67f69ef0ab9` |
| H_524 | 0x212 | 0x61 SUB-IMM | 0x50 0x178 | `498b87800200004881e87801000049898780020000c3` (22B) | `7f477a27dd9d8bb9` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-073 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_516.
- If the parent decides to serialize, append H_517.. at selectors `40 20B`..:
  - H_517 0x61 SUB-IMM (61 52 170) — pin `498b87900200004881e87001000049898790020000c3`
  - H_518 0x80 LDB (80 50 60 178) — pin `498b87000300004881c078010000480fb60049898780020000c3`
  - H_519 0x80 LDB (80 51 60 178) — pin `498b87000300004881c078010000480fb60049898788020000c3`
  - H_520 0x80 LDB (80 52 60 178) — pin `498b87000300004881c078010000480fb60049898790020000c3`
  - H_521 0x62 ADD-IMM (62 50 178) — pin `498b87800200004881c07801000049898780020000c3`
  - H_522 0x62 ADD-IMM (62 51 178) — pin `498b87880200004881c07801000049898788020000c3`
  - H_523 0x62 ADD-IMM (62 52 178) — pin `498b87900200004881c07801000049898790020000c3`
  - H_524 0x61 SUB-IMM (61 50 178) — pin `498b87800200004881e87801000049898780020000c3`
- Plus 1 Relock after append from pin `1a6cb44a…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-074 serialize PASSes + 1 Relock

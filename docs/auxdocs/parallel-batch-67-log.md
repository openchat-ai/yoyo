# parallel-batch-67 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-67-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-072 (pin `e1554db8…`, handlers = 515, H_501..H_508 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-072 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_508 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x170 | `498b87000300004881c070010000480fb60049898780020000c3` (26) | same | same | Y | `2880271f9ceddc44` | `2880271f9ceddc44` | PASS |
| 2 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x170 | `498b87000300004881c070010000480fb60049898788020000c3` (26) | same | same | Y | `f5ea323500e5fb12` | `f5ea323500e5fb12` | PASS |
| 3 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x170 | `498b87000300004881c070010000480fb60049898790020000c3` (26) | same | same | Y | `ee43e15d67b15204` | `ee43e15d67b15204` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x50 imm=0x170 | `498b87800200004881c07001000049898780020000c3` (22) | same | same | Y | `b5ced24e14fef8f3` | `b5ced24e14fef8f3` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x51 imm=0x170 | `498b87880200004881c07001000049898788020000c3` (22) | same | same | Y | `2bb85897a4abc0cf` | `2bb85897a4abc0cf` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x52 imm=0x170 | `498b87900200004881c07001000049898790020000c3` (22) | same | same | Y | `ccca022a923acf93` | `ccca022a923acf93` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x50 imm=0x170 | `498b87800200004881e87001000049898780020000c3` (22) | same | same | Y | `b78b97ec483ce762` | `b78b97ec483ce762` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x51 imm=0x170 | `498b87880200004881e87001000049898788020000c3` (22) | same | same | Y | `f6d1c92bf87d13e8` | `f6d1c92bf87d13e8` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x50 ss=0x60 oo=0x170 — **PASS**

- fixture: `_scratch_ldb_5060_170.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c070010000480fb60049898780020000c3`
- js-sha256: `2880271f9ceddc44129316182318f0a4cc57d86bad178d197cb3eb3250a3616e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x51 ss=0x60 oo=0x170 — **PASS**

- fixture: `_scratch_ldb_5160_170.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c070010000480fb60049898788020000c3`
- js-sha256: `f5ea323500e5fb12f22e4a01c43ed6eeb1a9ce1e15e16f01804cc82e0cdd37f6`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x52 ss=0x60 oo=0x170 — **PASS**

- fixture: `_scratch_ldb_5260_170.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c070010000480fb60049898790020000c3`
- js-sha256: `ee43e15d67b15204d6835a85cefc3b30c0c5cc7494d5c5a573e7493c9fad9b18`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x50 imm=0x170 — **PASS**

- fixture: `_scratch_addimm_h50_170.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c07001000049898780020000c3`
- js-sha256: `b5ced24e14fef8f3a3633aa4a77bd3222947c43a46960a244461d1821c38a1c5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x51 imm=0x170 — **PASS**

- fixture: `_scratch_addimm_h51_170.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c07001000049898788020000c3`
- js-sha256: `2bb85897a4abc0cfd1acafb79b5f84e44b0ef053a80135d5af7f40a99ef5e6fe`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x52 imm=0x170 — **PASS**

- fixture: `_scratch_addimm_h52_170.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c07001000049898790020000c3`
- js-sha256: `ccca022a923acf937590b39f321903fbb402924e8e44e71c35730d61a7acb504`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x50 imm=0x170 — **PASS**

- fixture: `_scratch_subimm_h50_170.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e87001000049898780020000c3`
- js-sha256: `b78b97ec483ce762da832656574acda2516310f7db63f49891579b23450f0dcb`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x51 imm=0x170 — **PASS**

- fixture: `_scratch_subimm_h51_170.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e87001000049898788020000c3`
- js-sha256: `f6d1c92bf87d13e82c11b94d812389f500c95906d3d1c57ff86b30648157a225`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=50/51/52 ss=60 oo=170 (start 170 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=170 (start 170 ADD triad; imm32 22B).
- SUB-IMM slot=50/51 imm=170 (start 170 SUB; imm32 22B; SUB 52 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 203`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5060_170.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_170.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_170.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_170.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_170.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_170.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_170.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_170.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-67-log.md` — this file
- `scripts/_probe/parallel-batch-67-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-073 serialize PASSes + 1 Relock**

Pass pin from body-extend-072 Relock: `e1554db8dcce9946348a88383bed73939d4a835e8dc0989a2788a72a590e6a6b`.
Handlers before consolidate = 515 (H_00..H_508). Next selectors `40 203`.. for H_509.. if all serialize.

PASS list for body-extend-073:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_509 | 0x203 | 0x80 LDB | 0x50 0x60 0x170 | `498b87000300004881c070010000480fb60049898780020000c3` (26B) | `2880271f9ceddc44` |
| H_510 | 0x204 | 0x80 LDB | 0x51 0x60 0x170 | `498b87000300004881c070010000480fb60049898788020000c3` (26B) | `f5ea323500e5fb12` |
| H_511 | 0x205 | 0x80 LDB | 0x52 0x60 0x170 | `498b87000300004881c070010000480fb60049898790020000c3` (26B) | `ee43e15d67b15204` |
| H_512 | 0x206 | 0x62 ADD-IMM | 0x50 0x170 | `498b87800200004881c07001000049898780020000c3` (22B) | `b5ced24e14fef8f3` |
| H_513 | 0x207 | 0x62 ADD-IMM | 0x51 0x170 | `498b87880200004881c07001000049898788020000c3` (22B) | `2bb85897a4abc0cf` |
| H_514 | 0x208 | 0x62 ADD-IMM | 0x52 0x170 | `498b87900200004881c07001000049898790020000c3` (22B) | `ccca022a923acf93` |
| H_515 | 0x209 | 0x61 SUB-IMM | 0x50 0x170 | `498b87800200004881e87001000049898780020000c3` (22B) | `b78b97ec483ce762` |
| H_516 | 0x20A | 0x61 SUB-IMM | 0x51 0x170 | `498b87880200004881e87001000049898788020000c3` (22B) | `f6d1c92bf87d13e8` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-072 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_508.
- If the parent decides to serialize, append H_509.. at selectors `40 203`..:
  - H_509 0x80 LDB (80 50 60 170) — pin `498b87000300004881c070010000480fb60049898780020000c3`
  - H_510 0x80 LDB (80 51 60 170) — pin `498b87000300004881c070010000480fb60049898788020000c3`
  - H_511 0x80 LDB (80 52 60 170) — pin `498b87000300004881c070010000480fb60049898790020000c3`
  - H_512 0x62 ADD-IMM (62 50 170) — pin `498b87800200004881c07001000049898780020000c3`
  - H_513 0x62 ADD-IMM (62 51 170) — pin `498b87880200004881c07001000049898788020000c3`
  - H_514 0x62 ADD-IMM (62 52 170) — pin `498b87900200004881c07001000049898790020000c3`
  - H_515 0x61 SUB-IMM (61 50 170) — pin `498b87800200004881e87001000049898780020000c3`
  - H_516 0x61 SUB-IMM (61 51 170) — pin `498b87880200004881e87001000049898788020000c3`
- Plus 1 Relock after append from pin `e1554db8…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-073 serialize PASSes + 1 Relock

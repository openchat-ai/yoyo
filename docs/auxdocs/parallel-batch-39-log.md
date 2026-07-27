# parallel-batch-39 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-39-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-044 (pin `3514c8c6…`, handlers = 292, H_278..H_285 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-044 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_285 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x52 ss=0x60 oo=0xB8 | `498b87000300004881c0b8000000480fb60049898790020000c3` (26) | same | same | Y | `1f2f5d3657c8a950` | `1f2f5d3657c8a950` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x50 imm=0xA0 | `498b87800200004881c0a000000049898780020000c3` (22) | same | same | Y | `c1ce6933aae1f9f6` | `c1ce6933aae1f9f6` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x51 imm=0xA0 | `498b87880200004881c0a000000049898788020000c3` (22) | same | same | Y | `7ee6f52e149ddaf7` | `7ee6f52e149ddaf7` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x52 imm=0xA0 | `498b87900200004881c0a000000049898790020000c3` (22) | same | same | Y | `21fbb86c3234cc5d` | `21fbb86c3234cc5d` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x50 imm=0xA0 | `498b87800200004881e8a000000049898780020000c3` (22) | same | same | Y | `1588c7457cf93fd9` | `1588c7457cf93fd9` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x51 imm=0xA0 | `498b87880200004881e8a000000049898788020000c3` (22) | same | same | Y | `8aca9b975c5fdce4` | `8aca9b975c5fdce4` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x52 imm=0xA0 | `498b87900200004881e8a000000049898790020000c3` (22) | same | same | Y | `fc8ca4c4c8e50fd5` | `fc8ca4c4c8e50fd5` | PASS |
| 8 | 0x80 LDB | dd=0x50 ss=0x60 oo=0xC0 | `498b87000300004881c0c0000000480fb60049898780020000c3` (26) | same | same | Y | `cf7c2bda3d5ae346` | `cf7c2bda3d5ae346` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x52 ss=0x60 oo=0xB8 — **PASS**

- fixture: `_scratch_ldb_5260_b8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0b8000000480fb60049898790020000c3`
- js-sha256: `1f2f5d3657c8a950cafb8678cacf0656ecf6fe49ea44d6ecf5b6a724e9ee20a4`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x50 imm=0xA0 — **PASS**

- fixture: `_scratch_addimm_h50_a0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0a000000049898780020000c3`
- js-sha256: `c1ce6933aae1f9f6b6fcd7793ad38e829661827aa72d0ec9ea01953f79037ab1`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x51 imm=0xA0 — **PASS**

- fixture: `_scratch_addimm_h51_a0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0a000000049898788020000c3`
- js-sha256: `7ee6f52e149ddaf76175450068bc46096b132caa063ba44fd8c32c8d133f6646`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x52 imm=0xA0 — **PASS**

- fixture: `_scratch_addimm_h52_a0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0a000000049898790020000c3`
- js-sha256: `21fbb86c3234cc5d1e9d789d63bcca613d6ec3e4d135415b8b71aed335262a2d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x50 imm=0xA0 — **PASS**

- fixture: `_scratch_subimm_h50_a0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8a000000049898780020000c3`
- js-sha256: `1588c7457cf93fd9bb89205aebd40431dfb361951ee4a07628822ce44c97474b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x51 imm=0xA0 — **PASS**

- fixture: `_scratch_subimm_h51_a0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8a000000049898788020000c3`
- js-sha256: `8aca9b975c5fdce4f5005adc81f373a3236dd512279793a7be1877b4b17c188a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x52 imm=0xA0 — **PASS**

- fixture: `_scratch_subimm_h52_a0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8a000000049898790020000c3`
- js-sha256: `fc8ca4c4c8e50fd520c005acecde274357878f918d629c21fe10b2f5ef7b3ef0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x50 ss=0x60 oo=0xC0 — **PASS**

- fixture: `_scratch_ldb_5060_c0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0c0000000480fb60049898780020000c3`
- js-sha256: `cf7c2bda3d5ae346f027aa17e3271265c1075a3675686ed8a859ca238d4e8356`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=52 ss=60 oo=B8 (finish B8 triad after H_284/H_285; imm32 26B).
- ADD-IMM slot=50/51/52 imm=A0 (fresh imm after 98; imm32 22B).
- SUB-IMM slot 50/51/52 imm=A0 (complements ADD-IMM * A0; imm=0xA0 → imm32 sub).
- LDB dd=50 ss=60 oo=C0 (fresh oo=C0 starter; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 124`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5260_b8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_a0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_a0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_a0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_a0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_a0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_a0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_c0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-39-log.md` — this file
- `scripts/_probe/parallel-batch-39-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-045 serialize PASSes + 1 Relock**

Pass pin from body-extend-044 Relock: `3514c8c6558f7028fdc93ea64a26dc007fe2df25592035494342ab5fbe6e102c`.
Handlers before consolidate = 292 (H_00..H_285). Next selectors `40 124`.. for H_286.. if all serialize.

PASS list for body-extend-045:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_286 | 0x124 | 0x80 LDB | 0x52 0x60 0xB8 | `498b87000300004881c0b8000000480fb60049898790020000c3` (26B) | `1f2f5d3657c8a950` |
| H_287 | 0x125 | 0x62 ADD-IMM | 0x50 0xA0 | `498b87800200004881c0a000000049898780020000c3` (22B) | `c1ce6933aae1f9f6` |
| H_288 | 0x126 | 0x62 ADD-IMM | 0x51 0xA0 | `498b87880200004881c0a000000049898788020000c3` (22B) | `7ee6f52e149ddaf7` |
| H_289 | 0x127 | 0x62 ADD-IMM | 0x52 0xA0 | `498b87900200004881c0a000000049898790020000c3` (22B) | `21fbb86c3234cc5d` |
| H_290 | 0x128 | 0x61 SUB-IMM | 0x50 0xA0 | `498b87800200004881e8a000000049898780020000c3` (22B) | `1588c7457cf93fd9` |
| H_291 | 0x129 | 0x61 SUB-IMM | 0x51 0xA0 | `498b87880200004881e8a000000049898788020000c3` (22B) | `8aca9b975c5fdce4` |
| H_292 | 0x12A | 0x61 SUB-IMM | 0x52 0xA0 | `498b87900200004881e8a000000049898790020000c3` (22B) | `fc8ca4c4c8e50fd5` |
| H_293 | 0x12B | 0x80 LDB | 0x50 0x60 0xC0 | `498b87000300004881c0c0000000480fb60049898780020000c3` (26B) | `cf7c2bda3d5ae346` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-044 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_285.
- If the parent decides to serialize, append H_286.. at selectors `40 124`..:
  - H_286 0x80 LDB (80 52 60 B8) — pin `498b87000300004881c0b8000000480fb60049898790020000c3`
  - H_287 0x62 ADD-IMM (62 50 A0) — pin `498b87800200004881c0a000000049898780020000c3`
  - H_288 0x62 ADD-IMM (62 51 A0) — pin `498b87880200004881c0a000000049898788020000c3`
  - H_289 0x62 ADD-IMM (62 52 A0) — pin `498b87900200004881c0a000000049898790020000c3`
  - H_290 0x61 SUB-IMM (61 50 A0) — pin `498b87800200004881e8a000000049898780020000c3`
  - H_291 0x61 SUB-IMM (61 51 A0) — pin `498b87880200004881e8a000000049898788020000c3`
  - H_292 0x61 SUB-IMM (61 52 A0) — pin `498b87900200004881e8a000000049898790020000c3`
  - H_293 0x80 LDB (80 50 60 C0) — pin `498b87000300004881c0c0000000480fb60049898780020000c3`
- Plus 1 Relock after append from pin `3514c8c6…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-045 serialize PASSes + 1 Relock

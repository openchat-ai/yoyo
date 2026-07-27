# parallel-batch-36 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-36-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-041 (pin `4cb65681…`, handlers = 268, H_254..H_261 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-041 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_261 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x52 imm=0x88 | `498b87900200004881c08800000049898790020000c3` (22) | same | same | Y | `97f31856e0e0bace` | `97f31856e0e0bace` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x50 imm=0x88 | `498b87800200004881e88800000049898780020000c3` (22) | same | same | Y | `031eecb381c11df4` | `031eecb381c11df4` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x51 imm=0x88 | `498b87880200004881e88800000049898788020000c3` (22) | same | same | Y | `e032f65c781b8d24` | `e032f65c781b8d24` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x52 imm=0x88 | `498b87900200004881e88800000049898790020000c3` (22) | same | same | Y | `a35fd747b10ad6c0` | `a35fd747b10ad6c0` | PASS |
| 5 | 0x80 LDB | dd=0x50 ss=0x60 oo=0xA8 | `498b87000300004881c0a8000000480fb60049898780020000c3` (26) | same | same | Y | `9406298c7e1a9bb7` | `9406298c7e1a9bb7` | PASS |
| 6 | 0x80 LDB | dd=0x51 ss=0x60 oo=0xA8 | `498b87000300004881c0a8000000480fb60049898788020000c3` (26) | same | same | Y | `21a57bbe40cd51a3` | `21a57bbe40cd51a3` | PASS |
| 7 | 0x80 LDB | dd=0x52 ss=0x60 oo=0xA8 | `498b87000300004881c0a8000000480fb60049898790020000c3` (26) | same | same | Y | `6ce7678316409535` | `6ce7678316409535` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x50 imm=0x90 | `498b87800200004881c09000000049898780020000c3` (22) | same | same | Y | `606ca6ba641f5721` | `606ca6ba641f5721` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x52 imm=0x88 — **PASS**

- fixture: `_scratch_addimm_h52_88.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c08800000049898790020000c3`
- js-sha256: `97f31856e0e0baceaed798f77604b2aff777ecb608b46a6719c6f4d0533e968e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x50 imm=0x88 — **PASS**

- fixture: `_scratch_subimm_h50_88.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e88800000049898780020000c3`
- js-sha256: `031eecb381c11df4dc03b6e63f7180b0548c2c6a4b5d7449c03f6b80737d8eda`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x51 imm=0x88 — **PASS**

- fixture: `_scratch_subimm_h51_88.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e88800000049898788020000c3`
- js-sha256: `e032f65c781b8d241d0ad68b35a5869154e97e6eea83386d3bdc567e1c5f06a7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x52 imm=0x88 — **PASS**

- fixture: `_scratch_subimm_h52_88.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e88800000049898790020000c3`
- js-sha256: `a35fd747b10ad6c00daa10d453b7ab1cf6634f2f583e8fdb97df753be554fb2a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x50 ss=0x60 oo=0xA8 — **PASS**

- fixture: `_scratch_ldb_5060_a8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0a8000000480fb60049898780020000c3`
- js-sha256: `9406298c7e1a9bb70892f6a9e517ae808a991428b3e948f3841407476c6fb62d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x51 ss=0x60 oo=0xA8 — **PASS**

- fixture: `_scratch_ldb_5160_a8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0a8000000480fb60049898788020000c3`
- js-sha256: `21a57bbe40cd51a35371ea11cf3e31a8c3048a3519a6d5757d9045dfa4930007`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x52 ss=0x60 oo=0xA8 — **PASS**

- fixture: `_scratch_ldb_5260_a8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0a8000000480fb60049898790020000c3`
- js-sha256: `6ce7678316409535d605ab8f7e632d45e6fa1dd39f9570eb6d264b01ed9e6036`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x50 imm=0x90 — **PASS**

- fixture: `_scratch_addimm_h50_90.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c09000000049898780020000c3`
- js-sha256: `606ca6ba641f572123be83228e402a8392e65cac1ed4b8e8dca1e566f5410610`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=52 imm=88 (complete ADD 88 triad with H_260/H_261; imm32 22B).
- SUB-IMM slot 50/51/52 imm=88 (complements ADD-IMM * 88; imm=0x88 → imm32 sub).
- LDB dd=50/51/52 ss=60 oo=A8 (fresh oo=A8 triad; imm32 26B).
- ADD-IMM slot=50 imm=90 (fresh imm=90 start; 0x90>127 → imm32 `48 81 c0` → 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 10C`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h52_88.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_88.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_88.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_88.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_a8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_a8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_a8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_90.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-36-log.md` — this file
- `scripts/_probe/parallel-batch-36-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-042 serialize PASSes + 1 Relock**

Pass pin from body-extend-041 Relock: `4cb656812b03c0fdb229b2d0d9278c479ab83b33d6cc7782e75f2397b0e165db`.
Handlers before consolidate = 268 (H_00..H_261). Next selectors `40 10C`.. for H_262.. if all serialize.

PASS list for body-extend-042:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_262 | 0x10C | 0x62 ADD-IMM | 0x52 0x88 | `498b87900200004881c08800000049898790020000c3` (22B) | `97f31856e0e0bace` |
| H_263 | 0x10D | 0x61 SUB-IMM | 0x50 0x88 | `498b87800200004881e88800000049898780020000c3` (22B) | `031eecb381c11df4` |
| H_264 | 0x10E | 0x61 SUB-IMM | 0x51 0x88 | `498b87880200004881e88800000049898788020000c3` (22B) | `e032f65c781b8d24` |
| H_265 | 0x10F | 0x61 SUB-IMM | 0x52 0x88 | `498b87900200004881e88800000049898790020000c3` (22B) | `a35fd747b10ad6c0` |
| H_266 | 0x110 | 0x80 LDB | 0x50 0x60 0xA8 | `498b87000300004881c0a8000000480fb60049898780020000c3` (26B) | `9406298c7e1a9bb7` |
| H_267 | 0x111 | 0x80 LDB | 0x51 0x60 0xA8 | `498b87000300004881c0a8000000480fb60049898788020000c3` (26B) | `21a57bbe40cd51a3` |
| H_268 | 0x112 | 0x80 LDB | 0x52 0x60 0xA8 | `498b87000300004881c0a8000000480fb60049898790020000c3` (26B) | `6ce7678316409535` |
| H_269 | 0x113 | 0x62 ADD-IMM | 0x50 0x90 | `498b87800200004881c09000000049898780020000c3` (22B) | `606ca6ba641f5721` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-041 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_261.
- If the parent decides to serialize, append H_262.. at selectors `40 10C`..:
  - H_262 0x62 ADD-IMM (62 52 88) — pin `498b87900200004881c08800000049898790020000c3`
  - H_263 0x61 SUB-IMM (61 50 88) — pin `498b87800200004881e88800000049898780020000c3`
  - H_264 0x61 SUB-IMM (61 51 88) — pin `498b87880200004881e88800000049898788020000c3`
  - H_265 0x61 SUB-IMM (61 52 88) — pin `498b87900200004881e88800000049898790020000c3`
  - H_266 0x80 LDB (80 50 60 A8) — pin `498b87000300004881c0a8000000480fb60049898780020000c3`
  - H_267 0x80 LDB (80 51 60 A8) — pin `498b87000300004881c0a8000000480fb60049898788020000c3`
  - H_268 0x80 LDB (80 52 60 A8) — pin `498b87000300004881c0a8000000480fb60049898790020000c3`
  - H_269 0x62 ADD-IMM (62 50 90) — pin `498b87800200004881c09000000049898780020000c3`
- Plus 1 Relock after append from pin `4cb65681…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-042 serialize PASSes + 1 Relock

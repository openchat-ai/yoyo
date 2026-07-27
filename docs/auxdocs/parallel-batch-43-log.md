# parallel-batch-43 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-43-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-048 (pin `9c2f924a…`, handlers = 324, H_310..H_317 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-048 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_317 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x51 ss=0x60 oo=0xD0 | `498b87000300004881c0d0000000480fb60049898788020000c3` (26) | same | same | Y | `2d00172cf7198885` | `2d00172cf7198885` | PASS |
| 2 | 0x80 LDB | dd=0x52 ss=0x60 oo=0xD0 | `498b87000300004881c0d0000000480fb60049898790020000c3` (26) | same | same | Y | `e5577873d59f39b9` | `e5577873d59f39b9` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x50 imm=0xC0 | `498b87800200004881c0c000000049898780020000c3` (22) | same | same | Y | `14116ca20ac2ff30` | `14116ca20ac2ff30` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x51 imm=0xC0 | `498b87880200004881c0c000000049898788020000c3` (22) | same | same | Y | `781fd0dd879b7d37` | `781fd0dd879b7d37` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x52 imm=0xC0 | `498b87900200004881c0c000000049898790020000c3` (22) | same | same | Y | `187eebc8371ba7f5` | `187eebc8371ba7f5` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x50 imm=0xC0 | `498b87800200004881e8c000000049898780020000c3` (22) | same | same | Y | `90c51fcf3eb0e0bb` | `90c51fcf3eb0e0bb` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x51 imm=0xC0 | `498b87880200004881e8c000000049898788020000c3` (22) | same | same | Y | `3c16c50a8e776b8a` | `3c16c50a8e776b8a` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x52 imm=0xC0 | `498b87900200004881e8c000000049898790020000c3` (22) | same | same | Y | `5bfec4655978ffd2` | `5bfec4655978ffd2` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x51 ss=0x60 oo=0xD0 — **PASS**

- fixture: `_scratch_ldb_5160_d0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0d0000000480fb60049898788020000c3`
- js-sha256: `2d00172cf7198885bd7347e251eaa7c33540a4a35e9e1a084dfcac50707b3b47`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x52 ss=0x60 oo=0xD0 — **PASS**

- fixture: `_scratch_ldb_5260_d0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0d0000000480fb60049898790020000c3`
- js-sha256: `e5577873d59f39b9aa84901bcf4220f8cb1527234f0b8a578d6e3ab5cc017659`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x50 imm=0xC0 — **PASS**

- fixture: `_scratch_addimm_h50_c0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0c000000049898780020000c3`
- js-sha256: `14116ca20ac2ff30af682f03a3fcb55fc731bee7b9182f70e6e72465b47cc582`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x51 imm=0xC0 — **PASS**

- fixture: `_scratch_addimm_h51_c0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0c000000049898788020000c3`
- js-sha256: `781fd0dd879b7d37866e8985d59e2de18b6ea08bbeb56afba78594d1ae3ff565`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x52 imm=0xC0 — **PASS**

- fixture: `_scratch_addimm_h52_c0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0c000000049898790020000c3`
- js-sha256: `187eebc8371ba7f52de7e17a77d90ac98a6cb7401c25cb0218e6696b3a2b3850`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x50 imm=0xC0 — **PASS**

- fixture: `_scratch_subimm_h50_c0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8c000000049898780020000c3`
- js-sha256: `90c51fcf3eb0e0bb7e65b7153d7accbdb9208c666a19ed0ed0c822dcd3dcc7f5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x51 imm=0xC0 — **PASS**

- fixture: `_scratch_subimm_h51_c0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8c000000049898788020000c3`
- js-sha256: `3c16c50a8e776b8aa864e0f7b40993705ad8259064013b1e680a242228aad37c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x52 imm=0xC0 — **PASS**

- fixture: `_scratch_subimm_h52_c0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8c000000049898790020000c3`
- js-sha256: `5bfec4655978ffd201ae719eaaebb6d51f32bd2b0672b3c6e628e68a51e8ebae`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=51/52 ss=60 oo=D0 (finish D0 triad after H_317; imm32 26B).
- ADD-IMM slot=50/51/52 imm=C0 (fresh imm after B8; imm32 22B).
- SUB-IMM slot=50/51/52 imm=C0 (complements ADD-IMM * C0; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 144`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5160_d0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_d0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_c0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_c0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_c0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_c0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_c0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_c0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-43-log.md` — this file
- `scripts/_probe/parallel-batch-43-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-049 serialize PASSes + 1 Relock**

Pass pin from body-extend-048 Relock: `9c2f924a2780d64647f590c707d39330fa4bff0e69a2c243c0550956ec2d41a2`.
Handlers before consolidate = 324 (H_00..H_317). Next selectors `40 144`.. for H_318.. if all serialize.

PASS list for body-extend-049:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_318 | 0x144 | 0x80 LDB | 0x51 0x60 0xD0 | `498b87000300004881c0d0000000480fb60049898788020000c3` (26B) | `2d00172cf7198885` |
| H_319 | 0x145 | 0x80 LDB | 0x52 0x60 0xD0 | `498b87000300004881c0d0000000480fb60049898790020000c3` (26B) | `e5577873d59f39b9` |
| H_320 | 0x146 | 0x62 ADD-IMM | 0x50 0xC0 | `498b87800200004881c0c000000049898780020000c3` (22B) | `14116ca20ac2ff30` |
| H_321 | 0x147 | 0x62 ADD-IMM | 0x51 0xC0 | `498b87880200004881c0c000000049898788020000c3` (22B) | `781fd0dd879b7d37` |
| H_322 | 0x148 | 0x62 ADD-IMM | 0x52 0xC0 | `498b87900200004881c0c000000049898790020000c3` (22B) | `187eebc8371ba7f5` |
| H_323 | 0x149 | 0x61 SUB-IMM | 0x50 0xC0 | `498b87800200004881e8c000000049898780020000c3` (22B) | `90c51fcf3eb0e0bb` |
| H_324 | 0x14A | 0x61 SUB-IMM | 0x51 0xC0 | `498b87880200004881e8c000000049898788020000c3` (22B) | `3c16c50a8e776b8a` |
| H_325 | 0x14B | 0x61 SUB-IMM | 0x52 0xC0 | `498b87900200004881e8c000000049898790020000c3` (22B) | `5bfec4655978ffd2` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-048 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_317.
- If the parent decides to serialize, append H_318.. at selectors `40 144`..:
  - H_318 0x80 LDB (80 51 60 D0) — pin `498b87000300004881c0d0000000480fb60049898788020000c3`
  - H_319 0x80 LDB (80 52 60 D0) — pin `498b87000300004881c0d0000000480fb60049898790020000c3`
  - H_320 0x62 ADD-IMM (62 50 C0) — pin `498b87800200004881c0c000000049898780020000c3`
  - H_321 0x62 ADD-IMM (62 51 C0) — pin `498b87880200004881c0c000000049898788020000c3`
  - H_322 0x62 ADD-IMM (62 52 C0) — pin `498b87900200004881c0c000000049898790020000c3`
  - H_323 0x61 SUB-IMM (61 50 C0) — pin `498b87800200004881e8c000000049898780020000c3`
  - H_324 0x61 SUB-IMM (61 51 C0) — pin `498b87880200004881e8c000000049898788020000c3`
  - H_325 0x61 SUB-IMM (61 52 C0) — pin `498b87900200004881e8c000000049898790020000c3`
- Plus 1 Relock after append from pin `9c2f924a…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-049 serialize PASSes + 1 Relock

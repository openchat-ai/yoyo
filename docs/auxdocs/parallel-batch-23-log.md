# parallel-batch-23 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-23-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-028 (pin `80287f8f…`, handlers = 164, H_150..H_157 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_157 and
> not already present as handlers in current `yoyo.ty` (skipped early
> INC/DEC 50, GET/ORV/ADDV/SUBV 50 51, IMUL 50 51, LDB 50 60 50=H_42).
> Slot/imm/dst variations of SET/ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x50 | `498b87000300004883c050480fb60049898788020000c3` (23) | same | same | Y | `9bdf8a7966f533c0` | `9bdf8a7966f533c0` | PASS |
| 2 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x50 | `498b87000300004883c050480fb60049898790020000c3` (23) | same | same | Y | `f20e9d7238f08a4a` | `f20e9d7238f08a4a` | PASS |
| 3 | 0x30 SET | slot=0x51 imm=0xCAFEF00D | `48b80df0feca0000000049898788020000c3` (18) | same | same | Y | `72c89add1c031d37` | `72c89add1c031d37` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x52 imm=0x32 | `498b87900200004883c03249898790020000c3` (19) | same | same | Y | `b1a04638a88d7ace` | `b1a04638a88d7ace` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x51 imm=0x32 | `498b87880200004883e83249898788020000c3` (19) | same | same | Y | `207c87cf78c25007` | `207c87cf78c25007` | PASS |
| 6 | 0x30 SET | slot=0x50 imm=0xCAFEF00D | `48b80df0feca0000000049898780020000c3` (18) | same | same | Y | `a7ecea443fabe02e` | `a7ecea443fabe02e` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x52 imm=0x32 | `498b87900200004883e83249898790020000c3` (19) | same | same | Y | `bc35f4068daa6365` | `bc35f4068daa6365` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x50 imm=0x3C | `498b87800200004883c03c49898780020000c3` (19) | same | same | Y | `6e63785554e168e2` | `6e63785554e168e2` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x51 ss=0x60 oo=0x50 — **PASS**

- fixture: `_scratch_ldb_5160_50.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c050480fb60049898788020000c3`
- js-sha256: `9bdf8a7966f533c04b1b85d59d61ece4ecb40b11b443a815496cea31b513b342`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x52 ss=0x60 oo=0x50 — **PASS**

- fixture: `_scratch_ldb_5260_50.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c050480fb60049898790020000c3`
- js-sha256: `f20e9d7238f08a4a1dd48ba2bc4816a51b20baaec649bac168672f09bddddd07`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x30 SET slot=0x51 imm=0xCAFEF00D — **PASS**

- fixture: `_scratch_set_51_cafef00d.ty` + `.code.hex`
- expected pin (18B): `48b80df0feca0000000049898788020000c3`
- js-sha256: `72c89add1c031d3714901bfc10a48a16d7b686a5545198aa6baf392a5c4c3188`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x52 imm=0x32 — **PASS**

- fixture: `_scratch_addimm_h52_32.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c03249898790020000c3`
- js-sha256: `b1a04638a88d7ace2c3bfca547038971b92f195c655c41dcd4050d424d79c35b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x51 imm=0x32 — **PASS**

- fixture: `_scratch_subimm_h51_32.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e83249898788020000c3`
- js-sha256: `207c87cf78c25007a9ff3bddafbf496fcda31ffbcbab9ec1e22edb48ed0ddb07`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x30 SET slot=0x50 imm=0xCAFEF00D — **PASS**

- fixture: `_scratch_set_50_cafef00d.ty` + `.code.hex`
- expected pin (18B): `48b80df0feca0000000049898780020000c3`
- js-sha256: `a7ecea443fabe02e84bcf03b40ec9f00f5b49c83f36a1ab6113b334403b0854d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x52 imm=0x32 — **PASS**

- fixture: `_scratch_subimm_h52_32.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e83249898790020000c3`
- js-sha256: `bc35f4068daa6365186e7b9c1d89189256e614bf6278ae045392852d26ace1ba`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x50 imm=0x3C — **PASS**

- fixture: `_scratch_addimm_h50_3c.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c03c49898780020000c3`
- js-sha256: `6e63785554e168e2b1435ec46c8609257c0b1e5eea1ae5592265bd98748ceada`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=51/52 ss=60 oo=50 (H_42=50 60 50; locked LDB oo through 48 at 50/51/52; H_153..H_155 oo=48).
- SET at slot 51/50 imm=CAFEF00D (H_53=52 CAFEBABE; locked SET at 51 ≠ CAFEF00D; slot 50 ≠ CAFEF00D).
- ADD-IMM at slot 52 imm=32 (H_151/H_156=50/51 32; H_78/97/115/129/144 other imm at 52).
- SUB-IMM at slot 51/52 imm=32 (H_157=50 28; H_149=51 28; no imm=32 SUB-IMM locked).
- ADD-IMM at slot 50 imm=3C (H_93=50 0F; H_108=50 14; H_123=50 1E; H_140=50 28; H_151=50 32).
- Skipped suggested INC/DEC 50 (H_17/H_18), GET/ORV/ADDV/SUBV 50 51 (early), IMUL 50 51 (H_34), LDB 50 60 50 (H_42).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5160_50.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_50.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_51_cafef00d.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_32.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_32.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_50_cafef00d.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_32.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_3c.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-23-log.md` — this file
- `scripts/_probe/parallel-batch-23-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-029 serialize PASSes + 1 Relock**

Pass pin from body-extend-028 Relock: `80287f8fe0a8eb0977a5b0cf8f6e39be7839c229229e6ded1853630d6430e33d`.
Handlers before consolidate = 164 (H_00..H_157). Next selectors 0xA4.. for H_158.. if all serialize.

PASS list for body-extend-029:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_158 | 0xA4 | 0x80 LDB | 0x51 0x60 0x50 | `498b87000300004883c050480fb60049898788020000c3` (23B) | `9bdf8a7966f533c0` |
| H_159 | 0xA5 | 0x80 LDB | 0x52 0x60 0x50 | `498b87000300004883c050480fb60049898790020000c3` (23B) | `f20e9d7238f08a4a` |
| H_160 | 0xA6 | 0x30 SET | 0x51 0xCAFEF00D | `48b80df0feca0000000049898788020000c3` (18B) | `72c89add1c031d37` |
| H_161 | 0xA7 | 0x62 ADD-IMM | 0x52 0x32 | `498b87900200004883c03249898790020000c3` (19B) | `b1a04638a88d7ace` |
| H_162 | 0xA8 | 0x61 SUB-IMM | 0x51 0x32 | `498b87880200004883e83249898788020000c3` (19B) | `207c87cf78c25007` |
| H_163 | 0xA9 | 0x30 SET | 0x50 0xCAFEF00D | `48b80df0feca0000000049898780020000c3` (18B) | `a7ecea443fabe02e` |
| H_164 | 0xAA | 0x61 SUB-IMM | 0x52 0x32 | `498b87900200004883e83249898790020000c3` (19B) | `bc35f4068daa6365` |
| H_165 | 0xAB | 0x62 ADD-IMM | 0x50 0x3C | `498b87800200004883c03c49898780020000c3` (19B) | `6e63785554e168e2` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_157.
- If the parent decides to serialize, append H_158.. at selectors 0xA4..:
  - H_158 0x80 LDB (80 51 60 50) — pin `498b87000300004883c050480fb60049898788020000c3`
  - H_159 0x80 LDB (80 52 60 50) — pin `498b87000300004883c050480fb60049898790020000c3`
  - H_160 0x30 SET (30 51 CAFEF00D) — pin `48b80df0feca0000000049898788020000c3`
  - H_161 0x62 ADD-IMM (62 52 32) — pin `498b87900200004883c03249898790020000c3`
  - H_162 0x61 SUB-IMM (61 51 32) — pin `498b87880200004883e83249898788020000c3`
  - H_163 0x30 SET (30 50 CAFEF00D) — pin `48b80df0feca0000000049898780020000c3`
  - H_164 0x61 SUB-IMM (61 52 32) — pin `498b87900200004883e83249898790020000c3`
  - H_165 0x62 ADD-IMM (62 50 3C) — pin `498b87800200004883c03c49898780020000c3`
- Plus 1 Relock after append from pin `80287f8f…`.

## §7. Consolidation handoff

parent next = body-extend-029 serialize PASSes + 1 Relock

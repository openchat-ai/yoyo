# parallel-batch-17 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-17-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-022 (pin `c2d51066…`, handlers = 116, H_102..H_109 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_109 and
> not already present as handlers in current `yoyo.ty` (skipped early
> INC/DEC 50, GET/ORV/ADDV/SUBV 50 51, IMUL 50 51, LDB 50 60 10).
> Slot/imm/dst variations of SET/ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x30 SET | slot=0x52 imm=0xDEADF00D | `48b80df0adde0000000049898790020000c3` (18) | same | same | Y | `34b8f29b8558e0c5` | `34b8f29b8558e0c5` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x51 imm=0x14 | `498b87880200004883c01449898788020000c3` (19) | same | same | Y | `0de1fe36c79129f6` | `0de1fe36c79129f6` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x51 imm=0x0A | `498b87880200004883e80a49898788020000c3` (19) | same | same | Y | `4da400c99cc085fe` | `4da400c99cc085fe` | PASS |
| 4 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x20 | `498b87000300004883c020480fb60049898788020000c3` (23) | same | same | Y | `5d16e28161ed63a9` | `5d16e28161ed63a9` | PASS |
| 5 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x20 | `498b87000300004883c020480fb60049898790020000c3` (23) | same | same | Y | `974c709509825da0` | `974c709509825da0` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x52 imm=0x14 | `498b87900200004883c01449898790020000c3` (19) | same | same | Y | `d868fff3f47795b7` | `d868fff3f47795b7` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x50 imm=0x0A | `498b87800200004883e80a49898780020000c3` (19) | same | same | Y | `ba5ad3395d4dc1a6` | `ba5ad3395d4dc1a6` | PASS |
| 8 | 0x30 SET | slot=0x51 imm=0xDEADF00D | `48b80df0adde0000000049898788020000c3` (18) | same | same | Y | `022feb111dc961ea` | `022feb111dc961ea` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x30 SET slot=0x52 imm=0xDEADF00D — **PASS**

- fixture: `_scratch_set_52_deadf00d.ty` + `.code.hex`
- expected pin (18B): `48b80df0adde0000000049898790020000c3`
- js-sha256: `34b8f29b8558e0c5141adeb334016ca4e9d2772977fef57f6cba0bdf996d97dd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x51 imm=0x14 — **PASS**

- fixture: `_scratch_addimm_h51_14.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c01449898788020000c3`
- js-sha256: `0de1fe36c79129f6d890a2f0016b461e975e29a850525c423c56a31a1f3034fd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x51 imm=0x0A — **PASS**

- fixture: `_scratch_subimm_h51_0a.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e80a49898788020000c3`
- js-sha256: `4da400c99cc085fe1e5d3af05b55f4fb95247f088aaac1fe3cd81a7f1f3c097c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x51 ss=0x60 oo=0x20 — **PASS**

- fixture: `_scratch_ldb_5160_20.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c020480fb60049898788020000c3`
- js-sha256: `5d16e28161ed63a9d62328a8e78d567501615ae5b44130537fa826a9657ef51a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x52 ss=0x60 oo=0x20 — **PASS**

- fixture: `_scratch_ldb_5260_20.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c020480fb60049898790020000c3`
- js-sha256: `974c709509825da0ece180b77ed2e346203c83fb87c7f4bc175fcb7f4d260f60`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x52 imm=0x14 — **PASS**

- fixture: `_scratch_addimm_h52_14.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c01449898790020000c3`
- js-sha256: `d868fff3f47795b7f7c6375c3520801612c72d96e581c8af6c2b61a354495bc3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x50 imm=0x0A — **PASS**

- fixture: `_scratch_subimm_h50_0a.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e80a49898780020000c3`
- js-sha256: `ba5ad3395d4dc1a6a591578990bc48ea8121809eb0798c0c28e5b9bc502d43a7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x30 SET slot=0x51 imm=0xDEADF00D — **PASS**

- fixture: `_scratch_set_51_deadf00d.ty` + `.code.hex`
- expected pin (18B): `48b80df0adde0000000049898788020000c3`
- js-sha256: `022feb111dc961ea3eedb3cdf88a5b34ebbf0cc39a32c1250b496ced508bbf64`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SET at slot 52 imm=DEADF00D (H_53 CAFEBABE; H_86 FEEDFACE; H_95 11111111).
- ADD-IMM at slot 51 imm=14 (H_64=51 07; H_80=51 0A; H_108=50 14).
- SUB-IMM at slot 51 imm=0A (H_70=51 03; H_81=50 05; H_96=50 08; H_106=52 08).
- LDB dd=51 ss=60 oo=20 (H_61=51 60 08; H_90=51 60 10; H_103=51 60 18; H_33=50 60 20).
- LDB dd=52 ss=60 oo=20 (H_69=52 60 08; H_98=52 60 10; H_104=52 60 18).
- ADD-IMM at slot 52 imm=14 (H_78=52 07; H_97=52 0A).
- SUB-IMM at slot 50 imm=0A (H_81=50 05; H_96=50 08).
- SET at slot 51 imm=DEADF00D (H_60 DEADBEEF; H_87 AABBCCDD; H_105 C0FFEE00).
- Skipped suggested INC/DEC 50 (H_17/H_18), GET/ORV/ADDV/SUBV 50 51 (early), IMUL 50 51 (H_34), LDB 50 60 10 (H_42).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_set_52_deadf00d.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_14.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_0a.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_20.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_20.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_14.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_0a.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_51_deadf00d.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-17-log.md` — this file
- `scripts/_probe/parallel-batch-17-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-023 serialize PASSes + 1 Relock**

Pass pin from body-extend-022 Relock: `c2d5106637e7fd4954668c06dba34a2d699f1b36a6053a8df027c19b251504eb`.
Handlers before consolidate = 116 (H_00..H_109). Next selectors 0x74.. for H_110.. if all serialize.

PASS list for body-extend-023:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_110 | 0x74 | 0x30 SET | 0x52 0xDEADF00D | `48b80df0adde0000000049898790020000c3` (18B) | `34b8f29b8558e0c5` |
| H_111 | 0x75 | 0x62 ADD-IMM | 0x51 0x14 | `498b87880200004883c01449898788020000c3` (19B) | `0de1fe36c79129f6` |
| H_112 | 0x76 | 0x61 SUB-IMM | 0x51 0x0A | `498b87880200004883e80a49898788020000c3` (19B) | `4da400c99cc085fe` |
| H_113 | 0x77 | 0x80 LDB | 0x51 0x60 0x20 | `498b87000300004883c020480fb60049898788020000c3` (23B) | `5d16e28161ed63a9` |
| H_114 | 0x78 | 0x80 LDB | 0x52 0x60 0x20 | `498b87000300004883c020480fb60049898790020000c3` (23B) | `974c709509825da0` |
| H_115 | 0x79 | 0x62 ADD-IMM | 0x52 0x14 | `498b87900200004883c01449898790020000c3` (19B) | `d868fff3f47795b7` |
| H_116 | 0x7A | 0x61 SUB-IMM | 0x50 0x0A | `498b87800200004883e80a49898780020000c3` (19B) | `ba5ad3395d4dc1a6` |
| H_117 | 0x7B | 0x30 SET | 0x51 0xDEADF00D | `48b80df0adde0000000049898788020000c3` (18B) | `022feb111dc961ea` |

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
  fresh slot/imm/dst combinations not in H_48..H_109.
- If the parent decides to serialize, append H_110.. at selectors 0x74..:
  - H_110 0x30 SET (30 52 DEADF00D) — pin `48b80df0adde0000000049898790020000c3`
  - H_111 0x62 ADD-IMM (62 51 14) — pin `498b87880200004883c01449898788020000c3`
  - H_112 0x61 SUB-IMM (61 51 0A) — pin `498b87880200004883e80a49898788020000c3`
  - H_113 0x80 LDB (80 51 60 20) — pin `498b87000300004883c020480fb60049898788020000c3`
  - H_114 0x80 LDB (80 52 60 20) — pin `498b87000300004883c020480fb60049898790020000c3`
  - H_115 0x62 ADD-IMM (62 52 14) — pin `498b87900200004883c01449898790020000c3`
  - H_116 0x61 SUB-IMM (61 50 0A) — pin `498b87800200004883e80a49898780020000c3`
  - H_117 0x30 SET (30 51 DEADF00D) — pin `48b80df0adde0000000049898788020000c3`
- Plus 1 Relock after append from pin `c2d51066…`.

## §7. Consolidation handoff

parent next = body-extend-023 serialize PASSes + 1 Relock

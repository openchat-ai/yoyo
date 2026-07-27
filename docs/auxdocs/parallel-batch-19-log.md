# parallel-batch-19 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-19-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-024 (pin `59f461e4…`, handlers = 132, H_118..H_125 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_125 and
> not already present as handlers in current `yoyo.ty` (skipped early
> INC/DEC 50, GET/ORV/ADDV/SUBV 50 51, IMUL 50 51).
> Slot/imm/dst variations of SET/ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x28 | `498b87000300004883c028480fb60049898790020000c3` (23) | same | same | Y | `79c28018959b4fc6` | `79c28018959b4fc6` | PASS |
| 2 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x30 | `498b87000300004883c030480fb60049898780020000c3` (23) | same | same | Y | `cd94626912ff725b` | `cd94626912ff725b` | PASS |
| 3 | 0x30 SET | slot=0x51 imm=0xBAADF00D | `48b80df0adba0000000049898788020000c3` (18) | same | same | Y | `4fdd3935ab5d005b` | `4fdd3935ab5d005b` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x52 imm=0x1E | `498b87900200004883c01e49898790020000c3` (19) | same | same | Y | `17f9786a60b3bf8e` | `17f9786a60b3bf8e` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x50 imm=0x14 | `498b87800200004883e81449898780020000c3` (19) | same | same | Y | `63dd43fcd1171d88` | `63dd43fcd1171d88` | PASS |
| 6 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x30 | `498b87000300004883c030480fb60049898788020000c3` (23) | same | same | Y | `76a78769a45c1add` | `76a78769a45c1add` | PASS |
| 7 | 0x30 SET | slot=0x52 imm=0xBAADF00D | `48b80df0adba0000000049898790020000c3` (18) | same | same | Y | `6a510ef468b0ac9d` | `6a510ef468b0ac9d` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x52 imm=0x14 | `498b87900200004883e81449898790020000c3` (19) | same | same | Y | `92d5ef49974024ee` | `92d5ef49974024ee` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x52 ss=0x60 oo=0x28 — **PASS**

- fixture: `_scratch_ldb_5260_28.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c028480fb60049898790020000c3`
- js-sha256: `79c28018959b4fc641d64335b8cd130aa655beb3dfd084de812ca8fcc4122699`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x50 ss=0x60 oo=0x30 — **PASS**

- fixture: `_scratch_ldb_5060_30.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c030480fb60049898780020000c3`
- js-sha256: `cd94626912ff725b577cdf1fae88078dcb29e27f41a817c902352fc6a0fa2e8b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x30 SET slot=0x51 imm=0xBAADF00D — **PASS**

- fixture: `_scratch_set_51_baadf00d.ty` + `.code.hex`
- expected pin (18B): `48b80df0adba0000000049898788020000c3`
- js-sha256: `4fdd3935ab5d005b9a36c467ddb9e2532f4ceeeb220bd6757ef32656add08249`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x52 imm=0x1E — **PASS**

- fixture: `_scratch_addimm_h52_1e.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c01e49898790020000c3`
- js-sha256: `17f9786a60b3bf8e97e60c506cf3dff061f2122d2bad6f3d71031fe2dda04d17`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x50 imm=0x14 — **PASS**

- fixture: `_scratch_subimm_h50_14.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e81449898780020000c3`
- js-sha256: `63dd43fcd1171d88350c1f0d2ec36b4857e70050b0c80613ecfc691f5003069e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x51 ss=0x60 oo=0x30 — **PASS**

- fixture: `_scratch_ldb_5160_30.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c030480fb60049898788020000c3`
- js-sha256: `76a78769a45c1add03ae3747a677f1b5f4be0d311117692daea4d392da51412b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x30 SET slot=0x52 imm=0xBAADF00D — **PASS**

- fixture: `_scratch_set_52_baadf00d.ty` + `.code.hex`
- expected pin (18B): `48b80df0adba0000000049898790020000c3`
- js-sha256: `6a510ef468b0ac9dc63b5ace1414a89d50132097f4d0da453224b64a81a5e28d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x52 imm=0x14 — **PASS**

- fixture: `_scratch_subimm_h52_14.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e81449898790020000c3`
- js-sha256: `92d5ef49974024eebc1f9163452ea5b6653348a640be3930a1bda13bc1d7d03a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=52 ss=60 oo=28 (H_121=50 60 28; H_125=51 60 28; H_69/H_98/H_104/H_114 other oo at 52).
- LDB dd=50 ss=60 oo=30 (H_99=50 60 18; H_121=50 60 28; early H_33/H_44/H_45 other oo).
- SET at slot 51 imm=BAADF00D (H_60 DEADBEEF; H_87 AABBCCDD; H_105 C0FFEE00; H_117 DEADF00D).
- ADD-IMM at slot 52 imm=1E (H_78=52 07; H_97=52 0A; H_115=52 14).
- SUB-IMM at slot 50 imm=14 (H_81=50 05; H_96=50 08; H_116=50 0A).
- LDB dd=51 ss=60 oo=30 (H_61/H_90/H_103/H_113/H_125 other oo at 51).
- SET at slot 52 imm=BAADF00D (H_53 CAFEBABE; H_86 FEEDFACE; H_95 11111111; H_110 DEADF00D; H_122 FACEFEED).
- SUB-IMM at slot 52 imm=14 (H_79=52 03; H_106=52 08; H_120=52 0A).
- Skipped suggested INC/DEC 50 (H_17/H_18), GET/ORV/ADDV/SUBV 50 51 (early), IMUL 50 51 (H_34).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5260_28.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_30.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_51_baadf00d.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_1e.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_14.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_30.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_52_baadf00d.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_14.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-19-log.md` — this file
- `scripts/_probe/parallel-batch-19-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-025 serialize PASSes + 1 Relock**

Pass pin from body-extend-024 Relock: `59f461e4f8bcb4fd42077f2664dcf375e427c5a651bf7c1b5e7da612e9ca8840`.
Handlers before consolidate = 132 (H_00..H_125). Next selectors 0x84.. for H_126.. if all serialize.

PASS list for body-extend-025:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_126 | 0x84 | 0x80 LDB | 0x52 0x60 0x28 | `498b87000300004883c028480fb60049898790020000c3` (23B) | `79c28018959b4fc6` |
| H_127 | 0x85 | 0x80 LDB | 0x50 0x60 0x30 | `498b87000300004883c030480fb60049898780020000c3` (23B) | `cd94626912ff725b` |
| H_128 | 0x86 | 0x30 SET | 0x51 0xBAADF00D | `48b80df0adba0000000049898788020000c3` (18B) | `4fdd3935ab5d005b` |
| H_129 | 0x87 | 0x62 ADD-IMM | 0x52 0x1E | `498b87900200004883c01e49898790020000c3` (19B) | `17f9786a60b3bf8e` |
| H_130 | 0x88 | 0x61 SUB-IMM | 0x50 0x14 | `498b87800200004883e81449898780020000c3` (19B) | `63dd43fcd1171d88` |
| H_131 | 0x89 | 0x80 LDB | 0x51 0x60 0x30 | `498b87000300004883c030480fb60049898788020000c3` (23B) | `76a78769a45c1add` |
| H_132 | 0x8A | 0x30 SET | 0x52 0xBAADF00D | `48b80df0adba0000000049898790020000c3` (18B) | `6a510ef468b0ac9d` |
| H_133 | 0x8B | 0x61 SUB-IMM | 0x52 0x14 | `498b87900200004883e81449898790020000c3` (19B) | `92d5ef49974024ee` |

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
  fresh slot/imm/dst combinations not in H_48..H_125.
- If the parent decides to serialize, append H_126.. at selectors 0x84..:
  - H_126 0x80 LDB (80 52 60 28) — pin `498b87000300004883c028480fb60049898790020000c3`
  - H_127 0x80 LDB (80 50 60 30) — pin `498b87000300004883c030480fb60049898780020000c3`
  - H_128 0x30 SET (30 51 BAADF00D) — pin `48b80df0adba0000000049898788020000c3`
  - H_129 0x62 ADD-IMM (62 52 1E) — pin `498b87900200004883c01e49898790020000c3`
  - H_130 0x61 SUB-IMM (61 50 14) — pin `498b87800200004883e81449898780020000c3`
  - H_131 0x80 LDB (80 51 60 30) — pin `498b87000300004883c030480fb60049898788020000c3`
  - H_132 0x30 SET (30 52 BAADF00D) — pin `48b80df0adba0000000049898790020000c3`
  - H_133 0x61 SUB-IMM (61 52 14) — pin `498b87900200004883e81449898790020000c3`
- Plus 1 Relock after append from pin `59f461e4…`.

## §7. Consolidation handoff

parent next = body-extend-025 serialize PASSes + 1 Relock

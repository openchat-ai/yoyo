# parallel-batch-21 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-21-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-026 (pin `6c42f38c…`, handlers = 148, H_134..H_141 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_141 and
> not already present as handlers in current `yoyo.ty` (skipped early
> INC/DEC 50, GET/ORV/ADDV/SUBV 50 51, IMUL 50 51, LDB 50 60 40=H_43).
> Slot/imm/dst variations of SET/ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x38 | `498b87000300004883c038480fb60049898790020000c3` (23) | same | same | Y | `3a77b354a8f367d9` | `3a77b354a8f367d9` | PASS |
| 2 | 0x30 SET | slot=0x51 imm=0xFEEDC0DE | `48b8dec0edfe0000000049898788020000c3` (18) | same | same | Y | `c5643d1114f105f8` | `c5643d1114f105f8` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x52 imm=0x28 | `498b87900200004883c02849898790020000c3` (19) | same | same | Y | `5550c0d36ce045ad` | `5550c0d36ce045ad` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x50 imm=0x1E | `498b87800200004883e81e49898780020000c3` (19) | same | same | Y | `2f7e70868b896f51` | `2f7e70868b896f51` | PASS |
| 5 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x40 | `498b87000300004883c040480fb60049898788020000c3` (23) | same | same | Y | `bedb61608d220fc2` | `bedb61608d220fc2` | PASS |
| 6 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x40 | `498b87000300004883c040480fb60049898790020000c3` (23) | same | same | Y | `579799f170fc91b1` | `579799f170fc91b1` | PASS |
| 7 | 0x30 SET | slot=0x52 imm=0xFEEDC0DE | `48b8dec0edfe0000000049898790020000c3` (18) | same | same | Y | `24133e376bdef965` | `24133e376bdef965` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x51 imm=0x28 | `498b87880200004883e82849898788020000c3` (19) | same | same | Y | `d552be0871d06b76` | `d552be0871d06b76` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x52 ss=0x60 oo=0x38 — **PASS**

- fixture: `_scratch_ldb_5260_38.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c038480fb60049898790020000c3`
- js-sha256: `3a77b354a8f367d96aee4d29da1821cd86894b36e7a0834dc9c3933a580cb3a8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x30 SET slot=0x51 imm=0xFEEDC0DE — **PASS**

- fixture: `_scratch_set_51_feedc0de.ty` + `.code.hex`
- expected pin (18B): `48b8dec0edfe0000000049898788020000c3`
- js-sha256: `c5643d1114f105f821fa0a4a5f9bc6a91cc05f332d7adac344bea52f320c965f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x52 imm=0x28 — **PASS**

- fixture: `_scratch_addimm_h52_28.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c02849898790020000c3`
- js-sha256: `5550c0d36ce045ade9841f009656d6384cc7cc4620c65ec7d582003db899dae0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x50 imm=0x1E — **PASS**

- fixture: `_scratch_subimm_h50_1e.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e81e49898780020000c3`
- js-sha256: `2f7e70868b896f51dd30804728c3048b61c1ecdc70a0e3b517c682f56a97b8e8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x51 ss=0x60 oo=0x40 — **PASS**

- fixture: `_scratch_ldb_5160_40.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c040480fb60049898788020000c3`
- js-sha256: `bedb61608d220fc24d9fb46d19fab1159d3b70d44b019053e389dba987cc8956`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x52 ss=0x60 oo=0x40 — **PASS**

- fixture: `_scratch_ldb_5260_40.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c040480fb60049898790020000c3`
- js-sha256: `579799f170fc91b118eb31bfa020cfca1206293e9825d40db79e431bce9b3f2c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x30 SET slot=0x52 imm=0xFEEDC0DE — **PASS**

- fixture: `_scratch_set_52_feedc0de.ty` + `.code.hex`
- expected pin (18B): `48b8dec0edfe0000000049898790020000c3`
- js-sha256: `24133e376bdef965f90c5f45d5d424475528135a7c11da6f6f56ee2742ab413f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x51 imm=0x28 — **PASS**

- fixture: `_scratch_subimm_h51_28.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e82849898788020000c3`
- js-sha256: `d552be0871d06b7664f633c8c48386942cc100afd48c4f22485296b818284ca4`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=52 ss=60 oo=38 (H_134=52 60 30; H_135=50 60 38; H_139=51 60 38).
- SET at slot 51 imm=FEEDC0DE (H_60 DEADBEEF; H_87 AABBCCDD; H_105 C0FFEE00; H_117 DEADF00D; H_128 BAADF00D; ≠ FEEDFACE).
- ADD-IMM at slot 52 imm=28 (H_78=52 07; H_97=52 0A; H_115=52 14; H_129=52 1E; H_137/140=51/50 28).
- SUB-IMM at slot 50 imm=1E (H_81=50 05; H_96=50 08; H_116=50 0A; H_130=50 14; H_138/141=51/52 1E).
- LDB dd=51 ss=60 oo=40 (H_43 is dd=50 oo=40; H_61/90/103/113/125/131/139 other oo at 51).
- LDB dd=52 ss=60 oo=40 (H_69/98/104/114/126/134 other oo at 52).
- SET at slot 52 imm=FEEDC0DE (H_53 CAFEBABE; H_86 FEEDFACE; H_95 11111111; H_110 DEADF00D; H_122 FACEFEED; H_132 BAADF00D).
- SUB-IMM at slot 51 imm=28 (H_70=51 03; H_112=51 0A; H_124=51 05; H_138=51 1E).
- Skipped suggested INC/DEC 50 (H_17/H_18), GET/ORV/ADDV/SUBV 50 51 (early), IMUL 50 51 (H_34), LDB 50 60 40 (H_43).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5260_38.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_51_feedc0de.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_28.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_1e.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_40.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_40.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_52_feedc0de.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_28.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-21-log.md` — this file
- `scripts/_probe/parallel-batch-21-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-027 serialize PASSes + 1 Relock**

Pass pin from body-extend-026 Relock: `6c42f38cd61a0603f8892cbfdf36ab3966be5f894ce6a053c403d014507a6cc7`.
Handlers before consolidate = 148 (H_00..H_141). Next selectors 0x94.. for H_142.. if all serialize.

PASS list for body-extend-027:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_142 | 0x94 | 0x80 LDB | 0x52 0x60 0x38 | `498b87000300004883c038480fb60049898790020000c3` (23B) | `3a77b354a8f367d9` |
| H_143 | 0x95 | 0x30 SET | 0x51 0xFEEDC0DE | `48b8dec0edfe0000000049898788020000c3` (18B) | `c5643d1114f105f8` |
| H_144 | 0x96 | 0x62 ADD-IMM | 0x52 0x28 | `498b87900200004883c02849898790020000c3` (19B) | `5550c0d36ce045ad` |
| H_145 | 0x97 | 0x61 SUB-IMM | 0x50 0x1E | `498b87800200004883e81e49898780020000c3` (19B) | `2f7e70868b896f51` |
| H_146 | 0x98 | 0x80 LDB | 0x51 0x60 0x40 | `498b87000300004883c040480fb60049898788020000c3` (23B) | `bedb61608d220fc2` |
| H_147 | 0x99 | 0x80 LDB | 0x52 0x60 0x40 | `498b87000300004883c040480fb60049898790020000c3` (23B) | `579799f170fc91b1` |
| H_148 | 0x9A | 0x30 SET | 0x52 0xFEEDC0DE | `48b8dec0edfe0000000049898790020000c3` (18B) | `24133e376bdef965` |
| H_149 | 0x9B | 0x61 SUB-IMM | 0x51 0x28 | `498b87880200004883e82849898788020000c3` (19B) | `d552be0871d06b76` |

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
  fresh slot/imm/dst combinations not in H_48..H_141.
- If the parent decides to serialize, append H_142.. at selectors 0x94..:
  - H_142 0x80 LDB (80 52 60 38) — pin `498b87000300004883c038480fb60049898790020000c3`
  - H_143 0x30 SET (30 51 FEEDC0DE) — pin `48b8dec0edfe0000000049898788020000c3`
  - H_144 0x62 ADD-IMM (62 52 28) — pin `498b87900200004883c02849898790020000c3`
  - H_145 0x61 SUB-IMM (61 50 1E) — pin `498b87800200004883e81e49898780020000c3`
  - H_146 0x80 LDB (80 51 60 40) — pin `498b87000300004883c040480fb60049898788020000c3`
  - H_147 0x80 LDB (80 52 60 40) — pin `498b87000300004883c040480fb60049898790020000c3`
  - H_148 0x30 SET (30 52 FEEDC0DE) — pin `48b8dec0edfe0000000049898790020000c3`
  - H_149 0x61 SUB-IMM (61 51 28) — pin `498b87880200004883e82849898788020000c3`
- Plus 1 Relock after append from pin `6c42f38c…`.

## §7. Consolidation handoff

parent next = body-extend-027 serialize PASSes + 1 Relock

# parallel-batch-22 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-22-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-027 (pin `2a14beec…`, handlers = 156, H_142..H_149 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_149 and
> not already present as handlers in current `yoyo.ty` (skipped early
> INC/DEC 50, GET/ORV/ADDV/SUBV 50 51, IMUL 50 51, LDB 50 60 40=H_43).
> Slot/imm/dst variations of SET/ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x30 SET | slot=0x50 imm=0xFEEDC0DE | `48b8dec0edfe0000000049898780020000c3` (18) | same | same | Y | `3d87228f78707f16` | `3d87228f78707f16` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x50 imm=0x32 | `498b87800200004883c03249898780020000c3` (19) | same | same | Y | `5cc13067b0ad0632` | `5cc13067b0ad0632` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x52 imm=0x28 | `498b87900200004883e82849898790020000c3` (19) | same | same | Y | `d336d72829e79f77` | `d336d72829e79f77` | PASS |
| 4 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x48 | `498b87000300004883c048480fb60049898780020000c3` (23) | same | same | Y | `db3f030b072b721d` | `db3f030b072b721d` | PASS |
| 5 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x48 | `498b87000300004883c048480fb60049898788020000c3` (23) | same | same | Y | `3e69600006d17327` | `3e69600006d17327` | PASS |
| 6 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x48 | `498b87000300004883c048480fb60049898790020000c3` (23) | same | same | Y | `0cfd11ffdf5be6f0` | `0cfd11ffdf5be6f0` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x51 imm=0x32 | `498b87880200004883c03249898788020000c3` (19) | same | same | Y | `344d6d45a4ba02f0` | `344d6d45a4ba02f0` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x50 imm=0x28 | `498b87800200004883e82849898780020000c3` (19) | same | same | Y | `533c4ac0d8d19f34` | `533c4ac0d8d19f34` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x30 SET slot=0x50 imm=0xFEEDC0DE — **PASS**

- fixture: `_scratch_set_50_feedc0de.ty` + `.code.hex`
- expected pin (18B): `48b8dec0edfe0000000049898780020000c3`
- js-sha256: `3d87228f78707f16eadf1b3ef249639c448b7dc1db64a3d9d8ad24221f88e9a5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x50 imm=0x32 — **PASS**

- fixture: `_scratch_addimm_h50_32.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c03249898780020000c3`
- js-sha256: `5cc13067b0ad0632d182035545563e32a5ab3bb143beae8b89fb7bceb1e04463`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x52 imm=0x28 — **PASS**

- fixture: `_scratch_subimm_h52_28.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e82849898790020000c3`
- js-sha256: `d336d72829e79f7758647ae6bf109d309d5797c99969f6983cbe433d90d292d3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x50 ss=0x60 oo=0x48 — **PASS**

- fixture: `_scratch_ldb_5060_48.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c048480fb60049898780020000c3`
- js-sha256: `db3f030b072b721d36372873f3d9e00220569c23443560255a7217cfa82849df`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x51 ss=0x60 oo=0x48 — **PASS**

- fixture: `_scratch_ldb_5160_48.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c048480fb60049898788020000c3`
- js-sha256: `3e69600006d1732791495789ca5a0f9ffea91fee45d2d94a6af4b3ac0459c47d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x52 ss=0x60 oo=0x48 — **PASS**

- fixture: `_scratch_ldb_5260_48.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c048480fb60049898790020000c3`
- js-sha256: `0cfd11ffdf5be6f0b031f500c1b90c644ba74f931c83b5d9da2706557aee32e3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x51 imm=0x32 — **PASS**

- fixture: `_scratch_addimm_h51_32.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c03249898788020000c3`
- js-sha256: `344d6d45a4ba02f09853820d0aa0320951fe86b20300873451f4f9f682e097f4`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x50 imm=0x28 — **PASS**

- fixture: `_scratch_subimm_h50_28.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e82849898780020000c3`
- js-sha256: `533c4ac0d8d19f3438724884f33fbf6467c2e2576634bea73720cd6872b8d977`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SET at slot 50 imm=FEEDC0DE (H_143/H_148 = 51/52 FEEDC0DE; H_68/76/94/109/118/136 other imm at 50).
- ADD-IMM at slot 50 imm=32 (H_93=50 0F; H_108=50 14; H_123=50 1E; H_140=50 28).
- SUB-IMM at slot 52 imm=28 (H_79=52 03; H_106=52 08; H_120=52 0A; H_133=52 14; H_141=52 1E; H_149=51 28).
- LDB dd=50/51/52 ss=60 oo=48 (locked LDB oo through 40; H_146/147 oo=40; early H_43=50 60 40).
- ADD-IMM at slot 51 imm=32 (H_64=51 07; H_80=51 0A; H_111=51 14; H_119=51 1E; H_137=51 28).
- SUB-IMM at slot 50 imm=28 (H_81=50 05; H_96=50 08; H_116=50 0A; H_130=50 14; H_145=50 1E; H_149=51 28).
- Skipped suggested INC/DEC 50 (H_17/H_18), GET/ORV/ADDV/SUBV 50 51 (early), IMUL 50 51 (H_34), LDB 50 60 40 (H_43).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_set_50_feedc0de.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_32.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_28.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_48.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_48.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_48.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_32.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_28.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-22-log.md` — this file
- `scripts/_probe/parallel-batch-22-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-028 serialize PASSes + 1 Relock**

Pass pin from body-extend-027 Relock: `2a14beec0f08ffdfd64656bc2230706c4ec1928a697bf00f3905ff724c4d28f2`.
Handlers before consolidate = 156 (H_00..H_149). Next selectors 0x9C.. for H_150.. if all serialize.

PASS list for body-extend-028:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_150 | 0x9C | 0x30 SET | 0x50 0xFEEDC0DE | `48b8dec0edfe0000000049898780020000c3` (18B) | `3d87228f78707f16` |
| H_151 | 0x9D | 0x62 ADD-IMM | 0x50 0x32 | `498b87800200004883c03249898780020000c3` (19B) | `5cc13067b0ad0632` |
| H_152 | 0x9E | 0x61 SUB-IMM | 0x52 0x28 | `498b87900200004883e82849898790020000c3` (19B) | `d336d72829e79f77` |
| H_153 | 0x9F | 0x80 LDB | 0x50 0x60 0x48 | `498b87000300004883c048480fb60049898780020000c3` (23B) | `db3f030b072b721d` |
| H_154 | 0xA0 | 0x80 LDB | 0x51 0x60 0x48 | `498b87000300004883c048480fb60049898788020000c3` (23B) | `3e69600006d17327` |
| H_155 | 0xA1 | 0x80 LDB | 0x52 0x60 0x48 | `498b87000300004883c048480fb60049898790020000c3` (23B) | `0cfd11ffdf5be6f0` |
| H_156 | 0xA2 | 0x62 ADD-IMM | 0x51 0x32 | `498b87880200004883c03249898788020000c3` (19B) | `344d6d45a4ba02f0` |
| H_157 | 0xA3 | 0x61 SUB-IMM | 0x50 0x28 | `498b87800200004883e82849898780020000c3` (19B) | `533c4ac0d8d19f34` |

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
  fresh slot/imm/dst combinations not in H_48..H_149.
- If the parent decides to serialize, append H_150.. at selectors 0x9C..:
  - H_150 0x30 SET (30 50 FEEDC0DE) — pin `48b8dec0edfe0000000049898780020000c3`
  - H_151 0x62 ADD-IMM (62 50 32) — pin `498b87800200004883c03249898780020000c3`
  - H_152 0x61 SUB-IMM (61 52 28) — pin `498b87900200004883e82849898790020000c3`
  - H_153 0x80 LDB (80 50 60 48) — pin `498b87000300004883c048480fb60049898780020000c3`
  - H_154 0x80 LDB (80 51 60 48) — pin `498b87000300004883c048480fb60049898788020000c3`
  - H_155 0x80 LDB (80 52 60 48) — pin `498b87000300004883c048480fb60049898790020000c3`
  - H_156 0x62 ADD-IMM (62 51 32) — pin `498b87880200004883c03249898788020000c3`
  - H_157 0x61 SUB-IMM (61 50 28) — pin `498b87800200004883e82849898780020000c3`
- Plus 1 Relock after append from pin `2a14beec…`.

## §7. Consolidation handoff

parent next = body-extend-028 serialize PASSes + 1 Relock

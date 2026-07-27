# parallel-batch-47 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-47-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-052 (pin `edee584a…`, handlers = 356, H_342..H_349 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-052 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_349 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x50 ss=0x60 oo=0xE8 | `498b87000300004881c0e8000000480fb60049898780020000c3` (26) | same | same | Y | `8707f42f9e69fe94` | `8707f42f9e69fe94` | PASS |
| 2 | 0x80 LDB | dd=0x51 ss=0x60 oo=0xE8 | `498b87000300004881c0e8000000480fb60049898788020000c3` (26) | same | same | Y | `1aa2e13843e522b5` | `1aa2e13843e522b5` | PASS |
| 3 | 0x80 LDB | dd=0x52 ss=0x60 oo=0xE8 | `498b87000300004881c0e8000000480fb60049898790020000c3` (26) | same | same | Y | `465cb3e854ecc953` | `465cb3e854ecc953` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x50 imm=0xE0 | `498b87800200004881c0e000000049898780020000c3` (22) | same | same | Y | `9ef1fb8eb620deee` | `9ef1fb8eb620deee` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x51 imm=0xE0 | `498b87880200004881c0e000000049898788020000c3` (22) | same | same | Y | `4d09c2a3e224d2d4` | `4d09c2a3e224d2d4` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x52 imm=0xE0 | `498b87900200004881c0e000000049898790020000c3` (22) | same | same | Y | `cd251baeb9a188f0` | `cd251baeb9a188f0` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x50 imm=0xE0 | `498b87800200004881e8e000000049898780020000c3` (22) | same | same | Y | `6d7c5904f21181f1` | `6d7c5904f21181f1` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x51 imm=0xE0 | `498b87880200004881e8e000000049898788020000c3` (22) | same | same | Y | `345b5a0581126cf4` | `345b5a0581126cf4` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x50 ss=0x60 oo=0xE8 — **PASS**

- fixture: `_scratch_ldb_5060_e8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0e8000000480fb60049898780020000c3`
- js-sha256: `8707f42f9e69fe94d91d27d11d46d30b579f1face242fca3b462ed6326d31418`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x51 ss=0x60 oo=0xE8 — **PASS**

- fixture: `_scratch_ldb_5160_e8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0e8000000480fb60049898788020000c3`
- js-sha256: `1aa2e13843e522b5978ad735a3835ccb3c5615f1c90f6c51068bf4a43ccb4d27`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x52 ss=0x60 oo=0xE8 — **PASS**

- fixture: `_scratch_ldb_5260_e8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0e8000000480fb60049898790020000c3`
- js-sha256: `465cb3e854ecc953423252e22214fd57713b1b578b7587ce65a901f56dc3e925`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x50 imm=0xE0 — **PASS**

- fixture: `_scratch_addimm_h50_e0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0e000000049898780020000c3`
- js-sha256: `9ef1fb8eb620deee26bffb39e983df220c047ba07c696a7ffd5b3abf58d20fac`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x51 imm=0xE0 — **PASS**

- fixture: `_scratch_addimm_h51_e0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0e000000049898788020000c3`
- js-sha256: `4d09c2a3e224d2d48d82a155a10e6aefa3c2d737f607d3472a32aa49b2457ede`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x52 imm=0xE0 — **PASS**

- fixture: `_scratch_addimm_h52_e0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0e000000049898790020000c3`
- js-sha256: `cd251baeb9a188f09b0b9a47fbb1160d773dc317eb7fecf06e455fc1fc20fc1e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x50 imm=0xE0 — **PASS**

- fixture: `_scratch_subimm_h50_e0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8e000000049898780020000c3`
- js-sha256: `6d7c5904f21181f10ab244e96496582e13be627cead076bdc5fe8af5a3b7af7a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x51 imm=0xE0 — **PASS**

- fixture: `_scratch_subimm_h51_e0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8e000000049898788020000c3`
- js-sha256: `345b5a0581126cf42158fac6fd12fd3b2d6b697ac475c2ce4891e9aa078641e4`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=50/51/52 ss=60 oo=E8 (next oo after E0 triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=E0 (fresh imm after D8; imm32 22B).
- SUB-IMM slot=50/51 imm=E0 (complements ADD-IMM * E0; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 164`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5060_e8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_e8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_e8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_e0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_e0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_e0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_e0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_e0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-47-log.md` — this file
- `scripts/_probe/parallel-batch-47-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-053 serialize PASSes + 1 Relock**

Pass pin from body-extend-052 Relock: `edee584aa21a26569fe08e60d5089daf8d823c9df4c829c62b788b10815f4a51`.
Handlers before consolidate = 356 (H_00..H_349). Next selectors `40 164`.. for H_350.. if all serialize.

PASS list for body-extend-053:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_350 | 0x164 | 0x80 LDB | 0x50 0x60 0xE8 | `498b87000300004881c0e8000000480fb60049898780020000c3` (26B) | `8707f42f9e69fe94` |
| H_351 | 0x165 | 0x80 LDB | 0x51 0x60 0xE8 | `498b87000300004881c0e8000000480fb60049898788020000c3` (26B) | `1aa2e13843e522b5` |
| H_352 | 0x166 | 0x80 LDB | 0x52 0x60 0xE8 | `498b87000300004881c0e8000000480fb60049898790020000c3` (26B) | `465cb3e854ecc953` |
| H_353 | 0x167 | 0x62 ADD-IMM | 0x50 0xE0 | `498b87800200004881c0e000000049898780020000c3` (22B) | `9ef1fb8eb620deee` |
| H_354 | 0x168 | 0x62 ADD-IMM | 0x51 0xE0 | `498b87880200004881c0e000000049898788020000c3` (22B) | `4d09c2a3e224d2d4` |
| H_355 | 0x169 | 0x62 ADD-IMM | 0x52 0xE0 | `498b87900200004881c0e000000049898790020000c3` (22B) | `cd251baeb9a188f0` |
| H_356 | 0x16A | 0x61 SUB-IMM | 0x50 0xE0 | `498b87800200004881e8e000000049898780020000c3` (22B) | `6d7c5904f21181f1` |
| H_357 | 0x16B | 0x61 SUB-IMM | 0x51 0xE0 | `498b87880200004881e8e000000049898788020000c3` (22B) | `345b5a0581126cf4` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-052 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_349.
- If the parent decides to serialize, append H_350.. at selectors `40 164`..:
  - H_350 0x80 LDB (80 50 60 E8) — pin `498b87000300004881c0e8000000480fb60049898780020000c3`
  - H_351 0x80 LDB (80 51 60 E8) — pin `498b87000300004881c0e8000000480fb60049898788020000c3`
  - H_352 0x80 LDB (80 52 60 E8) — pin `498b87000300004881c0e8000000480fb60049898790020000c3`
  - H_353 0x62 ADD-IMM (62 50 E0) — pin `498b87800200004881c0e000000049898780020000c3`
  - H_354 0x62 ADD-IMM (62 51 E0) — pin `498b87880200004881c0e000000049898788020000c3`
  - H_355 0x62 ADD-IMM (62 52 E0) — pin `498b87900200004881c0e000000049898790020000c3`
  - H_356 0x61 SUB-IMM (61 50 E0) — pin `498b87800200004881e8e000000049898780020000c3`
  - H_357 0x61 SUB-IMM (61 51 E0) — pin `498b87880200004881e8e000000049898788020000c3`
- Plus 1 Relock after append from pin `edee584a…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-053 serialize PASSes + 1 Relock

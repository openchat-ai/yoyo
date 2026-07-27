# parallel-batch-26 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-26-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-031 (pin `dc10b2bd…`, handlers = 188, H_174..H_181 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-031 reported DDC PE `.text` VirtualSize DIFFER — do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_181 and
> not already present as handlers in current `yoyo.ty` (skipped early
> INC/DEC 50, GET/ORV/ADDV/SUBV 50 51, IMUL 50 51; skipped H_46 LDB 50 60 60).
> Slot/imm/dst variations of SET/ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x30 SET | slot=0x52 imm=0xDEADC0DE | `48b8dec0adde0000000049898790020000c3` (18) | same | same | Y | `7a587d84beb9cc85` | `7a587d84beb9cc85` | PASS |
| 2 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x68 | `498b87000300004883c068480fb60049898780020000c3` (23) | same | same | Y | `bd2195e8c421a165` | `bd2195e8c421a165` | PASS |
| 3 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x68 | `498b87000300004883c068480fb60049898788020000c3` (23) | same | same | Y | `1ea59c358f5546e1` | `1ea59c358f5546e1` | PASS |
| 4 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x68 | `498b87000300004883c068480fb60049898790020000c3` (23) | same | same | Y | `766b3c1623cfc488` | `766b3c1623cfc488` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x50 imm=0x48 | `498b87800200004883c04849898780020000c3` (19) | same | same | Y | `16f582bad178a162` | `16f582bad178a162` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x51 imm=0x48 | `498b87880200004883c04849898788020000c3` (19) | same | same | Y | `cc49b12c560f1413` | `cc49b12c560f1413` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x50 imm=0x40 | `498b87800200004883e84049898780020000c3` (19) | same | same | Y | `96696eeac9b4038b` | `96696eeac9b4038b` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x51 imm=0x40 | `498b87880200004883e84049898788020000c3` (19) | same | same | Y | `49afb30429d07d3f` | `49afb30429d07d3f` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x30 SET slot=0x52 imm=0xDEADC0DE — **PASS**

- fixture: `_scratch_set_52_deadc0de.ty` + `.code.hex`
- expected pin (18B): `48b8dec0adde0000000049898790020000c3`
- js-sha256: `7a587d84beb9cc8525cdfb50268fde6d8239674cb38ced809aa942130f255c01`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x50 ss=0x60 oo=0x68 — **PASS**

- fixture: `_scratch_ldb_5060_68.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c068480fb60049898780020000c3`
- js-sha256: `bd2195e8c421a165a1063f5ac6478beb2ecfe026cea1684b756a429aeffaca09`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x51 ss=0x60 oo=0x68 — **PASS**

- fixture: `_scratch_ldb_5160_68.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c068480fb60049898788020000c3`
- js-sha256: `1ea59c358f5546e1a17b12489c09a2ca3529eb0b7989d013d0191d3a8ef7756c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x52 ss=0x60 oo=0x68 — **PASS**

- fixture: `_scratch_ldb_5260_68.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c068480fb60049898790020000c3`
- js-sha256: `766b3c1623cfc488f7aa4a524380d8a616d3d70e3691c96ef59c8fe7e248823b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x50 imm=0x48 — **PASS**

- fixture: `_scratch_addimm_h50_48.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c04849898780020000c3`
- js-sha256: `16f582bad178a162be629684e807c1e9bc225ae4464e4b78c4157370446da321`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x51 imm=0x48 — **PASS**

- fixture: `_scratch_addimm_h51_48.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c04849898788020000c3`
- js-sha256: `cc49b12c560f141385c8e59756b75c6026babd2a6102da5140fe468d0071cc52`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x50 imm=0x40 — **PASS**

- fixture: `_scratch_subimm_h50_40.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e84049898780020000c3`
- js-sha256: `96696eeac9b4038b83a169bb84ee7d10dd4dfc6c6810f0ff8f6879181a5d165c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x51 imm=0x40 — **PASS**

- fixture: `_scratch_subimm_h51_40.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e84049898788020000c3`
- js-sha256: `49afb30429d07d3f178460f3c763f843c7865f6fd4220684c4839d1ac243361b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SET at slot 52 imm=DEADC0DE (H_174/H_181 = 50/51 only; fresh slot).
- LDB dd=50/51/52 ss=60 oo=68 (no oo=68 in locked LDB set; H_46/H_47 = oo=60/70).
- ADD-IMM at slot 50/51 imm=48 (no imm=48 ADD-IMM locked; max was 40).
- SUB-IMM at slot 50/51 imm=40 (no imm=40 SUB-IMM locked; ADD-IMM 40 exists).
- Skipped suggested INC/DEC 50, GET/ORV/ADDV/SUBV 50 51, IMUL 50 51 (early), LDB 50 60 60 (H_46).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_set_52_deadc0de.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_68.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_68.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_68.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_48.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_48.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_40.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_40.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-26-log.md` — this file
- `scripts/_probe/parallel-batch-26-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-032 serialize PASSes + 1 Relock**

Pass pin from body-extend-031 Relock: `dc10b2bd70d2232bc015d3a87c88a02d58d5eaffd5ae572fd219dd84094db127`.
Handlers before consolidate = 188 (H_00..H_181). Next selectors 0xBC.. for H_182.. if all serialize.

PASS list for body-extend-032:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_182 | 0xBC | 0x30 SET | 0x52 0xDEADC0DE | `48b8dec0adde0000000049898790020000c3` (18B) | `7a587d84beb9cc85` |
| H_183 | 0xBD | 0x80 LDB | 0x50 0x60 0x68 | `498b87000300004883c068480fb60049898780020000c3` (23B) | `bd2195e8c421a165` |
| H_184 | 0xBE | 0x80 LDB | 0x51 0x60 0x68 | `498b87000300004883c068480fb60049898788020000c3` (23B) | `1ea59c358f5546e1` |
| H_185 | 0xBF | 0x80 LDB | 0x52 0x60 0x68 | `498b87000300004883c068480fb60049898790020000c3` (23B) | `766b3c1623cfc488` |
| H_186 | 0xC0 | 0x62 ADD-IMM | 0x50 0x48 | `498b87800200004883c04849898780020000c3` (19B) | `16f582bad178a162` |
| H_187 | 0xC1 | 0x62 ADD-IMM | 0x51 0x48 | `498b87880200004883c04849898788020000c3` (19B) | `cc49b12c560f1413` |
| H_188 | 0xC2 | 0x61 SUB-IMM | 0x50 0x40 | `498b87800200004883e84049898780020000c3` (19B) | `96696eeac9b4038b` |
| H_189 | 0xC3 | 0x61 SUB-IMM | 0x51 0x40 | `498b87880200004883e84049898788020000c3` (19B) | `49afb30429d07d3f` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-031 DDC PE `.text` VirtualSize DIFFER noted — no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_181.
- If the parent decides to serialize, append H_182.. at selectors 0xBC..:
  - H_182 0x30 SET (30 52 DEADC0DE) — pin `48b8dec0adde0000000049898790020000c3`
  - H_183 0x80 LDB (80 50 60 68) — pin `498b87000300004883c068480fb60049898780020000c3`
  - H_184 0x80 LDB (80 51 60 68) — pin `498b87000300004883c068480fb60049898788020000c3`
  - H_185 0x80 LDB (80 52 60 68) — pin `498b87000300004883c068480fb60049898790020000c3`
  - H_186 0x62 ADD-IMM (62 50 48) — pin `498b87800200004883c04849898780020000c3`
  - H_187 0x62 ADD-IMM (62 51 48) — pin `498b87880200004883c04849898788020000c3`
  - H_188 0x61 SUB-IMM (61 50 40) — pin `498b87800200004883e84049898780020000c3`
  - H_189 0x61 SUB-IMM (61 51 40) — pin `498b87880200004883e84049898788020000c3`
- Plus 1 Relock after append from pin `dc10b2bd…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-032 serialize PASSes + 1 Relock

# parallel-batch-41 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-41-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-046 (pin `422c8432…`, handlers = 308, H_294..H_301 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-046 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_301 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x50 ss=0x60 oo=0xC8 | `498b87000300004881c0c8000000480fb60049898780020000c3` (26) | same | same | Y | `236016ef799b3ff7` | `236016ef799b3ff7` | PASS |
| 2 | 0x80 LDB | dd=0x51 ss=0x60 oo=0xC8 | `498b87000300004881c0c8000000480fb60049898788020000c3` (26) | same | same | Y | `7eb39f3637eb2267` | `7eb39f3637eb2267` | PASS |
| 3 | 0x80 LDB | dd=0x52 ss=0x60 oo=0xC8 | `498b87000300004881c0c8000000480fb60049898790020000c3` (26) | same | same | Y | `b9fa804bcc69d95c` | `b9fa804bcc69d95c` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x50 imm=0xB0 | `498b87800200004881c0b000000049898780020000c3` (22) | same | same | Y | `9be2c80577bd6f4a` | `9be2c80577bd6f4a` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x51 imm=0xB0 | `498b87880200004881c0b000000049898788020000c3` (22) | same | same | Y | `e3c08eecc6fae6f3` | `e3c08eecc6fae6f3` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x52 imm=0xB0 | `498b87900200004881c0b000000049898790020000c3` (22) | same | same | Y | `9d760ed911115fb1` | `9d760ed911115fb1` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x50 imm=0xB0 | `498b87800200004881e8b000000049898780020000c3` (22) | same | same | Y | `1d73d8c916bc7e20` | `1d73d8c916bc7e20` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x51 imm=0xB0 | `498b87880200004881e8b000000049898788020000c3` (22) | same | same | Y | `3e629652dbf4e5ea` | `3e629652dbf4e5ea` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x50 ss=0x60 oo=0xC8 — **PASS**

- fixture: `_scratch_ldb_5060_c8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0c8000000480fb60049898780020000c3`
- js-sha256: `236016ef799b3ff7d89b78654d7883b6c39ca7d2c189119062bb8b33b86840be`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x51 ss=0x60 oo=0xC8 — **PASS**

- fixture: `_scratch_ldb_5160_c8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0c8000000480fb60049898788020000c3`
- js-sha256: `7eb39f3637eb22675442d8ffe75672093ad49f3f2dc4a05e2644c9728c72b192`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x52 ss=0x60 oo=0xC8 — **PASS**

- fixture: `_scratch_ldb_5260_c8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0c8000000480fb60049898790020000c3`
- js-sha256: `b9fa804bcc69d95c3299850d3f4e6ed7a796ed8bcf76a8eb0e7a295d9879a1b0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x50 imm=0xB0 — **PASS**

- fixture: `_scratch_addimm_h50_b0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0b000000049898780020000c3`
- js-sha256: `9be2c80577bd6f4a435643f56840c2814984507ebe8839d981cc6508aea8eced`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x51 imm=0xB0 — **PASS**

- fixture: `_scratch_addimm_h51_b0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0b000000049898788020000c3`
- js-sha256: `e3c08eecc6fae6f3bb9d6f51283e2059bf578617053767715801d4a537c476aa`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x52 imm=0xB0 — **PASS**

- fixture: `_scratch_addimm_h52_b0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0b000000049898790020000c3`
- js-sha256: `9d760ed911115fb192c20eff1e4bedbf28e0461c687efc20f466cdd9aca5d063`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x50 imm=0xB0 — **PASS**

- fixture: `_scratch_subimm_h50_b0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8b000000049898780020000c3`
- js-sha256: `1d73d8c916bc7e2063ccb408819463585335a4e013fe46aeceac142e33363813`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x51 imm=0xB0 — **PASS**

- fixture: `_scratch_subimm_h51_b0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8b000000049898788020000c3`
- js-sha256: `3e629652dbf4e5eafd4154012e99a9c8aa5c5b67369b29036257fb60ca4ea603`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=50/51/52 ss=60 oo=C8 (next oo after C0 triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=B0 (fresh imm after A8; imm32 22B).
- SUB-IMM slot 50/51 imm=B0 (complements ADD-IMM * B0; imm=0xB0 → imm32 sub).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 134`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5060_c8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_c8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_c8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_b0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_b0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_b0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_b0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_b0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-41-log.md` — this file
- `scripts/_probe/parallel-batch-41-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-047 serialize PASSes + 1 Relock**

Pass pin from body-extend-046 Relock: `422c843275989ac30c1ba7406c7ff47076310df79ef0c3193903bca15460afde`.
Handlers before consolidate = 308 (H_00..H_301). Next selectors `40 134`.. for H_302.. if all serialize.

PASS list for body-extend-047:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_302 | 0x134 | 0x80 LDB | 0x50 0x60 0xC8 | `498b87000300004881c0c8000000480fb60049898780020000c3` (26B) | `236016ef799b3ff7` |
| H_303 | 0x135 | 0x80 LDB | 0x51 0x60 0xC8 | `498b87000300004881c0c8000000480fb60049898788020000c3` (26B) | `7eb39f3637eb2267` |
| H_304 | 0x136 | 0x80 LDB | 0x52 0x60 0xC8 | `498b87000300004881c0c8000000480fb60049898790020000c3` (26B) | `b9fa804bcc69d95c` |
| H_305 | 0x137 | 0x62 ADD-IMM | 0x50 0xB0 | `498b87800200004881c0b000000049898780020000c3` (22B) | `9be2c80577bd6f4a` |
| H_306 | 0x138 | 0x62 ADD-IMM | 0x51 0xB0 | `498b87880200004881c0b000000049898788020000c3` (22B) | `e3c08eecc6fae6f3` |
| H_307 | 0x139 | 0x62 ADD-IMM | 0x52 0xB0 | `498b87900200004881c0b000000049898790020000c3` (22B) | `9d760ed911115fb1` |
| H_308 | 0x13A | 0x61 SUB-IMM | 0x50 0xB0 | `498b87800200004881e8b000000049898780020000c3` (22B) | `1d73d8c916bc7e20` |
| H_309 | 0x13B | 0x61 SUB-IMM | 0x51 0xB0 | `498b87880200004881e8b000000049898788020000c3` (22B) | `3e629652dbf4e5ea` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-046 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_301.
- If the parent decides to serialize, append H_302.. at selectors `40 134`..:
  - H_302 0x80 LDB (80 50 60 C8) — pin `498b87000300004881c0c8000000480fb60049898780020000c3`
  - H_303 0x80 LDB (80 51 60 C8) — pin `498b87000300004881c0c8000000480fb60049898788020000c3`
  - H_304 0x80 LDB (80 52 60 C8) — pin `498b87000300004881c0c8000000480fb60049898790020000c3`
  - H_305 0x62 ADD-IMM (62 50 B0) — pin `498b87800200004881c0b000000049898780020000c3`
  - H_306 0x62 ADD-IMM (62 51 B0) — pin `498b87880200004881c0b000000049898788020000c3`
  - H_307 0x62 ADD-IMM (62 52 B0) — pin `498b87900200004881c0b000000049898790020000c3`
  - H_308 0x61 SUB-IMM (61 50 B0) — pin `498b87800200004881e8b000000049898780020000c3`
  - H_309 0x61 SUB-IMM (61 51 B0) — pin `498b87880200004881e8b000000049898788020000c3`
- Plus 1 Relock after append from pin `422c8432…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-047 serialize PASSes + 1 Relock

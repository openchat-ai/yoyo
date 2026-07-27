# parallel-batch-53 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-53-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-058 (pin `c258ff32…`, handlers = 404, H_390..H_397 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-058 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_397 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x50 imm=0x108 | `498b87800200004881e80801000049898780020000c3` (22) | same | same | Y | `f139f28243c08957` | `f139f28243c08957` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x51 imm=0x108 | `498b87880200004881e80801000049898788020000c3` (22) | same | same | Y | `f9c122832287170d` | `f9c122832287170d` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x52 imm=0x108 | `498b87900200004881e80801000049898790020000c3` (22) | same | same | Y | `2f027342f5447eeb` | `2f027342f5447eeb` | PASS |
| 4 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x110 | `498b87000300004881c010010000480fb60049898780020000c3` (26) | same | same | Y | `215fc443528e6163` | `215fc443528e6163` | PASS |
| 5 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x110 | `498b87000300004881c010010000480fb60049898788020000c3` (26) | same | same | Y | `bfd294f2e3edf3d2` | `bfd294f2e3edf3d2` | PASS |
| 6 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x110 | `498b87000300004881c010010000480fb60049898790020000c3` (26) | same | same | Y | `d9d4fceaca2783f1` | `d9d4fceaca2783f1` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x50 imm=0x110 | `498b87800200004881c01001000049898780020000c3` (22) | same | same | Y | `b2f08439005e085c` | `b2f08439005e085c` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x51 imm=0x110 | `498b87880200004881c01001000049898788020000c3` (22) | same | same | Y | `2c0923f7af81d76c` | `2c0923f7af81d76c` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x50 imm=0x108 — **PASS**

- fixture: `_scratch_subimm_h50_108.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e80801000049898780020000c3`
- js-sha256: `f139f28243c08957d01875c549d2ab72beb6b6e844db45f306aedd137c994dd5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x51 imm=0x108 — **PASS**

- fixture: `_scratch_subimm_h51_108.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e80801000049898788020000c3`
- js-sha256: `f9c122832287170d748906fa59e9ce9e3085a10658b2ec201bd1e834774e3607`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x52 imm=0x108 — **PASS**

- fixture: `_scratch_subimm_h52_108.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e80801000049898790020000c3`
- js-sha256: `2f027342f5447eeb3b13996588d05f598230d8f22bf13d6c799cb964980195da`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x50 ss=0x60 oo=0x110 — **PASS**

- fixture: `_scratch_ldb_5060_110.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c010010000480fb60049898780020000c3`
- js-sha256: `215fc443528e616380c90b46f110f7b01f8c297ab5df4f7e43ba6f6517bf6451`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x51 ss=0x60 oo=0x110 — **PASS**

- fixture: `_scratch_ldb_5160_110.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c010010000480fb60049898788020000c3`
- js-sha256: `bfd294f2e3edf3d26b22892867c61dc4b0f3c145731d8adb4d7dbd2bebd50154`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x52 ss=0x60 oo=0x110 — **PASS**

- fixture: `_scratch_ldb_5260_110.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c010010000480fb60049898790020000c3`
- js-sha256: `d9d4fceaca2783f16f51d918c011da1cf2b3ef895013a53e0d84b507e328cdbe`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x50 imm=0x110 — **PASS**

- fixture: `_scratch_addimm_h50_110.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c01001000049898780020000c3`
- js-sha256: `b2f08439005e085cf5d7f2c196caa87d536da8ae5a92b10d081c36dfb7056598`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x51 imm=0x110 — **PASS**

- fixture: `_scratch_addimm_h51_110.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c01001000049898788020000c3`
- js-sha256: `2c0923f7af81d76cc393610f14c1931f17b91d275e9301c244237ca83c316164`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=50/51/52 imm=108 (finish 108 SUB triad after H_395..H_397 ADD; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=110 (next oo after 108 triad; imm32 26B).
- ADD-IMM slot=50/51 imm=110 (fresh imm after 108; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 194`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h50_108.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_108.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_108.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_110.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_110.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_110.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_110.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_110.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-53-log.md` — this file
- `scripts/_probe/parallel-batch-53-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-059 serialize PASSes + 1 Relock**

Pass pin from body-extend-058 Relock: `c258ff3271396e1822dba5baf34c98aae7003f19c10a916a0aa3967142f5c2dc`.
Handlers before consolidate = 404 (H_00..H_397). Next selectors `40 194`.. for H_398.. if all serialize.

PASS list for body-extend-059:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_398 | 0x194 | 0x61 SUB-IMM | 0x50 0x108 | `498b87800200004881e80801000049898780020000c3` (22B) | `f139f28243c08957` |
| H_399 | 0x195 | 0x61 SUB-IMM | 0x51 0x108 | `498b87880200004881e80801000049898788020000c3` (22B) | `f9c122832287170d` |
| H_400 | 0x196 | 0x61 SUB-IMM | 0x52 0x108 | `498b87900200004881e80801000049898790020000c3` (22B) | `2f027342f5447eeb` |
| H_401 | 0x197 | 0x80 LDB | 0x50 0x60 0x110 | `498b87000300004881c010010000480fb60049898780020000c3` (26B) | `215fc443528e6163` |
| H_402 | 0x198 | 0x80 LDB | 0x51 0x60 0x110 | `498b87000300004881c010010000480fb60049898788020000c3` (26B) | `bfd294f2e3edf3d2` |
| H_403 | 0x199 | 0x80 LDB | 0x52 0x60 0x110 | `498b87000300004881c010010000480fb60049898790020000c3` (26B) | `d9d4fceaca2783f1` |
| H_404 | 0x19A | 0x62 ADD-IMM | 0x50 0x110 | `498b87800200004881c01001000049898780020000c3` (22B) | `b2f08439005e085c` |
| H_405 | 0x19B | 0x62 ADD-IMM | 0x51 0x110 | `498b87880200004881c01001000049898788020000c3` (22B) | `2c0923f7af81d76c` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-058 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_397.
- If the parent decides to serialize, append H_398.. at selectors `40 194`..:
  - H_398 0x61 SUB-IMM (61 50 108) — pin `498b87800200004881e80801000049898780020000c3`
  - H_399 0x61 SUB-IMM (61 51 108) — pin `498b87880200004881e80801000049898788020000c3`
  - H_400 0x61 SUB-IMM (61 52 108) — pin `498b87900200004881e80801000049898790020000c3`
  - H_401 0x80 LDB (80 50 60 110) — pin `498b87000300004881c010010000480fb60049898780020000c3`
  - H_402 0x80 LDB (80 51 60 110) — pin `498b87000300004881c010010000480fb60049898788020000c3`
  - H_403 0x80 LDB (80 52 60 110) — pin `498b87000300004881c010010000480fb60049898790020000c3`
  - H_404 0x62 ADD-IMM (62 50 110) — pin `498b87800200004881c01001000049898780020000c3`
  - H_405 0x62 ADD-IMM (62 51 110) — pin `498b87880200004881c01001000049898788020000c3`
- Plus 1 Relock after append from pin `c258ff32…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-059 serialize PASSes + 1 Relock

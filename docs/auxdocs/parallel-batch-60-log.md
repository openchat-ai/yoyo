# parallel-batch-60 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-60-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-065 (pin `b84d7f1b…`, handlers = 459, H_445..H_452 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-065 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_452 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x51 imm=0x138 | `498b87880200004881e83801000049898788020000c3` (22) | same | same | Y | `23ad7ac033aa9ec5` | `23ad7ac033aa9ec5` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x52 imm=0x138 | `498b87900200004881e83801000049898790020000c3` (22) | same | same | Y | `5e85ef9f64f70096` | `5e85ef9f64f70096` | PASS |
| 3 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x140 | `498b87000300004881c040010000480fb60049898780020000c3` (26) | same | same | Y | `7b8558d3978f497d` | `7b8558d3978f497d` | PASS |
| 4 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x140 | `498b87000300004881c040010000480fb60049898788020000c3` (26) | same | same | Y | `cf076e94edbe5a82` | `cf076e94edbe5a82` | PASS |
| 5 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x140 | `498b87000300004881c040010000480fb60049898790020000c3` (26) | same | same | Y | `4468abc2e0b7e44b` | `4468abc2e0b7e44b` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x50 imm=0x140 | `498b87800200004881c04001000049898780020000c3` (22) | same | same | Y | `f60b265b7a3dc3f9` | `f60b265b7a3dc3f9` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x51 imm=0x140 | `498b87880200004881c04001000049898788020000c3` (22) | same | same | Y | `fe39737bd6fc8a3f` | `fe39737bd6fc8a3f` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x52 imm=0x140 | `498b87900200004881c04001000049898790020000c3` (22) | same | same | Y | `dfda4be88622d37d` | `dfda4be88622d37d` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x51 imm=0x138 — **PASS**

- fixture: `_scratch_subimm_h51_138.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e83801000049898788020000c3`
- js-sha256: `23ad7ac033aa9ec586f70f25b2715c58f15cbb38616f9083366a6130538c1180`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x52 imm=0x138 — **PASS**

- fixture: `_scratch_subimm_h52_138.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e83801000049898790020000c3`
- js-sha256: `5e85ef9f64f7009681de2d56b231c4d942d0e12e8c74148d68d6120478fd4c0c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x50 ss=0x60 oo=0x140 — **PASS**

- fixture: `_scratch_ldb_5060_140.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c040010000480fb60049898780020000c3`
- js-sha256: `7b8558d3978f497d9cb3923014e788f6dba285aaa36193f7e792f41060ebeb4d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x51 ss=0x60 oo=0x140 — **PASS**

- fixture: `_scratch_ldb_5160_140.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c040010000480fb60049898788020000c3`
- js-sha256: `cf076e94edbe5a823d5a090afc8f503a205803bd9a25b527673bb31a3d512bd9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x52 ss=0x60 oo=0x140 — **PASS**

- fixture: `_scratch_ldb_5260_140.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c040010000480fb60049898790020000c3`
- js-sha256: `4468abc2e0b7e44b42eb9d8dd9905d43c2d61425eb73ee496fc5b26da7370597`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x50 imm=0x140 — **PASS**

- fixture: `_scratch_addimm_h50_140.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c04001000049898780020000c3`
- js-sha256: `f60b265b7a3dc3f99919c45e013d87e9907eef3b224cfd06f7c7039bc09ae922`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x51 imm=0x140 — **PASS**

- fixture: `_scratch_addimm_h51_140.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c04001000049898788020000c3`
- js-sha256: `fe39737bd6fc8a3fdf55010dd7f7238946ff5917d3193e5b4084b4e9d72c147b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x52 imm=0x140 — **PASS**

- fixture: `_scratch_addimm_h52_140.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c04001000049898790020000c3`
- js-sha256: `dfda4be88622d37dece8c6497433408ae45eb9577e73a04c57d085b90fce066d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=51/52 imm=138 (finish 138 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=140 (start 140 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=140 (start 140 ADD triad; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 1CB`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h51_138.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_138.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_140.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_140.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_140.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_140.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_140.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_140.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-60-log.md` — this file
- `scripts/_probe/parallel-batch-60-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-066 serialize PASSes + 1 Relock**

Pass pin from body-extend-065 Relock: `b84d7f1b4bb1d8eefeca1832f12c3f7380658897813b1a321f2b75b27187258e`.
Handlers before consolidate = 459 (H_00..H_452). Next selectors `40 1CB`.. for H_453.. if all serialize.

PASS list for body-extend-066:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_453 | 0x1CB | 0x61 SUB-IMM | 0x51 0x138 | `498b87880200004881e83801000049898788020000c3` (22B) | `23ad7ac033aa9ec5` |
| H_454 | 0x1CC | 0x61 SUB-IMM | 0x52 0x138 | `498b87900200004881e83801000049898790020000c3` (22B) | `5e85ef9f64f70096` |
| H_455 | 0x1CD | 0x80 LDB | 0x50 0x60 0x140 | `498b87000300004881c040010000480fb60049898780020000c3` (26B) | `7b8558d3978f497d` |
| H_456 | 0x1CE | 0x80 LDB | 0x51 0x60 0x140 | `498b87000300004881c040010000480fb60049898788020000c3` (26B) | `cf076e94edbe5a82` |
| H_457 | 0x1CF | 0x80 LDB | 0x52 0x60 0x140 | `498b87000300004881c040010000480fb60049898790020000c3` (26B) | `4468abc2e0b7e44b` |
| H_458 | 0x1D0 | 0x62 ADD-IMM | 0x50 0x140 | `498b87800200004881c04001000049898780020000c3` (22B) | `f60b265b7a3dc3f9` |
| H_459 | 0x1D1 | 0x62 ADD-IMM | 0x51 0x140 | `498b87880200004881c04001000049898788020000c3` (22B) | `fe39737bd6fc8a3f` |
| H_460 | 0x1D2 | 0x62 ADD-IMM | 0x52 0x140 | `498b87900200004881c04001000049898790020000c3` (22B) | `dfda4be88622d37d` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-065 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_452.
- If the parent decides to serialize, append H_453.. at selectors `40 1CB`..:
  - H_453 0x61 SUB-IMM (61 51 138) — pin `498b87880200004881e83801000049898788020000c3`
  - H_454 0x61 SUB-IMM (61 52 138) — pin `498b87900200004881e83801000049898790020000c3`
  - H_455 0x80 LDB (80 50 60 140) — pin `498b87000300004881c040010000480fb60049898780020000c3`
  - H_456 0x80 LDB (80 51 60 140) — pin `498b87000300004881c040010000480fb60049898788020000c3`
  - H_457 0x80 LDB (80 52 60 140) — pin `498b87000300004881c040010000480fb60049898790020000c3`
  - H_458 0x62 ADD-IMM (62 50 140) — pin `498b87800200004881c04001000049898780020000c3`
  - H_459 0x62 ADD-IMM (62 51 140) — pin `498b87880200004881c04001000049898788020000c3`
  - H_460 0x62 ADD-IMM (62 52 140) — pin `498b87900200004881c04001000049898790020000c3`
- Plus 1 Relock after append from pin `b84d7f1b…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-066 serialize PASSes + 1 Relock

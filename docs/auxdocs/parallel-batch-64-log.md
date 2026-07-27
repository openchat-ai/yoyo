# parallel-batch-64 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-64-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-069 (pin `f9afff3e…`, handlers = 491, H_477..H_484 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-069 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_484 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x50 imm=0x158 | `498b87800200004881c05801000049898780020000c3` (22) | same | same | Y | `41094166f79d1c0b` | `41094166f79d1c0b` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x51 imm=0x158 | `498b87880200004881c05801000049898788020000c3` (22) | same | same | Y | `70fd4ef8381b04b2` | `70fd4ef8381b04b2` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x52 imm=0x158 | `498b87900200004881c05801000049898790020000c3` (22) | same | same | Y | `25deea9b5b4ae288` | `25deea9b5b4ae288` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x50 imm=0x158 | `498b87800200004881e85801000049898780020000c3` (22) | same | same | Y | `401d7f68292fe70a` | `401d7f68292fe70a` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x51 imm=0x158 | `498b87880200004881e85801000049898788020000c3` (22) | same | same | Y | `cb9589469f12483a` | `cb9589469f12483a` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x52 imm=0x158 | `498b87900200004881e85801000049898790020000c3` (22) | same | same | Y | `8b00fcdbb741f29c` | `8b00fcdbb741f29c` | PASS |
| 7 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x160 | `498b87000300004881c060010000480fb60049898780020000c3` (26) | same | same | Y | `0de356c3d4e6b935` | `0de356c3d4e6b935` | PASS |
| 8 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x160 | `498b87000300004881c060010000480fb60049898788020000c3` (26) | same | same | Y | `ca261b259166d021` | `ca261b259166d021` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x50 imm=0x158 — **PASS**

- fixture: `_scratch_addimm_h50_158.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c05801000049898780020000c3`
- js-sha256: `41094166f79d1c0b5848813a8003cf059904ad414e849c3664fa66da318d783e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x51 imm=0x158 — **PASS**

- fixture: `_scratch_addimm_h51_158.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c05801000049898788020000c3`
- js-sha256: `70fd4ef8381b04b209647c78f71758546b4a983306f15ffb8a450ad49b27b69a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x52 imm=0x158 — **PASS**

- fixture: `_scratch_addimm_h52_158.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c05801000049898790020000c3`
- js-sha256: `25deea9b5b4ae288372fe2d5c07eac0af632775ae25fc314b494c04fcfbaba38`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x50 imm=0x158 — **PASS**

- fixture: `_scratch_subimm_h50_158.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e85801000049898780020000c3`
- js-sha256: `401d7f68292fe70a4b207cbea1f37fee395f56a6a13e61f80b7b32c6aee335a2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x51 imm=0x158 — **PASS**

- fixture: `_scratch_subimm_h51_158.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e85801000049898788020000c3`
- js-sha256: `cb9589469f12483a3e83ace6902793c80f1567f10505e238d9e4d03f358b7668`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x52 imm=0x158 — **PASS**

- fixture: `_scratch_subimm_h52_158.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e85801000049898790020000c3`
- js-sha256: `8b00fcdbb741f29c732e134cf5d642097ffe98735e35a3508cf5fb4f061c992e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x50 ss=0x60 oo=0x160 — **PASS**

- fixture: `_scratch_ldb_5060_160.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c060010000480fb60049898780020000c3`
- js-sha256: `0de356c3d4e6b9355d7173a2506214efb8e07910d2f0574cf4601ed1e1385ed9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x51 ss=0x60 oo=0x160 — **PASS**

- fixture: `_scratch_ldb_5160_160.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c060010000480fb60049898788020000c3`
- js-sha256: `ca261b259166d0210ae6626da2f3dcb22b089cec2d01fd4c72cac9db7233b9ef`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=50/51/52 imm=158 (start 158 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=158 (start 158 SUB triad; imm32 22B).
- LDB dd=50/51 ss=60 oo=160 (start 160 LDB triad; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 1EB`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h50_158.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_158.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_158.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_158.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_158.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_158.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_160.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_160.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-64-log.md` — this file
- `scripts/_probe/parallel-batch-64-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-070 serialize PASSes + 1 Relock**

Pass pin from body-extend-069 Relock: `f9afff3e953337091fdaa161a919f6d92488d72c1f70687907395922a811ec42`.
Handlers before consolidate = 491 (H_00..H_484). Next selectors `40 1EB`.. for H_485.. if all serialize.

PASS list for body-extend-070:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_485 | 0x1EB | 0x62 ADD-IMM | 0x50 0x158 | `498b87800200004881c05801000049898780020000c3` (22B) | `41094166f79d1c0b` |
| H_486 | 0x1EC | 0x62 ADD-IMM | 0x51 0x158 | `498b87880200004881c05801000049898788020000c3` (22B) | `70fd4ef8381b04b2` |
| H_487 | 0x1ED | 0x62 ADD-IMM | 0x52 0x158 | `498b87900200004881c05801000049898790020000c3` (22B) | `25deea9b5b4ae288` |
| H_488 | 0x1EE | 0x61 SUB-IMM | 0x50 0x158 | `498b87800200004881e85801000049898780020000c3` (22B) | `401d7f68292fe70a` |
| H_489 | 0x1EF | 0x61 SUB-IMM | 0x51 0x158 | `498b87880200004881e85801000049898788020000c3` (22B) | `cb9589469f12483a` |
| H_490 | 0x1F0 | 0x61 SUB-IMM | 0x52 0x158 | `498b87900200004881e85801000049898790020000c3` (22B) | `8b00fcdbb741f29c` |
| H_491 | 0x1F1 | 0x80 LDB | 0x50 0x60 0x160 | `498b87000300004881c060010000480fb60049898780020000c3` (26B) | `0de356c3d4e6b935` |
| H_492 | 0x1F2 | 0x80 LDB | 0x51 0x60 0x160 | `498b87000300004881c060010000480fb60049898788020000c3` (26B) | `ca261b259166d021` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-069 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_484.
- If the parent decides to serialize, append H_485.. at selectors `40 1EB`..:
  - H_485 0x62 ADD-IMM (62 50 158) — pin `498b87800200004881c05801000049898780020000c3`
  - H_486 0x62 ADD-IMM (62 51 158) — pin `498b87880200004881c05801000049898788020000c3`
  - H_487 0x62 ADD-IMM (62 52 158) — pin `498b87900200004881c05801000049898790020000c3`
  - H_488 0x61 SUB-IMM (61 50 158) — pin `498b87800200004881e85801000049898780020000c3`
  - H_489 0x61 SUB-IMM (61 51 158) — pin `498b87880200004881e85801000049898788020000c3`
  - H_490 0x61 SUB-IMM (61 52 158) — pin `498b87900200004881e85801000049898790020000c3`
  - H_491 0x80 LDB (80 50 60 160) — pin `498b87000300004881c060010000480fb60049898780020000c3`
  - H_492 0x80 LDB (80 51 60 160) — pin `498b87000300004881c060010000480fb60049898788020000c3`
- Plus 1 Relock after append from pin `f9afff3e…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-070 serialize PASSes + 1 Relock

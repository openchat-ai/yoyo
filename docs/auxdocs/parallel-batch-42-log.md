# parallel-batch-42 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-42-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-047 (pin `000042c8…`, handlers = 316, H_302..H_309 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-047 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_309 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x52 imm=0xB0 | `498b87900200004881e8b000000049898790020000c3` (22) | same | same | Y | `1eabf19e87df5652` | `1eabf19e87df5652` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x50 imm=0xB8 | `498b87800200004881c0b800000049898780020000c3` (22) | same | same | Y | `9f7f7147fbb9f533` | `9f7f7147fbb9f533` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x51 imm=0xB8 | `498b87880200004881c0b800000049898788020000c3` (22) | same | same | Y | `3817887afb58b853` | `3817887afb58b853` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x52 imm=0xB8 | `498b87900200004881c0b800000049898790020000c3` (22) | same | same | Y | `65f24a01717f98f9` | `65f24a01717f98f9` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x50 imm=0xB8 | `498b87800200004881e8b800000049898780020000c3` (22) | same | same | Y | `a086a4139a5285c0` | `a086a4139a5285c0` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x51 imm=0xB8 | `498b87880200004881e8b800000049898788020000c3` (22) | same | same | Y | `d8eeef300a793b35` | `d8eeef300a793b35` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x52 imm=0xB8 | `498b87900200004881e8b800000049898790020000c3` (22) | same | same | Y | `3aecc01b59d73b5a` | `3aecc01b59d73b5a` | PASS |
| 8 | 0x80 LDB | dd=0x50 ss=0x60 oo=0xD0 | `498b87000300004881c0d0000000480fb60049898780020000c3` (26) | same | same | Y | `e88fcc130f63d22f` | `e88fcc130f63d22f` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x52 imm=0xB0 — **PASS**

- fixture: `_scratch_subimm_h52_b0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8b000000049898790020000c3`
- js-sha256: `1eabf19e87df565236fea87b7386ebdf32057acb914bf71a1d91dbcad74bc800`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x50 imm=0xB8 — **PASS**

- fixture: `_scratch_addimm_h50_b8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0b800000049898780020000c3`
- js-sha256: `9f7f7147fbb9f5333359d1ab52c82b5025716c11226323db3319ca238a908555`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x51 imm=0xB8 — **PASS**

- fixture: `_scratch_addimm_h51_b8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0b800000049898788020000c3`
- js-sha256: `3817887afb58b853de9a43c9b984b204ca5ed7b218ca7cbf84a07c931f2e4c28`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x52 imm=0xB8 — **PASS**

- fixture: `_scratch_addimm_h52_b8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0b800000049898790020000c3`
- js-sha256: `65f24a01717f98f95abef0f986cdff72cc6c7b38d8a33f9f77189bfe6015863b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x50 imm=0xB8 — **PASS**

- fixture: `_scratch_subimm_h50_b8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8b800000049898780020000c3`
- js-sha256: `a086a4139a5285c04e909715cc2de8b9e4dd5557810f8837b874506f66ed8c08`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x51 imm=0xB8 — **PASS**

- fixture: `_scratch_subimm_h51_b8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8b800000049898788020000c3`
- js-sha256: `d8eeef300a793b3563d7438618b19db38b99437e49d9efbbc9ac610ccb9b8e97`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x52 imm=0xB8 — **PASS**

- fixture: `_scratch_subimm_h52_b8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8b800000049898790020000c3`
- js-sha256: `3aecc01b59d73b5a6ab5b6143fd0773a97de0f4c10426684fb5786a6af8eeb55`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x50 ss=0x60 oo=0xD0 — **PASS**

- fixture: `_scratch_ldb_5060_d0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0d0000000480fb60049898780020000c3`
- js-sha256: `e88fcc130f63d22f79242d93e14170cd74259ced3b0711a0b2d6043d01943e49`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=52 imm=B0 (finish B0 triad after H_308/H_309; imm32 22B).
- ADD-IMM slot=50/51/52 imm=B8 (fresh imm after B0; imm32 22B).
- SUB-IMM slot=50/51/52 imm=B8 (complements ADD-IMM * B8; imm32 22B).
- LDB dd=50 ss=60 oo=D0 (next oo after C8 triad; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 13C`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h52_b0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_b8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_b8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_b8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_b8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_b8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_b8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_d0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-42-log.md` — this file
- `scripts/_probe/parallel-batch-42-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-048 serialize PASSes + 1 Relock**

Pass pin from body-extend-047 Relock: `000042c8ea316c07fce78e5bb05814229058adea09ac196d0d1e8a90987336f2`.
Handlers before consolidate = 316 (H_00..H_309). Next selectors `40 13C`.. for H_310.. if all serialize.

PASS list for body-extend-048:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_310 | 0x13C | 0x61 SUB-IMM | 0x52 0xB0 | `498b87900200004881e8b000000049898790020000c3` (22B) | `1eabf19e87df5652` |
| H_311 | 0x13D | 0x62 ADD-IMM | 0x50 0xB8 | `498b87800200004881c0b800000049898780020000c3` (22B) | `9f7f7147fbb9f533` |
| H_312 | 0x13E | 0x62 ADD-IMM | 0x51 0xB8 | `498b87880200004881c0b800000049898788020000c3` (22B) | `3817887afb58b853` |
| H_313 | 0x13F | 0x62 ADD-IMM | 0x52 0xB8 | `498b87900200004881c0b800000049898790020000c3` (22B) | `65f24a01717f98f9` |
| H_314 | 0x140 | 0x61 SUB-IMM | 0x50 0xB8 | `498b87800200004881e8b800000049898780020000c3` (22B) | `a086a4139a5285c0` |
| H_315 | 0x141 | 0x61 SUB-IMM | 0x51 0xB8 | `498b87880200004881e8b800000049898788020000c3` (22B) | `d8eeef300a793b35` |
| H_316 | 0x142 | 0x61 SUB-IMM | 0x52 0xB8 | `498b87900200004881e8b800000049898790020000c3` (22B) | `3aecc01b59d73b5a` |
| H_317 | 0x143 | 0x80 LDB | 0x50 0x60 0xD0 | `498b87000300004881c0d0000000480fb60049898780020000c3` (26B) | `e88fcc130f63d22f` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-047 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_309.
- If the parent decides to serialize, append H_310.. at selectors `40 13C`..:
  - H_310 0x61 SUB-IMM (61 52 B0) — pin `498b87900200004881e8b000000049898790020000c3`
  - H_311 0x62 ADD-IMM (62 50 B8) — pin `498b87800200004881c0b800000049898780020000c3`
  - H_312 0x62 ADD-IMM (62 51 B8) — pin `498b87880200004881c0b800000049898788020000c3`
  - H_313 0x62 ADD-IMM (62 52 B8) — pin `498b87900200004881c0b800000049898790020000c3`
  - H_314 0x61 SUB-IMM (61 50 B8) — pin `498b87800200004881e8b800000049898780020000c3`
  - H_315 0x61 SUB-IMM (61 51 B8) — pin `498b87880200004881e8b800000049898788020000c3`
  - H_316 0x61 SUB-IMM (61 52 B8) — pin `498b87900200004881e8b800000049898790020000c3`
  - H_317 0x80 LDB (80 50 60 D0) — pin `498b87000300004881c0d0000000480fb60049898780020000c3`
- Plus 1 Relock after append from pin `000042c8…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-048 serialize PASSes + 1 Relock

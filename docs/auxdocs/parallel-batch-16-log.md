# parallel-batch-16 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-16-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-021 (pin `07eee98c…`, handlers = 108, H_94..H_101 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_101 and
> not already present as handlers in current `yoyo.ty` (skipped early
> INC/DEC 50, GET/ORV/SUBV 50 51). Slot/imm/dst variations of
> CMP/LDB/SET/SUB-IMM/IMUL/ADD-IMM. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x65 CMP | (0x51, 0x52) | `498b8788020000498b8f900200004839c8c3` (18) | same | same | Y | `2cf366028a7416c3` | `2cf366028a7416c3` | PASS |
| 2 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x18 | `498b87000300004883c018480fb60049898788020000c3` (23) | same | same | Y | `0b1b7a7c7810f66b` | `0b1b7a7c7810f66b` | PASS |
| 3 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x18 | `498b87000300004883c018480fb60049898790020000c3` (23) | same | same | Y | `8137e5bda9f228f5` | `8137e5bda9f228f5` | PASS |
| 4 | 0x30 SET | slot=0x51 imm=0xC0FFEE00 | `48b800eeffc00000000049898788020000c3` (18) | same | same | Y | `6da3781de89ad437` | `6da3781de89ad437` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x52 imm=0x08 | `498b87900200004883e80849898790020000c3` (19) | same | same | Y | `6cd180e2545680bd` | `6cd180e2545680bd` | PASS |
| 6 | 0x63 IMUL | (0x51, 0x52) | `498b8788020000498b8f90020000480fafc149898788020000c3` (26) | same | same | Y | `3b7aa6ccd7e47092` | `3b7aa6ccd7e47092` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x50 imm=0x14 | `498b87800200004883c01449898780020000c3` (19) | same | same | Y | `8007f38af1d95403` | `8007f38af1d95403` | PASS |
| 8 | 0x30 SET | slot=0x50 imm=0xC0FFEE00 | `48b800eeffc00000000049898780020000c3` (18) | same | same | Y | `9f214984263cafa8` | `9f214984263cafa8` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x65 CMP (0x51, 0x52) — **PASS**

- fixture: `_scratch_cmp_5152.ty` + `.code.hex`
- expected pin (18B): `498b8788020000498b8f900200004839c8c3`
- js-sha256: `2cf366028a7416c3b45fff656f8f268f15a0042dbc7b34068f090780717badbb`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x51 ss=0x60 oo=0x18 — **PASS**

- fixture: `_scratch_ldb_5160_18.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c018480fb60049898788020000c3`
- js-sha256: `0b1b7a7c7810f66b4aedd4ce6f23bccc18783bef8477393ff3215b9201a311b1`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x52 ss=0x60 oo=0x18 — **PASS**

- fixture: `_scratch_ldb_5260_18.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c018480fb60049898790020000c3`
- js-sha256: `8137e5bda9f228f5a3c816362d0ce7500b280a56938a63e693163ae5af5a4ecd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x30 SET slot=0x51 imm=0xC0FFEE00 — **PASS**

- fixture: `_scratch_set_c0ffee00.ty` + `.code.hex`
- expected pin (18B): `48b800eeffc00000000049898788020000c3`
- js-sha256: `6da3781de89ad437035a6d41ae13c3bc9910d9c1986d680e659cc40a1ae54bde`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x52 imm=0x08 — **PASS**

- fixture: `_scratch_subimm_h52_08.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e80849898790020000c3`
- js-sha256: `6cd180e2545680bd1df2559b3da6103fc02790396b394febc11fa2a8c9077697`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x63 IMUL (0x51, 0x52) — **PASS**

- fixture: `_scratch_imul_5152.ty` + `.code.hex`
- expected pin (26B): `498b8788020000498b8f90020000480fafc149898788020000c3`
- js-sha256: `3b7aa6ccd7e470921429559d896f949942f78cd05400cbec002c0d91e1ff1301`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x50 imm=0x14 — **PASS**

- fixture: `_scratch_addimm_h50_14.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c01449898780020000c3`
- js-sha256: `8007f38af1d95403e15bbb55676b8c705ad1d9a851fa6bb04b26cda3bd9a3d37`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x30 SET slot=0x50 imm=0xC0FFEE00 — **PASS**

- fixture: `_scratch_set_50_c0ffee00.ty` + `.code.hex`
- expected pin (18B): `48b800eeffc00000000049898780020000c3`
- js-sha256: `9f214984263cafa8dfbce48ca2fe7be953c5bcd96d8957589e5e83ee0fc748cd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- CMP at 51 52 (H_58=51 50; H_65=52 51; H_77=52 50; H_89=50 52).
- LDB dd=51 ss=60 oo=18 (H_61=51 60 08; H_90=51 60 10; H_99=50 60 18).
- LDB dd=52 ss=60 oo=18 (H_69=52 60 08; H_98=52 60 10).
- SET at slot 51 imm=C0FFEE00 (H_60 DEADBEEF; H_87 AABBCCDD).
- SUB-IMM at slot 52 imm=08 (H_79=52 03; H_96=50 08; H_70=51 03).
- IMUL at 51 52 (H_56=51 50; H_57=52 51; H_85=50 52; H_91=52 50).
- ADD-IMM at slot 50 imm=14 (H_93=50 0F; H_64=51 07; H_78=52 07; H_80=51 0A; H_97=52 0A).
- SET at slot 50 imm=C0FFEE00 (H_68/H_76/H_94 other imm at 50; replaces early-dup ORV/SUBV 50 51).
- Skipped suggested INC/DEC 50 (H_17/H_18), GET/ORV/SUBV 50 51 (early handlers).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_cmp_5152.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_18.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_18.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_c0ffee00.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_08.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_imul_5152.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_14.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_50_c0ffee00.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-16-log.md` — this file
- `scripts/_probe/parallel-batch-16-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-022 serialize PASSes + 1 Relock**

Pass pin from body-extend-021 Relock: `07eee98cb95446f2c277bcd78b211e43e7b274f583ac02392691dfc1b204cd0a`.
Handlers before consolidate = 108 (H_00..H_101). Next selectors 0x6C.. for H_102.. if all serialize.

PASS list for body-extend-022:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_102 | 0x6C | 0x65 CMP | 0x51 0x52 | `498b8788020000498b8f900200004839c8c3` (18B) | `2cf366028a7416c3` |
| H_103 | 0x6D | 0x80 LDB | 0x51 0x60 0x18 | `498b87000300004883c018480fb60049898788020000c3` (23B) | `0b1b7a7c7810f66b` |
| H_104 | 0x6E | 0x80 LDB | 0x52 0x60 0x18 | `498b87000300004883c018480fb60049898790020000c3` (23B) | `8137e5bda9f228f5` |
| H_105 | 0x6F | 0x30 SET | 0x51 0xC0FFEE00 | `48b800eeffc00000000049898788020000c3` (18B) | `6da3781de89ad437` |
| H_106 | 0x70 | 0x61 SUB-IMM | 0x52 0x08 | `498b87900200004883e80849898790020000c3` (19B) | `6cd180e2545680bd` |
| H_107 | 0x71 | 0x63 IMUL | 0x51 0x52 | `498b8788020000498b8f90020000480fafc149898788020000c3` (26B) | `3b7aa6ccd7e47092` |
| H_108 | 0x72 | 0x62 ADD-IMM | 0x50 0x14 | `498b87800200004883c01449898780020000c3` (19B) | `8007f38af1d95403` |
| H_109 | 0x73 | 0x30 SET | 0x50 0xC0FFEE00 | `48b800eeffc00000000049898780020000c3` (18B) | `9f214984263cafa8` |

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
  fresh slot/imm/dst combinations not in H_48..H_101.
- If the parent decides to serialize, append H_102.. at selectors 0x6C..:
  - H_102 0x65 CMP (65 51 52) — pin `498b8788020000498b8f900200004839c8c3`
  - H_103 0x80 LDB (80 51 60 18) — pin `498b87000300004883c018480fb60049898788020000c3`
  - H_104 0x80 LDB (80 52 60 18) — pin `498b87000300004883c018480fb60049898790020000c3`
  - H_105 0x30 SET (30 51 C0FFEE00) — pin `48b800eeffc00000000049898788020000c3`
  - H_106 0x61 SUB-IMM (61 52 08) — pin `498b87900200004883e80849898790020000c3`
  - H_107 0x63 IMUL (63 51 52) — pin `498b8788020000498b8f90020000480fafc149898788020000c3`
  - H_108 0x62 ADD-IMM (62 50 14) — pin `498b87800200004883c01449898780020000c3`
  - H_109 0x30 SET (30 50 C0FFEE00) — pin `48b800eeffc00000000049898780020000c3`
- Plus 1 Relock after append from pin `07eee98c…`.

## §7. Consolidation handoff

parent next = body-extend-022 serialize PASSes + 1 Relock

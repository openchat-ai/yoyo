# parallel-batch-15 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-15-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-020 (pin `c922e4d4…`, handlers = 100, H_86..H_93 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_93 and
> not already present as handlers in current `yoyo.ty`. Slot/imm/dst
> variations of SET/SUB-IMM/ADD-IMM/LDB/SUBV/ADDV. Skipped D-1
> 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x30 SET | slot=0x50 imm=0xBEEFCAFE | `48b8fecaefbe0000000049898780020000c3` (18) | same | same | Y | `b72d25116f116e99` | `b72d25116f116e99` | PASS |
| 2 | 0x30 SET | slot=0x52 imm=0x11111111 | `48b8111111110000000049898790020000c3` (18) | same | same | Y | `0d3e14e67a06fc73` | `0d3e14e67a06fc73` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x50 imm=0x08 | `498b87800200004883e80849898780020000c3` (19) | same | same | Y | `f6f0be6715ebc155` | `f6f0be6715ebc155` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x52 imm=0x0A | `498b87900200004883c00a49898790020000c3` (19) | same | same | Y | `125226ff4633167f` | `125226ff4633167f` | PASS |
| 5 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x10 | `498b87000300004883c010480fb60049898790020000c3` (23) | same | same | Y | `fed00067e5604398` | `fed00067e5604398` | PASS |
| 6 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x18 | `498b87000300004883c018480fb60049898780020000c3` (23) | same | same | Y | `56296ca0160c87f5` | `56296ca0160c87f5` | PASS |
| 7 | 0x6A SUBV | (0x51, 0x52) | `498b8788020000498b8f900200004829c849898788020000c3` (25) | same | same | Y | `47760053769fc7f2` | `47760053769fc7f2` | PASS |
| 8 | 0x68 ADDV | (0x52, 0x50) | `498b8790020000498b8f800200004801c849898790020000c3` (25) | same | same | Y | `5e5f7578c2ee8989` | `5e5f7578c2ee8989` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x30 SET slot=0x50 imm=0xBEEFCAFE — **PASS**

- fixture: `_scratch_set_beefcafe.ty` + `.code.hex`
- expected pin (18B): `48b8fecaefbe0000000049898780020000c3`
- js-sha256: `b72d25116f116e993f9ffc5dd9bebffcec956c47cc7447e27931754e37027a31`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x30 SET slot=0x52 imm=0x11111111 — **PASS**

- fixture: `_scratch_set_11111111.ty` + `.code.hex`
- expected pin (18B): `48b8111111110000000049898790020000c3`
- js-sha256: `0d3e14e67a06fc73469bc396e4af546c173dcf528043e42b93c7f7d59d69e518`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x50 imm=0x08 — **PASS**

- fixture: `_scratch_subimm_h50_08.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e80849898780020000c3`
- js-sha256: `f6f0be6715ebc155ca2f2b1183eaebf7064c69bb0d15a75a1e1abc2efda095ac`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x52 imm=0x0A — **PASS**

- fixture: `_scratch_addimm_h52_0a.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c00a49898790020000c3`
- js-sha256: `125226ff4633167f3d94147ecdd0ce4e4263f081b9295e7e9e0ae21ee021cf92`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x52 ss=0x60 oo=0x10 — **PASS**

- fixture: `_scratch_ldb_5260_10.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c010480fb60049898790020000c3`
- js-sha256: `fed00067e560439858a8933d9f80c58e04d591ff5d66d29320907fbccfa78581`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x50 ss=0x60 oo=0x18 — **PASS**

- fixture: `_scratch_ldb_5060_18.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c018480fb60049898780020000c3`
- js-sha256: `56296ca0160c87f5542065c808bc007ebceef6ab2eaa6f0136588e630eecac5f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x6A SUBV (0x51, 0x52) — **PASS**

- fixture: `_scratch_subv_5152.ty` + `.code.hex`
- expected pin (25B): `498b8788020000498b8f900200004829c849898788020000c3`
- js-sha256: `47760053769fc7f2ee4d69ffd5d6e027dcce195d6d4dda1b7ec5a17549bac233`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x68 ADDV (0x52, 0x50) — **PASS**

- fixture: `_scratch_addv_5250.ty` + `.code.hex`
- expected pin (25B): `498b8790020000498b8f800200004801c849898790020000c3`
- js-sha256: `5e5f7578c2ee89891c546d91f5297185696b7f91fbd3d2568b3ab66f26e593cf`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SET at slot 0x50 imm=BEEFCAFE (H_68/H_76 use other imm at 50; H_53/H_86 are slot 52; H_60/H_87 are slot 51).
- SET at slot 0x52 imm=11111111 (H_53 CAFEBABE; H_86 FEEDFACE).
- SUB-IMM at slot 50 imm=08 (H_81=50 05; H_70=51 03; H_79=52 03).
- ADD-IMM at slot 52 imm=0A (H_78=52 07; H_80=51 0A; H_64=51 07; H_93=50 0F).
- LDB dd=52 ss=60 oo=10 (H_69=52 60 08; H_61=51 60 08; H_90=51 60 10).
- LDB dd=50 ss=60 oo=18 (fresh dd+oo vs locked LDB set).
- SUBV at 51 52 (H_50=51 50; H_55=52 51; H_74=50 52; H_83=52 50).
- ADDV at 52 50 (H_48=51 50; H_52=52 51; H_66=50 52; H_84=51 52).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_set_beefcafe.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_11111111.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_08.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_0a.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_10.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_18.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subv_5152.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addv_5250.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-15-log.md` — this file
- `scripts/_probe/parallel-batch-15-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-021 serialize PASSes + 1 Relock**

Pass pin from body-extend-020 Relock: `c922e4d482e1f82e939d24a790483b1b35e791d864e6adf3c26fe49e2dbe2ce1`.
Handlers before consolidate = 100 (H_00..H_93). Next selectors 0x64.. for H_94.. if all serialize.

PASS list for body-extend-021:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_94 | 0x64 | 0x30 SET | 0x50 0xBEEFCAFE | `48b8fecaefbe0000000049898780020000c3` (18B) | `b72d25116f116e99` |
| H_95 | 0x65 | 0x30 SET | 0x52 0x11111111 | `48b8111111110000000049898790020000c3` (18B) | `0d3e14e67a06fc73` |
| H_96 | 0x66 | 0x61 SUB-IMM | 0x50 0x08 | `498b87800200004883e80849898780020000c3` (19B) | `f6f0be6715ebc155` |
| H_97 | 0x67 | 0x62 ADD-IMM | 0x52 0x0A | `498b87900200004883c00a49898790020000c3` (19B) | `125226ff4633167f` |
| H_98 | 0x68 | 0x80 LDB | 0x52 0x60 0x10 | `498b87000300004883c010480fb60049898790020000c3` (23B) | `fed00067e5604398` |
| H_99 | 0x69 | 0x80 LDB | 0x50 0x60 0x18 | `498b87000300004883c018480fb60049898780020000c3` (23B) | `56296ca0160c87f5` |
| H_100 | 0x6A | 0x6A SUBV | 0x51 0x52 | `498b8788020000498b8f900200004829c849898788020000c3` (25B) | `47760053769fc7f2` |
| H_101 | 0x6B | 0x68 ADDV | 0x52 0x50 | `498b8790020000498b8f800200004801c849898790020000c3` (25B) | `5e5f7578c2ee8989` |

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
  fresh slot/imm/dst combinations not in H_48..H_93.
- If the parent decides to serialize, append H_94.. at selectors 0x64..:
  - H_94 0x30 SET (30 50 BEEFCAFE) — pin `48b8fecaefbe0000000049898780020000c3`
  - H_95 0x30 SET (30 52 11111111) — pin `48b8111111110000000049898790020000c3`
  - H_96 0x61 SUB-IMM (61 50 08) — pin `498b87800200004883e80849898780020000c3`
  - H_97 0x62 ADD-IMM (62 52 0A) — pin `498b87900200004883c00a49898790020000c3`
  - H_98 0x80 LDB (80 52 60 10) — pin `498b87000300004883c010480fb60049898790020000c3`
  - H_99 0x80 LDB (80 50 60 18) — pin `498b87000300004883c018480fb60049898780020000c3`
  - H_100 0x6A SUBV (6A 51 52) — pin `498b8788020000498b8f900200004829c849898788020000c3`
  - H_101 0x68 ADDV (68 52 50) — pin `498b8790020000498b8f800200004801c849898790020000c3`
- Plus 1 Relock after append from pin `c922e4d4…`.

## §7. Consolidation handoff

parent next = body-extend-021 serialize PASSes + 1 Relock

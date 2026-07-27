# parallel-batch-18 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-18-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-023 (pin `6fe414da…`, handlers = 124, H_110..H_117 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_117 and
> not already present as handlers in current `yoyo.ty` (skipped early
> INC/DEC 50, GET/ORV/ADDV/SUBV 50 51, IMUL 50 51, LDB 50 60 10/20).
> Slot/imm/dst variations of SET/ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x30 SET | slot=0x50 imm=0xFACEFEED | `48b8edfecefa0000000049898780020000c3` (18) | same | same | Y | `65776d5025793718` | `65776d5025793718` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x51 imm=0x1E | `498b87880200004883c01e49898788020000c3` (19) | same | same | Y | `04112b58beeaf745` | `04112b58beeaf745` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x52 imm=0x0A | `498b87900200004883e80a49898790020000c3` (19) | same | same | Y | `94c2473adbf34f73` | `94c2473adbf34f73` | PASS |
| 4 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x28 | `498b87000300004883c028480fb60049898780020000c3` (23) | same | same | Y | `c3ce682b77a27be5` | `c3ce682b77a27be5` | PASS |
| 5 | 0x30 SET | slot=0x52 imm=0xFACEFEED | `48b8edfecefa0000000049898790020000c3` (18) | same | same | Y | `3f12741045d591bb` | `3f12741045d591bb` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x50 imm=0x1E | `498b87800200004883c01e49898780020000c3` (19) | same | same | Y | `a9f2b7fd723605d1` | `a9f2b7fd723605d1` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x51 imm=0x05 | `498b87880200004883e80549898788020000c3` (19) | same | same | Y | `635c2e3c5a6e9f0f` | `635c2e3c5a6e9f0f` | PASS |
| 8 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x28 | `498b87000300004883c028480fb60049898788020000c3` (23) | same | same | Y | `8a29be86a3eeac5c` | `8a29be86a3eeac5c` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x30 SET slot=0x50 imm=0xFACEFEED — **PASS**

- fixture: `_scratch_set_50_facefeed.ty` + `.code.hex`
- expected pin (18B): `48b8edfecefa0000000049898780020000c3`
- js-sha256: `65776d5025793718800a5b39f9a042b1ada6d763504caf77b029214d3f402a27`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x51 imm=0x1E — **PASS**

- fixture: `_scratch_addimm_h51_1e.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c01e49898788020000c3`
- js-sha256: `04112b58beeaf74592b660f2f94860d3963e527e9f78a022193aa48ceed55288`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x52 imm=0x0A — **PASS**

- fixture: `_scratch_subimm_h52_0a.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e80a49898790020000c3`
- js-sha256: `94c2473adbf34f734ec417bb9f8aa4798ed9fae7f2196e8946bf05e0bffccd97`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x50 ss=0x60 oo=0x28 — **PASS**

- fixture: `_scratch_ldb_5060_28.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c028480fb60049898780020000c3`
- js-sha256: `c3ce682b77a27be5035c0979869867a5083d5dc0ed83a0b62231412ec7f6fe09`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x30 SET slot=0x52 imm=0xFACEFEED — **PASS**

- fixture: `_scratch_set_52_facefeed.ty` + `.code.hex`
- expected pin (18B): `48b8edfecefa0000000049898790020000c3`
- js-sha256: `3f12741045d591bb9cff930749a948500d458ed77a5f2de2213d5742d9ea8a83`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x50 imm=0x1E — **PASS**

- fixture: `_scratch_addimm_h50_1e.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c01e49898780020000c3`
- js-sha256: `a9f2b7fd723605d15baadd3100248907d491b07b4cb0b81981400b67b0773d05`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x51 imm=0x05 — **PASS**

- fixture: `_scratch_subimm_h51_05.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e80549898788020000c3`
- js-sha256: `635c2e3c5a6e9f0fa33dfdada38c7bed66dd6487fe0fbed1d4e00625ef634908`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x51 ss=0x60 oo=0x28 — **PASS**

- fixture: `_scratch_ldb_5160_28.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c028480fb60049898788020000c3`
- js-sha256: `8a29be86a3eeac5c375af596b5e67e93c3cc2627813248b43c655eebfbdd660a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SET at slot 50 imm=FACEFEED (H_68 12345678; H_76 F00DBABE; H_94 BEEFCAFE; H_109 C0FFEE00).
- ADD-IMM at slot 51 imm=1E (H_64=51 07; H_80=51 0A; H_111=51 14).
- SUB-IMM at slot 52 imm=0A (H_79=52 03; H_106=52 08).
- LDB dd=50 ss=60 oo=28 (H_99=50 60 18; early H_33=50 60 20; H_44=50 60 10).
- SET at slot 52 imm=FACEFEED (H_53 CAFEBABE; H_86 FEEDFACE; H_95 11111111; H_110 DEADF00D).
- ADD-IMM at slot 50 imm=1E (H_93=50 0F; H_108=50 14).
- SUB-IMM at slot 51 imm=05 (H_70=51 03; H_112=51 0A; H_81=50 05).
- LDB dd=51 ss=60 oo=28 (H_61=51 60 08; H_90=51 60 10; H_103=51 60 18; H_113=51 60 20).
- Skipped suggested INC/DEC 50 (H_17/H_18), GET/ORV/ADDV/SUBV 50 51 (early), IMUL 50 51 (H_34), LDB 50 60 10/20 (H_44/H_33).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_set_50_facefeed.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_1e.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_0a.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_28.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_52_facefeed.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_1e.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_05.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_28.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-18-log.md` — this file
- `scripts/_probe/parallel-batch-18-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-024 serialize PASSes + 1 Relock**

Pass pin from body-extend-023 Relock: `6fe414da02ce4723b40f2ced361cfd0a8da744443de39617fd307a74efd5b626`.
Handlers before consolidate = 124 (H_00..H_117). Next selectors 0x7C.. for H_118.. if all serialize.

PASS list for body-extend-024:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_118 | 0x7C | 0x30 SET | 0x50 0xFACEFEED | `48b8edfecefa0000000049898780020000c3` (18B) | `65776d5025793718` |
| H_119 | 0x7D | 0x62 ADD-IMM | 0x51 0x1E | `498b87880200004883c01e49898788020000c3` (19B) | `04112b58beeaf745` |
| H_120 | 0x7E | 0x61 SUB-IMM | 0x52 0x0A | `498b87900200004883e80a49898790020000c3` (19B) | `94c2473adbf34f73` |
| H_121 | 0x7F | 0x80 LDB | 0x50 0x60 0x28 | `498b87000300004883c028480fb60049898780020000c3` (23B) | `c3ce682b77a27be5` |
| H_122 | 0x80 | 0x30 SET | 0x52 0xFACEFEED | `48b8edfecefa0000000049898790020000c3` (18B) | `3f12741045d591bb` |
| H_123 | 0x81 | 0x62 ADD-IMM | 0x50 0x1E | `498b87800200004883c01e49898780020000c3` (19B) | `a9f2b7fd723605d1` |
| H_124 | 0x82 | 0x61 SUB-IMM | 0x51 0x05 | `498b87880200004883e80549898788020000c3` (19B) | `635c2e3c5a6e9f0f` |
| H_125 | 0x83 | 0x80 LDB | 0x51 0x60 0x28 | `498b87000300004883c028480fb60049898788020000c3` (23B) | `8a29be86a3eeac5c` |

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
  fresh slot/imm/dst combinations not in H_48..H_117.
- If the parent decides to serialize, append H_118.. at selectors 0x7C..:
  - H_118 0x30 SET (30 50 FACEFEED) — pin `48b8edfecefa0000000049898780020000c3`
  - H_119 0x62 ADD-IMM (62 51 1E) — pin `498b87880200004883c01e49898788020000c3`
  - H_120 0x61 SUB-IMM (61 52 0A) — pin `498b87900200004883e80a49898790020000c3`
  - H_121 0x80 LDB (80 50 60 28) — pin `498b87000300004883c028480fb60049898780020000c3`
  - H_122 0x30 SET (30 52 FACEFEED) — pin `48b8edfecefa0000000049898790020000c3`
  - H_123 0x62 ADD-IMM (62 50 1E) — pin `498b87800200004883c01e49898780020000c3`
  - H_124 0x61 SUB-IMM (61 51 05) — pin `498b87880200004883e80549898788020000c3`
  - H_125 0x80 LDB (80 51 60 28) — pin `498b87000300004883c028480fb60049898788020000c3`
- Plus 1 Relock after append from pin `6fe414da…`.

## §7. Consolidation handoff

parent next = body-extend-024 serialize PASSes + 1 Relock

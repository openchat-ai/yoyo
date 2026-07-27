# parallel-batch-99 Log · 8-pick P2 imm boundary scratch sweep

> Tag: `parallel-batch-99-EXPERIMENTAL-8-pick-p2-boundary` · 2026-07-26 (UTC+8).
> Following body-extend-105 DDC fix (pin `20391de3e4855c52…`, handlers = 771, H_00..H_764 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
>
> Picks from `docs/auxdocs/selfhost-emit-matrix.md` P2 rows (imm boundary ground truth, §4S.3.1).
> 8 picks: 2 LDB imm8 boundary + 3 ADD-IMM boundary + 3 SUB-IMM boundary.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dst=0x60 src=0x50 oo=127 | `498b87800200004883c07f480fb60049898700030000c3` (23) | same | same | Y | `70e494ef742c7042` | `70e494ef742c7042` | PASS |
| 2 | 0x80 LDB | dst=0x60 src=0x50 oo=-128 | `498b87800200004883c080480fb60049898700030000c3` (23) | DIFF | DIFF | N | `aa356076afe96107` | `aa356076afe96107` | REJECT |
| 3 | 0x62 ADD-IMM | slot=0x50 imm=127 | `498b87800200004883c07f49898780020000c3` (19) | same | same | Y | `df7da850e8308744` | `df7da850e8308744` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x50 imm=128 | `498b87800200004881c08000000049898780020000c3` (22) | same | DIFF | N | `483e67e06faf0c03` | `FAIL` | REJECT |
| 5 | 0x62 ADD-IMM | slot=0x50 imm=-1 | `498b87800200004881c0ff00000049898780020000c3` (22) | same | same | Y | `28bf04ba57d52c56` | `28bf04ba57d52c56` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x51 imm=-128 | `498b87880200004881e88000000049898788020000c3` (22) | same | same | Y | `f76a1690a99750ff` | `f76a1690a99750ff` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x51 imm=-129 | `498b87880200004881e87fffffff49898788020000c3` (22) | DIFF | DIFF | N | `e48a806fc07a0c4a` | `FAIL` | REJECT |
| 8 | 0x61 SUB-IMM | slot=0x51 imm=-1 | `498b87880200004881e8ff00000049898788020000c3` (22) | same | same | Y | `7a46aab2d98a29c0` | `7a46aab2d98a29c0` | PASS |

**Summary**: 5 PASS / 3 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dst=0x60 src=0x50 oo=127 — **PASS**

- fixture: `_scratch_ldb_60_50_127.ty` + `.code.hex`
- expected pin (23B): `498b87800200004883c07f480fb60049898700030000c3`
- js-sha256: `70e494ef742c70428ddca9b83c7e9fcc8c8e6397772ebab88229dac90f6f1434`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dst=0x60 src=0x50 oo=-128 — **REJECT**

- fixture: `_scratch_ldb_60_50_m128.ty` + `.code.hex`
- expected pin (23B): `498b87800200004883c080480fb60049898700030000c3`
- js-sha256: `aa356076afe96107cd6deba52537a36c540fa69353ae1f6af8d18092f6847c63`
- rust-sha256: `aa356076afe96107cd6deba52537a36c540fa69353ae1f6af8d18092f6847c63`
- byte-eq JS↔Rust↔expected: N

### Pick 3: 0x62 ADD-IMM slot=0x50 imm=127 — **PASS**

- fixture: `_scratch_addimm_50_127.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c07f49898780020000c3`
- js-sha256: `df7da850e83087448dbcb647a81e02e4892f0f91dfc1e57a831719ae6e7fe256`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x50 imm=128 — **REJECT**

- fixture: `_scratch_addimm_50_128.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c08000000049898780020000c3`
- js-sha256: `483e67e06faf0c0321f2f9a7a9c0d76d9fda2837f36b63c927a12069ef27ef78`
- rust-sha256: `FAIL`
- byte-eq JS↔Rust↔expected: N

### Pick 5: 0x62 ADD-IMM slot=0x50 imm=-1 — **PASS**

- fixture: `_scratch_addimm_50_m1.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0ff00000049898780020000c3`
- js-sha256: `28bf04ba57d52c5626526b7b8ecbca40005a003a85e8882b1ba89f13c7400083`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x51 imm=-128 — **PASS**

- fixture: `_scratch_subimm_51_m128.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e88000000049898788020000c3`
- js-sha256: `f76a1690a99750ff96f35a052519004f469b3b306da7134812d7c6e8d10cf962`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x51 imm=-129 — **REJECT**

- fixture: `_scratch_subimm_51_m129.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e87fffffff49898788020000c3`
- js-sha256: `e48a806fc07a0c4a20c0b20daa83bd59d3b3eb943a104a073c3cd2161122e036`
- rust-sha256: `FAIL`
- byte-eq JS↔Rust↔expected: N

### Pick 8: 0x61 SUB-IMM slot=0x51 imm=-1 — **PASS**

- fixture: `_scratch_subimm_51_m1.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8ff00000049898788020000c3`
- js-sha256: `7a46aab2d98a29c0351a236447433398498b55aaaf9f98d94881d9acf1dca60e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale (matrix trace)

- **P2 · 0x80 LDB imm8 boundary**: dst=60 src=50 oo=127 (max positive) and oo=-128 (min negative); ground truth for §4S.3.1 imm8/imm32 selection.
- **P2 · 0x62 ADD-IMM boundary**: imm=127 (imm8 max), imm=128 (imm32 start), imm=-1 (imm8 max negative).
- **P2 · 0x61 SUB-IMM boundary**: imm=-128 (imm8 min), imm=-129 (imm32 start), imm=-1 (imm8 max negative).
- All selfhost-need=NO — for matrix completeness / §4S.3.1 ground truth.

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_60_50_127.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_60_50_m128.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_50_127.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_50_128.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_50_m1.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_51_m128.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_51_m129.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_51_m1.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-99-log.md` — this file
- `scripts/_probe/parallel-batch-99-run.mjs` — probe runner (uses shared concurrent lib)
- `scripts/_probe/parallel-batch-scratch-lib.mjs` — ≤8 scratch workers

## §4. Parent next

**parent next = body-extend-106 serialize PASSes + 1 Relock** (consolidator)

Pass pin from body-extend-105 DDC fix: `20391de3e4855c52` (abbrev).
Handlers before consolidate = 771 (H_00..H_764). Next selectors `40 303`.. for H_765.. if all serialize.

PASS list for body-extend-106:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_765 | 0x303 | 0x80 LDB | 0x60 0x50 127 | `498b87800200004883c07f480fb60049898700030000c3` (23B) | `70e494ef742c7042` |
| H_766 | 0x304 | 0x62 ADD-IMM | 0x50 127 | `498b87800200004883c07f49898780020000c3` (19B) | `df7da850e8308744` |
| H_767 | 0x305 | 0x62 ADD-IMM | 0x50 -1 | `498b87800200004881c0ff00000049898780020000c3` (22B) | `28bf04ba57d52c56` |
| H_768 | 0x306 | 0x61 SUB-IMM | 0x51 -128 | `498b87880200004881e88000000049898788020000c3` (22B) | `f76a1690a99750ff` |
| H_769 | 0x307 | 0x61 SUB-IMM | 0x51 -1 | `498b87880200004881e8ff00000049898788020000c3` (22B) | `7a46aab2d98a29c0` |

## §5. Honesty override checks

- Peer JS/Rust divergence: **NONE** (fail-closed on divergence).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.

## §6. Consolidation handoff

parent next = body-extend-106 serialize PASSes + 1 Relock

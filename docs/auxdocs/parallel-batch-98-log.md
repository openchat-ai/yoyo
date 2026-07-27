# parallel-batch-98 Log · 8-pick matrix-priority scratch sweep (P1 focus)

> Tag: `parallel-batch-98-EXPERIMENTAL-8-pick-P1` · 2026-07-26 (UTC+8).
> Following body-extend-103 (pin `82709dac80fafbbf75421ea1e1b3493a4249f107f85115bfa0509f2d8cf11653`, handlers = 763, H_00..H_756 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (still do not invent-green).
>
> **STRATEGY**: matrix-priority picks from `docs/auxdocs/selfhost-emit-matrix.md` P1 rows —
> fresh dst/src/slot combos for ADDV/SUBV/IMUL/ORV/CMP (P1). Skips MEMCPY (needs real impl),
> skips imm ladders (P3, not selfhost-need).
>
> 8 picks: 2 ADDV + 2 SUBV + 2 IMUL + 1 ORV + 1 CMP — all multi-slot high-slot (≥0x60) pairs.
> Next selectors after 0x2FA: `40 2FB`..
>
> MEMCPY_DATA/STATE (P0) remain PARTIAL (stub=C3). INC/DEC expanded in batch-97; this beat
> prioritizes ADDV/SUBV/IMUL/CMP/ORV coverage.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x68 ADDV | dst=0x60 src=0x52 | `498b8700030000498b8f900200004801c849898700030000c3` (25) | same | same | Y | `8ff391002cbea550` | `8ff391002cbea550` | PASS |
| 2 | 0x68 ADDV | dst=0x62 src=0x50 | `498b8710030000498b8f800200004801c849898710030000c3` (25) | same | same | Y | `073788843bf7750a` | `073788843bf7750a` | PASS |
| 3 | 0x6A SUBV | dst=0x62 src=0x60 | `498b8710030000498b8f000300004829c849898710030000c3` (25) | same | same | Y | `99486e0deda02d10` | `99486e0deda02d10` | PASS |
| 4 | 0x6A SUBV | dst=0x62 src=0x50 | `498b8710030000498b8f800200004829c849898710030000c3` (25) | same | same | Y | `25e655acc3725ccf` | `25e655acc3725ccf` | PASS |
| 5 | 0x63 IMUL | dst=0x60 src=0x62 | `498b8700030000498b8f10030000480fafc149898700030000c3` (26) | same | same | Y | `b48b13130a2b4ebd` | `b48b13130a2b4ebd` | PASS |
| 6 | 0x63 IMUL | dst=0x62 src=0x61 | `498b8710030000498b8f08030000480fafc149898710030000c3` (26) | same | same | Y | `e2ff97cc9333b2bb` | `e2ff97cc9333b2bb` | PASS |
| 7 | 0x69 ORV | dst=0x60 src=0x62 | `498b8700030000498b8f100300004809c849898700030000c3` (25) | same | same | Y | `2a08a3bf815bd601` | `2a08a3bf815bd601` | PASS |
| 8 | 0x65 CMP | a=0x61 b=0x60 | `498b8708030000498b8f000300004839c8c3` (18) | same | same | Y | `8354e63f68f24924` | `8354e63f68f24924` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x68 ADDV dst=0x60 src=0x52 — **PASS**

- fixture: `_scratch_addv_60_52.ty` + `.code.hex`
- expected pin (25B): `498b8700030000498b8f900200004801c849898700030000c3`
- js-sha256: `8ff391002cbea550fc893223283d625146bfc3ec0bbdbb58811a035ec74f16e3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x68 ADDV dst=0x62 src=0x50 — **PASS**

- fixture: `_scratch_addv_62_50.ty` + `.code.hex`
- expected pin (25B): `498b8710030000498b8f800200004801c849898710030000c3`
- js-sha256: `073788843bf7750ab671110cb36b7bb11724933f38e63ece028f2cd20952c470`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x6A SUBV dst=0x62 src=0x60 — **PASS**

- fixture: `_scratch_subv_62_60.ty` + `.code.hex`
- expected pin (25B): `498b8710030000498b8f000300004829c849898710030000c3`
- js-sha256: `99486e0deda02d1076a0a53ca55b201dfd04638453b13b10d03d0299624f4fac`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x6A SUBV dst=0x62 src=0x50 — **PASS**

- fixture: `_scratch_subv_62_50.ty` + `.code.hex`
- expected pin (25B): `498b8710030000498b8f800200004829c849898710030000c3`
- js-sha256: `25e655acc3725ccf2bf691cd16ed75e7ffc09fc9f0f623077a3ba2f46946ae74`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x63 IMUL dst=0x60 src=0x62 — **PASS**

- fixture: `_scratch_imul_60_62.ty` + `.code.hex`
- expected pin (26B): `498b8700030000498b8f10030000480fafc149898700030000c3`
- js-sha256: `b48b13130a2b4ebd4c2ef58dfb1ac01430d1db73d88ea1f6a09daa8838920db9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x63 IMUL dst=0x62 src=0x61 — **PASS**

- fixture: `_scratch_imul_62_61.ty` + `.code.hex`
- expected pin (26B): `498b8710030000498b8f08030000480fafc149898710030000c3`
- js-sha256: `e2ff97cc9333b2bbc28913e64da4fdaf11ed70cff2dce8a910b76d238ddb0af7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x69 ORV dst=0x60 src=0x62 — **PASS**

- fixture: `_scratch_orv_60_62.ty` + `.code.hex`
- expected pin (25B): `498b8700030000498b8f100300004809c849898700030000c3`
- js-sha256: `2a08a3bf815bd601305fedd6a3b20fbaffd6d7f07ef26b64e37556f1e578386e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x65 CMP a=0x61 b=0x60 — **PASS**

- fixture: `_scratch_cmp_61_60.ty` + `.code.hex`
- expected pin (18B): `498b8708030000498b8f000300004839c8c3`
- js-sha256: `8354e63f68f249243e74ad4d7ac44689d85bebd8f2e4b9c4485101555cfd04ac`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale (matrix trace)

- **P1 · 68 ADDV (2 picks)**: dst=60 src=52; dst=62 src=50 — high-slot pairs filling slots ≥0x60.
- **P1 · 6A SUBV (2 picks)**: dst=62 src=60; dst=62 src=50 — same rationale.
- **P1 · 63 IMUL (2 picks)**: dst=60 src=62; dst=62 src=61 — high-slot multiply pairs.
- **P1 · 69 ORV (1 pick)**: dst=60 src=62 — fresh bitwise OR pair.
- **P1 · 65 CMP (1 pick)**: a=61 b=60 — fresh comparison pair for Jcc loops.
- No D-1 0x20/0x50/0x51, no D-2 0x64. No MEMCPY (deferred, needs real impl). No imm ladders (P3).
- yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_addv_60_52.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addv_62_50.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subv_62_60.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subv_62_50.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_imul_60_62.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_imul_62_61.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_orv_60_62.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_cmp_61_60.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-98-log.md` — this file
- `scripts/_probe/parallel-batch-98-run.mjs` — probe runner (uses shared concurrent lib)
- `scripts/_probe/parallel-batch-scratch-lib.mjs` — ≤8 scratch workers

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-104 serialize PASSes + 1 Relock** (consolidator adds H_757..H_764).

Pass pin from body-extend-103 Relock: `82709dac80fafbbf75421ea1e1b3493a4249f107f85115bfa0509f2d8cf11653`.
Handlers before consolidate = 763 (H_00..H_756). Next selectors `40 2FB`.. for H_757.. if all serialize.

PASS list for body-extend-104:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_757 | 0x2FB | 0x68 ADDV | 0x60 0x52 | `498b8700030000498b8f900200004801c849898700030000c3` (25B) | `8ff391002cbea550` |
| H_758 | 0x2FC | 0x68 ADDV | 0x62 0x50 | `498b8710030000498b8f800200004801c849898710030000c3` (25B) | `073788843bf7750a` |
| H_759 | 0x2FD | 0x6A SUBV | 0x62 0x60 | `498b8710030000498b8f000300004829c849898710030000c3` (25B) | `99486e0deda02d10` |
| H_760 | 0x2FE | 0x6A SUBV | 0x62 0x50 | `498b8710030000498b8f800200004829c849898710030000c3` (25B) | `25e655acc3725ccf` |
| H_761 | 0x2FF | 0x63 IMUL | 0x60 0x62 | `498b8700030000498b8f10030000480fafc149898700030000c3` (26B) | `b48b13130a2b4ebd` |
| H_762 | 0x300 | 0x63 IMUL | 0x62 0x61 | `498b8710030000498b8f08030000480fafc149898710030000c3` (26B) | `e2ff97cc9333b2bb` |
| H_763 | 0x301 | 0x69 ORV | 0x60 0x62 | `498b8700030000498b8f100300004809c849898700030000c3` (25B) | `2a08a3bf815bd601` |
| H_764 | 0x302 | 0x65 CMP | 0x61 0x60 | `498b8708030000498b8f000300004839c8c3` (18B) | `8354e63f68f24924` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- MEMCPY_DATA/STATE (P0) remain PARTIAL (stub=C3) — semantic gap, not invented-green.

## §6. Consolidation handoff

parent next = body-extend-104 serialize PASSes + 1 Relock (INC/DEC slots ≥0x62 may need next beat if 8 picks saturated)

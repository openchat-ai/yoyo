# parallel-batch-97 Log · 8-pick matrix-priority scratch sweep (P1)

> Tag: `parallel-batch-97-EXPERIMENTAL-8-pick-matrix-p1` · 2026-07-26 (UTC+8).
> Following body-extend-102 (pin `6532ea809c58c7a9…`, handlers = 755, H_00..H_748 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
>
> Picks from `docs/auxdocs/selfhost-emit-matrix.md` priority gaps (P1).
> MEMCPY (P0) skipped — requires real impl, not scratch bytes.
> 8 picks: 2 ADDV multi-combo + 2 SUBV multi-combo + 1 CMP multi-slot + 2 INC multi-slot + 1 DEC multi-slot.
> Next selectors after 0x2F2: `40 2F3`..

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x68 ADDV | dst=0x52 src=0x50 | `498b8790020000498b8f800200004801c849898790020000c3` (25) | same | same | Y | `5e5f7578c2ee8989` | `5e5f7578c2ee8989` | PASS |
| 2 | 0x68 ADDV | dst=0x50 src=0x51 | `498b8780020000498b8f880200004801c849898780020000c3` (25) | same | same | Y | `966a2e4950812b85` | `966a2e4950812b85` | PASS |
| 3 | 0x6A SUBV | dst=0x60 src=0x61 | `498b8700030000498b8f080300004829c849898700030000c3` (25) | same | same | Y | `d65a8f5935dd476c` | `d65a8f5935dd476c` | PASS |
| 4 | 0x6A SUBV | dst=0x61 src=0x62 | `498b8708030000498b8f100300004829c849898708030000c3` (25) | same | same | Y | `0a66bb2d15bbfcb4` | `0a66bb2d15bbfcb4` | PASS |
| 5 | 0x65 CMP | a=0x62 b=0x60 | `498b8710030000498b8f000300004839c8c3` (18) | same | same | Y | `6f62c844a1d0cce2` | `6f62c844a1d0cce2` | PASS |
| 6 | 0x66 INC | slot=0x60 | `498b870003000048ffc049898700030000c3` (18) | same | same | Y | `1867a2276c66120e` | `1867a2276c66120e` | PASS |
| 7 | 0x66 INC | slot=0x61 | `498b870803000048ffc049898708030000c3` (18) | same | same | Y | `c57b80b38b63cc91` | `c57b80b38b63cc91` | PASS |
| 8 | 0x67 DEC | slot=0x60 | `498b870003000048ffc849898700030000c3` (18) | same | same | Y | `9f4e8cb4c42073aa` | `9f4e8cb4c42073aa` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x68 ADDV dst=0x52 src=0x50 — **PASS**

- fixture: `_scratch_addv_52_50.ty` + `.code.hex`
- expected pin (25B): `498b8790020000498b8f800200004801c849898790020000c3`
- js-sha256: `5e5f7578c2ee89891c546d91f5297185696b7f91fbd3d2568b3ab66f26e593cf`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x68 ADDV dst=0x50 src=0x51 — **PASS**

- fixture: `_scratch_addv_50_51.ty` + `.code.hex`
- expected pin (25B): `498b8780020000498b8f880200004801c849898780020000c3`
- js-sha256: `966a2e4950812b858caccc890e53e1e5eb94e7b482ca71aa5e92eaac47fccfac`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x6A SUBV dst=0x60 src=0x61 — **PASS**

- fixture: `_scratch_subv_60_61.ty` + `.code.hex`
- expected pin (25B): `498b8700030000498b8f080300004829c849898700030000c3`
- js-sha256: `d65a8f5935dd476c1ba308be1ba326adb248b8b65288c1e1556cc9bad42d6a6d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x6A SUBV dst=0x61 src=0x62 — **PASS**

- fixture: `_scratch_subv_61_62.ty` + `.code.hex`
- expected pin (25B): `498b8708030000498b8f100300004829c849898708030000c3`
- js-sha256: `0a66bb2d15bbfcb4ef94543b1a22768aa356c7498e6318090afb4b80da134b16`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x65 CMP a=0x62 b=0x60 — **PASS**

- fixture: `_scratch_cmp_62_60.ty` + `.code.hex`
- expected pin (18B): `498b8710030000498b8f000300004839c8c3`
- js-sha256: `6f62c844a1d0cce2162205b6bf3ae687c47ec829cb2bd51db6f4825f64d04c41`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x66 INC slot=0x60 — **PASS**

- fixture: `_scratch_inc_60.ty` + `.code.hex`
- expected pin (18B): `498b870003000048ffc049898700030000c3`
- js-sha256: `1867a2276c66120ed8a3b60cc520b8439f13862d326d8adfb35c626f563244e0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x66 INC slot=0x61 — **PASS**

- fixture: `_scratch_inc_61.ty` + `.code.hex`
- expected pin (18B): `498b870803000048ffc049898708030000c3`
- js-sha256: `c57b80b38b63cc9106cef6c935ed3b043f141b86c8888f723c7ee7f6d8d662a6`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x67 DEC slot=0x60 — **PASS**

- fixture: `_scratch_dec_60.ty` + `.code.hex`
- expected pin (18B): `498b870003000048ffc849898700030000c3`
- js-sha256: `9f4e8cb4c42073aaf42ebed8676a0a4018176735e34c08ebfb76525bb34d94dd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale (matrix trace)

- **P1 · 68 ADDV multi-combo**: dst=52 src=50 and dst=50 src=51; fresh slot permutations for self-host arithmetic loops.
- **P1 · 6A SUBV multi-combo**: dst=60 src=61 and dst=61 src=62; fresh high-slot subtraction pairs.
- **P1 · 65 CMP multi-slot**: a=62 b=60; extends after H_748 (a=60 b=52) for Jcc self-host condition coverage.
- **P1 · 66 INC multi-slot**: slot=60 and slot=61; existing handlers only on 50/51/52; extends loop-counter range.
- **P1 · 67 DEC multi-slot**: slot=60; extends loop-counter decrement range.
- MEMCPY_DATA/STATE (P0) skipped — real implementation required before true DONE; not scratch bytes.
- No D-1 0x20/0x50/0x51 body use as primary opcodes. No D-2 0x64.
- yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_addv_52_50.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addv_50_51.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subv_60_61.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subv_61_62.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_cmp_62_60.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_inc_60.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_inc_61.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_dec_60.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-97-log.md` — this file
- `scripts/_probe/parallel-batch-97-run.mjs` — probe runner (uses shared concurrent lib)
- `scripts/_probe/parallel-batch-scratch-lib.mjs` — ≤8 scratch workers

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-103 serialize PASSes + 1 Relock** (consolidator)

Pass pin from body-extend-102 Relock: `6532ea809c58c7a9` (abbrev).
Handlers before consolidate = 755 (H_00..H_748). Next selectors `40 2F3`.. for H_749.. if all serialize.

PASS list for body-extend-103:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_749 | 0x2F3 | 0x68 ADDV | 0x52 0x50 | `498b8790020000498b8f800200004801c849898790020000c3` (25B) | `5e5f7578c2ee8989` |
| H_750 | 0x2F4 | 0x68 ADDV | 0x50 0x51 | `498b8780020000498b8f880200004801c849898780020000c3` (25B) | `966a2e4950812b85` |
| H_751 | 0x2F5 | 0x6A SUBV | 0x60 0x61 | `498b8700030000498b8f080300004829c849898700030000c3` (25B) | `d65a8f5935dd476c` |
| H_752 | 0x2F6 | 0x6A SUBV | 0x61 0x62 | `498b8708030000498b8f100300004829c849898708030000c3` (25B) | `0a66bb2d15bbfcb4` |
| H_753 | 0x2F7 | 0x65 CMP | 0x62 0x60 | `498b8710030000498b8f000300004839c8c3` (18B) | `6f62c844a1d0cce2` |
| H_754 | 0x2F8 | 0x66 INC | 0x60 | `498b870003000048ffc049898700030000c3` (18B) | `1867a2276c66120e` |
| H_755 | 0x2F9 | 0x66 INC | 0x61 | `498b870803000048ffc049898708030000c3` (18B) | `c57b80b38b63cc91` |
| H_756 | 0x2FA | 0x67 DEC | 0x60 | `498b870003000048ffc849898700030000c3` (18B) | `9f4e8cb4c42073aa` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.

## §6. Consolidation handoff

parent next = body-extend-103 serialize PASSes + 1 Relock

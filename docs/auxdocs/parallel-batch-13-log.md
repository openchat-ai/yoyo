# parallel-batch-13 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-13-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-018 (pin `e8603542…`, handlers = 84, H_70..H_77 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_77 and
> not already present as handlers in current `yoyo.ty`. Slot/imm/dst
> variations of ADD-IMM/SUB-IMM/ORV/SUBV/ADDV/IMUL. Skipped D-1
> 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x52 imm=0x07 | `498b87900200004883c00749898790020000c3` (19) | same | same | Y | `91fb897b55e83009` | `91fb897b55e83009` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x52 imm=0x03 | `498b87900200004883e80349898790020000c3` (19) | same | same | Y | `7da5e9ad5e34bec2` | `7da5e9ad5e34bec2` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x51 imm=0x0A | `498b87880200004883c00a49898788020000c3` (19) | same | same | Y | `ab876607753fb047` | `ab876607753fb047` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x50 imm=0x05 | `498b87800200004883e80549898780020000c3` (19) | same | same | Y | `326b330587908211` | `326b330587908211` | PASS |
| 5 | 0x69 ORV | (0x52, 0x50) | `498b8790020000498b8f800200004809c849898790020000c3` (25) | same | same | Y | `b6176d2903429c75` | `b6176d2903429c75` | PASS |
| 6 | 0x6A SUBV | (0x52, 0x50) | `498b8790020000498b8f800200004829c849898790020000c3` (25) | same | same | Y | `72eb8545d6b795df` | `72eb8545d6b795df` | PASS |
| 7 | 0x68 ADDV | (0x51, 0x52) | `498b8788020000498b8f900200004801c849898788020000c3` (25) | same | same | Y | `a443a523a2ee234c` | `a443a523a2ee234c` | PASS |
| 8 | 0x63 IMUL | (0x50, 0x52) | `498b8780020000498b8f90020000480fafc149898780020000c3` (26) | same | same | Y | `88201baeacfccce1` | `88201baeacfccce1` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x52 imm=0x07 — **PASS**

- fixture: `_scratch_addimm_h52.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c00749898790020000c3`
- js-sha256: `91fb897b55e830091367ecf46826ad3a9f3ae2b222ddc5c899ca7a2df97de861`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x52 imm=0x03 — **PASS**

- fixture: `_scratch_subimm_h52_03.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e80349898790020000c3`
- js-sha256: `7da5e9ad5e34bec2bf62e4e1c1079d98094795d5dd241efd62265ef646d6b337`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x51 imm=0x0A — **PASS**

- fixture: `_scratch_addimm_h51_0a.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c00a49898788020000c3`
- js-sha256: `ab876607753fb047c61dcf6b09e1731c8343af67607a6b51979b9975633e37b2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x50 imm=0x05 — **PASS**

- fixture: `_scratch_subimm_h50_05.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e80549898780020000c3`
- js-sha256: `326b3305879082119a17f60c6a277f73ddf416875ed9048fb25879e54db1aac4`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x69 ORV (0x52, 0x50) — **PASS**

- fixture: `_scratch_orv_5250.ty` + `.code.hex`
- expected pin (25B): `498b8790020000498b8f800200004809c849898790020000c3`
- js-sha256: `b6176d2903429c7570f6a8e12beb9a358ce22e5fa4c37c0658f7b4d36180c922`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x6A SUBV (0x52, 0x50) — **PASS**

- fixture: `_scratch_subv_5250.ty` + `.code.hex`
- expected pin (25B): `498b8790020000498b8f800200004829c849898790020000c3`
- js-sha256: `72eb8545d6b795df91a13982d608b1987bf85970c0d8f286a42196a0e5ea8351`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x68 ADDV (0x51, 0x52) — **PASS**

- fixture: `_scratch_addv_5152.ty` + `.code.hex`
- expected pin (25B): `498b8788020000498b8f900200004801c849898788020000c3`
- js-sha256: `a443a523a2ee234c3a7f963006d2da2503b733270f27464223d248e32a5c7a83`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x63 IMUL (0x50, 0x52) — **PASS**

- fixture: `_scratch_imul_5052.ty` + `.code.hex`
- expected pin (26B): `498b8780020000498b8f90020000480fafc149898780020000c3`
- js-sha256: `88201baeacfccce189b3f2b9f8b48a9e2b95ccd26344e389768367761b50bbcd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM at slot 0x52 imm=0x07 (H_64 uses 51 07; H_13 uses 50).
- SUB-IMM at slot 0x52 imm=0x03 (H_70 uses 51 03; H_23 uses 50 03).
- ADD-IMM at slot 0x51 imm=0x0A (fresh imm vs H_64's 0x07).
- SUB-IMM at slot 0x50 imm=0x05 (fresh imm vs H_23's 0x03).
- ORV/SUBV at 52 50 (H_49/H_50 are 51 50; H_54/H_55 are 52 51; H_73/H_74 are 50 52).
- ADDV at 51 52 (H_48=51 50; H_52=52 51; H_66=50 52).
- IMUL at 50 52 (H_34=50 51; H_56=51 50; H_57=52 51).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h52.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_03.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_0a.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_05.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_orv_5250.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subv_5250.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addv_5152.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_imul_5052.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-13-log.md` — this file
- `scripts/_probe/parallel-batch-13-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-019 serialize PASSes + 1 Relock**

Pass pin from body-extend-018 Relock: `e8603542fb13c5f027b3bea34b63aa0b8b20e82bb087ffe06568bd8193b401a2`.
Handlers before consolidate = 84 (H_00..H_77). Next selectors 0x54.. for H_78.. if all serialize.

PASS list for body-extend-019:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_78 | 0x54 | 0x62 ADD-IMM | 0x52 0x07 | `498b87900200004883c00749898790020000c3` (19B) | `91fb897b55e83009` |
| H_79 | 0x55 | 0x61 SUB-IMM | 0x52 0x03 | `498b87900200004883e80349898790020000c3` (19B) | `7da5e9ad5e34bec2` |
| H_80 | 0x56 | 0x62 ADD-IMM | 0x51 0x0A | `498b87880200004883c00a49898788020000c3` (19B) | `ab876607753fb047` |
| H_81 | 0x57 | 0x61 SUB-IMM | 0x50 0x05 | `498b87800200004883e80549898780020000c3` (19B) | `326b330587908211` |
| H_82 | 0x58 | 0x69 ORV | 0x52 0x50 | `498b8790020000498b8f800200004809c849898790020000c3` (25B) | `b6176d2903429c75` |
| H_83 | 0x59 | 0x6A SUBV | 0x52 0x50 | `498b8790020000498b8f800200004829c849898790020000c3` (25B) | `72eb8545d6b795df` |
| H_84 | 0x5A | 0x68 ADDV | 0x51 0x52 | `498b8788020000498b8f900200004801c849898788020000c3` (25B) | `a443a523a2ee234c` |
| H_85 | 0x5B | 0x63 IMUL | 0x50 0x52 | `498b8780020000498b8f90020000480fafc149898780020000c3` (26B) | `88201baeacfccce1` |

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
  fresh slot/imm/dst combinations not in H_48..H_77.
- If the parent decides to serialize, append H_78.. at selectors 0x54..:
  - H_78 0x62 ADD-IMM (62 52 07) — pin `498b87900200004883c00749898790020000c3`
  - H_79 0x61 SUB-IMM (61 52 03) — pin `498b87900200004883e80349898790020000c3`
  - H_80 0x62 ADD-IMM (62 51 0A) — pin `498b87880200004883c00a49898788020000c3`
  - H_81 0x61 SUB-IMM (61 50 05) — pin `498b87800200004883e80549898780020000c3`
  - H_82 0x69 ORV (69 52 50) — pin `498b8790020000498b8f800200004809c849898790020000c3`
  - H_83 0x6A SUBV (6A 52 50) — pin `498b8790020000498b8f800200004829c849898790020000c3`
  - H_84 0x68 ADDV (68 51 52) — pin `498b8788020000498b8f900200004801c849898788020000c3`
  - H_85 0x63 IMUL (63 50 52) — pin `498b8780020000498b8f90020000480fafc149898780020000c3`
- Plus 1 Relock after append from pin `e8603542…`.

## §7. Consolidation handoff

parent next = body-extend-019 serialize PASSes + 1 Relock

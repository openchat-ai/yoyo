# body-extend-019 SPAWN · consolidate parallel-batch-13

> Continuous queue handoff from parallel-batch-13 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `e8603542fb13c5f027b3bea34b63aa0b8b20e82bb087ffe06568bd8193b401a2` (abbrev `e8603542…`).
> Handlers = 84 (H_00..H_77). Last selectors: 0x4C..0x53 = H_70..H_77.
> Source: `docs/auxdocs/parallel-batch-13-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-018-log.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.

## Task: body-extend-019 (serialize + Relock)

Mirror body-extend-018 / body-extend-017 protocol:

1. Hand-author append H_78..H_85 to `yoyo/projects/yoyo.ty` at selectors `40 54` .. `40 5B`.
2. Promote fixtures from `_scratch_{addimm_h52,subimm_h52_03,addimm_h51_0a,subimm_h50_05,orv_5250,subv_5250,addv_5152,imul_5052}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect 68→76 JS, 76→84 Rust).
4. Verify + Relock once chaining from `e8603542fb13c5f0…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-019-log.md`.
7. Auto-spawn parallel-batch-14 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-14-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_78 | 0x54 | 0x62 ADD-IMM | 52 07 | `498b87900200004883c00749898790020000c3` (19B) | `91fb897b55e83009` |
| H_79 | 0x55 | 0x61 SUB-IMM | 52 03 | `498b87900200004883e80349898790020000c3` (19B) | `7da5e9ad5e34bec2` |
| H_80 | 0x56 | 0x62 ADD-IMM | 51 0A | `498b87880200004883c00a49898788020000c3` (19B) | `ab876607753fb047` |
| H_81 | 0x57 | 0x61 SUB-IMM | 50 05 | `498b87800200004883e80549898780020000c3` (19B) | `326b330587908211` |
| H_82 | 0x58 | 0x69 ORV | 52 50 | `498b8790020000498b8f800200004809c849898790020000c3` (25B) | `b6176d2903429c75` |
| H_83 | 0x59 | 0x6A SUBV | 52 50 | `498b8790020000498b8f800200004829c849898790020000c3` (25B) | `72eb8545d6b795df` |
| H_84 | 0x5A | 0x68 ADDV | 51 52 | `498b8788020000498b8f900200004801c849898788020000c3` (25B) | `a443a523a2ee234c` |
| H_85 | 0x5B | 0x63 IMUL | 50 52 | `498b8780020000498b8f90020000480fafc149898780020000c3` (26B) | `88201baeacfccce1` |

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_78 | `91fb897b55e830091367ecf46826ad3a9f3ae2b222ddc5c899ca7a2df97de861` |
| H_79 | `7da5e9ad5e34bec2bf62e4e1c1079d98094795d5dd241efd62265ef646d6b337` |
| H_80 | `ab876607753fb047c61dcf6b09e1731c8343af67607a6b51979b9975633e37b2` |
| H_81 | `326b3305879082119a17f60c6a277f73ddf416875ed9048fb25879e54db1aac4` |
| H_82 | `b6176d2903429c7570f6a8e12beb9a358ce22e5fa4c37c0658f7b4d36180c922` |
| H_83 | `72eb8545d6b795df91a13982d608b1987bf85970c0d8f286a42196a0e5ea8351` |
| H_84 | `a443a523a2ee234c3a7f963006d2da2503b733270f27464223d248e32a5c7a83` |
| H_85 | `88201baeacfccce189b3f2b9f8b48a9e2b95ccd26344e389768367761b50bbcd` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h52`, `_scratch_subimm_h52_03`, `_scratch_addimm_h51_0a`, `_scratch_subimm_h50_05`,
`_scratch_orv_5250`, `_scratch_subv_5250`, `_scratch_addv_5152`, `_scratch_imul_5052`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY 0x84/0x85.
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64.

# body-extend-044 SPAWN · consolidate parallel-batch-38

> Continuous queue handoff from parallel-batch-38 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `113decd0cbfa7a1106ae3f17f82ba7b6a135c8ad6a3b579b7c30978ffb96d7a0` (abbrev `113decd0…`).
> Handlers = 284 (H_00..H_277). Last selectors: 0x114..0x11B = H_270..H_277 (`40 114`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-38-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-043-log.md` / `docs/auxdocs/body-extend-043-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-043 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 11C`.. for H_278.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-044 (serialize + Relock)

Mirror body-extend-043 / body-extend-042 protocol:

1. Hand-author append H_278..H_285 to `yoyo/projects/yoyo.ty` at selectors `40 11C` .. `40 123` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h50_98,addimm_h51_98,addimm_h52_98,subimm_h50_98,subimm_h51_98,subimm_h52_98,ldb_5060_b8,ldb_5160_b8}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `113decd0cbfa7a11…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-044-log.md`.
7. Auto-spawn parallel-batch-39 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-39-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_278 | 0x11C | 0x62 ADD-IMM | 50 98 | `498b87800200004881c09800000049898780020000c3` (22B) | `13b9014e066c9897` |
| H_279 | 0x11D | 0x62 ADD-IMM | 51 98 | `498b87880200004881c09800000049898788020000c3` (22B) | `eaf423344be083bb` |
| H_280 | 0x11E | 0x62 ADD-IMM | 52 98 | `498b87900200004881c09800000049898790020000c3` (22B) | `0374f755088d14c3` |
| H_281 | 0x11F | 0x61 SUB-IMM | 50 98 | `498b87800200004881e89800000049898780020000c3` (22B) | `39737d6b950d19d4` |
| H_282 | 0x120 | 0x61 SUB-IMM | 51 98 | `498b87880200004881e89800000049898788020000c3` (22B) | `7dd6789e588e0525` |
| H_283 | 0x121 | 0x61 SUB-IMM | 52 98 | `498b87900200004881e89800000049898790020000c3` (22B) | `4df6f69f74da2e8d` |
| H_284 | 0x122 | 0x80 LDB | 50 60 B8 | `498b87000300004881c0b8000000480fb60049898780020000c3` (26B) | `c0d9668174c58dd0` |
| H_285 | 0x123 | 0x80 LDB | 51 60 B8 | `498b87000300004881c0b8000000480fb60049898788020000c3` (26B) | `0e4180bb03065699` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x98 uses imm32 add (`48 81 c0`) → 22B pins (H_278..H_280); not imm8.
SUB-IMM imm=0x98 uses imm32 sub (`48 81 e8`) → 22B pins (H_281..H_283); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_284..H_285).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_278 | `13b9014e066c98976376ee2850cb7d3fc2e6bd0d3ef80bbbd8a9cd13073e161d` |
| H_279 | `eaf423344be083bbb8984ea6223837dbfcae32483079cd2e5df726fa4bd5d54e` |
| H_280 | `0374f755088d14c3b6ad6c142a967885cadce8c9b9089f1bac6176163ae8a547` |
| H_281 | `39737d6b950d19d41d864cb081f21bb99f092d6264a976bcafff2bf604469148` |
| H_282 | `7dd6789e588e052568acffd479f315f11b111ef6b2d017c8053e4b9bfd53a4e9` |
| H_283 | `4df6f69f74da2e8d20a961321a4d177483303f8324be356adb8e4905a322c54e` |
| H_284 | `c0d9668174c58dd0043d62f032c5e053d16b39f223de538506fe4a12e5e7ab4f` |
| H_285 | `0e4180bb03065699dda791bd44881918b2db0e4467e62f0216505871c4f58873` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h50_98`, `_scratch_addimm_h51_98`, `_scratch_addimm_h52_98`, `_scratch_subimm_h50_98`,
`_scratch_subimm_h51_98`, `_scratch_subimm_h52_98`, `_scratch_ldb_5060_b8`, `_scratch_ldb_5160_b8`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 11C`.. for H_278.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

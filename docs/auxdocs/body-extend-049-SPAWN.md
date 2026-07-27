# body-extend-049 SPAWN · consolidate parallel-batch-43

> Continuous queue handoff from parallel-batch-43 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `9c2f924a2780d64647f590c707d39330fa4bff0e69a2c243c0550956ec2d41a2` (abbrev `9c2f924a…`).
> Handlers = 324 (H_00..H_317). Last selectors: 0x13C..0x143 = H_310..H_317 (`40 13C`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-43-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-048-log.md` / `docs/auxdocs/body-extend-048-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-048 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 144`.. for H_318.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-049 (serialize + Relock)

Mirror body-extend-048 / body-extend-047 protocol:

1. Hand-author append H_318..H_325 to `yoyo/projects/yoyo.ty` at selectors `40 144` .. `40 14B` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5160_d0,ldb_5260_d0,addimm_h50_c0,addimm_h51_c0,addimm_h52_c0,subimm_h50_c0,subimm_h51_c0,subimm_h52_c0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `9c2f924a2780d646…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-049-log.md`.
7. Auto-spawn parallel-batch-44 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-44-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_318 | 0x144 | 0x80 LDB | 51 60 D0 | `498b87000300004881c0d0000000480fb60049898788020000c3` (26B) | `2d00172cf7198885` |
| H_319 | 0x145 | 0x80 LDB | 52 60 D0 | `498b87000300004881c0d0000000480fb60049898790020000c3` (26B) | `e5577873d59f39b9` |
| H_320 | 0x146 | 0x62 ADD-IMM | 50 C0 | `498b87800200004881c0c000000049898780020000c3` (22B) | `14116ca20ac2ff30` |
| H_321 | 0x147 | 0x62 ADD-IMM | 51 C0 | `498b87880200004881c0c000000049898788020000c3` (22B) | `781fd0dd879b7d37` |
| H_322 | 0x148 | 0x62 ADD-IMM | 52 C0 | `498b87900200004881c0c000000049898790020000c3` (22B) | `187eebc8371ba7f5` |
| H_323 | 0x149 | 0x61 SUB-IMM | 50 C0 | `498b87800200004881e8c000000049898780020000c3` (22B) | `90c51fcf3eb0e0bb` |
| H_324 | 0x14A | 0x61 SUB-IMM | 51 C0 | `498b87880200004881e8c000000049898788020000c3` (22B) | `3c16c50a8e776b8a` |
| H_325 | 0x14B | 0x61 SUB-IMM | 52 C0 | `498b87900200004881e8c000000049898790020000c3` (22B) | `5bfec4655978ffd2` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0xC0 uses imm32 add (`48 81 c0`) → 22B pins (H_320..H_322); not imm8.
SUB-IMM imm=0xC0 uses imm32 sub (`48 81 e8`) → 22B pins (H_323..H_325); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_318..H_319).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_318 | `2d00172cf7198885bd7347e251eaa7c33540a4a35e9e1a084dfcac50707b3b47` |
| H_319 | `e5577873d59f39b9aa84901bcf4220f8cb1527234f0b8a578d6e3ab5cc017659` |
| H_320 | `14116ca20ac2ff30af682f03a3fcb55fc731bee7b9182f70e6e72465b47cc582` |
| H_321 | `781fd0dd879b7d37866e8985d59e2de18b6ea08bbeb56afba78594d1ae3ff565` |
| H_322 | `187eebc8371ba7f52de7e17a77d90ac98a6cb7401c25cb0218e6696b3a2b3850` |
| H_323 | `90c51fcf3eb0e0bb7e65b7153d7accbdb9208c666a19ed0ed0c822dcd3dcc7f5` |
| H_324 | `3c16c50a8e776b8aa864e0f7b40993705ad8259064013b1e680a242228aad37c` |
| H_325 | `5bfec4655978ffd201ae719eaaebb6d51f32bd2b0672b3c6e628e68a51e8ebae` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5160_d0`, `_scratch_ldb_5260_d0`, `_scratch_addimm_h50_c0`, `_scratch_addimm_h51_c0`,
`_scratch_addimm_h52_c0`, `_scratch_subimm_h50_c0`, `_scratch_subimm_h51_c0`, `_scratch_subimm_h52_c0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 144`.. for H_318.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

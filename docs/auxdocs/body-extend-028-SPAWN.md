# body-extend-028 SPAWN · consolidate parallel-batch-22

> Continuous queue handoff from parallel-batch-22 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `2a14beec0f08ffdfd64656bc2230706c4ec1928a697bf00f3905ff724c4d28f2` (abbrev `2a14beec…`).
> Handlers = 156 (H_00..H_149). Last selectors: 0x94..0x9B = H_142..H_149.
> Source: `docs/auxdocs/parallel-batch-22-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-027-log.md` / `docs/auxdocs/body-extend-027-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.

## Task: body-extend-028 (serialize + Relock)

Mirror body-extend-027 / body-extend-026 protocol:

1. Hand-author append H_150..H_157 to `yoyo/projects/yoyo.ty` at selectors `40 9C` .. `40 A3`.
2. Promote fixtures from `_scratch_{set_50_feedc0de,addimm_h50_32,subimm_h52_28,ldb_5060_48,ldb_5160_48,ldb_5260_48,addimm_h51_32,subimm_h50_28}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `2a14beec0f08ffdf…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-028-log.md`.
7. Auto-spawn parallel-batch-23 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-23-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_150 | 0x9C | 0x30 SET | 50 FEEDC0DE | `48b8dec0edfe0000000049898780020000c3` (18B) | `3d87228f78707f16` |
| H_151 | 0x9D | 0x62 ADD-IMM | 50 32 | `498b87800200004883c03249898780020000c3` (19B) | `5cc13067b0ad0632` |
| H_152 | 0x9E | 0x61 SUB-IMM | 52 28 | `498b87900200004883e82849898790020000c3` (19B) | `d336d72829e79f77` |
| H_153 | 0x9F | 0x80 LDB | 50 60 48 | `498b87000300004883c048480fb60049898780020000c3` (23B) | `db3f030b072b721d` |
| H_154 | 0xA0 | 0x80 LDB | 51 60 48 | `498b87000300004883c048480fb60049898788020000c3` (23B) | `3e69600006d17327` |
| H_155 | 0xA1 | 0x80 LDB | 52 60 48 | `498b87000300004883c048480fb60049898790020000c3` (23B) | `0cfd11ffdf5be6f0` |
| H_156 | 0xA2 | 0x62 ADD-IMM | 51 32 | `498b87880200004883c03249898788020000c3` (19B) | `344d6d45a4ba02f0` |
| H_157 | 0xA3 | 0x61 SUB-IMM | 50 28 | `498b87800200004883e82849898780020000c3` (19B) | `533c4ac0d8d19f34` |

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_150 | `3d87228f78707f16eadf1b3ef249639c448b7dc1db64a3d9d8ad24221f88e9a5` |
| H_151 | `5cc13067b0ad0632d182035545563e32a5ab3bb143beae8b89fb7bceb1e04463` |
| H_152 | `d336d72829e79f7758647ae6bf109d309d5797c99969f6983cbe433d90d292d3` |
| H_153 | `db3f030b072b721d36372873f3d9e00220569c23443560255a7217cfa82849df` |
| H_154 | `3e69600006d1732791495789ca5a0f9ffea91fee45d2d94a6af4b3ac0459c47d` |
| H_155 | `0cfd11ffdf5be6f0b031f500c1b90c644ba74f931c83b5d9da2706557aee32e3` |
| H_156 | `344d6d45a4ba02f09853820d0aa0320951fe86b20300873451f4f9f682e097f4` |
| H_157 | `533c4ac0d8d19f3438724884f33fbf6467c2e2576634bea73720cd6872b8d977` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_set_50_feedc0de`, `_scratch_addimm_h50_32`, `_scratch_subimm_h52_28`, `_scratch_ldb_5060_48`,
`_scratch_ldb_5160_48`, `_scratch_ldb_5260_48`, `_scratch_addimm_h51_32`, `_scratch_subimm_h50_28`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 9C`.. for H_150.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

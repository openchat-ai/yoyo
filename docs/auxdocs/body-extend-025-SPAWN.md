# body-extend-025 SPAWN · consolidate parallel-batch-19

> Continuous queue handoff from parallel-batch-19 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `59f461e4f8bcb4fd42077f2664dcf375e427c5a651bf7c1b5e7da612e9ca8840` (abbrev `59f461e4…`).
> Handlers = 132 (H_00..H_125). Last selectors: 0x7C..0x83 = H_118..H_125.
> Source: `docs/auxdocs/parallel-batch-19-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-024-log.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.

## Task: body-extend-025 (serialize + Relock)

Mirror body-extend-024 / body-extend-023 protocol:

1. Hand-author append H_126..H_133 to `yoyo/projects/yoyo.ty` at selectors `40 84` .. `40 8B`.
2. Promote fixtures from `_scratch_{ldb_5260_28,ldb_5060_30,set_51_baadf00d,addimm_h52_1e,subimm_h50_14,ldb_5160_30,set_52_baadf00d,subimm_h52_14}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `59f461e4f8bcb4fd…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-025-log.md`.
7. Auto-spawn parallel-batch-20 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-20-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_126 | 0x84 | 0x80 LDB | 52 60 28 | `498b87000300004883c028480fb60049898790020000c3` (23B) | `79c28018959b4fc6` |
| H_127 | 0x85 | 0x80 LDB | 50 60 30 | `498b87000300004883c030480fb60049898780020000c3` (23B) | `cd94626912ff725b` |
| H_128 | 0x86 | 0x30 SET | 51 BAADF00D | `48b80df0adba0000000049898788020000c3` (18B) | `4fdd3935ab5d005b` |
| H_129 | 0x87 | 0x62 ADD-IMM | 52 1E | `498b87900200004883c01e49898790020000c3` (19B) | `17f9786a60b3bf8e` |
| H_130 | 0x88 | 0x61 SUB-IMM | 50 14 | `498b87800200004883e81449898780020000c3` (19B) | `63dd43fcd1171d88` |
| H_131 | 0x89 | 0x80 LDB | 51 60 30 | `498b87000300004883c030480fb60049898788020000c3` (23B) | `76a78769a45c1add` |
| H_132 | 0x8A | 0x30 SET | 52 BAADF00D | `48b80df0adba0000000049898790020000c3` (18B) | `6a510ef468b0ac9d` |
| H_133 | 0x8B | 0x61 SUB-IMM | 52 14 | `498b87900200004883e81449898790020000c3` (19B) | `92d5ef49974024ee` |

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_126 | `79c28018959b4fc641d64335b8cd130aa655beb3dfd084de812ca8fcc4122699` |
| H_127 | `cd94626912ff725b577cdf1fae88078dcb29e27f41a817c902352fc6a0fa2e8b` |
| H_128 | `4fdd3935ab5d005b9a36c467ddb9e2532f4ceeeb220bd6757ef32656add08249` |
| H_129 | `17f9786a60b3bf8e97e60c506cf3dff061f2122d2bad6f3d71031fe2dda04d17` |
| H_130 | `63dd43fcd1171d88350c1f0d2ec36b4857e70050b0c80613ecfc691f5003069e` |
| H_131 | `76a78769a45c1add03ae3747a677f1b5f4be0d311117692daea4d392da51412b` |
| H_132 | `6a510ef468b0ac9dc63b5ace1414a89d50132097f4d0da453224b64a81a5e28d` |
| H_133 | `92d5ef49974024eebc1f9163452ea5b6653348a640be3930a1bda13bc1d7d03a` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5260_28`, `_scratch_ldb_5060_30`, `_scratch_set_51_baadf00d`, `_scratch_addimm_h52_1e`,
`_scratch_subimm_h50_14`, `_scratch_ldb_5160_30`, `_scratch_set_52_baadf00d`, `_scratch_subimm_h52_14`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 84` / `40 85` for H_126/H_127 is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

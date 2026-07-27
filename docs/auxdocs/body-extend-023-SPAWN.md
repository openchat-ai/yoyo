# body-extend-023 SPAWN · consolidate parallel-batch-17

> Continuous queue handoff from parallel-batch-17 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `c2d5106637e7fd4954668c06dba34a2d699f1b36a6053a8df027c19b251504eb` (abbrev `c2d51066…`).
> Handlers = 116 (H_00..H_109). Last selectors: 0x6C..0x73 = H_102..H_109.
> Source: `docs/auxdocs/parallel-batch-17-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-022-log.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.

## Task: body-extend-023 (serialize + Relock)

Mirror body-extend-022 / body-extend-021 protocol:

1. Hand-author append H_110..H_117 to `yoyo/projects/yoyo.ty` at selectors `40 74` .. `40 7B`.
2. Promote fixtures from `_scratch_{set_52_deadf00d,addimm_h51_14,subimm_h51_0a,ldb_5160_20,ldb_5260_20,addimm_h52_14,subimm_h50_0a,set_51_deadf00d}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `c2d5106637e7fd49…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-023-log.md`.
7. Auto-spawn parallel-batch-18 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-18-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_110 | 0x74 | 0x30 SET | 52 DEADF00D | `48b80df0adde0000000049898790020000c3` (18B) | `34b8f29b8558e0c5` |
| H_111 | 0x75 | 0x62 ADD-IMM | 51 14 | `498b87880200004883c01449898788020000c3` (19B) | `0de1fe36c79129f6` |
| H_112 | 0x76 | 0x61 SUB-IMM | 51 0A | `498b87880200004883e80a49898788020000c3` (19B) | `4da400c99cc085fe` |
| H_113 | 0x77 | 0x80 LDB | 51 60 20 | `498b87000300004883c020480fb60049898788020000c3` (23B) | `5d16e28161ed63a9` |
| H_114 | 0x78 | 0x80 LDB | 52 60 20 | `498b87000300004883c020480fb60049898790020000c3` (23B) | `974c709509825da0` |
| H_115 | 0x79 | 0x62 ADD-IMM | 52 14 | `498b87900200004883c01449898790020000c3` (19B) | `d868fff3f47795b7` |
| H_116 | 0x7A | 0x61 SUB-IMM | 50 0A | `498b87800200004883e80a49898780020000c3` (19B) | `ba5ad3395d4dc1a6` |
| H_117 | 0x7B | 0x30 SET | 51 DEADF00D | `48b80df0adde0000000049898788020000c3` (18B) | `022feb111dc961ea` |

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_110 | `34b8f29b8558e0c5141adeb334016ca4e9d2772977fef57f6cba0bdf996d97dd` |
| H_111 | `0de1fe36c79129f6d890a2f0016b461e975e29a850525c423c56a31a1f3034fd` |
| H_112 | `4da400c99cc085fe1e5d3af05b55f4fb95247f088aaac1fe3cd81a7f1f3c097c` |
| H_113 | `5d16e28161ed63a9d62328a8e78d567501615ae5b44130537fa826a9657ef51a` |
| H_114 | `974c709509825da0ece180b77ed2e346203c83fb87c7f4bc175fcb7f4d260f60` |
| H_115 | `d868fff3f47795b7f7c6375c3520801612c72d96e581c8af6c2b61a354495bc3` |
| H_116 | `ba5ad3395d4dc1a6a591578990bc48ea8121809eb0798c0c28e5b9bc502d43a7` |
| H_117 | `022feb111dc961ea3eedb3cdf88a5b34ebbf0cc39a32c1250b496ced508bbf64` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_set_52_deadf00d`, `_scratch_addimm_h51_14`, `_scratch_subimm_h51_0a`, `_scratch_ldb_5160_20`,
`_scratch_ldb_5260_20`, `_scratch_addimm_h52_14`, `_scratch_subimm_h50_0a`, `_scratch_set_51_deadf00d`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY 0x84/0x85.
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode — selector `40 74` for H_110 is fine).

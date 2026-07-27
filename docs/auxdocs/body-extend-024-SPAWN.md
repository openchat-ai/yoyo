# body-extend-024 SPAWN · consolidate parallel-batch-18

> Continuous queue handoff from parallel-batch-18 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `6fe414da02ce4723b40f2ced361cfd0a8da744443de39617fd307a74efd5b626` (abbrev `6fe414da…`).
> Handlers = 124 (H_00..H_117). Last selectors: 0x74..0x7B = H_110..H_117.
> Source: `docs/auxdocs/parallel-batch-18-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-023-log.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.

## Task: body-extend-024 (serialize + Relock)

Mirror body-extend-023 / body-extend-022 protocol:

1. Hand-author append H_118..H_125 to `yoyo/projects/yoyo.ty` at selectors `40 7C` .. `40 83`.
2. Promote fixtures from `_scratch_{set_50_facefeed,addimm_h51_1e,subimm_h52_0a,ldb_5060_28,set_52_facefeed,addimm_h50_1e,subimm_h51_05,ldb_5160_28}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `6fe414da02ce4723…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-024-log.md`.
7. Auto-spawn parallel-batch-19 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-19-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_118 | 0x7C | 0x30 SET | 50 FACEFEED | `48b8edfecefa0000000049898780020000c3` (18B) | `65776d5025793718` |
| H_119 | 0x7D | 0x62 ADD-IMM | 51 1E | `498b87880200004883c01e49898788020000c3` (19B) | `04112b58beeaf745` |
| H_120 | 0x7E | 0x61 SUB-IMM | 52 0A | `498b87900200004883e80a49898790020000c3` (19B) | `94c2473adbf34f73` |
| H_121 | 0x7F | 0x80 LDB | 50 60 28 | `498b87000300004883c028480fb60049898780020000c3` (23B) | `c3ce682b77a27be5` |
| H_122 | 0x80 | 0x30 SET | 52 FACEFEED | `48b8edfecefa0000000049898790020000c3` (18B) | `3f12741045d591bb` |
| H_123 | 0x81 | 0x62 ADD-IMM | 50 1E | `498b87800200004883c01e49898780020000c3` (19B) | `a9f2b7fd723605d1` |
| H_124 | 0x82 | 0x61 SUB-IMM | 51 05 | `498b87880200004883e80549898788020000c3` (19B) | `635c2e3c5a6e9f0f` |
| H_125 | 0x83 | 0x80 LDB | 51 60 28 | `498b87000300004883c028480fb60049898788020000c3` (23B) | `8a29be86a3eeac5c` |

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_118 | `65776d5025793718800a5b39f9a042b1ada6d763504caf77b029214d3f402a27` |
| H_119 | `04112b58beeaf74592b660f2f94860d3963e527e9f78a022193aa48ceed55288` |
| H_120 | `94c2473adbf34f734ec417bb9f8aa4798ed9fae7f2196e8946bf05e0bffccd97` |
| H_121 | `c3ce682b77a27be5035c0979869867a5083d5dc0ed83a0b62231412ec7f6fe09` |
| H_122 | `3f12741045d591bb9cff930749a948500d458ed77a5f2de2213d5742d9ea8a83` |
| H_123 | `a9f2b7fd723605d15baadd3100248907d491b07b4cb0b81981400b67b0773d05` |
| H_124 | `635c2e3c5a6e9f0fa33dfdada38c7bed66dd6487fe0fbed1d4e00625ef634908` |
| H_125 | `8a29be86a3eeac5c375af596b5e67e93c3cc2627813248b43c655eebfbdd660a` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_set_50_facefeed`, `_scratch_addimm_h51_1e`, `_scratch_subimm_h52_0a`, `_scratch_ldb_5060_28`,
`_scratch_set_52_facefeed`, `_scratch_addimm_h50_1e`, `_scratch_subimm_h51_05`, `_scratch_ldb_5160_28`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY 0x84/0x85.
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode — selector `40 7C` for H_118 is fine).

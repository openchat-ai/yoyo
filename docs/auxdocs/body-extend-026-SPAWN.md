# body-extend-026 SPAWN · consolidate parallel-batch-20

> Continuous queue handoff from parallel-batch-20 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `e59ddfae905aeea50f440cf46d763e29d869274866bc9b57cb3ab33886716fa2` (abbrev `e59ddfae…`).
> Handlers = 140 (H_00..H_133). Last selectors: 0x84..0x8B = H_126..H_133.
> Source: `docs/auxdocs/parallel-batch-20-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-025-log.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.

## Task: body-extend-026 (serialize + Relock)

Mirror body-extend-025 / body-extend-024 protocol:

1. Hand-author append H_134..H_141 to `yoyo/projects/yoyo.ty` at selectors `40 8C` .. `40 93`.
2. Promote fixtures from `_scratch_{ldb_5260_30,ldb_5060_38,set_50_0badf00d,addimm_h51_28,subimm_h51_1e,ldb_5160_38,addimm_h50_28,subimm_h52_1e}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `e59ddfae905aeea50f…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-026-log.md`.
7. Auto-spawn parallel-batch-21 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-21-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_134 | 0x8C | 0x80 LDB | 52 60 30 | `498b87000300004883c030480fb60049898790020000c3` (23B) | `b24f11cd6c12dc39` |
| H_135 | 0x8D | 0x80 LDB | 50 60 38 | `498b87000300004883c038480fb60049898780020000c3` (23B) | `f97682dbb19b0928` |
| H_136 | 0x8E | 0x30 SET | 50 0BADF00D | `48b80df0ad0b0000000049898780020000c3` (18B) | `5753e9efa883ecb9` |
| H_137 | 0x8F | 0x62 ADD-IMM | 51 28 | `498b87880200004883c02849898788020000c3` (19B) | `87a17504336759cb` |
| H_138 | 0x90 | 0x61 SUB-IMM | 51 1E | `498b87880200004883e81e49898788020000c3` (19B) | `d28f48426b980e60` |
| H_139 | 0x91 | 0x80 LDB | 51 60 38 | `498b87000300004883c038480fb60049898788020000c3` (23B) | `7595918efb0d5e8e` |
| H_140 | 0x92 | 0x62 ADD-IMM | 50 28 | `498b87800200004883c02849898780020000c3` (19B) | `7da4341eb02983a9` |
| H_141 | 0x93 | 0x61 SUB-IMM | 52 1E | `498b87900200004883e81e49898790020000c3` (19B) | `5e4e1c6e05df64c6` |

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_134 | `b24f11cd6c12dc39e53b95fe2f4c91ed236a0b1356621974059da28144730c9f` |
| H_135 | `f97682dbb19b0928bedacc0627817aec3eb17873860d92916d8b7c0d562f18e8` |
| H_136 | `5753e9efa883ecb9ffcf006ef1e00df90f7b646be15211d10ac3fbc2a4b53b1e` |
| H_137 | `87a17504336759cb8c7789840f722dcbe7c26388d69baaeeef66d99ea3b83b6d` |
| H_138 | `d28f48426b980e60499ad1b9c06d9fa6214a3d88cbffb9994824fb1ca455b1d7` |
| H_139 | `7595918efb0d5e8e3fa4424e32c35d87881c69c3eff6b7cb1b86e69febbff4e5` |
| H_140 | `7da4341eb02983a9e5f19adb1ef2a7bbcbd481ff6ae3b4bf5e37966738f86618` |
| H_141 | `5e4e1c6e05df64c6714a71ff811564502a11d8a703f037528d754b5b9c3906b4` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5260_30`, `_scratch_ldb_5060_38`, `_scratch_set_50_0badf00d`, `_scratch_addimm_h51_28`,
`_scratch_subimm_h51_1e`, `_scratch_ldb_5160_38`, `_scratch_addimm_h50_28`, `_scratch_subimm_h52_1e`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 8C`.. for H_134.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

# body-extend-034 SPAWN · consolidate parallel-batch-28

> Continuous queue handoff from parallel-batch-28 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `0f0fce9a754e262914c8e2a78ca2558bd8af31ab0d532339f49018c2354cdac2` (abbrev `0f0fce9a…`).
> Handlers = 204 (H_00..H_197). Last selectors: 0xC4..0xCB = H_190..H_197.
> Source: `docs/auxdocs/parallel-batch-28-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-033-log.md` / `docs/auxdocs/body-extend-033-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-033 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.

## Task: body-extend-034 (serialize + Relock)

Mirror body-extend-033 / body-extend-032 protocol:

1. Hand-author append H_198..H_205 to `yoyo/projects/yoyo.ty` at selectors `40 CC` .. `40 D3`.
2. Promote fixtures from `_scratch_{addimm_h52_50,subimm_h50_48,subimm_h52_48,ldb_5060_78,set_51_c0dec0de,addimm_h50_58,subimm_h51_50,ldb_5160_78}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `0f0fce9a754e2629…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-034-log.md`.
7. Auto-spawn parallel-batch-29 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-29-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_198 | 0xCC | 0x62 ADD-IMM | 52 50 | `498b87900200004883c05049898790020000c3` (19B) | `684324dfa8a4c08b` |
| H_199 | 0xCD | 0x61 SUB-IMM | 50 48 | `498b87800200004883e84849898780020000c3` (19B) | `5f68485aac429a89` |
| H_200 | 0xCE | 0x61 SUB-IMM | 52 48 | `498b87900200004883e84849898790020000c3` (19B) | `d3786a374b0a48db` |
| H_201 | 0xCF | 0x80 LDB | 50 60 78 | `498b87000300004883c078480fb60049898780020000c3` (23B) | `431d73b2dfe3fbd1` |
| H_202 | 0xD0 | 0x30 SET | 51 C0DEC0DE | `48b8dec0dec00000000049898788020000c3` (18B) | `8b80a408a82bd068` |
| H_203 | 0xD1 | 0x62 ADD-IMM | 50 58 | `498b87800200004883c05849898780020000c3` (19B) | `84fd334ba8eecae0` |
| H_204 | 0xD2 | 0x61 SUB-IMM | 51 50 | `498b87880200004883e85049898788020000c3` (19B) | `3eba365fe5dedefd` |
| H_205 | 0xD3 | 0x80 LDB | 51 60 78 | `498b87000300004883c078480fb60049898788020000c3` (23B) | `ed2e4285f92ea9f6` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_198 | `684324dfa8a4c08be9943b0b80b73507cf9116256b66132d1beca41fc12dd7ee` |
| H_199 | `5f68485aac429a893e71839cb7376422ddb8bdb740a2f5c4d9d2f75844ec2ec9` |
| H_200 | `d3786a374b0a48dbd7385ab5965bdcc6e5b74bdc42823b40cc3cfef30e25b36e` |
| H_201 | `431d73b2dfe3fbd18b4f5aefb72090288ff85a6ad0a182e86419c70af2ecd2ec` |
| H_202 | `8b80a408a82bd06813705de9302ef1b7467a026b445611a93a18de7fc8d6a488` |
| H_203 | `84fd334ba8eecae0c74fed633f37dafdb241c87091eef0403fb1df04501c6060` |
| H_204 | `3eba365fe5dedefd45b965fd031f7d08f94f28834b389c93061a42487534e466` |
| H_205 | `ed2e4285f92ea9f658d7123d18212b7bcabc0235a2a473295930a90860fb0b04` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h52_50`, `_scratch_subimm_h50_48`, `_scratch_subimm_h52_48`, `_scratch_ldb_5060_78`,
`_scratch_set_51_c0dec0de`, `_scratch_addimm_h50_58`, `_scratch_subimm_h51_50`, `_scratch_ldb_5160_78`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 CC`.. for H_198.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

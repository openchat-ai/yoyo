# body-extend-022 SPAWN · consolidate parallel-batch-16

> Continuous queue handoff from parallel-batch-16 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `07eee98cb95446f2c277bcd78b211e43e7b274f583ac02392691dfc1b204cd0a` (abbrev `07eee98c…`).
> Handlers = 108 (H_00..H_101). Last selectors: 0x64..0x6B = H_94..H_101.
> Source: `docs/auxdocs/parallel-batch-16-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-021-log.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.

## Task: body-extend-022 (serialize + Relock)

Mirror body-extend-021 / body-extend-020 protocol:

1. Hand-author append H_102..H_109 to `yoyo/projects/yoyo.ty` at selectors `40 6C` .. `40 73`.
2. Promote fixtures from `_scratch_{cmp_5152,ldb_5160_18,ldb_5260_18,set_c0ffee00,subimm_h52_08,imul_5152,addimm_h50_14,set_50_c0ffee00}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `07eee98cb95446f2…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-022-log.md`.
7. Auto-spawn parallel-batch-17 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-17-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_102 | 0x6C | 0x65 CMP | 51 52 | `498b8788020000498b8f900200004839c8c3` (18B) | `2cf366028a7416c3` |
| H_103 | 0x6D | 0x80 LDB | 51 60 18 | `498b87000300004883c018480fb60049898788020000c3` (23B) | `0b1b7a7c7810f66b` |
| H_104 | 0x6E | 0x80 LDB | 52 60 18 | `498b87000300004883c018480fb60049898790020000c3` (23B) | `8137e5bda9f228f5` |
| H_105 | 0x6F | 0x30 SET | 51 C0FFEE00 | `48b800eeffc00000000049898788020000c3` (18B) | `6da3781de89ad437` |
| H_106 | 0x70 | 0x61 SUB-IMM | 52 08 | `498b87900200004883e80849898790020000c3` (19B) | `6cd180e2545680bd` |
| H_107 | 0x71 | 0x63 IMUL | 51 52 | `498b8788020000498b8f90020000480fafc149898788020000c3` (26B) | `3b7aa6ccd7e47092` |
| H_108 | 0x72 | 0x62 ADD-IMM | 50 14 | `498b87800200004883c01449898780020000c3` (19B) | `8007f38af1d95403` |
| H_109 | 0x73 | 0x30 SET | 50 C0FFEE00 | `48b800eeffc00000000049898780020000c3` (18B) | `9f214984263cafa8` |

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_102 | `2cf366028a7416c3b45fff656f8f268f15a0042dbc7b34068f090780717badbb` |
| H_103 | `0b1b7a7c7810f66b4aedd4ce6f23bccc18783bef8477393ff3215b9201a311b1` |
| H_104 | `8137e5bda9f228f5a3c816362d0ce7500b280a56938a63e693163ae5af5a4ecd` |
| H_105 | `6da3781de89ad437035a6d41ae13c3bc9910d9c1986d680e659cc40a1ae54bde` |
| H_106 | `6cd180e2545680bd1df2559b3da6103fc02790396b394febc11fa2a8c9077697` |
| H_107 | `3b7aa6ccd7e470921429559d896f949942f78cd05400cbec002c0d91e1ff1301` |
| H_108 | `8007f38af1d95403e15bbb55676b8c705ad1d9a851fa6bb04b26cda3bd9a3d37` |
| H_109 | `9f214984263cafa8dfbce48ca2fe7be953c5bcd96d8957589e5e83ee0fc748cd` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_cmp_5152`, `_scratch_ldb_5160_18`, `_scratch_ldb_5260_18`, `_scratch_set_c0ffee00`,
`_scratch_subimm_h52_08`, `_scratch_imul_5152`, `_scratch_addimm_h50_14`, `_scratch_set_50_c0ffee00`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY 0x84/0x85.
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode — selector `40 6C` for H_102 is fine).

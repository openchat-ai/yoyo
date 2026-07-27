# body-extend-021 SPAWN · consolidate parallel-batch-15

> Continuous queue handoff from parallel-batch-15 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `c922e4d482e1f82e939d24a790483b1b35e791d864e6adf3c26fe49e2dbe2ce1` (abbrev `c922e4d4…`).
> Handlers = 100 (H_00..H_93). Last selectors: 0x5C..0x63 = H_86..H_93.
> Source: `docs/auxdocs/parallel-batch-15-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-020-log.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.

## Task: body-extend-021 (serialize + Relock)

Mirror body-extend-020 / body-extend-019 protocol:

1. Hand-author append H_94..H_101 to `yoyo/projects/yoyo.ty` at selectors `40 64` .. `40 6B`.
2. Promote fixtures from `_scratch_{set_beefcafe,set_11111111,subimm_h50_08,addimm_h52_0a,ldb_5260_10,ldb_5060_18,subv_5152,addv_5250}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `c922e4d482e1f82e…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-021-log.md`.
7. Auto-spawn parallel-batch-16 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-16-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_94 | 0x64 | 0x30 SET | 50 BEEFCAFE | `48b8fecaefbe0000000049898780020000c3` (18B) | `b72d25116f116e99` |
| H_95 | 0x65 | 0x30 SET | 52 11111111 | `48b8111111110000000049898790020000c3` (18B) | `0d3e14e67a06fc73` |
| H_96 | 0x66 | 0x61 SUB-IMM | 50 08 | `498b87800200004883e80849898780020000c3` (19B) | `f6f0be6715ebc155` |
| H_97 | 0x67 | 0x62 ADD-IMM | 52 0A | `498b87900200004883c00a49898790020000c3` (19B) | `125226ff4633167f` |
| H_98 | 0x68 | 0x80 LDB | 52 60 10 | `498b87000300004883c010480fb60049898790020000c3` (23B) | `fed00067e5604398` |
| H_99 | 0x69 | 0x80 LDB | 50 60 18 | `498b87000300004883c018480fb60049898780020000c3` (23B) | `56296ca0160c87f5` |
| H_100 | 0x6A | 0x6A SUBV | 51 52 | `498b8788020000498b8f900200004829c849898788020000c3` (25B) | `47760053769fc7f2` |
| H_101 | 0x6B | 0x68 ADDV | 52 50 | `498b8790020000498b8f800200004801c849898790020000c3` (25B) | `5e5f7578c2ee8989` |

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_94 | `b72d25116f116e993f9ffc5dd9bebffcec956c47cc7447e27931754e37027a31` |
| H_95 | `0d3e14e67a06fc73469bc396e4af546c173dcf528043e42b93c7f7d59d69e518` |
| H_96 | `f6f0be6715ebc155ca2f2b1183eaebf7064c69bb0d15a75a1e1abc2efda095ac` |
| H_97 | `125226ff4633167f3d94147ecdd0ce4e4263f081b9295e7e9e0ae21ee021cf92` |
| H_98 | `fed00067e560439858a8933d9f80c58e04d591ff5d66d29320907fbccfa78581` |
| H_99 | `56296ca0160c87f5542065c808bc007ebceef6ab2eaa6f0136588e630eecac5f` |
| H_100 | `47760053769fc7f2ee4d69ffd5d6e027dcce195d6d4dda1b7ec5a17549bac233` |
| H_101 | `5e5f7578c2ee89891c546d91f5297185696b7f91fbd3d2568b3ab66f26e593cf` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_set_beefcafe`, `_scratch_set_11111111`, `_scratch_subimm_h50_08`, `_scratch_addimm_h52_0a`,
`_scratch_ldb_5260_10`, `_scratch_ldb_5060_18`, `_scratch_subv_5152`, `_scratch_addv_5250`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY 0x84/0x85.
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode — selector `40 64` for H_94 is fine).

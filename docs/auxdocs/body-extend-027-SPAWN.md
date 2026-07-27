# body-extend-027 SPAWN · consolidate parallel-batch-21

> Continuous queue handoff from parallel-batch-21 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `6c42f38cd61a0603f8892cbfdf36ab3966be5f894ce6a053c403d014507a6cc7` (abbrev `6c42f38c…`).
> Handlers = 148 (H_00..H_141). Last selectors: 0x8C..0x93 = H_134..H_141.
> Source: `docs/auxdocs/parallel-batch-21-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-026-log.md` / `docs/auxdocs/body-extend-026-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.

## Task: body-extend-027 (serialize + Relock)

Mirror body-extend-026 / body-extend-025 protocol:

1. Hand-author append H_142..H_149 to `yoyo/projects/yoyo.ty` at selectors `40 94` .. `40 9B`.
2. Promote fixtures from `_scratch_{ldb_5260_38,set_51_feedc0de,addimm_h52_28,subimm_h50_1e,ldb_5160_40,ldb_5260_40,set_52_feedc0de,subimm_h51_28}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `6c42f38cd61a0603…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-027-log.md`.
7. Auto-spawn parallel-batch-22 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-22-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_142 | 0x94 | 0x80 LDB | 52 60 38 | `498b87000300004883c038480fb60049898790020000c3` (23B) | `3a77b354a8f367d9` |
| H_143 | 0x95 | 0x30 SET | 51 FEEDC0DE | `48b8dec0edfe0000000049898788020000c3` (18B) | `c5643d1114f105f8` |
| H_144 | 0x96 | 0x62 ADD-IMM | 52 28 | `498b87900200004883c02849898790020000c3` (19B) | `5550c0d36ce045ad` |
| H_145 | 0x97 | 0x61 SUB-IMM | 50 1E | `498b87800200004883e81e49898780020000c3` (19B) | `2f7e70868b896f51` |
| H_146 | 0x98 | 0x80 LDB | 51 60 40 | `498b87000300004883c040480fb60049898788020000c3` (23B) | `bedb61608d220fc2` |
| H_147 | 0x99 | 0x80 LDB | 52 60 40 | `498b87000300004883c040480fb60049898790020000c3` (23B) | `579799f170fc91b1` |
| H_148 | 0x9A | 0x30 SET | 52 FEEDC0DE | `48b8dec0edfe0000000049898790020000c3` (18B) | `24133e376bdef965` |
| H_149 | 0x9B | 0x61 SUB-IMM | 51 28 | `498b87880200004883e82849898788020000c3` (19B) | `d552be0871d06b76` |

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_142 | `3a77b354a8f367d96aee4d29da1821cd86894b36e7a0834dc9c3933a580cb3a8` |
| H_143 | `c5643d1114f105f821fa0a4a5f9bc6a91cc05f332d7adac344bea52f320c965f` |
| H_144 | `5550c0d36ce045ade9841f009656d6384cc7cc4620c65ec7d582003db899dae0` |
| H_145 | `2f7e70868b896f51dd30804728c3048b61c1ecdc70a0e3b517c682f56a97b8e8` |
| H_146 | `bedb61608d220fc24d9fb46d19fab1159d3b70d44b019053e389dba987cc8956` |
| H_147 | `579799f170fc91b118eb31bfa020cfca1206293e9825d40db79e431bce9b3f2c` |
| H_148 | `24133e376bdef965f90c5f45d5d424475528135a7c11da6f6f56ee2742ab413f` |
| H_149 | `d552be0871d06b7664f633c8c48386942cc100afd48c4f22485296b818284ca4` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5260_38`, `_scratch_set_51_feedc0de`, `_scratch_addimm_h52_28`, `_scratch_subimm_h50_1e`,
`_scratch_ldb_5160_40`, `_scratch_ldb_5260_40`, `_scratch_set_52_feedc0de`, `_scratch_subimm_h51_28`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 94`.. for H_142.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

# body-extend-029 SPAWN · consolidate parallel-batch-23

> Continuous queue handoff from parallel-batch-23 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `80287f8fe0a8eb0977a5b0cf8f6e39be7839c229229e6ded1853630d6430e33d` (abbrev `80287f8f…`).
> Handlers = 164 (H_00..H_157). Last selectors: 0x9C..0xA3 = H_150..H_157.
> Source: `docs/auxdocs/parallel-batch-23-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-028-log.md` / `docs/auxdocs/body-extend-028-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.

## Task: body-extend-029 (serialize + Relock)

Mirror body-extend-028 / body-extend-027 protocol:

1. Hand-author append H_158..H_165 to `yoyo/projects/yoyo.ty` at selectors `40 A4` .. `40 AB`.
2. Promote fixtures from `_scratch_{ldb_5160_50,ldb_5260_50,set_51_cafef00d,addimm_h52_32,subimm_h51_32,set_50_cafef00d,subimm_h52_32,addimm_h50_3c}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `80287f8fe0a8eb09…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-029-log.md`.
7. Auto-spawn parallel-batch-24 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-24-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_158 | 0xA4 | 0x80 LDB | 51 60 50 | `498b87000300004883c050480fb60049898788020000c3` (23B) | `9bdf8a7966f533c0` |
| H_159 | 0xA5 | 0x80 LDB | 52 60 50 | `498b87000300004883c050480fb60049898790020000c3` (23B) | `f20e9d7238f08a4a` |
| H_160 | 0xA6 | 0x30 SET | 51 CAFEF00D | `48b80df0feca0000000049898788020000c3` (18B) | `72c89add1c031d37` |
| H_161 | 0xA7 | 0x62 ADD-IMM | 52 32 | `498b87900200004883c03249898790020000c3` (19B) | `b1a04638a88d7ace` |
| H_162 | 0xA8 | 0x61 SUB-IMM | 51 32 | `498b87880200004883e83249898788020000c3` (19B) | `207c87cf78c25007` |
| H_163 | 0xA9 | 0x30 SET | 50 CAFEF00D | `48b80df0feca0000000049898788020000c3` (18B) | `a7ecea443fabe02e` |
| H_164 | 0xAA | 0x61 SUB-IMM | 52 32 | `498b87900200004883e83249898790020000c3` (19B) | `bc35f4068daa6365` |
| H_165 | 0xAB | 0x62 ADD-IMM | 50 3C | `498b87800200004883c03c49898780020000c3` (19B) | `6e63785554e168e2` |

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_158 | `9bdf8a7966f533c04b1b85d59d61ece4ecb40b11b443a815496cea31b513b342` |
| H_159 | `f20e9d7238f08a4a1dd48ba2bc4816a51b20baaec649bac168672f09bddddd07` |
| H_160 | `72c89add1c031d3714901bfc10a48a16d7b686a5545198aa6baf392a5c4c3188` |
| H_161 | `b1a04638a88d7ace2c3bfca547038971b92f195c655c41dcd4050d424d79c35b` |
| H_162 | `207c87cf78c25007a9ff3bddafbf496fcda31ffbcbab9ec1e22edb48ed0ddb07` |
| H_163 | `a7ecea443fabe02e84bcf03b40ec9f00f5b49c83f36a1ab6113b334403b0854d` |
| H_164 | `bc35f4068daa6365186e7b9c1d89189256e614bf6278ae045392852d26ace1ba` |
| H_165 | `6e63785554e168e2b1435ec46c8609257c0b1e5eea1ae5592265bd98748ceada` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5160_50`, `_scratch_ldb_5260_50`, `_scratch_set_51_cafef00d`, `_scratch_addimm_h52_32`,
`_scratch_subimm_h51_32`, `_scratch_set_50_cafef00d`, `_scratch_subimm_h52_32`, `_scratch_addimm_h50_3c`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 A4`.. for H_158.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

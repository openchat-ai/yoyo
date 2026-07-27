# body-extend-046 SPAWN · consolidate parallel-batch-40

> Continuous queue handoff from parallel-batch-40 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `8c80a6fa783440b2ef724beb1860f295c81cde46c53f35d0cdcc40ff8798519c` (abbrev `8c80a6fa…`).
> Handlers = 300 (H_00..H_293). Last selectors: 0x124..0x12B = H_286..H_293 (`40 124`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-40-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-045-log.md` / `docs/auxdocs/body-extend-045-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-045 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 12C`.. for H_294.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-046 (serialize + Relock)

Mirror body-extend-045 / body-extend-044 protocol:

1. Hand-author append H_294..H_301 to `yoyo/projects/yoyo.ty` at selectors `40 12C` .. `40 133` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5160_c0,ldb_5260_c0,addimm_h50_a8,addimm_h51_a8,addimm_h52_a8,subimm_h50_a8,subimm_h51_a8,subimm_h52_a8}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `8c80a6fa783440b2…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-046-log.md`.
7. Auto-spawn parallel-batch-41 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-41-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_294 | 0x12C | 0x80 LDB | 51 60 C0 | `498b87000300004881c0c0000000480fb60049898788020000c3` (26B) | `452adbaebbd767ae` |
| H_295 | 0x12D | 0x80 LDB | 52 60 C0 | `498b87000300004881c0c0000000480fb60049898790020000c3` (26B) | `766e4e7e953a3e88` |
| H_296 | 0x12E | 0x62 ADD-IMM | 50 A8 | `498b87800200004881c0a800000049898780020000c3` (22B) | `6fb232e091ad8e33` |
| H_297 | 0x12F | 0x62 ADD-IMM | 51 A8 | `498b87880200004881c0a800000049898788020000c3` (22B) | `0eac0a774b9d0193` |
| H_298 | 0x130 | 0x62 ADD-IMM | 52 A8 | `498b87900200004881c0a800000049898790020000c3` (22B) | `1acbcee68dee9520` |
| H_299 | 0x131 | 0x61 SUB-IMM | 50 A8 | `498b87800200004881e8a800000049898780020000c3` (22B) | `f1d0cdaaa848cd64` |
| H_300 | 0x132 | 0x61 SUB-IMM | 51 A8 | `498b87880200004881e8a800000049898788020000c3` (22B) | `446a3deafbac2416` |
| H_301 | 0x133 | 0x61 SUB-IMM | 52 A8 | `498b87900200004881e8a800000049898790020000c3` (22B) | `254705f23c21fb17` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0xA8 uses imm32 add (`48 81 c0`) → 22B pins (H_296..H_298); not imm8.
SUB-IMM imm=0xA8 uses imm32 sub (`48 81 e8`) → 22B pins (H_299..H_301); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_294, H_295).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_294 | `452adbaebbd767aeb96d51b4ee2f91aab3a6da4d566bdecd875b2dae8e88624f` |
| H_295 | `766e4e7e953a3e88b74d40e47edb12de908d14326001e1f95ec1eb8ca413174f` |
| H_296 | `6fb232e091ad8e3345bf7f59cc06614258634daa24a17fb196b6c5bd3187d1e5` |
| H_297 | `0eac0a774b9d01933218fb2e52b3493e302865857756c8d49ec0005d2728bb65` |
| H_298 | `1acbcee68dee9520a277ffc40faddcee9de1599e4ff9232c115a38c8ac45b06f` |
| H_299 | `f1d0cdaaa848cd641cecf15959fa70b1421ea7b141fee07751570ab1c4604152` |
| H_300 | `446a3deafbac2416e875e75fe8792542ee68cb12d88eb6a428474692e6ef20bd` |
| H_301 | `254705f23c21fb17cc3b4a40523f0888fb075124c8c861d7f9615d21d6e78e23` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5160_c0`, `_scratch_ldb_5260_c0`, `_scratch_addimm_h50_a8`, `_scratch_addimm_h51_a8`,
`_scratch_addimm_h52_a8`, `_scratch_subimm_h50_a8`, `_scratch_subimm_h51_a8`, `_scratch_subimm_h52_a8`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 12C`.. for H_294.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

# body-extend-085 SPAWN · consolidate parallel-batch-79

> Continuous queue handoff from parallel-batch-79 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `9eafc9ce0376d389043b0e77ec2c1ff2bc44dda11b4fb8f6449cc4ea811798ac` (abbrev `9eafc9ce…`).
> Handlers = 611 (H_00..H_604). Last selectors: 0x25B..0x262 = H_597..H_604 (`40 25B`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-79-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-084-log.md` / `docs/auxdocs/body-extend-084-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-084 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 263`.. for H_605.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 263`/`40 264` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-085 (serialize + Relock)

Mirror body-extend-084 / body-extend-083 protocol:

1. Hand-author append H_605..H_612 to `yoyo/projects/yoyo.ty` at selectors `40 263` .. `40 26A` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h50_1C0,subimm_h51_1C0,subimm_h52_1C0,ldb_5060_1C8,ldb_5160_1C8,ldb_5260_1C8,addimm_h50_1C8,addimm_h51_1C8}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `9eafc9ce0376d389043b0e77ec2c1ff2bc44dda11b4fb8f6449cc4ea811798ac`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-085-log.md`.
7. Auto-spawn parallel-batch-80 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-80-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_605 | 0x263 | 0x61 SUB-IMM | 50 1C0 | `498b87800200004881e8c001000049898780020000c3` (22B) | `2dd291d1df0ff186` |
| H_606 | 0x264 | 0x61 SUB-IMM | 51 1C0 | `498b87880200004881e8c001000049898788020000c3` (22B) | `162f63e6a4ed8641` |
| H_607 | 0x265 | 0x61 SUB-IMM | 52 1C0 | `498b87900200004881e8c001000049898790020000c3` (22B) | `649c06ddcb80956d` |
| H_608 | 0x266 | 0x80 LDB | 50 60 1C8 | `498b87000300004881c0c8010000480fb60049898780020000c3` (26B) | `b299fd62cea22ef7` |
| H_609 | 0x267 | 0x80 LDB | 51 60 1C8 | `498b87000300004881c0c8010000480fb60049898788020000c3` (26B) | `18e61721bdda72c3` |
| H_610 | 0x268 | 0x80 LDB | 52 60 1C8 | `498b87000300004881c0c8010000480fb60049898790020000c3` (26B) | `9612ef36d64f34eb` |
| H_611 | 0x269 | 0x62 ADD-IMM | 50 1C8 | `498b87800200004881c0c801000049898780020000c3` (22B) | `435f20ebb01bbc21` |
| H_612 | 0x26A | 0x62 ADD-IMM | 51 1C8 | `498b87880200004881c0c801000049898788020000c3` (22B) | `d6e88e4f8c96211e` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
SUB-IMM imm=0x1C0 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1C8 uses imm32 add (`48 81 c0`) → 26B pins.
ADD-IMM imm=0x1C8 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM slot=50/51/52 imm=1C0 starts deferred 1C0 SUB triad (H_605/H_606/H_607).
LDB oo=0x1C8 starts 1C8 LDB triad (H_608/H_609/H_610 dd=50/51/52).
ADD-IMM slot=50/51 imm=1C8 starts deferred 1C8 ADD triad (H_611/H_612; slot=52 deferred).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_605 | `2dd291d1df0ff186d43b5edce6e88baac2de7cdfe15cbf3067278a79a62ce4ac` |
| H_606 | `162f63e6a4ed8641cbcc38e6bb5c8bbbfaca6370421c764393cab71092041e2a` |
| H_607 | `649c06ddcb80956d738bd791477f5e28f3d562d44ae564f9ebe68323ea677074` |
| H_608 | `b299fd62cea22ef7655b78fa27f156de19e0b5359ff7616f7e6dd739060ca0c9` |
| H_609 | `18e61721bdda72c38fbeac19fbe27d118aa52b762f7908a80ccf1dc295ad0549` |
| H_610 | `9612ef36d64f34ebefdf68541995941ac8d39b3cd84816f3da60e3955d94ae33` |
| H_611 | `435f20ebb01bbc215ee27da9f187dece6dce5783705b4241c548b9be6deccecc` |
| H_612 | `d6e88e4f8c96211e39163d7f39a8e213245a69e2c1dccfbc34e708ea1d6c6639` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h50_1C0`, `_scratch_subimm_h51_1C0`, `_scratch_subimm_h52_1C0`, `_scratch_ldb_5060_1C8`,
`_scratch_ldb_5160_1C8`, `_scratch_ldb_5260_1C8`, `_scratch_addimm_h50_1C8`, `_scratch_addimm_h51_1C8`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 263`.. for H_605.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-80

- ADD-IMM slot=52 imm=1C8 (finish 1C8 ADD triad)
- SUB-IMM slot=50/51/52 imm=1C8 (start 1C8 SUB triad)
- LDB / ADD-IMM / SUB-IMM next imm ladder (1D0…) if continuing
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked

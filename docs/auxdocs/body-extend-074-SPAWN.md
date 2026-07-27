# body-extend-074 SPAWN · consolidate parallel-batch-68

> Continuous queue handoff from parallel-batch-68 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `1a6cb44aa28367d25d6727eec5206e5895c3c948be080a60dcadb7d853bc8bac` (abbrev `1a6cb44a…`).
> Handlers = 523 (H_00..H_516). Last selectors: 0x203..0x20A = H_509..H_516 (`40 203`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-68-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-073-log.md` / `docs/auxdocs/body-extend-073-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-073 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 20B`.. for H_517.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 20B`/`40 20C` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-074 (serialize + Relock)

Mirror body-extend-073 / body-extend-072 protocol:

1. Hand-author append H_517..H_524 to `yoyo/projects/yoyo.ty` at selectors `40 20B` .. `40 212` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h52_170,ldb_5060_178,ldb_5160_178,ldb_5260_178,addimm_h50_178,addimm_h51_178,addimm_h52_178,subimm_h50_178}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `1a6cb44aa28367d25d6727eec5206e5895c3c948be080a60dcadb7d853bc8bac`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-074-log.md`.
7. Auto-spawn parallel-batch-69 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-69-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_517 | 0x20B | 0x61 SUB-IMM | 52 170 | `498b87900200004881e87001000049898790020000c3` (22B) | `dad788fae9b7dc6d` |
| H_518 | 0x20C | 0x80 LDB | 50 60 178 | `498b87000300004881c078010000480fb60049898780020000c3` (26B) | `88e184b59a6db03c` |
| H_519 | 0x20D | 0x80 LDB | 51 60 178 | `498b87000300004881c078010000480fb60049898788020000c3` (26B) | `9ed7c675af239145` |
| H_520 | 0x20E | 0x80 LDB | 52 60 178 | `498b87000300004881c078010000480fb60049898790020000c3` (26B) | `acf695cec1340844` |
| H_521 | 0x20F | 0x62 ADD-IMM | 50 178 | `498b87800200004881c07801000049898780020000c3` (22B) | `90d4b604f3d3217f` |
| H_522 | 0x210 | 0x62 ADD-IMM | 51 178 | `498b87880200004881c07801000049898788020000c3` (22B) | `ef600aa63170300a` |
| H_523 | 0x211 | 0x62 ADD-IMM | 52 178 | `498b87900200004881c07801000049898790020000c3` (22B) | `720aa67f69ef0ab9` |
| H_524 | 0x212 | 0x61 SUB-IMM | 50 178 | `498b87800200004881e87801000049898780020000c3` (22B) | `7f477a27dd9d8bb9` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x178 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x170/0x178 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x178 starts 178 LDB triad (H_518 dd=50, H_519 dd=51, H_520 dd=52).
SUB-IMM slot=52 imm=170 finishes deferred 170 SUB triad (H_517).
SUB-IMM slot=51/52 imm=178 deferred to a later scratch batch.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_517 | `dad788fae9b7dc6dafc5ea335eca5824067264a818bad9ee6d5ffb8c9e8d42bf` |
| H_518 | `88e184b59a6db03c16e09aa71334849160fa599f85d16c5b3283e4d8c4c55b64` |
| H_519 | `9ed7c675af2391459a5915d0ac92bd5e3bd2853636b301dba220141937a8bffb` |
| H_520 | `acf695cec13408443b9a7b595578b18affe93081974efceb3835395bc576eca3` |
| H_521 | `90d4b604f3d3217f47485394b669a7b4cec7a67a74da24c4c771bf7cfd5f3df3` |
| H_522 | `ef600aa63170300aaa59c1bbd33286d9c16e5fd4ec6ee4d92fbbf96f46666345` |
| H_523 | `720aa67f69ef0ab902f2b09e87034db0ab374ea25740647882fb61a376db44df` |
| H_524 | `7f477a27dd9d8bb9fee4694d75cb1f273b1f8d7da249471070d19e73ed003989` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h52_170`, `_scratch_ldb_5060_178`, `_scratch_ldb_5160_178`, `_scratch_ldb_5260_178`,
`_scratch_addimm_h50_178`, `_scratch_addimm_h51_178`, `_scratch_addimm_h52_178`, `_scratch_subimm_h50_178`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 20B`.. for H_517.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

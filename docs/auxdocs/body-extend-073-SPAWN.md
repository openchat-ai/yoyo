# body-extend-073 SPAWN · consolidate parallel-batch-67

> Continuous queue handoff from parallel-batch-67 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `e1554db8dcce9946348a88383bed73939d4a835e8dc0989a2788a72a590e6a6b` (abbrev `e1554db8…`).
> Handlers = 515 (H_00..H_508). Last selectors: 0x1FB..0x202 = H_501..H_508 (`40 1FB`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-67-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-072-log.md` / `docs/auxdocs/body-extend-072-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-072 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 203`.. for H_509.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 203`/`40 204` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-073 (serialize + Relock)

Mirror body-extend-072 / body-extend-071 protocol:

1. Hand-author append H_509..H_516 to `yoyo/projects/yoyo.ty` at selectors `40 203` .. `40 20A` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5060_170,ldb_5160_170,ldb_5260_170,addimm_h50_170,addimm_h51_170,addimm_h52_170,subimm_h50_170,subimm_h51_170}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `e1554db8dcce9946348a88383bed73939d4a835e8dc0989a2788a72a590e6a6b`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-073-log.md`.
7. Auto-spawn parallel-batch-68 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-68-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_509 | 0x203 | 0x80 LDB | 50 60 170 | `498b87000300004881c070010000480fb60049898780020000c3` (26B) | `2880271f9ceddc44` |
| H_510 | 0x204 | 0x80 LDB | 51 60 170 | `498b87000300004881c070010000480fb60049898788020000c3` (26B) | `f5ea323500e5fb12` |
| H_511 | 0x205 | 0x80 LDB | 52 60 170 | `498b87000300004881c070010000480fb60049898790020000c3` (26B) | `ee43e15d67b15204` |
| H_512 | 0x206 | 0x62 ADD-IMM | 50 170 | `498b87800200004881c07001000049898780020000c3` (22B) | `b5ced24e14fef8f3` |
| H_513 | 0x207 | 0x62 ADD-IMM | 51 170 | `498b87880200004881c07001000049898788020000c3` (22B) | `2bb85897a4abc0cf` |
| H_514 | 0x208 | 0x62 ADD-IMM | 52 170 | `498b87900200004881c07001000049898790020000c3` (22B) | `ccca022a923acf93` |
| H_515 | 0x209 | 0x61 SUB-IMM | 50 170 | `498b87800200004881e87001000049898780020000c3` (22B) | `b78b97ec483ce762` |
| H_516 | 0x20A | 0x61 SUB-IMM | 51 170 | `498b87880200004881e87001000049898788020000c3` (22B) | `f6d1c92bf87d13e8` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x170 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x170 uses imm32 sub (`48 81 e8`) → 22B pins (H_515..H_516); not imm8.
LDB oo=0x170 starts 170 LDB triad (H_509 dd=50, H_510 dd=51, H_511 dd=52).
SUB-IMM slot=52 imm=170 deferred to a later scratch batch.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_509 | `2880271f9ceddc44129316182318f0a4cc57d86bad178d197cb3eb3250a3616e` |
| H_510 | `f5ea323500e5fb12f22e4a01c43ed6eeb1a9ce1e15e16f01804cc82e0cdd37f6` |
| H_511 | `ee43e15d67b15204d6835a85cefc3b30c0c5cc7494d5c5a573e7493c9fad9b18` |
| H_512 | `b5ced24e14fef8f3a3633aa4a77bd3222947c43a46960a244461d1821c38a1c5` |
| H_513 | `2bb85897a4abc0cfd1acafb79b5f84e44b0ef053a80135d5af7f40a99ef5e6fe` |
| H_514 | `ccca022a923acf937590b39f321903fbb402924e8e44e71c35730d61a7acb504` |
| H_515 | `b78b97ec483ce762da832656574acda2516310f7db63f49891579b23450f0dcb` |
| H_516 | `f6d1c92bf87d13e82c11b94d812389f500c95906d3d1c57ff86b30648157a225` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5060_170`, `_scratch_ldb_5160_170`, `_scratch_ldb_5260_170`, `_scratch_addimm_h50_170`,
`_scratch_addimm_h51_170`, `_scratch_addimm_h52_170`, `_scratch_subimm_h50_170`, `_scratch_subimm_h51_170`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 203`.. for H_509.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

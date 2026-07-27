# body-extend-082 SPAWN · consolidate parallel-batch-76

> Continuous queue handoff from parallel-batch-76 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `267c611dbb648db15251e6e6ee8a52287434680892e9f2ad290fd161eb2b916c` (abbrev `267c611d…`).
> Handlers = 587 (H_00..H_580). Last selectors: 0x243..0x24A = H_573..H_580 (`40 243`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-76-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-081-log.md` / `docs/auxdocs/body-extend-081-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-081 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 24B`.. for H_581.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 24B`/`40 24C` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-082 (serialize + Relock)

Mirror body-extend-081 / body-extend-080 protocol:

1. Hand-author append H_581..H_588 to `yoyo/projects/yoyo.ty` at selectors `40 24B` .. `40 252` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5060_1B0,ldb_5160_1B0,ldb_5260_1B0,addimm_h50_1B0,addimm_h51_1B0,addimm_h52_1B0,subimm_h50_1B0,subimm_h51_1B0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `267c611dbb648db15251e6e6ee8a52287434680892e9f2ad290fd161eb2b916c`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-082-log.md`.
7. Auto-spawn parallel-batch-77 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-77-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_581 | 0x24B | 0x80 LDB | 50 60 1B0 | `498b87000300004881c0b0010000480fb60049898780020000c3` (26B) | `4a28b7afe67cd9c8` |
| H_582 | 0x24C | 0x80 LDB | 51 60 1B0 | `498b87000300004881c0b0010000480fb60049898788020000c3` (26B) | `bbbb35dd922e35f8` |
| H_583 | 0x24D | 0x80 LDB | 52 60 1B0 | `498b87000300004881c0b0010000480fb60049898790020000c3` (26B) | `114e9beed1fbb101` |
| H_584 | 0x24E | 0x62 ADD-IMM | 50 1B0 | `498b87800200004881c0b001000049898780020000c3` (22B) | `449e70ae9ce9bc48` |
| H_585 | 0x24F | 0x62 ADD-IMM | 51 1B0 | `498b87880200004881c0b001000049898788020000c3` (22B) | `19cf91fa4836bb0d` |
| H_586 | 0x250 | 0x62 ADD-IMM | 52 1B0 | `498b87900200004881c0b001000049898790020000c3` (22B) | `e719980bb34c73f8` |
| H_587 | 0x251 | 0x61 SUB-IMM | 50 1B0 | `498b87800200004881e8b001000049898780020000c3` (22B) | `fde148880489e4d4` |
| H_588 | 0x252 | 0x61 SUB-IMM | 51 1B0 | `498b87880200004881e8b001000049898788020000c3` (22B) | `2f842240d885a210` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x1B0 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x1B0 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1B0 starts 1B0 LDB triad (H_581/H_582/H_583 dd=50/51/52).
ADD-IMM slot=50/51/52 imm=1B0 starts deferred 1B0 ADD triad (H_584/H_585/H_586).
SUB-IMM slot=50/51 imm=1B0 starts deferred 1B0 SUB triad (H_587/H_588; slot=52 deferred).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_581 | `4a28b7afe67cd9c840ea5ac4136ffbb9dc07e089b32e15b301e6ab29b66cb172` |
| H_582 | `bbbb35dd922e35f80a9b091c9ab3ae835e62e3e2a5a7e3d052e1e0f595c32886` |
| H_583 | `114e9beed1fbb101f01c8ac6fee6a766ac4ec89c366141996c5b3f68d9bd99ab` |
| H_584 | `449e70ae9ce9bc487dd3080a33c3bcefcebc8a74bfaa15a365c2810940e1aa82` |
| H_585 | `19cf91fa4836bb0d24c5b5b86de66eafe96102070d286056353404ec01fdc5c2` |
| H_586 | `e719980bb34c73f833082099bf01e553d22845cce6e56d4e86a5f87c40f903a9` |
| H_587 | `fde148880489e4d43f5120baa475d665d7fbf50b433516e9bf0737020ed998c5` |
| H_588 | `2f842240d885a21044d7a16b541f11024e764239d3ffd51ecd89583de2844da4` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5060_1B0`, `_scratch_ldb_5160_1B0`, `_scratch_ldb_5260_1B0`, `_scratch_addimm_h50_1B0`,
`_scratch_addimm_h51_1B0`, `_scratch_addimm_h52_1B0`, `_scratch_subimm_h50_1B0`, `_scratch_subimm_h51_1B0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 24B`.. for H_581.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-77

- SUB-IMM slot=52 imm=1B0 (finish 1B0 SUB triad)
- LDB 50/51/52 60 1B8 (start 1B8 LDB triad)
- ADD-IMM/SUB-IMM slot=50/51/52 imm=1B8 (start 1B8 ADD/SUB triads)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked

# body-extend-081 SPAWN · consolidate parallel-batch-75

> Continuous queue handoff from parallel-batch-75 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `e255cd93a26ec455cc4def0ceb38c1cfc93bcb1ec7476f9e57ecd062d1be065a` (abbrev `e255cd93…`).
> Handlers = 579 (H_00..H_572). Last selectors: 0x23B..0x242 = H_565..H_572 (`40 23B`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-75-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-080-log.md` / `docs/auxdocs/body-extend-080-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-080 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 243`.. for H_573.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 243`/`40 244` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-081 (serialize + Relock)

Mirror body-extend-080 / body-extend-079 protocol:

1. Hand-author append H_573..H_580 to `yoyo/projects/yoyo.ty` at selectors `40 243` .. `40 24A` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5160_1A8,ldb_5260_1A8,addimm_h50_1A8,addimm_h51_1A8,addimm_h52_1A8,subimm_h50_1A8,subimm_h51_1A8,subimm_h52_1A8}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `e255cd93a26ec455cc4def0ceb38c1cfc93bcb1ec7476f9e57ecd062d1be065a`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-081-log.md`.
7. Auto-spawn parallel-batch-76 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-76-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_573 | 0x243 | 0x80 LDB | 51 60 1A8 | `498b87000300004881c0a8010000480fb60049898788020000c3` (26B) | `fbea55b03005c5a5` |
| H_574 | 0x244 | 0x80 LDB | 52 60 1A8 | `498b87000300004881c0a8010000480fb60049898790020000c3` (26B) | `7db0bd86b3e802a1` |
| H_575 | 0x245 | 0x62 ADD-IMM | 50 1A8 | `498b87800200004881c0a801000049898780020000c3` (22B) | `5a3272ce14feca9a` |
| H_576 | 0x246 | 0x62 ADD-IMM | 51 1A8 | `498b87880200004881c0a801000049898788020000c3` (22B) | `6aecaccb918f42df` |
| H_577 | 0x247 | 0x62 ADD-IMM | 52 1A8 | `498b87900200004881c0a801000049898790020000c3` (22B) | `f2ea24f19b1f387c` |
| H_578 | 0x248 | 0x61 SUB-IMM | 50 1A8 | `498b87800200004881e8a801000049898780020000c3` (22B) | `2a655dd4d2adee0c` |
| H_579 | 0x249 | 0x61 SUB-IMM | 51 1A8 | `498b87880200004881e8a801000049898788020000c3` (22B) | `44c2fed0d54d8b28` |
| H_580 | 0x24A | 0x61 SUB-IMM | 52 1A8 | `498b87900200004881e8a801000049898790020000c3` (22B) | `2c5130704cf19491` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x1A8 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x1A8 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1A8 finishes 1A8 LDB triad (H_573/H_574 dd=51/52; H_572 already locked dd=50).
ADD-IMM slot=50/51/52 imm=1A8 starts deferred 1A8 ADD triad (H_575/H_576/H_577).
SUB-IMM slot=50/51/52 imm=1A8 starts deferred 1A8 SUB triad (H_578/H_579/H_580).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_573 | `fbea55b03005c5a5e5f040c713fda4d857774e5abe360051f5996da6e30bfd82` |
| H_574 | `7db0bd86b3e802a19bff232db71fde4967ec75a1969973f1b9841101faa934be` |
| H_575 | `5a3272ce14feca9acc6662aa72a89d53c9afabc861ba1a14f92752976384a6a8` |
| H_576 | `6aecaccb918f42dfee3967683d0e1b30b8740a0d4a14a936c518a5fcd91cafc9` |
| H_577 | `f2ea24f19b1f387c5b5d415f109609bc51a9f3f1982a083fa4e3206a60ea9483` |
| H_578 | `2a655dd4d2adee0c5ec4c24070e01fdd7668bc56fb85738917c511dc703ead4b` |
| H_579 | `44c2fed0d54d8b28b537faff3f8aa11ef7ea078c50930a5bf143336291a4a767` |
| H_580 | `2c5130704cf19491ce73bb03a34a507ef0c97d712f177935760f956f92bcd6d8` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5160_1A8`, `_scratch_ldb_5260_1A8`, `_scratch_addimm_h50_1A8`, `_scratch_addimm_h51_1A8`,
`_scratch_addimm_h52_1A8`, `_scratch_subimm_h50_1A8`, `_scratch_subimm_h51_1A8`, `_scratch_subimm_h52_1A8`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 243`.. for H_573.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-76

- LDB 50/51/52 60 1B0 (start 1B0 LDB triad)
- ADD-IMM/SUB-IMM slot=50/51/52 imm=1B0 (start 1B0 ADD/SUB triads)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked

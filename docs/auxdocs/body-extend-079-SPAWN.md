# body-extend-079 SPAWN · consolidate parallel-batch-73

> Continuous queue handoff from parallel-batch-73 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `4c42576df4f80a8d3f4e57074fb4fc081bc16d37c9638b9fd0659ddae86fd42b` (abbrev `4c42576d…`).
> Handlers = 563 (H_00..H_556). Last selectors: 0x22B..0x232 = H_549..H_556 (`40 22B`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-73-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-078-log.md` / `docs/auxdocs/body-extend-078-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-078 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 233`.. for H_557.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 233`/`40 234` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-079 (serialize + Relock)

Mirror body-extend-078 / body-extend-077 protocol:

1. Hand-author append H_557..H_564 to `yoyo/projects/yoyo.ty` at selectors `40 233` .. `40 23A` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h50_198,addimm_h51_198,addimm_h52_198,subimm_h50_198,subimm_h51_198,subimm_h52_198,ldb_5060_1A0,ldb_5160_1A0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `4c42576df4f80a8d3f4e57074fb4fc081bc16d37c9638b9fd0659ddae86fd42b`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-079-log.md`.
7. Auto-spawn parallel-batch-74 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-74-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_557 | 0x233 | 0x62 ADD-IMM | 50 198 | `498b87800200004881c09801000049898780020000c3` (22B) | `b9a1454084d99711` |
| H_558 | 0x234 | 0x62 ADD-IMM | 51 198 | `498b87880200004881c09801000049898788020000c3` (22B) | `6dfea21cc077f979` |
| H_559 | 0x235 | 0x62 ADD-IMM | 52 198 | `498b87900200004881c09801000049898790020000c3` (22B) | `b4bced2f75175884` |
| H_560 | 0x236 | 0x61 SUB-IMM | 50 198 | `498b87800200004881e89801000049898780020000c3` (22B) | `7dca7636d1845a95` |
| H_561 | 0x237 | 0x61 SUB-IMM | 51 198 | `498b87880200004881e89801000049898788020000c3` (22B) | `5b1facdbbae86c25` |
| H_562 | 0x238 | 0x61 SUB-IMM | 52 198 | `498b87900200004881e89801000049898790020000c3` (22B) | `3b46829def05556b` |
| H_563 | 0x239 | 0x80 LDB | 50 60 1A0 | `498b87000300004881c0a0010000480fb60049898780020000c3` (26B) | `bcf7781865161f65` |
| H_564 | 0x23A | 0x80 LDB | 51 60 1A0 | `498b87000300004881c0a0010000480fb60049898788020000c3` (26B) | `55cd34d122a07524` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x198 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x198 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1A0 starts 1A0 LDB triad (H_563 dd=50, H_564 dd=51; leave dd=52 deferred for batch-74).
ADD-IMM slot=50/51/52 imm=198 starts deferred 198 ADD triad (H_557/H_558/H_559).
SUB-IMM slot=50/51/52 imm=198 starts deferred 198 SUB triad (H_560/H_561/H_562).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_557 | `b9a1454084d99711e863c60d89ef428ac9c1000ac459788d80ae9b81237e9a8d` |
| H_558 | `6dfea21cc077f97993970fba0b515ea60b4a8dd2713f7742970c1846130b496c` |
| H_559 | `b4bced2f75175884c281ebe07167b1c07f4c22ed228efd262cf82c72003aace7` |
| H_560 | `7dca7636d1845a95121362d02954825a9ac14150259681115527e9b451cd7b69` |
| H_561 | `5b1facdbbae86c25f64f38daf14816bb07bc7424cf3996f41a5f0b02baf132e4` |
| H_562 | `3b46829def05556bb9655db0f7ca21419879296a60f941c45f8f476e9e6b8cfc` |
| H_563 | `bcf7781865161f6502cc7b701f7de3ee7650446a480df69236b456e2cbe63530` |
| H_564 | `55cd34d122a075243914eb017cd595cb0a6c5281fb1c2d2ff2e6f816d0057416` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h50_198`, `_scratch_addimm_h51_198`, `_scratch_addimm_h52_198`, `_scratch_subimm_h50_198`,
`_scratch_subimm_h51_198`, `_scratch_subimm_h52_198`, `_scratch_ldb_5060_1A0`, `_scratch_ldb_5160_1A0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 233`.. for H_557.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-74

- LDB 52 60 1A0 (finish 1A0 LDB triad)
- ADD-IMM/SUB-IMM slot=50/51/52 imm=1A0 (start 1A0 ADD/SUB triads)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked

# body-extend-093 SPAWN · consolidate parallel-batch-87

> Continuous queue handoff from parallel-batch-87 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `1991af8484d67ec19980bf14771d523d332f85c9974e1da09d45496baf46ebb5` (abbrev `1991af84…`).
> Handlers = 675 (H_00..H_668). Last selectors: 0x29B..0x2A2 = H_661..H_668 (`40 29B`..`40 2A2` via label-width A).
> Source: `docs/auxdocs/parallel-batch-87-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-092-log.md` / `docs/auxdocs/body-extend-092-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-092 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 2A3`.. for H_669.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 2A3`/`40 2AA` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Task: body-extend-093 (serialize + Relock)

Mirror body-extend-092 / body-extend-091 protocol:

1. Hand-author append H_669..H_676 to `yoyo/projects/yoyo.ty` at selectors `40 2A3` .. `40 2AA` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h51_1F8,subimm_h52_1F8,ldb_5060_200,ldb_5160_200,ldb_5260_200,addimm_h50_200,addimm_h51_200,addimm_h52_200}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `1991af8484d67ec19980bf14771d523d332f85c9974e1da09d45496baf46ebb5`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-093-log.md`.
7. Auto-spawn parallel-batch-88 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-88-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_669 | 0x2A3 | 0x61 SUB-IMM | 51 1F8 | `498b87880200004881e8f801000049898788020000c3` (22B) | `cd8053ace6652cd9` |
| H_670 | 0x2A4 | 0x61 SUB-IMM | 52 1F8 | `498b87900200004881e8f801000049898790020000c3` (22B) | `512b7b4c08728ca7` |
| H_671 | 0x2A5 | 0x80 LDB | 50 60 200 | `498b87000300004881c000020000480fb60049898780020000c3` (26B) | `8ef97152f880c8bf` |
| H_672 | 0x2A6 | 0x80 LDB | 51 60 200 | `498b87000300004881c000020000480fb60049898788020000c3` (26B) | `ae88f23839b7ed37` |
| H_673 | 0x2A7 | 0x80 LDB | 52 60 200 | `498b87000300004881c000020000480fb60049898790020000c3` (26B) | `623de62f88220d56` |
| H_674 | 0x2A8 | 0x62 ADD-IMM | 50 200 | `498b87800200004881c00002000049898780020000c3` (22B) | `cba55979366f2bab` |
| H_675 | 0x2A9 | 0x62 ADD-IMM | 51 200 | `498b87880200004881c00002000049898788020000c3` (22B) | `d48330be708021e4` |
| H_676 | 0x2AA | 0x62 ADD-IMM | 52 200 | `498b87900200004881c00002000049898790020000c3` (22B) | `563af54479f67bd3` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x200 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x1F8 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x200 uses imm32 add (`48 81 c0`) → 26B pins.
SUB-IMM slot=51/52 imm=1F8 finishes deferred 1F8 SUB triad (H_669/H_670).
LDB dd=50/51/52 ss=60 oo=200 starts 200 LDB triad (H_671/H_672/H_673).
ADD-IMM slot=50/51/52 imm=200 starts 200 ADD triad (H_674/H_675/H_676).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_669 | `cd8053ace6652cd9f67f1021123bdc5222b2a7a5e4dec6586639ae6d3bbb95d4` |
| H_670 | `512b7b4c08728ca7793738789ea4129b0a3982166e627307e6ad89d2b009f471` |
| H_671 | `8ef97152f880c8bf58ccff2b1e71f0d5607d5659d0fb45acda46d9f3dfc13490` |
| H_672 | `ae88f23839b7ed37a87ae8ca78f67b76185a4b74f88574f6a84689f8d40bea2c` |
| H_673 | `623de62f88220d56cb2e73f2807b3a7503641858552eb732a5e5cef420bd803d` |
| H_674 | `cba55979366f2bab0d63b00bca48823c7d7b80965c4c2f5ac4691cd11977ea07` |
| H_675 | `d48330be708021e46e3d010b4b35804437f8e844842cf428ac1bf7fa6f7f5348` |
| H_676 | `563af54479f67bd3329f812c6f699999bbfb34097e119ae3ab3f39516c6a022f` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h51_1F8`, `_scratch_subimm_h52_1F8`, `_scratch_ldb_5060_200`, `_scratch_ldb_5160_200`,
`_scratch_ldb_5260_200`, `_scratch_addimm_h50_200`, `_scratch_addimm_h51_200`, `_scratch_addimm_h52_200`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 2A3`.. for H_669.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-88

- SUB-IMM slot=50/51/52 imm=200 (start/finish 200 SUB triad)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked

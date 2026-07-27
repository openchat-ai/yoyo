# body-extend-096 SPAWN · consolidate parallel-batch-90

> Continuous queue handoff from parallel-batch-90 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `aef6d89f98ceb7c8d9770950da9a584d7165f7e0d6713fc30c1d3f14c92552ee` (abbrev `aef6d89f…`).
> Handlers = 699 (H_00..H_692). Last selectors: 0x2B3..0x2BA = H_685..H_692 (`40 2B3`..`40 2BA` via label-width A).
> Source: `docs/auxdocs/parallel-batch-90-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-095-log.md` / `docs/auxdocs/body-extend-095-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-095 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 2BB`.. for H_693.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 2BB`/`40 2C2` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Task: body-extend-096 (serialize + Relock)

Mirror body-extend-095 / body-extend-094 protocol:

1. Hand-author append H_693..H_700 to `yoyo/projects/yoyo.ty` at selectors `40 2BB` .. `40 2C2` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h51_210,addimm_h52_210,subimm_h50_210,subimm_h51_210,subimm_h52_210,ldb_5060_218,ldb_5160_218,ldb_5260_218}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `aef6d89f98ceb7c8d9770950da9a584d7165f7e0d6713fc30c1d3f14c92552ee`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-096-log.md`.
7. Auto-spawn parallel-batch-91 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-91-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_693 | 0x2BB | 0x62 ADD-IMM | 51 210 | `498b87880200004881c01002000049898788020000c3` (22B) | `f59a9a17f02eae7c` |
| H_694 | 0x2BC | 0x62 ADD-IMM | 52 210 | `498b87900200004881c01002000049898790020000c3` (22B) | `b6abb627bf849fc0` |
| H_695 | 0x2BD | 0x61 SUB-IMM | 50 210 | `498b87800200004881e81002000049898780020000c3` (22B) | `f77af100f9fabd84` |
| H_696 | 0x2BE | 0x61 SUB-IMM | 51 210 | `498b87880200004881e81002000049898788020000c3` (22B) | `dbfd9ece27cb16d9` |
| H_697 | 0x2BF | 0x61 SUB-IMM | 52 210 | `498b87900200004881e81002000049898790020000c3` (22B) | `b4bcf1859605c71c` |
| H_698 | 0x2C0 | 0x80 LDB | 50 60 218 | `498b87000300004881c018020000480fb60049898780020000c3` (26B) | `c6cb4e7e1fac02c9` |
| H_699 | 0x2C1 | 0x80 LDB | 51 60 218 | `498b87000300004881c018020000480fb60049898788020000c3` (26B) | `6296837a29daedeb` |
| H_700 | 0x2C2 | 0x80 LDB | 52 60 218 | `498b87000300004881c018020000480fb60049898790020000c3` (26B) | `8e68e69170dde74d` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x210 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x210 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x218 uses imm32 add (`48 81 c0`) → 26B pins.
ADD-IMM slot=51/52 imm=210 finishes deferred 210 ADD triad (H_693/H_694).
SUB-IMM slot=50/51/52 imm=210 starts 210 SUB triad (H_695/H_696/H_697).
LDB dd=50/51/52 ss=60 oo=218 starts 218 LDB triad (H_698/H_699/H_700).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_693 | `f59a9a17f02eae7c09283f04ad040634e7fda687dda507038344fa9cc758be6c` |
| H_694 | `b6abb627bf849fc0a9cebf0c9b09d36e0ca9c67bf705287c2bfc5e25301c690a` |
| H_695 | `f77af100f9fabd84ef73e82bfcfed4011049214dccf6d64fc50a8931a9015fa2` |
| H_696 | `dbfd9ece27cb16d9e60e5a74e1a1bfac06a2ed48396cb7fddae9db85ef6576e8` |
| H_697 | `b4bcf1859605c71c1618d398a81d2e3a1fd0f0d47298a2a62af9164f9f7080f9` |
| H_698 | `c6cb4e7e1fac02c9cea83b983dc954c4f3066cd8a67a026fa8c2b35e92aea8a3` |
| H_699 | `6296837a29daedeba1df94ff6f0c6173e11264bc97593e06246e6cc71544234b` |
| H_700 | `8e68e69170dde74dc3221b9ba81b23012c1fd0d5957b564ae1ad73489451dc85` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h51_210`, `_scratch_addimm_h52_210`, `_scratch_subimm_h50_210`, `_scratch_subimm_h51_210`,
`_scratch_subimm_h52_210`, `_scratch_ldb_5060_218`, `_scratch_ldb_5160_218`, `_scratch_ldb_5260_218`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 2BB`.. for H_693.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-91

- ADD-IMM slot=50/51/52 imm=218 (start 218 ADD triad)
- SUB-IMM slot=50/51/52 imm=218 (start 218 SUB triad)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked

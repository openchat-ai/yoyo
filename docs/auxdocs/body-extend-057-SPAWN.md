# body-extend-057 SPAWN · consolidate parallel-batch-51

> Continuous queue handoff from parallel-batch-51 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `824207c608fe5d03e4bd1c3bca1f33aec844dd62f4bc66ca4a6877364538314b` (abbrev `824207c6…`).
> Handlers = 388 (H_00..H_381). Last selectors: 0x17C..0x183 = H_374..H_381 (`40 17C`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-51-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-056-log.md` / `docs/auxdocs/body-extend-056-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-056 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 184`.. for H_382.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 184`/`40 185` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-057 (serialize + Relock)

Mirror body-extend-056 / body-extend-055 protocol:

1. Hand-author append H_382..H_389 to `yoyo/projects/yoyo.ty` at selectors `40 184` .. `40 18B` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h52_f8,ldb_5060_100,ldb_5160_100,ldb_5260_100,addimm_h50_100,addimm_h51_100,addimm_h52_100,subimm_h50_100}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `824207c608fe5d03…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-057-log.md`.
7. Auto-spawn parallel-batch-52 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-52-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_382 | 0x184 | 0x61 SUB-IMM | 52 F8 | `498b87900200004881e8f800000049898790020000c3` (22B) | `69b7068d45f8bf5d` |
| H_383 | 0x185 | 0x80 LDB | 50 60 100 | `498b87000300004881c000010000480fb60049898780020000c3` (26B) | `435a012fe7d4460d` |
| H_384 | 0x186 | 0x80 LDB | 51 60 100 | `498b87000300004881c000010000480fb60049898788020000c3` (26B) | `efcb4fa1a01828f3` |
| H_385 | 0x187 | 0x80 LDB | 52 60 100 | `498b87000300004881c000010000480fb60049898790020000c3` (26B) | `a26708edf890025c` |
| H_386 | 0x188 | 0x62 ADD-IMM | 50 100 | `498b87800200004881c00001000049898780020000c3` (22B) | `220b570f6901c757` |
| H_387 | 0x189 | 0x62 ADD-IMM | 51 100 | `498b87880200004881c00001000049898788020000c3` (22B) | `2bca9f9743f2fb78` |
| H_388 | 0x18A | 0x62 ADD-IMM | 52 100 | `498b87900200004881c00001000049898790020000c3` (22B) | `6f99edae6e28e2a6` |
| H_389 | 0x18B | 0x61 SUB-IMM | 50 100 | `498b87800200004881e80001000049898780020000c3` (22B) | `a89c3aeffbbddb04` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
SUB-IMM imm=0xF8 (H_382) / imm=0x100 (H_389) use imm32 sub (`48 81 e8`) → 22B pins; not imm8.
ADD-IMM imm=0x100 uses imm32 add (`48 81 c0`) → 22B pins (H_386..H_388); not imm8.
LDB oo=0x100 uses imm32 add (`48 81 c0`) → 26B pins (H_383..H_385).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_382 | `69b7068d45f8bf5d56a1e5bd830dde9f4cd9778f5817a9c7a8784f4385ce4ff7` |
| H_383 | `435a012fe7d4460d49e5cebc9808b62429de0ade9256ee8f08064b4519bfa22e` |
| H_384 | `efcb4fa1a01828f3d962df2b84ec47c48ee8192c945b455e60e78bb11b16bd14` |
| H_385 | `a26708edf890025cee4cdf25c62650d88ed29141f6092408f6d3056b32060303` |
| H_386 | `220b570f6901c7579bf2b076c8d91301acf3569c6d2ef852c244cfb547170de9` |
| H_387 | `2bca9f9743f2fb7814418f7214f35ced43c2277247d18be3b4d5204cb8aa89ec` |
| H_388 | `6f99edae6e28e2a68d3a497fa92492ba1d6d6c4f5afde100b4a8fa5e0f03eae8` |
| H_389 | `a89c3aeffbbddb04d1a96a36f68c5c10de1d43961651ddcac70c6037cc35191b` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h52_f8`, `_scratch_ldb_5060_100`, `_scratch_ldb_5160_100`, `_scratch_ldb_5260_100`,
`_scratch_addimm_h50_100`, `_scratch_addimm_h51_100`, `_scratch_addimm_h52_100`, `_scratch_subimm_h50_100`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 184`.. for H_382.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

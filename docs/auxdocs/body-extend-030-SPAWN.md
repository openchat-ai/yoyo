# body-extend-030 SPAWN · consolidate parallel-batch-24

> Continuous queue handoff from parallel-batch-24 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `1dd8234623853194c8b159ddd7635c2cf8d83d2195cbe480b63c7335b10ea7c2` (abbrev `1dd82346…`).
> Handlers = 172 (H_00..H_165). Last selectors: 0xA4..0xAB = H_158..H_165.
> Source: `docs/auxdocs/parallel-batch-24-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-029-log.md` / `docs/auxdocs/body-extend-029-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.

## Task: body-extend-030 (serialize + Relock)

Mirror body-extend-029 / body-extend-028 protocol:

1. Hand-author append H_166..H_173 to `yoyo/projects/yoyo.ty` at selectors `40 AC` .. `40 B3`.
2. Promote fixtures from `_scratch_{set_52_cafef00d,ldb_5060_58,addimm_h51_3c,subimm_h50_3c,ldb_5260_58,ldb_5160_58,addimm_h52_3c,subimm_h51_3c}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `1dd8234623853194…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-030-log.md`.
7. Auto-spawn parallel-batch-25 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-25-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_166 | 0xAC | 0x30 SET | 52 CAFEF00D | `48b80df0feca0000000049898790020000c3` (18B) | `1d191b40e1afa7fb` |
| H_167 | 0xAD | 0x80 LDB | 50 60 58 | `498b87000300004883c058480fb60049898780020000c3` (23B) | `79fc958e25bf6b1a` |
| H_168 | 0xAE | 0x62 ADD-IMM | 51 3C | `498b87880200004883c03c49898788020000c3` (19B) | `4aa8dc968083160f` |
| H_169 | 0xAF | 0x61 SUB-IMM | 50 3C | `498b87800200004883e83c49898780020000c3` (19B) | `2a63a066b3ef82ab` |
| H_170 | 0xB0 | 0x80 LDB | 52 60 58 | `498b87000300004883c058480fb60049898790020000c3` (23B) | `7b4f4bc7fe9fb608` |
| H_171 | 0xB1 | 0x80 LDB | 51 60 58 | `498b87000300004883c058480fb60049898788020000c3` (23B) | `53655a866d4eb1b9` |
| H_172 | 0xB2 | 0x62 ADD-IMM | 52 3C | `498b87900200004883c03c49898790020000c3` (19B) | `4025f950cb9d1906` |
| H_173 | 0xB3 | 0x61 SUB-IMM | 51 3C | `498b87880200004883e83c49898788020000c3` (19B) | `a436ca73806b6293` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_166 | `1d191b40e1afa7fb4ee86538e903aac8f80bf8a7a946122f76304bbfcfcc6abb` |
| H_167 | `79fc958e25bf6b1a90cbf9009fae8a4fd8f9884aa759ba912883d978b05c1a2f` |
| H_168 | `4aa8dc968083160f5970bb3f4ee1bd13df09b7a45d82d9eda083824fc0ba27d4` |
| H_169 | `2a63a066b3ef82ab1ef3d8ddd432f270d866abb3cc9ed7b1f84ba089677b053c` |
| H_170 | `7b4f4bc7fe9fb608f91d2fd3066f23f0c683be8a196616f7bc1ea0c7e15604e8` |
| H_171 | `53655a866d4eb1b95c0917caa6b70cb551bc9558d7b48068c5f62d53b793478d` |
| H_172 | `4025f950cb9d19064c054fa767e552ae3433e7f24140b8dc2b358a03605bc090` |
| H_173 | `a436ca73806b62935c3e08485447df162fd9ed4000f4cffa0a1144a8113ccfdc` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_set_52_cafef00d`, `_scratch_ldb_5060_58`, `_scratch_addimm_h51_3c`, `_scratch_subimm_h50_3c`,
`_scratch_ldb_5260_58`, `_scratch_ldb_5160_58`, `_scratch_addimm_h52_3c`, `_scratch_subimm_h51_3c`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 AC`.. for H_166.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

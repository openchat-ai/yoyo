# body-extend-037 SPAWN · consolidate parallel-batch-31

> Continuous queue handoff from parallel-batch-31 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `3bf549a652a2746e26d16216f3c3d1e6c8c65a6b6403472091240f753d1545ec` (abbrev `3bf549a6…`).
> Handlers = 228 (H_00..H_221). Last selectors: 0xDC..0xE3 = H_214..H_221.
> Source: `docs/auxdocs/parallel-batch-31-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-036-log.md` / `docs/auxdocs/body-extend-036-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-036 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.

## Task: body-extend-037 (serialize + Relock)

Mirror body-extend-036 / body-extend-035 protocol:

1. Hand-author append H_222..H_229 to `yoyo/projects/yoyo.ty` at selectors `40 E4` .. `40 EB`.
2. Promote fixtures from `_scratch_{addimm_h51_68,addimm_h52_68,subimm_h50_60,subimm_h51_60,subimm_h52_60,ldb_5060_88,ldb_5160_88,ldb_5260_88}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `3bf549a652a2746e…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-037-log.md`.
7. Auto-spawn parallel-batch-32 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-32-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_222 | 0xE4 | 0x62 ADD-IMM | 51 68 | `498b87880200004883c06849898788020000c3` (19B) | `b2f72feaae60803e` |
| H_223 | 0xE5 | 0x62 ADD-IMM | 52 68 | `498b87900200004883c06849898790020000c3` (19B) | `7819936ee9d0c007` |
| H_224 | 0xE6 | 0x61 SUB-IMM | 50 60 | `498b87800200004883e86049898780020000c3` (19B) | `140f19aded02db3b` |
| H_225 | 0xE7 | 0x61 SUB-IMM | 51 60 | `498b87880200004883e86049898788020000c3` (19B) | `17f59cbf3cc854a2` |
| H_226 | 0xE8 | 0x61 SUB-IMM | 52 60 | `498b87900200004883e86049898790020000c3` (19B) | `af095c6f5e0afc0b` |
| H_227 | 0xE9 | 0x80 LDB | 50 60 88 | `498b87000300004881c088000000480fb60049898780020000c3` (26B) | `5edbd7f24b9a903a` |
| H_228 | 0xEA | 0x80 LDB | 51 60 88 | `498b87000300004881c088000000480fb60049898788020000c3` (26B) | `3bee10754f19b9d5` |
| H_229 | 0xEB | 0x80 LDB | 52 60 88 | `498b87000300004881c088000000480fb60049898790020000c3` (26B) | `74c53973c0c6f552` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_227/H_228/H_229).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_222 | `b2f72feaae60803e16768a53b93b44880403f9d1e1a34f24d81c6c745259cfec` |
| H_223 | `7819936ee9d0c007938b5a6cb91c8126b28a3fc78f520f144d7d05275bf7a71a` |
| H_224 | `140f19aded02db3bd2fe0ca338c60ebd0dabad07f1f6c8a97d6cc6230034b825` |
| H_225 | `17f59cbf3cc854a2a5f4e58863e09535a12c16203d6fa8d73ef61511504fd537` |
| H_226 | `af095c6f5e0afc0ba0fabafa9b4bf6b287acca562d811694b8b5c2e8e8e87fd8` |
| H_227 | `5edbd7f24b9a903a7ffc7898018a1624bb79299314c68aea52fd6eb6068b2aba` |
| H_228 | `3bee10754f19b9d5d05f54a85541cf63a9d8eefcea658906415157d0bc582972` |
| H_229 | `74c53973c0c6f552ab8faa8f418161edbdcbb9d2217c9a55bd63e0e0553b88a3` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h51_68`, `_scratch_addimm_h52_68`, `_scratch_subimm_h50_60`, `_scratch_subimm_h51_60`,
`_scratch_subimm_h52_60`, `_scratch_ldb_5060_88`, `_scratch_ldb_5160_88`, `_scratch_ldb_5260_88`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 E4`.. for H_222.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

# body-extend-038 SPAWN · consolidate parallel-batch-32

> Continuous queue handoff from parallel-batch-32 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `39d454a194359d1d682b0638381fa14cbdec617e707f26b2b2405e05be7f9ede` (abbrev `39d454a1…`).
> Handlers = 236 (H_00..H_229). Last selectors: 0xE4..0xEB = H_222..H_229.
> Source: `docs/auxdocs/parallel-batch-32-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-037-log.md` / `docs/auxdocs/body-extend-037-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-037 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.

## Task: body-extend-038 (serialize + Relock)

Mirror body-extend-037 / body-extend-036 protocol:

1. Hand-author append H_230..H_237 to `yoyo/projects/yoyo.ty` at selectors `40 EC` .. `40 F3`.
2. Promote fixtures from `_scratch_{addimm_h50_70,addimm_h51_70,addimm_h52_70,subimm_h50_68,subimm_h51_68,subimm_h52_68,ldb_5060_90,ldb_5160_90}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `39d454a194359d1d…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-038-log.md`.
7. Auto-spawn parallel-batch-33 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-33-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_230 | 0xEC | 0x62 ADD-IMM | 50 70 | `498b87800200004883c07049898780020000c3` (19B) | `fd00d3aaf8d154fd` |
| H_231 | 0xED | 0x62 ADD-IMM | 51 70 | `498b87880200004883c07049898788020000c3` (19B) | `0b9f43d82535758d` |
| H_232 | 0xEE | 0x62 ADD-IMM | 52 70 | `498b87900200004883c07049898790020000c3` (19B) | `a84dbb9e54bc5205` |
| H_233 | 0xEF | 0x61 SUB-IMM | 50 68 | `498b87800200004883e86849898780020000c3` (19B) | `310e437ef9fb3edd` |
| H_234 | 0xF0 | 0x61 SUB-IMM | 51 68 | `498b87880200004883e86849898788020000c3` (19B) | `9bb82476b37c5941` |
| H_235 | 0xF1 | 0x61 SUB-IMM | 52 68 | `498b87900200004883e86849898790020000c3` (19B) | `0f9edc3307cfe318` |
| H_236 | 0xF2 | 0x80 LDB | 50 60 90 | `498b87000300004881c090000000480fb60049898780020000c3` (26B) | `19191871913c0878` |
| H_237 | 0xF3 | 0x80 LDB | 51 60 90 | `498b87000300004881c090000000480fb60049898788020000c3` (26B) | `7571ee40b3a097be` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_236/H_237).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_230 | `fd00d3aaf8d154fd67813dce47a22b94d7b873f88a54a6187373bbd345b317fd` |
| H_231 | `0b9f43d82535758d3016e610890cb6662aaea3b3e9daa768569cd9f5a98fad78` |
| H_232 | `a84dbb9e54bc52054cba7e051f37be51ba73cb5e14a796e2d7e561b2a3082ab3` |
| H_233 | `310e437ef9fb3eddfc216b6eb9c48ca3926cefd6a4bd9796427519f8d0f6d1de` |
| H_234 | `9bb82476b37c59414d4485fb8128f88e2677c25ebae95a9bf85edfe8eca78c37` |
| H_235 | `0f9edc3307cfe318c714a83ef611b8fff5e11f64103836bbbd9cf7d7dd53b132` |
| H_236 | `19191871913c0878f8e01a7aaaf640a795bfb030f435257a444c57d5c3c3c87f` |
| H_237 | `7571ee40b3a097be8f9903019305214612c61ce5dcb4100285340fe9c63084b0` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h50_70`, `_scratch_addimm_h51_70`, `_scratch_addimm_h52_70`, `_scratch_subimm_h50_68`,
`_scratch_subimm_h51_68`, `_scratch_subimm_h52_68`, `_scratch_ldb_5060_90`, `_scratch_ldb_5160_90`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 EC`.. for H_230.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

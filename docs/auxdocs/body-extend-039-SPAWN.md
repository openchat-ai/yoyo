# body-extend-039 SPAWN · consolidate parallel-batch-33

> Continuous queue handoff from parallel-batch-33 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `aa95228f49b6131c88315b4af43e02b76e8c67070322eab4c200944e839a99fa` (abbrev `aa95228f…`).
> Handlers = 244 (H_00..H_237). Last selectors: 0xEC..0xF3 = H_230..H_237.
> Source: `docs/auxdocs/parallel-batch-33-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-038-log.md` / `docs/auxdocs/body-extend-038-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-038 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.

## Task: body-extend-039 (serialize + Relock)

Mirror body-extend-038 / body-extend-037 protocol:

1. Hand-author append H_238..H_245 to `yoyo/projects/yoyo.ty` at selectors `40 F4` .. `40 FB`.
2. Promote fixtures from `_scratch_{ldb_5260_90,subimm_h50_70,subimm_h51_70,subimm_h52_70,addimm_h50_78,addimm_h51_78,addimm_h52_78,ldb_5060_98}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `aa95228f49b6131c…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-039-log.md`.
7. Auto-spawn parallel-batch-34 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-34-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_238 | 0xF4 | 0x80 LDB | 52 60 90 | `498b87000300004881c090000000480fb60049898790020000c3` (26B) | `515d9290ccd5b51f` |
| H_239 | 0xF5 | 0x61 SUB-IMM | 50 70 | `498b87800200004883e87049898780020000c3` (19B) | `864bf0ef8581dfff` |
| H_240 | 0xF6 | 0x61 SUB-IMM | 51 70 | `498b87880200004883e87049898788020000c3` (19B) | `29334b7d85f1f4df` |
| H_241 | 0xF7 | 0x61 SUB-IMM | 52 70 | `498b87900200004883e87049898790020000c3` (19B) | `ab68fcd1813d0252` |
| H_242 | 0xF8 | 0x62 ADD-IMM | 50 78 | `498b87800200004883c07849898780020000c3` (19B) | `abb251d39c0c52c4` |
| H_243 | 0xF9 | 0x62 ADD-IMM | 51 78 | `498b87880200004883c07849898788020000c3` (19B) | `b981458127112570` |
| H_244 | 0xFA | 0x62 ADD-IMM | 52 78 | `498b87900200004883c07849898790020000c3` (19B) | `dfdb811b3af776d0` |
| H_245 | 0xFB | 0x80 LDB | 50 60 98 | `498b87000300004881c098000000480fb60049898780020000c3` (26B) | `20ef671052bbdb81` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_238/H_245).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_238 | `515d9290ccd5b51faf05da65936bf6be35120ef7b2d019d002e6814b5ee9a861` |
| H_239 | `864bf0ef8581dfff3a39e838bdd20e7d3b7568060ef9de1e826a8b64e8dc2cf3` |
| H_240 | `29334b7d85f1f4df1380dc899730d972a6fc88c1012335f2aa6030130747397d` |
| H_241 | `ab68fcd1813d0252dbf73dfbe57376d310f5534bcedec86ed8101216de85ddbb` |
| H_242 | `abb251d39c0c52c4c8dd2dbe49864c6b66ec987716af864b7a5047013bf3d68e` |
| H_243 | `b9814581271125709b7ac1e5908dfca420301c4cdf2e99a5cb5e91bd1dc763cc` |
| H_244 | `dfdb811b3af776d091dc51f9bbfcf093e207f5dc0aec404df9a98096300a7f30` |
| H_245 | `20ef671052bbdb815e3dcdba6aa72d4f01dade01043449c151f3baf16c8376bf` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5260_90`, `_scratch_subimm_h50_70`, `_scratch_subimm_h51_70`, `_scratch_subimm_h52_70`,
`_scratch_addimm_h50_78`, `_scratch_addimm_h51_78`, `_scratch_addimm_h52_78`, `_scratch_ldb_5060_98`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 F4`.. for H_238.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

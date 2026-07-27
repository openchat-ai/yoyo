# body-extend-097 SPAWN · consolidate parallel-batch-91

> Continuous queue handoff from parallel-batch-91 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `0a02f49ed0c94a2df5078022a7737c92d4021cab62c41dcbbfc5bb728f32f29c` (abbrev `0a02f49e…`).
> Handlers = 707 (H_00..H_700). Last selectors: 0x2BB..0x2C2 = H_693..H_700 (`40 2BB`..`40 2C2` via label-width A).
> Source: `docs/auxdocs/parallel-batch-91-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-096-log.md` / `docs/auxdocs/body-extend-096-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-096 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 2C3`.. for H_701.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 2C3`/`40 2CA` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Task: body-extend-097 (serialize + Relock)

Mirror body-extend-096 / body-extend-095 protocol:

1. Hand-author append H_701..H_708 to `yoyo/projects/yoyo.ty` at selectors `40 2C3` .. `40 2CA` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h50_218,addimm_h51_218,addimm_h52_218,subimm_h50_218,subimm_h51_218,subimm_h52_218,ldb_5060_220,ldb_5160_220}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `0a02f49ed0c94a2df5078022a7737c92d4021cab62c41dcbbfc5bb728f32f29c`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-097-log.md`.
7. Auto-spawn parallel-batch-92 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-92-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_701 | 0x2C3 | 0x62 ADD-IMM | 50 218 | `498b87800200004881c01802000049898780020000c3` (22B) | `4ab4f6b000bfc170` |
| H_702 | 0x2C4 | 0x62 ADD-IMM | 51 218 | `498b87880200004881c01802000049898788020000c3` (22B) | `f2a5fff94e8993ce` |
| H_703 | 0x2C5 | 0x62 ADD-IMM | 52 218 | `498b87900200004881c01802000049898790020000c3` (22B) | `b8f58bcc6a95b935` |
| H_704 | 0x2C6 | 0x61 SUB-IMM | 50 218 | `498b87800200004881e81802000049898780020000c3` (22B) | `d98dffc59593a3e5` |
| H_705 | 0x2C7 | 0x61 SUB-IMM | 51 218 | `498b87880200004881e81802000049898788020000c3` (22B) | `a2df94a8e97fec79` |
| H_706 | 0x2C8 | 0x61 SUB-IMM | 52 218 | `498b87900200004881e81802000049898790020000c3` (22B) | `501f0c3b69e446a3` |
| H_707 | 0x2C9 | 0x80 LDB | 50 60 220 | `498b87000300004881c020020000480fb60049898780020000c3` (26B) | `38dd8dd1ab3ef61c` |
| H_708 | 0x2CA | 0x80 LDB | 51 60 220 | `498b87000300004881c020020000480fb60049898788020000c3` (26B) | `6633a1f5ac21e65f` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x218 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x218 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x220 uses imm32 add (`48 81 c0`) → 26B pins.
ADD-IMM slot=50/51/52 imm=218 starts deferred 218 ADD triad (H_701/H_702/H_703).
SUB-IMM slot=50/51/52 imm=218 starts 218 SUB triad (H_704/H_705/H_706).
LDB dd=50/51 ss=60 oo=220 starts 220 LDB ladder (H_707/H_708; LDB 52 220 deferred).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_701 | `4ab4f6b000bfc170429e5110542aa259ae4323684ecd0111ced31957ca2ed16a` |
| H_702 | `f2a5fff94e8993ce3881609c5b31db4ca87a058fc1737abeae2dacfa6c0e6eea` |
| H_703 | `b8f58bcc6a95b93566f9489f12aa546360d390c7233847963b448dffbc6e5986` |
| H_704 | `d98dffc59593a3e5a92ed2b70d52f837b419c60bf8178c3e645177aa43293294` |
| H_705 | `a2df94a8e97fec797174a0f4370ecf60ca0d2085543ee3c4795b586dbc22bcf3` |
| H_706 | `501f0c3b69e446a3f382e8f726884e6348c763eb6e365ef6dc05e7097aeff88a` |
| H_707 | `38dd8dd1ab3ef61c0576bb5f5c590a0e710f7996c93657f9f915f2546903cdd0` |
| H_708 | `6633a1f5ac21e65f54355ae582c8f2364ba596270e71b1d3f28c2d6cd7bf6503` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h50_218`, `_scratch_addimm_h51_218`, `_scratch_addimm_h52_218`, `_scratch_subimm_h50_218`,
`_scratch_subimm_h51_218`, `_scratch_subimm_h52_218`, `_scratch_ldb_5060_220`, `_scratch_ldb_5160_220`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 2C3`.. for H_701.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-92

- LDB dd=52 ss=60 oo=220 (finish 220 LDB triad)
- ADD-IMM slot=50/51/52 imm=220 (start 220 ADD triad)
- SUB-IMM slot=50/51/52 imm=220 (start 220 SUB triad)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked

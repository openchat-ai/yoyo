# body-extend-040 SPAWN · consolidate parallel-batch-34

> Continuous queue handoff from parallel-batch-34 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `cc64da680d967e6b17ebc9767a74dd670251df82f614d0c72df1240126c05642` (abbrev `cc64da68…`).
> Handlers = 252 (H_00..H_245). Last selectors: 0xF4..0xFB = H_238..H_245.
> Source: `docs/auxdocs/parallel-batch-34-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-039-log.md` / `docs/auxdocs/body-extend-039-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-039 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 FC`.. for H_246.. (0xFF RET is opcode namespace only — HANDLER labels OK).
> LABEL-WIDTH: H_250.. need ids past `40 FF`; current parsers mask `args[0]&0xff` — consolidator must widen label space or otherwise assign without colliding H_00.. before appending those four.

## Task: body-extend-040 (serialize + Relock)

Mirror body-extend-039 / body-extend-038 protocol:

1. Hand-author append H_246..H_253 to `yoyo/projects/yoyo.ty` at selectors `40 FC` .. (past `40 FF` requires label-width fix first — see note above).
2. Promote fixtures from `_scratch_{ldb_5160_98,ldb_5260_98,subimm_h50_78,subimm_h51_78,subimm_h52_78,addimm_h50_80,addimm_h51_80,addimm_h52_80}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `cc64da680d967e6b…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-040-log.md`.
7. Auto-spawn parallel-batch-35 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-35-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_246 | 0xFC | 0x80 LDB | 51 60 98 | `498b87000300004881c098000000480fb60049898788020000c3` (26B) | `3c0d495ee6537c54` |
| H_247 | 0xFD | 0x80 LDB | 52 60 98 | `498b87000300004881c098000000480fb60049898790020000c3` (26B) | `08b6a771b863baeb` |
| H_248 | 0xFE | 0x61 SUB-IMM | 50 78 | `498b87800200004883e87849898780020000c3` (19B) | `15fb68e82133705a` |
| H_249 | 0xFF | 0x61 SUB-IMM | 51 78 | `498b87880200004883e87849898788020000c3` (19B) | `362f4b6c5b190470` |
| H_250 | 0x100† | 0x61 SUB-IMM | 52 78 | `498b87900200004883e87849898790020000c3` (19B) | `1d069becb63d59dd` |
| H_251 | 0x101† | 0x62 ADD-IMM | 50 80 | `498b87800200004881c08000000049898780020000c3` (22B) | `483e67e06faf0c03` |
| H_252 | 0x102† | 0x62 ADD-IMM | 51 80 | `498b87880200004881c08000000049898788020000c3` (22B) | `3ce4b6b0b760a9ba` |
| H_253 | 0x103† | 0x62 ADD-IMM | 52 80 | `498b87900200004881c08000000049898790020000c3` (22B) | `fd2b59647a997f33` |

† Selector ids past 0xFF are nominal queue numbers — require label-space widen before `40 XX` encode (do not wrap to `40 00`.. and collide H_00..).

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_246/H_247).
ADD-IMM imm=0x80 uses imm32 add (`48 81 c0`) → 22B pins (H_251..H_253); not imm8.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_246 | `3c0d495ee6537c545e0c9883db84a4ef2e7d96f4db95fc3be5f6e7be68cd88c1` |
| H_247 | `08b6a771b863baebe49eb4b818ba6eb40575669b1dc65367b0b7eaaac2abbde8` |
| H_248 | `15fb68e82133705a6ffa5de8b82ee48a2d56df43c045f98334fcadbfd6be15c9` |
| H_249 | `362f4b6c5b190470bdcb4d893d7e3b4e0553a065e237961fa15fdfc1a8f2fe17` |
| H_250 | `1d069becb63d59ddf346ae7eb0d4540bce021cccb0817181c9000993f7ddff04` |
| H_251 | `483e67e06faf0c0321f2f9a7a9c0d76d9fda2837f36b63c927a12069ef27ef78` |
| H_252 | `3ce4b6b0b760a9ba7c98df6eac28f20c1ef745ea0b3b8fdced62fd9b26f0bac2` |
| H_253 | `fd2b59647a997f333a00b14ab0e9497355357b43ece8c23e87763376de7fa27c` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5160_98`, `_scratch_ldb_5260_98`, `_scratch_subimm_h50_78`, `_scratch_subimm_h51_78`,
`_scratch_subimm_h52_78`, `_scratch_addimm_h50_80`, `_scratch_addimm_h51_80`, `_scratch_addimm_h52_80`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 FC`.. for H_246.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

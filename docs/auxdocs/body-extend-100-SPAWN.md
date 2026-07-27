# body-extend-100 SPAWN · consolidate parallel-batch-94

> Continuous queue handoff from parallel-batch-94 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `3fc618f9e14a881a91460a8c1be733bade35794eca50282f64c5eb0cabb57021` (abbrev `3fc618f9…`).
> Handlers = 731 (H_00..H_724). Last selectors: 0x2D3..0x2DA = H_717..H_724 (`40 2D3`..`40 2DA` via label-width A).
> Source: `docs/auxdocs/parallel-batch-94-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-099-log.md` / `docs/auxdocs/body-extend-099-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-099 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green. DDC may EQUAL or DIFFER next beat.
> Next selectors: `40 2DB`.. for H_725.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 2DB`/`40 2E2` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Queue control

See `docs/auxdocs/body-extend-queue.md`: scratch pool ≤8 concurrent; **one** consolidator Relock; **AUTO-STOP** after Relock if handlers ≥ 800 (do not auto-spawn parallel-batch-95). This beat 731→739 if all 8 land — continue queue after Relock.

## Task: body-extend-100 (serialize + Relock)

Mirror body-extend-099 / body-extend-098 protocol:

1. Hand-author append H_725..H_732 to `yoyo/projects/yoyo.ty` at selectors `40 2DB` .. `40 2E2` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5060_230,ldb_5160_230,ldb_5260_230,addimm_h50_230,addimm_h51_230,addimm_h52_230,subimm_h50_230,subimm_h51_230}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `3fc618f9e14a881a91460a8c1be733bade35794eca50282f64c5eb0cabb57021`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize EQUAL or DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-100-log.md`.
7. If handlers after Relock **≥ 800**: AUTO-STOP（见 queue 控制面）；else auto-spawn parallel-batch-95 scratch-only, or write `docs/auxdocs/parallel-batch-95-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_725 | 0x2DB | 0x80 LDB | 50 60 230 | `498b87000300004881c030020000480fb60049898780020000c3` (26B) | `5c33c35f5fd9760b` |
| H_726 | 0x2DC | 0x80 LDB | 51 60 230 | `498b87000300004881c030020000480fb60049898788020000c3` (26B) | `e574a865427adbbc` |
| H_727 | 0x2DD | 0x80 LDB | 52 60 230 | `498b87000300004881c030020000480fb60049898790020000c3` (26B) | `b066434bf619727b` |
| H_728 | 0x2DE | 0x62 ADD-IMM | 50 230 | `498b87800200004881c03002000049898780020000c3` (22B) | `a2091c2a78abf623` |
| H_729 | 0x2DF | 0x62 ADD-IMM | 51 230 | `498b87880200004881c03002000049898788020000c3` (22B) | `e5fd2243f67268fd` |
| H_730 | 0x2E0 | 0x62 ADD-IMM | 52 230 | `498b87900200004881c03002000049898790020000c3` (22B) | `7fb8cfe02d18cafc` |
| H_731 | 0x2E1 | 0x61 SUB-IMM | 50 230 | `498b87800200004881e83002000049898780020000c3` (22B) | `6057b3357f248ea9` |
| H_732 | 0x2E2 | 0x61 SUB-IMM | 51 230 | `498b87880200004881e83002000049898788020000c3` (22B) | `7b44c3ca05a14832` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x230 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x230 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x230 uses imm32 add (`48 81 c0`) → 26B pins.
LDB dd=50/51/52 ss=60 oo=230 starts deferred 230 LDB ladder (H_725/H_726/H_727).
ADD-IMM slot=50/51/52 imm=230 starts deferred 230 ADD triad (H_728/H_729/H_730).
SUB-IMM slot=50/51 imm=230 starts 230 SUB triad (H_731/H_732; SUB 52 deferred).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_725 | `5c33c35f5fd9760bff127c50a7190077e8179b418de6f54d667aa08bdc69f34b` |
| H_726 | `e574a865427adbbcc9fceca251487fa299ab80135e0abbf0be72094e6d08331b` |
| H_727 | `b066434bf619727bfc38fae08251b8be38f39b2d12d9ae89e23f066de712ce77` |
| H_728 | `a2091c2a78abf623843bf75666d8bbed15a0959397a01bef4e66efd2fcca4f1a` |
| H_729 | `e5fd2243f67268fde53d644b88bec57be0e74d8b0db703b799b9d2d6b6165a18` |
| H_730 | `7fb8cfe02d18cafc02e295bf962c249fa332e1c18499b482aac3a18a6f369905` |
| H_731 | `6057b3357f248ea9b548c111e3b2ce2a2b8b17a8a385e592eda7998f822642f8` |
| H_732 | `7b44c3ca05a148325d4d462a1a872b18bf20c1994a1c0b3551bb0ebef49583d7` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5060_230`, `_scratch_ldb_5160_230`, `_scratch_ldb_5260_230`, `_scratch_addimm_h50_230`,
`_scratch_addimm_h51_230`, `_scratch_addimm_h52_230`, `_scratch_subimm_h50_230`, `_scratch_subimm_h51_230`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 2DB`.. for H_725.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-95

- SUB-IMM slot=52 imm=230 (finish 230 SUB triad)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
- Next ladder if continuing (e.g. LDB/ADD-IMM/SUB-IMM beyond 230) — only fresh, not duplicates

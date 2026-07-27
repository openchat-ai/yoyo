# body-extend-077 SPAWN · consolidate parallel-batch-71

> Continuous queue handoff from parallel-batch-71 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `ebbc6d765fcc0fcdc045848e93a3839d47ffdf287646adb781170a66d80690be` (abbrev `ebbc6d76…`).
> Handlers = 547 (H_00..H_540). Last selectors: 0x21B..0x222 = H_533..H_540 (`40 21B`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-71-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-076-log.md` / `docs/auxdocs/body-extend-076-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-076 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 223`.. for H_541.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 223`/`40 224` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-077 (serialize + Relock)

Mirror body-extend-076 / body-extend-075 protocol:

1. Hand-author append H_541..H_548 to `yoyo/projects/yoyo.ty` at selectors `40 223` .. `40 22A` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h52_188,subimm_h50_188,subimm_h51_188,subimm_h52_188,ldb_5060_190,ldb_5160_190,ldb_5260_190,addimm_h50_190}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `ebbc6d765fcc0fcdc045848e93a3839d47ffdf287646adb781170a66d80690be`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-077-log.md`.
7. Auto-spawn parallel-batch-72 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-72-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_541 | 0x223 | 0x62 ADD-IMM | 52 188 | `498b87900200004881c08801000049898790020000c3` (22B) | `ef2cfed790c9d301` |
| H_542 | 0x224 | 0x61 SUB-IMM | 50 188 | `498b87800200004881e88801000049898780020000c3` (22B) | `4576822a906e44b8` |
| H_543 | 0x225 | 0x61 SUB-IMM | 51 188 | `498b87880200004881e88801000049898788020000c3` (22B) | `6c36bec002d9aa7d` |
| H_544 | 0x226 | 0x61 SUB-IMM | 52 188 | `498b87900200004881e88801000049898790020000c3` (22B) | `c77a089b4ef783bb` |
| H_545 | 0x227 | 0x80 LDB | 50 60 190 | `498b87000300004881c090010000480fb60049898780020000c3` (26B) | `e4ad649adfa675bd` |
| H_546 | 0x228 | 0x80 LDB | 51 60 190 | `498b87000300004881c090010000480fb60049898788020000c3` (26B) | `251c22877545c901` |
| H_547 | 0x229 | 0x80 LDB | 52 60 190 | `498b87000300004881c090010000480fb60049898790020000c3` (26B) | `21f0254d615d4969` |
| H_548 | 0x22A | 0x62 ADD-IMM | 50 190 | `498b87800200004881c09001000049898780020000c3` (22B) | `0b1729a7a8c31cb9` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x188/0x190 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x188 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x190 starts 190 LDB triad (H_545 dd=50, H_546 dd=51, H_547 dd=52).
ADD-IMM slot=52 imm=188 finishes deferred 188 ADD triad (H_541).
SUB-IMM slot=50/51/52 imm=188 starts 188 SUB triad (H_542/H_543/H_544).
ADD-IMM slot=51/52 imm=190 deferred to a later scratch batch.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_541 | `ef2cfed790c9d301a5e0e1ab548235f8151a62bb286305c852eb8a574a858dc2` |
| H_542 | `4576822a906e44b809a53bc7a47d6f3fed4fe69032568c8eda7f87908db4d346` |
| H_543 | `6c36bec002d9aa7d47823bae9b798b4dc708bf86beb972c41a859b8a0306e6e2` |
| H_544 | `c77a089b4ef783bb9bd65accaf84e7c7c8c93a11feb6eaf13367e292569b410d` |
| H_545 | `e4ad649adfa675bdf2ebda458ed06c9a0467812a402f3d0ebcf72a1de375467d` |
| H_546 | `251c22877545c901900760bd670ecc3c4cb5859fe25dd5a312a1dc18c86bb180` |
| H_547 | `21f0254d615d4969cfeabc75fd1bed32d0186f70199fa80476540ef9edd1fcb2` |
| H_548 | `0b1729a7a8c31cb9d196acf729c90a88f1fb4eec35c56ed3180ed11842146c58` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h52_188`, `_scratch_subimm_h50_188`, `_scratch_subimm_h51_188`, `_scratch_subimm_h52_188`,
`_scratch_ldb_5060_190`, `_scratch_ldb_5160_190`, `_scratch_ldb_5260_190`, `_scratch_addimm_h50_190`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 223`.. for H_541.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

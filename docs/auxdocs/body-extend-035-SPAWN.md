# body-extend-035 SPAWN · consolidate parallel-batch-29

> Continuous queue handoff from parallel-batch-29 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `e531a0a8962e21ecce4f085df042195b60eb72b69f90d468f04cfeaa9c283588` (abbrev `e531a0a8…`).
> Handlers = 212 (H_00..H_205). Last selectors: 0xCC..0xD3 = H_198..H_205.
> Source: `docs/auxdocs/parallel-batch-29-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-034-log.md` / `docs/auxdocs/body-extend-034-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-034 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.

## Task: body-extend-035 (serialize + Relock)

Mirror body-extend-034 / body-extend-033 protocol:

1. Hand-author append H_206..H_213 to `yoyo/projects/yoyo.ty` at selectors `40 D4` .. `40 DB`.
2. Promote fixtures from `_scratch_{addimm_h51_58,addimm_h52_58,subimm_h50_50,subimm_h52_50,ldb_5260_78,set_52_c0dec0de,addimm_h50_60,ldb_5060_80}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `e531a0a8962e21ec…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-035-log.md`.
7. Auto-spawn parallel-batch-30 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-30-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_206 | 0xD4 | 0x62 ADD-IMM | 51 58 | `498b87880200004883c05849898788020000c3` (19B) | `4ff049a8441518ba` |
| H_207 | 0xD5 | 0x62 ADD-IMM | 52 58 | `498b87900200004883c05849898790020000c3` (19B) | `44445f68d85c340f` |
| H_208 | 0xD6 | 0x61 SUB-IMM | 50 50 | `498b87800200004883e85049898780020000c3` (19B) | `e51df228ac034429` |
| H_209 | 0xD7 | 0x61 SUB-IMM | 52 50 | `498b87900200004883e85049898790020000c3` (19B) | `1b61da415449f276` |
| H_210 | 0xD8 | 0x80 LDB | 52 60 78 | `498b87000300004883c078480fb60049898790020000c3` (23B) | `f7221a4afaec1410` |
| H_211 | 0xD9 | 0x30 SET | 52 C0DEC0DE | `48b8dec0dec00000000049898790020000c3` (18B) | `20960f8da0f70a8e` |
| H_212 | 0xDA | 0x62 ADD-IMM | 50 60 | `498b87800200004883c06049898780020000c3` (19B) | `88c5f7c3de52c972` |
| H_213 | 0xDB | 0x80 LDB | 50 60 80 | `498b87000300004881c080000000480fb60049898780020000c3` (26B) | `5c4e0e3a942cbe06` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_206 | `4ff049a8441518baae98d945eef56610c0acafd2a50bfbd12e247392eef0d0c5` |
| H_207 | `44445f68d85c340f9c168ae79fcfd5182458fd37f62ee76a7f2b576060b0598d` |
| H_208 | `e51df228ac03442961671193993852ff712426ecfdc091920f9b81d60381eae5` |
| H_209 | `1b61da415449f276ec05815d2e5feeb54e563d3a3967eb8fef1d5fddf0204b9a` |
| H_210 | `f7221a4afaec14106ec5535002b27e4aac7e3c192bfe76d3dc4bf186a8409d9f` |
| H_211 | `20960f8da0f70a8efaf8cbd49d4df2683a22cbccb8e45c57be90e1a9fa912a43` |
| H_212 | `88c5f7c3de52c9726d0ff4aa8d0fc20425af6466b1397e0fa049250ccfd1fbbd` |
| H_213 | `5c4e0e3a942cbe06bb59e8e0a4b9bb801e02d1ded417a8ac20a1eedac6e9737b` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h51_58`, `_scratch_addimm_h52_58`, `_scratch_subimm_h50_50`, `_scratch_subimm_h52_50`,
`_scratch_ldb_5260_78`, `_scratch_set_52_c0dec0de`, `_scratch_addimm_h50_60`, `_scratch_ldb_5060_80`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 D4`.. for H_206.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

# body-extend-020 SPAWN · consolidate parallel-batch-14

> Continuous queue handoff from parallel-batch-14 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `ea348e8b7a43f285121c1755b572a87940a50432ef5d0482be6ecc3c575a98bd` (abbrev `ea348e8b…`).
> Handlers = 92 (H_00..H_85). Last selectors: 0x54..0x5B = H_78..H_85.
> Source: `docs/auxdocs/parallel-batch-14-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-019-log.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.

## Task: body-extend-020 (serialize + Relock)

Mirror body-extend-019 / body-extend-018 protocol:

1. Hand-author append H_86..H_93 to `yoyo/projects/yoyo.ty` at selectors `40 5C` .. `40 63`.
2. Promote fixtures from `_scratch_{set_feedface,set_aabbccdd,get_5052,cmp_5052,ldb_5160_10,imul_5250,orv_5152,addimm_h50_0f}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `ea348e8b7a43f285…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-020-log.md`.
7. Auto-spawn parallel-batch-15 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-15-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_86 | 0x5C | 0x30 SET | 52 FEEDFACE | `48b8cefaedfe0000000049898790020000c3` (18B) | `e66d020e76069da7` |
| H_87 | 0x5D | 0x30 SET | 51 AABBCCDD | `48b8ddccbbaa0000000049898788020000c3` (18B) | `2a98933dfb0d8cdd` |
| H_88 | 0x5E | 0x60 GET | 50 52 | `498b879002000049898780020000c3` (15B) | `ce17131dfed4ee14` |
| H_89 | 0x5F | 0x65 CMP | 50 52 | `498b8780020000498b8f900200004839c8c3` (18B) | `594c4a8e7b724cf5` |
| H_90 | 0x60 | 0x80 LDB | 51 60 10 | `498b87000300004883c010480fb60049898788020000c3` (23B) | `d3253d0131cd96d0` |
| H_91 | 0x61 | 0x63 IMUL | 52 50 | `498b8790020000498b8f80020000480fafc149898790020000c3` (26B) | `ba2a57ad864330da` |
| H_92 | 0x62 | 0x69 ORV | 51 52 | `498b8788020000498b8f900200004809c849898788020000c3` (25B) | `df8b41f4c74b2540` |
| H_93 | 0x63 | 0x62 ADD-IMM | 50 0F | `498b87800200004883c00f49898780020000c3` (19B) | `899a90c682241183` |

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_86 | `e66d020e76069da7f2aeb30d6618f61321d1c7b396f15ed6d03a1858326e589e` |
| H_87 | `2a98933dfb0d8cdda9161df415cb8a9a9635ff1387085d7168024e8869a6688f` |
| H_88 | `ce17131dfed4ee14af0697f07f0d04f1a0b667aff33bef843c12e28b08399120` |
| H_89 | `594c4a8e7b724cf54fbf766598061808a0fbd5b4965f2b1d858d7c2f2fb68ab0` |
| H_90 | `d3253d0131cd96d0f544e7149d58883b77ce84fe3deea780b141e7adabf943af` |
| H_91 | `ba2a57ad864330daa4c0158da3ecfb56887a5f08df92b7e86c397ab9d1669f92` |
| H_92 | `df8b41f4c74b2540fd7aa2f3437d35001f0d4753344f5157bf836cd26c24864b` |
| H_93 | `899a90c68224118374482202720a205f23b7e1cb0dd41f92127a4303f7c7a4ca` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_set_feedface`, `_scratch_set_aabbccdd`, `_scratch_get_5052`, `_scratch_cmp_5052`,
`_scratch_ldb_5160_10`, `_scratch_imul_5250`, `_scratch_orv_5152`, `_scratch_addimm_h50_0f`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY 0x84/0x85.
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64.

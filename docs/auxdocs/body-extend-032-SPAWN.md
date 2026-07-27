# body-extend-032 SPAWN · consolidate parallel-batch-26

> Continuous queue handoff from parallel-batch-26 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `dc10b2bd70d2232bc015d3a87c88a02d58d5eaffd5ae572fd219dd84094db127` (abbrev `dc10b2bd…`).
> Handlers = 188 (H_00..H_181). Last selectors: 0xB4..0xBB = H_174..H_181.
> Source: `docs/auxdocs/parallel-batch-26-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-031-log.md` / `docs/auxdocs/body-extend-031-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-031 reported DDC PE `.text` VirtualSize DIFFER (4096 vs 8192) while stub EQUAL — do not invent-green; W-START stays EXPERIMENTAL · NON-GREEN.

## Task: body-extend-032 (serialize + Relock)

Mirror body-extend-031 / body-extend-030 protocol:

1. Hand-author append H_182..H_189 to `yoyo/projects/yoyo.ty` at selectors `40 BC` .. `40 C3`.
2. Promote fixtures from `_scratch_{set_52_deadc0de,ldb_5060_68,ldb_5160_68,ldb_5260_68,addimm_h50_48,addimm_h51_48,subimm_h50_40,subimm_h51_40}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `dc10b2bd70d2232b…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-032-log.md`.
7. Auto-spawn parallel-batch-27 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-27-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_182 | 0xBC | 0x30 SET | 52 DEADC0DE | `48b8dec0adde0000000049898790020000c3` (18B) | `7a587d84beb9cc85` |
| H_183 | 0xBD | 0x80 LDB | 50 60 68 | `498b87000300004883c068480fb60049898780020000c3` (23B) | `bd2195e8c421a165` |
| H_184 | 0xBE | 0x80 LDB | 51 60 68 | `498b87000300004883c068480fb60049898788020000c3` (23B) | `1ea59c358f5546e1` |
| H_185 | 0xBF | 0x80 LDB | 52 60 68 | `498b87000300004883c068480fb60049898790020000c3` (23B) | `766b3c1623cfc488` |
| H_186 | 0xC0 | 0x62 ADD-IMM | 50 48 | `498b87800200004883c04849898780020000c3` (19B) | `16f582bad178a162` |
| H_187 | 0xC1 | 0x62 ADD-IMM | 51 48 | `498b87880200004883c04849898788020000c3` (19B) | `cc49b12c560f1413` |
| H_188 | 0xC2 | 0x61 SUB-IMM | 50 40 | `498b87800200004883e84049898780020000c3` (19B) | `96696eeac9b4038b` |
| H_189 | 0xC3 | 0x61 SUB-IMM | 51 40 | `498b87880200004883e84049898788020000c3` (19B) | `49afb30429d07d3f` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_182 | `7a587d84beb9cc8525cdfb50268fde6d8239674cb38ced809aa942130f255c01` |
| H_183 | `bd2195e8c421a165a1063f5ac6478beb2ecfe026cea1684b756a429aeffaca09` |
| H_184 | `1ea59c358f5546e1a17b12489c09a2ca3529eb0b7989d013d0191d3a8ef7756c` |
| H_185 | `766b3c1623cfc488f7aa4a524380d8a616d3d70e3691c96ef59c8fe7e248823b` |
| H_186 | `16f582bad178a162be629684e807c1e9bc225ae4464e4b78c4157370446da321` |
| H_187 | `cc49b12c560f141385c8e59756b75c6026babd2a6102da5140fe468d0071cc52` |
| H_188 | `96696eeac9b4038b83a169bb84ee7d10dd4dfc6c6810f0ff8f6879181a5d165c` |
| H_189 | `49afb30429d07d3f178460f3c763f843c7865f6fd4220684c4839d1ac243361b` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_set_52_deadc0de`, `_scratch_ldb_5060_68`, `_scratch_ldb_5160_68`, `_scratch_ldb_5260_68`,
`_scratch_addimm_h50_48`, `_scratch_addimm_h51_48`, `_scratch_subimm_h50_40`, `_scratch_subimm_h51_40`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 BC`.. for H_182.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

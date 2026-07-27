# body-extend-031 SPAWN · consolidate parallel-batch-25

> Continuous queue handoff from parallel-batch-25 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `9fddb56b31ab513c92e4435193619de1193f4ea543bbb4b2a239531eeefae0ea` (abbrev `9fddb56b…`).
> Handlers = 180 (H_00..H_173). Last selectors: 0xAC..0xB3 = H_166..H_173.
> Source: `docs/auxdocs/parallel-batch-25-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-030-log.md` / `docs/auxdocs/body-extend-030-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.

## Task: body-extend-031 (serialize + Relock)

Mirror body-extend-030 / body-extend-029 protocol:

1. Hand-author append H_174..H_181 to `yoyo/projects/yoyo.ty` at selectors `40 B4` .. `40 BB`.
2. Promote fixtures from `_scratch_{set_50_deadc0de,ldb_5160_60,ldb_5260_60,addimm_h50_40,addimm_h51_40,addimm_h52_40,subimm_h52_3c,set_51_deadc0de}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `9fddb56b31ab513c…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-031-log.md`.
7. Auto-spawn parallel-batch-26 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-26-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_174 | 0xB4 | 0x30 SET | 50 DEADC0DE | `48b8dec0adde0000000049898780020000c3` (18B) | `2a769aa9aba9805c` |
| H_175 | 0xB5 | 0x80 LDB | 51 60 60 | `498b87000300004883c060480fb60049898788020000c3` (23B) | `abf0f5b80eb452c0` |
| H_176 | 0xB6 | 0x80 LDB | 52 60 60 | `498b87000300004883c060480fb60049898790020000c3` (23B) | `24b65657d4e28852` |
| H_177 | 0xB7 | 0x62 ADD-IMM | 50 40 | `498b87800200004883c04049898780020000c3` (19B) | `600b3eb1029e26ea` |
| H_178 | 0xB8 | 0x62 ADD-IMM | 51 40 | `498b87880200004883c04049898788020000c3` (19B) | `ed54fe4ff3d8414c` |
| H_179 | 0xB9 | 0x62 ADD-IMM | 52 40 | `498b87900200004883c04049898790020000c3` (19B) | `e98fc8f93f052ba2` |
| H_180 | 0xBA | 0x61 SUB-IMM | 52 3C | `498b87900200004883e83c49898790020000c3` (19B) | `c57d88a68c708a91` |
| H_181 | 0xBB | 0x30 SET | 51 DEADC0DE | `48b8dec0adde0000000049898788020000c3` (18B) | `946ee015447d1bab` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_174 | `2a769aa9aba9805cfbe044a2c7277573c58028335f0105c5c625b1493e824440` |
| H_175 | `abf0f5b80eb452c05ef5f31d0662765533caf1f3554d0958a13714c29345927a` |
| H_176 | `24b65657d4e2885251bc9155a92cef65c2f25d5b0befd78ec4b9d2b54403ab0d` |
| H_177 | `600b3eb1029e26eadd62a6dbb9051e77408a3051c4657f6305c68f4faddb7f92` |
| H_178 | `ed54fe4ff3d8414c6dccbc87bfd8968fac1cd1faf81b700c40891e996e6d1a4e` |
| H_179 | `e98fc8f93f052ba2ed1f09f1401b0d3210025ce318a6e30a244a244ef3556b48` |
| H_180 | `c57d88a68c708a912b4b1d21fdb37e90ab04745efae589abb3123bbbb6e78122` |
| H_181 | `946ee015447d1bab70ed0740cb54b4b764be36fbcba10168acd72ada7d360b75` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_set_50_deadc0de`, `_scratch_ldb_5160_60`, `_scratch_ldb_5260_60`, `_scratch_addimm_h50_40`,
`_scratch_addimm_h51_40`, `_scratch_addimm_h52_40`, `_scratch_subimm_h52_3c`, `_scratch_set_51_deadc0de`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 B4`.. for H_174.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

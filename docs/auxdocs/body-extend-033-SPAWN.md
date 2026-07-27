# body-extend-033 SPAWN · consolidate parallel-batch-27

> Continuous queue handoff from parallel-batch-27 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `a0cb2642b1b3a3e03be8b82602ae26da1234e8f88170f4c49d836a84caed429d` (abbrev `a0cb2642…`).
> Handlers = 196 (H_00..H_189). Last selectors: 0xBC..0xC3 = H_182..H_189.
> Source: `docs/auxdocs/parallel-batch-27-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-032-log.md` / `docs/auxdocs/body-extend-032-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-032 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.

## Task: body-extend-033 (serialize + Relock)

Mirror body-extend-032 / body-extend-031 protocol:

1. Hand-author append H_190..H_197 to `yoyo/projects/yoyo.ty` at selectors `40 C4` .. `40 CB`.
2. Promote fixtures from `_scratch_{addimm_h52_48,subimm_h52_40,ldb_5160_70,ldb_5260_70,set_50_c0dec0de,addimm_h50_50,subimm_h51_48,addimm_h51_50}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `a0cb2642b1b3a3e0…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-033-log.md`.
7. Auto-spawn parallel-batch-28 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-28-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_190 | 0xC4 | 0x62 ADD-IMM | 52 48 | `498b87900200004883c04849898790020000c3` (19B) | `87d6ef901773b519` |
| H_191 | 0xC5 | 0x61 SUB-IMM | 52 40 | `498b87900200004883e84049898790020000c3` (19B) | `6389a07c533b54d4` |
| H_192 | 0xC6 | 0x80 LDB | 51 60 70 | `498b87000300004883c070480fb60049898788020000c3` (23B) | `a36507620f4b048d` |
| H_193 | 0xC7 | 0x80 LDB | 52 60 70 | `498b87000300004883c070480fb60049898790020000c3` (23B) | `29dddd3529790413` |
| H_194 | 0xC8 | 0x30 SET | 50 C0DEC0DE | `48b8dec0dec00000000049898780020000c3` (18B) | `b41a84acb6668560` |
| H_195 | 0xC9 | 0x62 ADD-IMM | 50 50 | `498b87800200004883c05049898780020000c3` (19B) | `137444f465f92575` |
| H_196 | 0xCA | 0x61 SUB-IMM | 51 48 | `498b87880200004883e84849898788020000c3` (19B) | `29980365da8b1f33` |
| H_197 | 0xCB | 0x62 ADD-IMM | 51 50 | `498b87880200004883c05049898788020000c3` (19B) | `c608d7b30f277885` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_190 | `87d6ef901773b519a500a91f6252d58150b3acfa7555c0a934daaaa42a69a1b1` |
| H_191 | `6389a07c533b54d4fcc1d87bda8720d1c10b1abcf2479d9816436e5624ebd1f5` |
| H_192 | `a36507620f4b048d5f4453fe55ee6766a951709a6cba30dca050399a81745031` |
| H_193 | `29dddd3529790413b7ca1825690551d66a12c5630b3604113995fc61b97c3969` |
| H_194 | `b41a84acb6668560c2a0889fe0a5502765fe1b67fa208494af74dc6e8a75779b` |
| H_195 | `137444f465f92575826ca9341c5ad44108cc7e01f6179c6ffff4280dfc863df9` |
| H_196 | `29980365da8b1f33a028126169fd44c31b9de2f57460e0194cd7271930e3e217` |
| H_197 | `c608d7b30f27788517113f3812ba2fbfd8d0a38fe144921406789a2667eccd17` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h52_48`, `_scratch_subimm_h52_40`, `_scratch_ldb_5160_70`, `_scratch_ldb_5260_70`,
`_scratch_set_50_c0dec0de`, `_scratch_addimm_h50_50`, `_scratch_subimm_h51_48`, `_scratch_addimm_h51_50`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 C4`.. for H_190.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

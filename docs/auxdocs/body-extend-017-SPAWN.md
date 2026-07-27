# body-extend-017 SPAWN · consolidate parallel-batch-11

> Continuous queue handoff from parallel-batch-11 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `8ecc0f9383c79897da33a3539cdaa292872bbb3025a04c8f8f33e8d614c47b19` (abbrev `8ecc0f93…`).
> Handlers ≈ 68 (H_00..H_61). Last selectors: 0x3C..0x43 = H_54..H_61.
> Source: `docs/auxdocs/parallel-batch-11-log.md`.

## Task: body-extend-017 (serialize + Relock)

Mirror body-extend-016 / body-extend-015 protocol:

1. Hand-author append H_62..H_69 to `yoyo/projects/yoyo.ty` at selectors `40 44` .. `40 4B`.
2. Promote fixtures from `_scratch_{inc_h51,dec_h51,addimm_h51,cmp_h52,addv_5052,get_5150,set_12345678,ldb_dst52}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect 52→60 JS, 59→67 Rust).
4. Verify + Relock once chaining from `8ecc0f9383c79897…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-017-log.md`.
7. Auto-spawn parallel-batch-12 scratch-only (continuous queue).

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_62 | 0x44 | 0x66 INC | 51 | `498b878802000048ffc049898788020000c3` | `bd325a942a6f34f9` |
| H_63 | 0x45 | 0x67 DEC | 51 | `498b878802000048ffc849898788020000c3` | `55b6d3c3472ebe20` |
| H_64 | 0x46 | 0x62 ADD-IMM | 51 07 | `498b87880200004883c00749898788020000c3` | `689cb441b74287bd` |
| H_65 | 0x47 | 0x65 CMP | 52 51 | `498b8790020000498b8f880200004839c8c3` | `c00b3b5f20ff99f7` |
| H_66 | 0x48 | 0x68 ADDV | 50 52 | `498b8780020000498b8f900200004801c849898780020000c3` | `b26e2da9b4b08d57` |
| H_67 | 0x49 | 0x60 GET | 51 50 | `498b878002000049898788020000c3` | `bb9aebf5e262fb01` |
| H_68 | 0x4A | 0x30 SET | 50 12345678 | `48b8785634120000000049898780020000c3` | `e33984c971e7503f` |
| H_69 | 0x4B | 0x80 LDB | 52 60 08 | `498b87000300004883c008480fb60049898790020000c3` | `8e12ac3f5fcec6a8` |

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY 0x84/0x85.

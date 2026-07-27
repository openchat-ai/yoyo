# body-extend-040 Log · parallel-batch-34 consolidation (H_246..H_253)

> Tag: `body-extend-040-EXPERIMENTAL-batch34-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-34-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `cc64da680d967e6b…` → `a58ead289233c42b…`.
> **handler count: 252 → 260** (+8 at selectors 0xFC..0xFF + 0x100..0x103).
> **LABEL-WIDTH: action A** — dual-peer widen (no PROMPT edit / no trusted-encoder redesign / no GREEN claim).

## 0. Label-width (action A)

H_250..H_253 require ids past `40 FF`. Prior peers masked `args[0]&0xff` / cast `as u8`, which would wrap `40 100`→H_00 and collide — **FORBIDDEN**.

Minimal dual-peer widen applied this beat (fail-closed on divergence):

| peer | change |
|------|--------|
| JS `yoyo.js` + `golden.js` | `labelId(args)` keeps full numeric arg (0..0xffff); no `&0xff` |
| Rust `fixup.rs` | fixed table 256→`LABEL_CAP=512`; `hh: u16` |
| Rust `emit.rs` / `executor.rs` / `types.rs` | label ids `u16`; `compile_one_handler(hh: u16)` |
| `.ty` encode | H_250.. use multi-digit tokens `40 100`..`40 103` (arity-1 arg, not wrap) |

Verified: defining `0x100` does not overwrite `0x00`. No PROMPT edit. Still EXPERIMENTAL · NON-GREEN.

## 1. Consolidated picks (ALL 8 — no REJECTED / none deferred)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_246 | 0xFC | 0x80 LDB | 51 60 98 | 26 | `3c0d495ee6537c54` |
| H_247 | 0xFD | 0x80 LDB | 52 60 98 | 26 | `08b6a771b863baeb` |
| H_248 | 0xFE | 0x61 SUB-IMM | 50 78 | 19 | `15fb68e82133705a` |
| H_249 | 0xFF | 0x61 SUB-IMM | 51 78 | 19 | `362f4b6c5b190470` |
| H_250 | 0x100 | 0x61 SUB-IMM | 52 78 | 19 | `1d069becb63d59dd` |
| H_251 | 0x101 | 0x62 ADD-IMM | 50 80 | 22 | `483e67e06faf0c03` |
| H_252 | 0x102 | 0x62 ADD-IMM | 51 80 | 22 | `3ce4b6b0b760a9ba` |
| H_253 | 0x103 | 0x62 ADD-IMM | 52 80 | 22 | `fd2b59647a997f33` |

**REJECTED (not added):** none (batch-34 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 FC`..`40 FF` / `40 100`..`40 103` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_246/H_247 LDB oo≥0x80 use imm32 path (`48 81 c0 …`), pin 26B. H_251..H_253 ADD-IMM imm=0x80 use imm32 (`48 81 c0 …`), pin 22B.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_246..H_253 at selectors 0xFC..0xFF + 0x100..0x103. Not RAW_BYTE; mirrors H_238..H_245 comment style (body-extend-040 / parallel-batch-34).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5160_98,ldb_5260_98,subimm_h50_78,subimm_h51_78,subimm_h52_78,addimm_h50_80,addimm_h51_80,addimm_h52_80}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **244/244 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **252/252 PASS**.
- Full canonical emit: JS=Rust=**5545B** code (was 5370B; +175B); byte-equal **Y**; sha `9b192b46219ce396…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `a58ead289233c42b…`; previous chained to `cc64da680d967e6b…`.
- DDC: `verify-selfhost.ps1` ran (release peer rebuilt after label-width); PE `.text` DDC **EQUAL** (compared_bytes=5632; both peers code=5545). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted-encoder redesign beyond the minimal label-id widen above.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-34 (slot/imm/dst variants) + label-width A first.
2. Encoder: existing JS/Rust paths retained; label-id space widened (u16 / LABEL_CAP=512).
3. Hand-author: H_246..H_253 at selectors 0xFC..0xFF + 0x100..0x103.
4. Selftest: exact pins PASS (26/26/19/19/19/22/22/22B).
5. Goldens: JS 244/244 and Rust 252/252 PASS; full emit byte-equal Y at 5545B.
6. Lock: Relock once → `a58ead289233c42b…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat after release rebuild).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-35: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_246..H_253), writing `parallel-batch-35-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. After batch-35 scratches done: parent next = body-extend-041 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-35-SPAWN.md` (no Task tool on this consolidator). No deferred PASSes from this beat.

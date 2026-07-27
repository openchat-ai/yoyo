# body-extend-102 SPAWN · consolidate + Relock (post parallel-batch-96)

> Continuous queue handoff from parallel-batch-96 scratch.
> Pin after body-extend-101 Relock: `514ff62ce8663a1507d07f965ba205e7483428b503c34e58f0a35dcfe4064719` (abbrev `514ff62c…`).
> Handlers = 747 (H_00..H_740). Last selectors: 0x2E3..0x2EA = H_733..H_740.
> W-START: **EXPERIMENTAL · NON-GREEN**. Do not invent-green.
>
> **STRATEGY**: body-extend-102 is the first consolidation under matrix-priority
> (replaces imm-ladder strategy starting batch-96). Matrix target:
> `docs/auxdocs/selfhost-emit-matrix.md` P0/P1 YES+MISSING rows.

## Task: body-extend-102 (consolidate + Relock)

### 1. Scope of work
- **MEMCPY implementation (P0)**: Replace stub `0xc3` for `0x84 MEMCPY_DATA` / `0x85 MEMCPY_STATE` with real encoding in both JS (`encodeOp`) and Rust (`compile.rs`). `MEMCPY_DATA` = copy arbitrary bytes; `MEMCPY_STATE` = copy from state slots. Implement load(src)/store(dst)/loop(n). This is the D-3 semantic gap; byte-eq alone does not mark DONE.
- **Serialize 8 PASS scratch handlers** (H_741..H_748) from `parallel-batch-96-log.md`:
  | H | sel | opcode | body | pin hex |
  |---|-----|--------|------|---------|
  | H_741 | 0x2EB | 0x84 MEMCPY_DATA | `84 50 51 40` | `c3c3` (stub until MEMCPY impl) |
  | H_742 | 0x2EC | 0x85 MEMCPY_STATE | `85 50 51 40` | `c3c3` (stub until MEMCPY impl) |
  | H_743 | 0x2ED | 0x60 GET | `60 60 50` | `498b878002000049898700030000c3` |
  | H_744 | 0x2EE | 0x60 GET | `60 50 60` | `498b870003000049898780020000c3` |
  | H_745 | 0x2EF | 0x30 SET | `30 50 fff` | `48b8ff0f00000000000049898780020000c3` |
  | H_746 | 0x2F0 | 0x30 SET | `30 51 10000` | `48b8000001000000000049898788020000c3` |
  | H_747 | 0x2F1 | 0x69 ORV | `69 50 62` | `498b8780020000498b8f100300004809c849898780020000c3` |
  | H_748 | 0x2F2 | 0x65 CMP | `65 60 52` | `498b8700030000498b8f900200004839c8c3` |
- Next selectors: `40 2EB`..`40 2F2` (H_741..H_748). Handlers after consolidate: 755. LABEL_CAP=1024 — well within limit.

### 2. Protocol (per-beat Relock)
- Write `scripts/_probe/_tmp_be102_relock.mjs`
- Append H_741..H_748 to `yoyo/projects/yoyo.ty` (after H_740)
- Re-generate `yoyo/tests/golden/golden.js`, `self_test.rs`, `main.rs`
- Verify: `node scripts/verify-yoyo-ty.mjs` + `cargo test --package yoyo-verifier --test self_test`
- Write `yoyo/projects/yoyo.ty.lock` with new pin
- **Only 1 writer** — no parallel append

### 3. MEMCPY implementation notes
- `MEMCPY_DATA`: dst/src are byte pointers, n is byte count. Encode loop: `mov rax, [src+i]; mov [dst+i], rax; inc i; cmp i, n; jl loop`. Keep encoding deterministic and reproducible by both JS & Rust.
- `MEMCPY_STATE`: dst/src are state slot indices, n is byte count. Load via state disp, copy, store via state disp.
- After MEMCPY impl, re-run `parallel-batch-96-run.mjs` to confirm H_741/H_742 pin changes from `c3c3` to the real bytes, still PASS.
- **Do not** mark matrix row DONE until both js↔rust emit real copy bytes (not stub).

### 4. Output artifacts
- `docs/auxdocs/body-extend-102-log.md` — consolidate log + Relock result
- Updated `yoyo/projects/yoyo.ty` (755 handlers, H_741..H_748 appended)
- Updated `yoyo.ty.lock` pin
- Updated `docs/auxdocs/selfhost-emit-matrix.md` — mark completed rows
- Spawn `docs/auxdocs/parallel-batch-97-SPAWN.md` if continuing matrix (remaining P0/P1/P2 rows)

### 5. Honesty checks
- No PROMPT edit. No version bump.
- No git commit (W-START convention).
- D-3 MEMCPY: log honestly that stub→real implementation; do not invent-green.
- body-extend-101 DDC PE EQUAL noted — still EXPERIMENTAL · NON-GREEN.

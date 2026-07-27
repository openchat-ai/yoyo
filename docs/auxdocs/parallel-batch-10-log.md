# parallel-batch-10 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-10-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-015 (pin `34d2cbb0…`, handlers ≈ 60, H_48..H_53 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_53 (batch-09) and
> not already present as handlers in current `yoyo.ty`. Slot/imm/dst
> variations of ORV/SUBV/IMUL/CMP/GET/SET/LDB. No AND/XOR in ISA
> (`isa_table.txt`); skipped D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x69 ORV | (0x52, 0x51) | `498b8790020000498b8f880200004809c849898790020000c3` (25) | same | same | Y | `382860b30cfecf9c` | `382860b30cfecf9c` | PASS |
| 2 | 0x6A SUBV | (0x52, 0x51) | `498b8790020000498b8f880200004829c849898790020000c3` (25) | same | same | Y | `42cae40b3f2af91a` | `42cae40b3f2af91a` | PASS |
| 3 | 0x63 IMUL | (0x51, 0x50) | `498b8788020000498b8f80020000480fafc149898788020000c3` (26) | same | same | Y | `198ee0d48f5ee313` | `198ee0d48f5ee313` | PASS |
| 4 | 0x63 IMUL | (0x52, 0x51) | `498b8790020000498b8f88020000480fafc149898790020000c3` (26) | same | same | Y | `159a27bf27330831` | `159a27bf27330831` | PASS |
| 5 | 0x65 CMP | (0x51, 0x50) | `498b8788020000498b8f800200004839c8c3` (18) | same | same | Y | `8f946554be6d3b78` | `8f946554be6d3b78` | PASS |
| 6 | 0x60 GET | (0x52, 0x50) | `498b878002000049898790020000c3` (15) | same | same | Y | `5a7ab8a520b7161a` | `5a7ab8a520b7161a` | PASS |
| 7 | 0x30 SET | slot=0x51 imm=0xDEADBEEF | `48b8efbeadde0000000049898788020000c3` (18) | same | same | Y | `363eaa79a8c8b498` | `363eaa79a8c8b498` | PASS |
| 8 | 0x80 LDB | (0x51, 0x60, 0x08) | `498b87000300004883c008480fb60049898788020000c3` (23) | same | same | Y | `ddcb219757fb451f` | `ddcb219757fb451f` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

### §1.1 ORV-h52 scratch — 0x69 dst=0x52 src=0x51
- expected: `498b8790020000498b8f880200004809c849898790020000c3` (25B)
- js-actual: `498b8790020000498b8f880200004809c849898790020000c3` (25B; matches)
- rust-actual: `498b8790020000498b8f880200004809c849898790020000c3` (25B; matches after stripping 1B startup_blob prefix)
- js-sha256: `382860b30cfecf9ce2efd2e4c169368a8c6c6ba3c73d36c6230dc7031ffbb0d5`
- rust-sha256: `382860b30cfecf9ce2efd2e4c169368a8c6c6ba3c73d36c6230dc7031ffbb0d5`
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x52,rax)=498b8790020000 (7B, disp32=0x290 LE) +
  load_state(0x51,rcx)=498b8f88020000 (7B, disp32=0x288 LE) +
  or rax,rcx=4809c8 (3B) +
  store_state(0x52,rax)=49898790020000 (7B) + ret=c3 (1B) = 25B.

### §1.2 SUBV-h52 scratch — 0x6A dst=0x52 src=0x51
- expected: `498b8790020000498b8f880200004829c849898790020000c3` (25B)
- js-actual: matches; rust-actual: matches
- js-sha256: `42cae40b3f2af91a879a70d0668a8bc6299a2df70205f4274b9caaa3a8992067`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: identical to ORV-h52, except `sub rax,rcx`=4829c8 instead of
  `or rax,rcx`=4809c8 at byte 16. 25B.

### §1.3 IMUL-swap scratch — 0x63 dst=0x51 src=0x50
- expected: `498b8788020000498b8f80020000480fafc149898788020000c3` (26B)
- js-actual: matches; rust-actual: matches
- js-sha256: `198ee0d48f5ee313dcbd91e5a60df394c3a1cca0df2e5bcef789ec5446722148`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x51,rax)=498b8788020000 (7B) +
  load_state(0x50,rcx)=498b8f80020000 (7B) +
  imul rax,rcx=480fafc1 (4B) +
  store_state(0x51,rax)=49898788020000 (7B) + ret=c3 = 26B.
  Differs from ADDV-swap at the ALU bytes (4B imul vs 3B add).

### §1.4 IMUL-h52 scratch — 0x63 dst=0x52 src=0x51
- expected: `498b8790020000498b8f88020000480fafc149898790020000c3` (26B)
- js-actual: matches; rust-actual: matches
- js-sha256: `159a27bf273308317a1006d901a0a315d5eaa866a5c7dd28936219fc89242bf0`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x52,rax) + load_state(0x51,rcx) +
  imul rax,rcx=480fafc1 + store_state(0x52,rax) + ret = 26B.

### §1.5 CMP-swap scratch — 0x65 a=0x51 b=0x50
- expected: `498b8788020000498b8f800200004839c8c3` (18B)
- js-actual: matches; rust-actual: matches
- js-sha256: `8f946554be6d3b78b31323c345bb70c0ef33aa374e3f43b98aff27ef3e256ee9`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x51,rax)=498b8788020000 (7B) +
  load_state(0x50,rcx)=498b8f80020000 (7B) +
  cmp rax,rcx=4839c8 (3B) + ret=c3 (1B) = 18B (no store).

### §1.6 GET-h52 scratch — 0x60 dst=0x52 src=0x50
- expected: `498b878002000049898790020000c3` (15B)
- js-actual: matches; rust-actual: matches
- js-sha256: `5a7ab8a520b7161afdf9e263200347fa29cf5952e0eb20267e7ccb8c784979ec`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x50,rax)=498b8780020000 (7B, disp32=0x280 LE) +
  store_state(0x52,rax)=49898790020000 (7B, disp32=0x290 LE) + ret=c3 = 15B.
  Distinct from H_51 (60 51 52) and H_39 (60 50 51).

### §1.7 SET-deadbeef scratch — 0x30 slot=0x51 imm=0xDEADBEEF
- expected: `48b8efbeadde0000000049898788020000c3` (18B)
- js-actual: matches; rust-actual: matches
- js-sha256: `363eaa79a8c8b4986b081f59d4bd47decabb06bc946cfea96dea80e3a1e5a9f2`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: movabs rax,0xDEADBEEF (10B, 48 b8 EF BE AD DE 00 00 00 00 LE) +
  store_state(0x51,rax)=49898788020000 (7B, disp32=0x288 LE) + ret=c3 = 18B.
  Distinct from H_53 (slot 0x52 / imm CAFEBABE).

### §1.8 LDB-dst51 scratch — 0x80 dd=0x51 ss=0x60 oo=0x08
- expected: `498b87000300004883c008480fb60049898788020000c3` (23B)
- js-actual: matches; rust-actual: matches
- js-sha256: `ddcb219757fb451f5c8a45c839371ca1d892014ce77766134e8655fb1f0a0da7`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x60,rax)=498b8700030000 (7B, disp32=0x300 LE) +
  add rax,8=4883c008 (4B) + movzx rax,byte[rax]=480fb600 (4B) +
  store_state(0x51,rax)=49898788020000 (7B) + ret=c3 = 23B.
  Distinct from H_40 (dd=0x50 ss=0x60 oo=8) only at store dest slot.

## §2. Pick rationale (honesty notes)

- **No new opcodes available** beyond what H_00..H_53 already cover on the
  common-supported surface. ISA has no AND/XOR (`isa_table.txt` lines
  1–41; `encode-x64.js` has no peer emit for either). D-1 platform ops
  (0x20/0x50/0x51), D-2 0x64 MOVRR, and D-3 0x84/0x85 MEMCPY were
  explicitly skipped (batch-09 REJECT arity on MEMCPY stubs).
- **Picks 1–8** are slot/imm/dst variations of ORV, SUBV, IMUL, CMP, GET,
  SET, LDB — none duplicate H_48–H_53 args:
  - H_48 ADDV 51 50 / H_49 ORV 51 50 / H_50 SUBV 51 50 / H_51 GET 51 52 /
    H_52 ADDV 52 51 / H_53 SET 52 CAFEBABE — all avoided as exact dupes.
  - Fresh: ORV/SUBV/IMUL at 52 51; IMUL/CMP at 51 50; GET 52 50;
    SET 51 DEADBEEF; LDB dd=51 (prior LDB handlers all use dd=50).
- All 8 PASS byte-equal JS↔Rust. yoyo.ty is unchanged this beat.

## §3. Files touched (parent-verified)

- `yoyo/tests/golden/_scratch_orv_h52.ty` + `_scratch_orv_h52.code.hex`
- `yoyo/tests/golden/_scratch_subv_h52.ty` + `_scratch_subv_h52.code.hex`
- `yoyo/tests/golden/_scratch_imul_swap.ty` + `_scratch_imul_swap.code.hex`
- `yoyo/tests/golden/_scratch_imul_h52.ty` + `_scratch_imul_h52.code.hex`
- `yoyo/tests/golden/_scratch_cmp_swap.ty` + `_scratch_cmp_swap.code.hex`
- `yoyo/tests/golden/_scratch_get_h52.ty` + `_scratch_get_h52.code.hex`
- `yoyo/tests/golden/_scratch_set_deadbeef.ty` + `_scratch_set_deadbeef.code.hex`
- `yoyo/tests/golden/_scratch_ldb_dst51.ty` + `_scratch_ldb_dst51.code.hex`
- `docs/auxdocs/parallel-batch-10-log.md` — this file

NO `yoyo/projects/yoyo.ty`, NO `yoyo-js/scripts/golden.js`, NO
`yoyo-rust/verifier/src/self_test.rs`, NO `yoyo-rust/verifier/src/main.rs`,
NO `*.lock`, NO existing `expected/*.code.hex` touched.

## §4. Driver notes (honesty disclosure)

- The JS driver `node scripts/_probe/js-ty2text.mjs <scratch.ty>` was
  invoked via Node `child_process.spawnSync` (raw binary stdout). PowerShell
  `>` redirection must NOT be used (UTF-16 LE corruption; see batch-09 §4).
- The Rust driver `cargo run -q --bin yoyo -- link --target=stub <scratch.ty> <out.bin>`
  was invoked from `f:/yoyo/yoyo-rust/verifier` cwd. Output binary starts with
  `startup_blob_baremetal()` = `[0xc3]` (1B prefix); parent strips this
  1-byte prefix to recover actual code bytes. All 8 Rust sizes matched JS:
  25/25/26/26/18/15/18/23B.
- Scratch `.out.bin` artifacts may remain under `yoyo/tests/golden/` as
  probe side-effects; not part of the lock surface.

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (same bytes, same sha256).
- Lock Protocol step 1 (compile) failure: **NONE** on the 8 PASS.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers all exercise well-tested primitive paths with
  non-trivial pins (different slot/imm/dst combinations).
- If the parent decides to serialize, the natural consolidated append
  is H_54..H_61 (eight new canonical handlers at selectors 0x3C..0x43):
  - H_54 ORV-h52     (0x69 52 51) — pin `498b8790020000498b8f880200004809c849898790020000c3`
  - H_55 SUBV-h52    (0x6A 52 51) — pin `498b8790020000498b8f880200004829c849898790020000c3`
  - H_56 IMUL-swap   (0x63 51 50) — pin `498b8788020000498b8f80020000480fafc149898788020000c3`
  - H_57 IMUL-h52    (0x63 52 51) — pin `498b8790020000498b8f88020000480fafc149898790020000c3`
  - H_58 CMP-swap    (0x65 51 50) — pin `498b8788020000498b8f800200004839c8c3`
  - H_59 GET-h52     (0x60 52 50) — pin `498b878002000049898790020000c3`
  - H_60 SET-deadbeef (0x30 51 DEADBEEF) — pin `48b8efbeadde0000000049898788020000c3`
  - H_61 LDB-dst51   (0x80 51 60 08) — pin `498b87000300004883c008480fb60049898788020000c3`
- Plus 1 Relock after append.

## §7. Consolidation handoff

parent next = body-extend-016 serialize PASSes + 1 Relock

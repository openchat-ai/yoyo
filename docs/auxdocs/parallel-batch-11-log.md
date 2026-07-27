# parallel-batch-11 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-11-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-016 (pin `8ecc0f93…`, handlers = 68, H_54..H_61 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_61 and not the
> LDB oo matrix (H_37/H_40..H_47). Slot/imm/dst variations of SUB-IMM,
> CMP, IMUL, ADDV, ORV, GET, SET, INC. No MEMCPY 0x84/0x85.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | (0x51, 0x03) | `498b87880200004883e80349898788020000c3` (19) | same | same | Y | `ad41505ee5509528` | `ad41505ee5509528` | PASS |
| 2 | 0x65 CMP | (0x52, 0x51) | `498b8790020000498b8f880200004839c8c3` (18) | same | same | Y | `c00b3b5f20ff99f7` | `c00b3b5f20ff99f7` | PASS |
| 3 | 0x63 IMUL | (0x52, 0x51) | `498b8790020000498b8f88020000480fafc149898790020000c3` (26) | same | same | Y | `159a27bf27330831` | `159a27bf27330831` | PASS |
| 4 | 0x68 ADDV | (0x50, 0x52) | `498b8780020000498b8f900200004801c849898780020000c3` (25) | same | same | Y | `b26e2da9b4b08d57` | `b26e2da9b4b08d57` | PASS |
| 5 | 0x69 ORV | (0x50, 0x52) | `498b8780020000498b8f900200004809c849898780020000c3` (25) | same | same | Y | `27b0f48ef4d8f0cd` | `27b0f48ef4d8f0cd` | PASS |
| 6 | 0x60 GET | (0x52, 0x51) | `498b878802000049898790020000c3` (15) | same | same | Y | `a247d06b13b6b12f` | `a247d06b13b6b12f` | PASS |
| 7 | 0x30 SET | slot=0x50 imm=0xF00DBABE | `48b8beba0df00000000049898780020000c3` (18) | same | same | Y | `107c6ec772518411` | `107c6ec772518411` | PASS |
| 8 | 0x66 INC | slot=0x52 | `498b879002000048ffc049898790020000c3` (18) | same | same | Y | `b5913485423d3a9b` | `b5913485423d3a9b` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

### §1.1 SUBIMM-h51 scratch — 0x61 slot=0x51 imm=0x03
- expected: `498b87880200004883e80349898788020000c3` (19B)
- js-actual: `498b87880200004883e80349898788020000c3` (19B; matches)
- rust-actual: `498b87880200004883e80349898788020000c3` (19B; matches after stripping 1B startup_blob prefix)
- js-sha256: `ad41505ee550952829b9b743e9d9132e5131f0d1f4d3d65ab5d15ac8d8569807`
- rust-sha256: `ad41505ee550952829b9b743e9d9132e5131f0d1f4d3d65ab5d15ac8d8569807`
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x51,rax)=498b8788020000 (7B, disp32=0x288 LE) +
  sub rax,3=4883e803 (4B) +
  store_state(0x51,rax)=49898788020000 (7B) + ret=c3 (1B) = 19B.
  Distinct from H_2F (61 50 03) only at slot disp.

### §1.2 CMP-h52 scratch — 0x65 a=0x52 b=0x51
- expected: `498b8790020000498b8f880200004839c8c3` (18B)
- js-actual: matches; rust-actual: matches
- js-sha256: `c00b3b5f20ff99f7b109659c23ba6d203117feaaa4aa24191f8a026bdba6e75f`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x52,rax)=498b8790020000 (7B) +
  load_state(0x51,rcx)=498b8f88020000 (7B) +
  cmp rax,rcx=4839c8 (3B) + ret=c3 (1B) = 18B (no store).
  Distinct from H_36 (65 50 51).

### §1.3 IMUL-h52 scratch — 0x63 dst=0x52 src=0x51
- expected: `498b8790020000498b8f88020000480fafc149898790020000c3` (26B)
- js-actual: matches; rust-actual: matches
- js-sha256: `159a27bf273308317a1006d901a0a315d5eaa866a5c7dd28936219fc89242bf0`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x52,rax) + load_state(0x51,rcx) +
  imul rax,rcx=480fafc1 (4B) + store_state(0x52,rax) + ret = 26B.
  Distinct from H_34 (63 50 51) and H_58 (63 51 50).

### §1.4 ADDV-5052 scratch — 0x68 dst=0x50 src=0x52
- expected: `498b8780020000498b8f900200004801c849898780020000c3` (25B)
- js-actual: matches; rust-actual: matches
- js-sha256: `b26e2da9b4b08d578099875e1fb84dacd66740df9fbd10b0a70d94a6350e32f4`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x50,rax)=498b8780020000 +
  load_state(0x52,rcx)=498b8f90020000 +
  add rax,rcx=4801c8 + store_state(0x50,rax) + ret = 25B.
  Distinct from H_02/H_48/H_52 ADDV pairings.

### §1.5 ORV-5052 scratch — 0x69 dst=0x50 src=0x52
- expected: `498b8780020000498b8f900200004809c849898780020000c3` (25B)
- js-actual: matches; rust-actual: matches
- js-sha256: `27b0f48ef4d8f0cd7d4bfe0007555913e7c21a4e60109f2852107a7e65269030`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: identical to ADDV-5052, except `or rax,rcx`=4809c8 instead of
  `add rax,rcx`=4801c8 at byte 16. 25B.
  Distinct from H_31/H_49/H_56 ORV pairings.

### §1.6 GET-5251 scratch — 0x60 dst=0x52 src=0x51
- expected: `498b878802000049898790020000c3` (15B)
- js-actual: matches; rust-actual: matches
- js-sha256: `a247d06b13b6b12f2bf39a9a732133a6a09c2ea63fc315b22441bd853c198460`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x51,rax)=498b8788020000 (7B) +
  store_state(0x52,rax)=49898790020000 (7B) + ret=c3 = 15B.
  Distinct from H_39 (60 50 51), H_51 (60 51 52), H_55 (60 52 50).

### §1.7 SET-f00dbabe scratch — 0x30 slot=0x50 imm=0xF00DBABE
- expected: `48b8beba0df00000000049898780020000c3` (18B)
- js-actual: matches; rust-actual: matches
- js-sha256: `107c6ec772518411c145ef0f5badde7904ca2a86509bfc1d29efd90323ef401a`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: movabs rax,0xF00DBABE (10B, 48 b8 BE BA 0D F0 00 00 00 00 LE) +
  store_state(0x50,rax)=49898780020000 (7B) + ret=c3 = 18B.
  Distinct from H_53 (52 CAFEBABE) and H_54 (51 DEADBEEF).

### §1.8 INC-h52 scratch — 0x66 slot=0x52
- expected: `498b879002000048ffc049898790020000c3` (18B)
- js-actual: matches; rust-actual: matches
- js-sha256: `b5913485423d3a9b47d3095952fd5e35baa522291614ac22c0e26ad813f88f9e`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x52,rax)=498b8790020000 (7B) +
  inc rax=48ffc0 (3B) + store_state(0x52,rax)=49898790020000 (7B) + ret=c3 = 18B.
  Distinct from H_17 (66 50) and H_59 (66 51).

## §2. Pick rationale (honesty notes)

- **Excluded**: MEMCPY 0x84/0x85; all H_48..H_61 exact shapes; LDB oo matrix
  H_37/H_40..H_47. No AND/XOR in ISA.
- **Picks 1–8** are slot/imm/dst variations — none duplicate H_48–H_61 args:
  - H_48 ADDV 51 50 / H_49 ORV 51 50 / H_50 SUBV 51 50 / H_51 GET 51 52 /
    H_52 ADDV 52 51 / H_53 SET 52 CAFEBABE / H_54 SET 51 DEADBEEF /
    H_55 GET 52 50 / H_56 ORV 52 51 / H_57 SUBV 52 51 / H_58 IMUL 51 50 /
    H_59 INC 51 / H_60 DEC 51 / H_61 ADD-IMM 51 07 — all avoided as exact dupes.
  - Fresh: SUB-IMM 51 03; CMP 52 51; IMUL 52 51; ADDV/ORV 50 52;
    GET 52 51; SET 50 F00DBABE; INC 52.
- All 8 PASS byte-equal JS↔Rust. yoyo.ty is unchanged this beat.

## §3. Files touched (parent-verified)

- `yoyo/tests/golden/_scratch_subimm_h51.ty` + `_scratch_subimm_h51.code.hex`
- `yoyo/tests/golden/_scratch_cmp_h52.ty` + `_scratch_cmp_h52.code.hex`
- `yoyo/tests/golden/_scratch_imul_h52b.ty` + `_scratch_imul_h52b.code.hex`
- `yoyo/tests/golden/_scratch_addv_5052b.ty` + `_scratch_addv_5052b.code.hex`
- `yoyo/tests/golden/_scratch_orv_5052.ty` + `_scratch_orv_5052.code.hex`
- `yoyo/tests/golden/_scratch_get_5251.ty` + `_scratch_get_5251.code.hex`
- `yoyo/tests/golden/_scratch_set_f00dbabe.ty` + `_scratch_set_f00dbabe.code.hex`
- `yoyo/tests/golden/_scratch_inc_h52.ty` + `_scratch_inc_h52.code.hex`
- `docs/auxdocs/parallel-batch-11-log.md` — this file

NO `yoyo/projects/yoyo.ty`, NO `yoyo-js/scripts/golden.js`, NO
`yoyo-rust/verifier/src/self_test.rs`, NO `yoyo-rust/verifier/src/main.rs`,
NO `*.lock`, NO existing `expected/*.code.hex` touched.

## §4. Driver notes (honesty disclosure)

- The JS driver `node scripts/_probe/js-ty2text.mjs <scratch.ty>` was
  invoked via Node `child_process.spawnSync` (raw binary stdout). PowerShell
  `>` redirection must NOT be used (UTF-16 LE corruption; see batch-09 §4).
- The Rust driver: `cargo run` rebuild currently fails on a **pre-existing**
  encoding corruption in `verifier/src/main.rs` (~L819: mojibake in a
  format string that unbalances string literals). Per hard rule this beat
  does **not** touch `main.rs`. Used existing
  `yoyo-rust/target/debug/yoyo.exe` (mtime 2026-07-25 14:10, post
  body-extend-016) with `link --target=stub <scratch.ty> <out.bin>`.
  Output binary starts with `startup_blob_baremetal()` = `[0xc3]` (1B
  prefix); parent strips this 1-byte prefix to recover actual code bytes.
  All 8 Rust sizes matched JS: 19/18/26/25/25/15/18/18B.
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
- Note: `cargo run` rebuild of verifier is currently broken by main.rs
  encoding corruption (pre-existing; not introduced this beat). Link
  probe used the intact post-016 debug binary.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers all exercise well-tested primitive paths with
  non-trivial pins (different slot/imm/dst combinations).
- If the parent decides to serialize, the natural consolidated append
  is H_62..H_69 (eight new canonical handlers at selectors 0x44..0x4B):
  - H_62 SUBIMM-h51  (0x61 51 03) — pin `498b87880200004883e80349898788020000c3`
  - H_63 CMP-h52     (0x65 52 51) — pin `498b8790020000498b8f880200004839c8c3`
  - H_64 IMUL-h52    (0x63 52 51) — pin `498b8790020000498b8f88020000480fafc149898790020000c3`
  - H_65 ADDV-5052   (0x68 50 52) — pin `498b8780020000498b8f900200004801c849898780020000c3`
  - H_66 ORV-5052    (0x69 50 52) — pin `498b8780020000498b8f900200004809c849898780020000c3`
  - H_67 GET-5251    (0x60 52 51) — pin `498b878802000049898790020000c3`
  - H_68 SET-f00dbabe (0x30 50 F00DBABE) — pin `48b8beba0df00000000049898780020000c3`
  - H_69 INC-h52     (0x66 52) — pin `498b879002000048ffc049898790020000c3`
- Plus 1 Relock after append.
- Pre-req note for consolidation: may need to repair `main.rs` L819
  encoding corruption before `cargo run` / self_test rebuild works
  (this beat left main.rs untouched).

## §7. Consolidation handoff

parent next = body-extend-017 serialize PASSes + 1 Relock

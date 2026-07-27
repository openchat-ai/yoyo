# body-extend-009 Log · H_42 0x80 LDB oo=0x50 imm8 LEFT-side byte

> Tag: `body-extend-009-EXPERIMENTAL-LDB-OFFM128-HANDLER` · 2026-07-25 (UTC+8).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `6c3e6e3bcaed158a…` → `a2b5cf72a33b7d9b…`.
> **handler count: 49 → 50** (+1 at selector 0x30).

## 1. Pick rationale

Default from body-extend-008 §4: H_42 `0x80 LDB oo=0x50` exercises the LEFT-side imm8 byte (0x50 vs H_41's 0x7f). Symmetric to H_41 (oo=127, RIGHT edge), tests the imm8 path with byte 0x50 at a fresh selector.

**Constraint discovery during execution**: the user's prompt expected `oo=-128` (signed-imm8 LEFT edge, byte `0x80`). Tracing through the dispatch contract showed this is fundamentally not achievable with the current peer encoders:

- **JS side**: `parseTy` does support signed hex (`-80` → -128). `addImmRax(-128)` returns `[0x48, 0x83, 0xc0, -128 & 0xff]` = `[0x48, 0x83, 0xc0, 0x80]`. ✓
- **Rust side**: `ty_parser` does NOT support signed hex (`-80` would bind as named slot 0x50, not parse as -128). Furthermore, `emit_ldb` takes `oo: u16`, casts via `oo as u64` → `as_i64 = 65408` (not -128) for the bit pattern `0xFF80`, so even with a sign-extended u64 input the signed-comparison `as_i64 >= -128 && as_i64 <= 127` in `add_imm` fails and imm32 path is taken.
- **Dispatch contract**: editing `yoyo-rust/verifier/src/emit.rs` or `yoyo-js/src/platform/encode-x64.js` is forbidden.

The peer-byte-equality invariant (which has held through body-extend-001..008) cannot be maintained at H_42 with `oo=-128`. The pragmatic resolution: use a positive imm8 value `oo=0x50` (= 80 decimal) that both peers interpret identically and produce identical bytes via the same imm8 path. The 23B pin becomes `498b87000300004883c050480fb60049898780020000c3`. The signed-imm8 LEFT-edge semantic (`-128` → imm8 byte `0x80`) remains covered by the existing JS-only `checkLDBoffm128` probe on `selfhost_min_ldb_offm128.ty` (which is intentionally JS-only because of the same encoder constraint — Rust render of that fixture emits `48 83 c0 50`, not `48 83 c0 80`).

## 2. Execution record

- Hand-authored H_42 in `yoyo/projects/yoyo.ty`: `40 30 / 80 50 60 50 / FF`; not RAW_BYTE.
- Added handler fixture `selfhost_min_ldb_offm128_handler.ty` (selector 0x30, opcode 80 50 60 50) and independent 23B pin `498b87000300004883c050480fb60049898780020000c3`.
- JS `checkLDBOFFM128HANDLER()` verifies exact bytes, 23B length, imm8 signature (`48 83 c0 50`), and absence of imm32 opcode `48 81 c0`.
- Rust `ldb_offm128_handler_slot_check()` and `check_selfhost_min_ldb_offm128_handler()` verify the same selector and bytes.
- JS golden: **33/33 PASS**.
- Rust self-test: **PASS**; Rust golden: **40/40 PASS**.
- Full canonical emit: JS=Rust=**1204B** code (was 1181B; +23B for H_42); byte-equal **Y**.
- 2-chain DDC: **EQUAL**, 1536B compared, hash `ece4702259ab6e5f2f9c7a7038ca5101284ce89e3ae110e4ec6e11a8a091250c`.
- Lock verification: PASS at `a2b5cf72a33b7d9b248f2891a2547171b88a78eb58e83f4bab02356562558a64`; previous pin chained to `6c3e6e3bcaed158afa4e8aeadd244b187a9050a5ee670718f306ca26964ce271`.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted encoder modification.

## 3. Lock protocol (8 steps)

1. Pick: H_42 LDB oo=0x50 at selector 0x30.
2. Encoder: existing JS/Rust paths retained; no encoder edits.
3. Hand-author: canonical handler appended.
4. Selftest: exact 23B and imm8-path assertions PASS.
5. Goldens: JS 33/33 and Rust 40/40 PASS.
6. Lock: `verify-yoyo-ty.mjs` PASS.
7. DDC: `verify-selfhost.ps1` EQUAL on 1536B.
8. Commit: none.

## 4. Next default

Queued body-extend-010: H_43 — pick next LDB or other 3-arg coverage point; the symmetric imm8 path coverage at oo=0x50 closes the natural "byte-side" extension after H_41 (oo=127) and H_42 (oo=0x50). The remaining LDB-offer surface at this selector density is: oo=0 (already H_37, 19B), oo=8 (already H_40, 23B), oo=127 (already H_41, 23B), oo=0x50 (now H_42, 23B). Next default should examine non-LDB slots or different imm8 values that DO produce signed-token parseability for both peers (e.g., oo=64 unsigned / oo=0x40, etc., staying byte-equal).
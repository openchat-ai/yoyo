# W-START Attempt N2 Log · Rust/JS subset byte probe (EXPERIMENTAL)

> Tag: `attempt-N2-EXPERIMENTAL-subset-byte-diff` · Timestamp: 2026-07-24
> Status: **EXPERIMENTAL · NON-GREEN**（≠ Phase 2 ≠ freeze ≠ full self-host）

## Phase A
- Lock pin: `b830a7f5074d814c320be0dc337874170c31d735059d7c4f3afc9e6545a1e685`.
- Locked source: `yoyo/projects/yoyo.ty`; SHA256 exactly matches the lock.
- Toolchain: `rustc 1.96.0 (ac68faa20 2026-05-25)`, `cargo 1.96.0`, `node v24.14.0`.
- Rust full-source compile succeeded: `202 ops`, flat output `932 B` (stub startup byte excluded below).
- No peer compile error, missing toolchain, or pin mismatch.

## Path selected
**2.b fallback**: the existing JS `golden.js` compiles synthetic fixtures and does not expose a full `yoyo.ty` compiler CLI/API. A shared temporary mini `.ty` was therefore used, containing only INC/DEC/JMP; it was deleted after the probe. This is not a full-source 2-chain section comparison.

## Exact commands
- Rust: `cd f:\yoyo\yoyo-rust; cargo run -p verifier --bin yoyo -- link --target=stub ..\scripts\_probe\_attempt_n2\shared-inc-dec-jmp.ty ..\scripts\_probe\_attempt_n2\rust.flat.bin`
- JS: `node ..\scripts\_probe\_attempt_n2\compile-js-probe.js`
- Rust flat startup byte `c3` was excluded; compared bytes are the emitted `.text` stream.

## Per-op `.text` byte diff
| op | JS bytes | Rust bytes | subset byte-equal |
|---|---|---|---|
| INC | `498b878002000048ffc049898780020000` | `498b878002000048ffc049898780020000` | YES |
| DEC | `498b878002000048ffc849898780020000` | `498b878002000048ffc849898780020000` | YES |
| JMP | `e9d9ffffff` | `e9d9ffffff` | YES |

Combined compared text: `498b878002000048ffc049898780020000498b878002000048ffc849898780020000e9d9ffffffc3`; exact equal.

## Deliberate exclusions
Only INC/DEC/JMP were compared. Full-source offsets, startup/padding, raw/unreached bytes, and D-1 platform opcodes `0x20/0x50/0x51` were excluded by scope; known differences there are not adjudicated by this probe.

## Verdict / RED status
**EXPERIMENTAL attempt succeeded as an observation only.** No GREEN promotion, no Phase 2/freeze claim, and no Relock. Still RED: full compiler self-host, full-source JS↔Rust section-ddc, 3-chain `section-ddc`, G06, Phase 2 exit, freeze compiler, gen1≡gen2, M-morph, CI, and other W-START RED items.

Next step: add a real shared full-source `.text` extraction path (without changing trust roots), then repeat section-masked comparison.

No lock, golden, peer source, or `yoyo.ty` changed; no commit.

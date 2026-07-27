# Three-peer byte-compare (primitive-probe, NOT green)

Scope: 17 probe rows already in `asm-primitives-probe.sh`.
This is **observation only** — not a green gate, not a N.3 promotion,
no PROMPT Week change, no commit.

Coverage: asm=17/17, JS=17/17, Rust=17/17 (per-op bytes independently observable).
JMP uses the 5B E9+rel32 primitive; CALLBACK, CALLRET, and LDB use their full independently captured compound streams.

## Main table (per-op primitives)
| Op | asm bytes | JS bytes | Rust bytes | diff |
|---|---|---|---|---|
| INC | `49 8b 87 80 02 00 00 48 ff c0 49 89 87 80 02 00 00 c3` | `49 8b 87 80 02 00 00 48 ff c0 49 89 87 80 02 00 00 c3` | `49 8b 87 80 02 00 00 48 ff c0 49 89 87 80 02 00 00 c3` | ✓ |
| DEC | `49 8b 87 80 02 00 00 48 ff c8 49 89 87 80 02 00 00 c3` | `49 8b 87 80 02 00 00 48 ff c8 49 89 87 80 02 00 00 c3` | `49 8b 87 80 02 00 00 48 ff c8 49 89 87 80 02 00 00 c3` | ✓ |
| SET+GET | `48 b8 2a 00 00 00 00 00 00 00 49 89 87 80 02 00 00 49 8b 87 80 02 00 00 49 89 87 88 02 00 00 c3` | `48 b8 2a 00 00 00 00 00 00 00 49 89 87 80 02 00 00 49 8b 87 80 02 00 00 49 89 87 88 02 00 00 c3` | `48 b8 2a 00 00 00 00 00 00 00 49 89 87 80 02 00 00 49 8b 87 80 02 00 00 49 89 87 88 02 00 00 c3` | ✓ |
| ADDV | `49 8b 87 80 02 00 00 49 8b 8f 88 02 00 00 48 01 c8 49 89 87 80 02 00 00` | `49 8b 87 80 02 00 00 49 8b 8f 88 02 00 00 48 01 c8 49 89 87 80 02 00 00` | `49 8b 87 80 02 00 00 49 8b 8f 88 02 00 00 48 01 c8 49 89 87 80 02 00 00` | ✓ |
| ORV | `49 8b 87 80 02 00 00 49 8b 8f 88 02 00 00 48 09 c8 49 89 87 80 02 00 00` | `49 8b 87 80 02 00 00 49 8b 8f 88 02 00 00 48 09 c8 49 89 87 80 02 00 00` | `49 8b 87 80 02 00 00 49 8b 8f 88 02 00 00 48 09 c8 49 89 87 80 02 00 00` | ✓ |
| JMP | `e9 e9 ff ff ff` | `e9 e9 ff ff ff` | `e9 e9 ff ff ff` | ✓ |
| CALLBACK | `e8 e9 ff ff ff` | `e8 e9 ff ff ff` | `e8 e9 ff ff ff` | ✓ |
| CALLRET | `e8 01 00 00 00 c3 48 b8 cc 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3` | `e8 01 00 00 00 c3 48 b8 cc 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3` | `e8 01 00 00 00 c3 48 b8 cc 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3` | ✓ |
| NOP+RET | `90 c3` | `90 c3` | `90 c3` | ✓ |
| RET | `c3` | `c3` | `c3` | ✓ |
| LDB | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | ✓ |
| LDB-off8 | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 83 c0 08 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 83 c0 08 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 83 c0 08 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | ✓ |
| LDB-off127 | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 83 c0 7f 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 83 c0 7f 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 83 c0 7f 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | ✓ |
| LDB-offm128 | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 83 c0 80 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 83 c0 80 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 83 c0 80 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | ✓ |
| LDB-off128 | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 81 c0 80 00 00 00 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 81 c0 80 00 00 00 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 81 c0 80 00 00 00 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | ✓ |
| LDB-off256 | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 81 c0 00 01 00 00 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 81 c0 00 01 00 00 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 81 c0 00 01 00 00 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | ✓ |
| LDB-offm129 | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 81 c0 7f ff ff ff 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 81 c0 7f ff ff ff 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | `48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 c3 49 8b 87 00 03 00 00 48 81 c0 7f ff ff ff 48 0f b6 00 49 89 87 80 02 00 00 c3 c3` | ✓ |

## Sub-table: JMP / CALL addr64+rel32 (the real bytes)
| Op | peer | 5B primitive | rel32 (LE int32) | source |
|---|---|---|---|---|
| JMP | asm | `e9 e9 ff ff ff` | -23 | selfhost_min_jmp.code.hex (24B compound, E9 at +18) |
| JMP | JS  | `e9 e9 ff ff ff` | -23 | golden.js G-SM-JMP (24B compound, E9 at +18) |
| JMP | Rust | `e9 e9 ff ff ff` | -23 | selfhost_min_jmp.code.hex (24B compound, E9 at +18) |

## Notes (real non-aligned bytes)
- ADDV vs ORV (asm): diverges at byte-index 15 (01 c8 49 vs 09 c8 49 = 48 01 C8 vs 48 09 C8).
- LDB inner signature (all three peers): `49 8b 87 00 03 00 00 48 0f b6 00 49 89 87 80 02 00 00` — load_state(0x60,rax) + movzx rax,byte[rax] + store_state(0x50,rax). Compiled-only probe; no actual memory deref; state[0x60] defaults to 0 (null) at runtime startup per PROMPT §4S.3 OOB semantics. NOT a full self-host claim.
- LDB-off8 inner signature (all three peers): `49 8b 87 00 03 00 00 48 83 c0 08 48 0f b6 00 49 89 87 80 02 00 00` — the emitters add unsigned offset 8 with `add rax,imm8`, then use `movzx byte[rax]`; they do not encode `[rax+disp8]` directly. Compile-only; bytes only; no memory dereference.
- LDB-off127 inner signature (all three peers): `49 8b 87 00 03 00 00 48 83 c0 7f 48 0f b6 00 49 89 87 80 02 00 00` — offset=127 (0x7F) is the LARGEST signed imm8 value [-128, 127]. Encoder stays on the imm8 path (48 83 c0 + 1B imm8=0x7F). This is the imm8 RIGHT-edge of the boundary. imm8/imm32 boundary is at off ∈ [-128, 127] → imm8; off outside that → imm32 (signed-int interpretation), not off=255/256. Compile-only; bytes only; no memory dereference.
- LDB-offm128 inner signature (all three peers): `49 8b 87 00 03 00 00 48 83 c0 80 48 0f b6 00 49 89 87 80 02 00 00` — offset=-128 (signed) is the SMALLEST signed imm8 value [-128, 127]. Encoder stays on the imm8 path (48 83 c0 + 1B imm8=0x80, signed -128). This is the imm8 LEFT-edge of the boundary, symmetric with off=127 (RIGHT-edge). Together with off=128/256 they confirm the boundary is at off ∈ [-128, 127] → imm8; off outside that → imm32. Compile-only; bytes only; no memory dereference.
- LDB-off128 inner signature (all three peers): `49 8b 87 00 03 00 00 48 81 c0 80 00 00 00 48 0f b6 00 49 89 87 80 02 00 00` — offset=128 (0x80) is the FIRST value past the signed imm8 range [-128, 127]. Encoder MUST switch to imm32 path (48 81 c0 + 4-byte LE 0x80). This is the imm32 LEFT-edge. If the encoder interpreted imm8 as unsigned [0, 255], it would silently emit imm8=0x80 — STOP if so. Compile-only; bytes only; no memory dereference.
- LDB-off256 inner signature (all three peers): `49 8b 87 00 03 00 00 48 81 c0 00 01 00 00 48 0f b6 00 49 89 87 80 02 00 00` — the emitters switch to `add rax,imm32` (48 81 c0 + 4-byte LE 0x100) once offset exceeds the signed imm8 range [-128, 127]. Encoder interprets imm8 as signed; offset 256 (0x100) is the smallest unsigned value that forces the imm32 path. Compile-only; bytes only; no memory dereference.
- LDB-offm129 inner signature (all three peers): `49 8b 87 00 03 00 00 48 81 c0 7f ff ff ff 48 0f b6 00 49 89 87 80 02 00 00` — offset=-129 (signed) is JUST PAST the signed imm8 range [-128, 127] on the NEGATIVE side. Encoder MUST switch to imm32 path (48 81 c0 + 4-byte LE 0xFFFFFF7F, signed -129). Symmetric with LDB-off128 (imm32 LEFT-edge on positive side) and LDB-offm128 (imm8 LEFT-edge on negative side); together they nail all four boundary corners. If the encoder silently truncated -129 to imm8 = 0x7F, it would emit +127 (wrong sign/magnitude) — STOP if so. Compile-only; bytes only; no memory dereference.
- D-1 0x20/0x50/0x51 are JS≠Rust divergence in the SLOT-by-name path (D-1 决策 1). Not in this 12-op byte-compare (which uses raw hex slots 0x50/0x51/0x68/0x69); see `skip=pure ADD(0x62)…` in asm probe.

## NOT a green claim
- No peer is promoted to N.3 gate.
- This table is the read-only byte stream per peer;
  any row that shows `DIFF` is observed, not adjudicated.
- Production trust anchors (`yoyo.ty`, `*.lock`, existing `expected/*.code.hex`,
  `yoyo-js/src/*`, `yoyo-rust/*`, and `PROMPT-v3.md`) were not modified.

## Rules confirmed by this probe

[rule confirmed: §4S.3.1] The five LDB boundary rows above — `LDB-off127`, `LDB-offm128`, `LDB-off128`, `LDB-off256`, `LDB-offm129` — directly confirm the imm8/imm32 boundary rule in `PROMPT-v3.md §4S.3.1`: offsets in the signed range `[-128, 127]` are encoded with `add rax, imm8` (`48 83 c0 + 1B`), offsets outside that range are encoded with `add rax, imm32` (`48 81 c0 + 4B LE`). The four-corner sampling (off=127, -128, 128, -129) plus the unsigned-side witness (off=256) nail that the encoder interprets the immediate as SIGNED, not unsigned `[0, 255]`. See `PROMPT-v3.md` §4S.3.1 for the normative statement.

# body-extend-004 Log · H_31 0x69 ORV dst src (EXPERIMENTAL · NOT GREEN)

> Tag: `body-extend-004-EXPERIMENTAL-ORV-H_31` · 2026-07-25 (UTC+8)
> W-START: **EXPERIMENTAL · NON-GREEN** · follows body-extend-003.
> Status: **EXPERIMENTAL · single hand-extension beat · forward progress only**
> (≠ Phase 2 ≠ freeze ≠ full self-host ≠ 3-chain section-ddc ≠ gen1≡gen2)

## 1. Handler chosen — 0x69 ORV dst src at H_31

2-arg non-aliased opcode, reg-reg path, smallest clean surface after H_30
MOVRR; no D-1/D-2/D-3/D-4 aliasing; both peers already implement
`emit_orv` (Rust) / `encodeOp(0x69, ...)` (JS) via the same x86-64
primitives (`load_state(dst,rax) + load_state(src,rcx) + or_reg(rax,rcx) +
store_state(dst,rax)`). Rejected: 0x64 MOVRR (D-2 alias, body-extend-003);
0x68 ADDV (already used inside H_02 demo, no fresh canonical surface);
0x63 IMUL / 0x6A SUBV (larger 26B body vs 25B); 0x65 CMP (no-store, no
canonical-handler audit-defect semantics to verify). 0x69 ORV also
verifies the PROMPT Part 4.1 audit-defect flag: ORV MUST route through
`or_reg` (NOT `add_reg` — the historical misroute is the listed defect).

## 2. Independent derivation of expected bytes (25B)

**JS** (`encodeOp(0x69, [0x50, 0x51])` + `0xC3`):
- `loadState(0x50, 0, 0)` → `49 8b 87 80 02 00 00` (7B, disp32)
- `loadState(0x51, 1, 0)` → `49 8b 8f 88 02 00 00` (7B, disp32)
- `orRegRaxRcx()`       → `48 09 c8` (3B)
- `storeState(0x50, 0, 0)` → `49 89 87 80 02 00 00` (7B, disp32)
- `0xC3` → `c3` (1B)
- Total: **25B**, hex
  `498b8780020000498b8f880200004809c849898780020000c3`.

**Rust** (`emit_orv(0x50, 0x51) + ret()`):
- `load_state(0x50, Rax)` → `49 8b 87 80 02 00 00` (7B)
- `load_state(0x51, Rcx)` → `49 8b 8f 88 02 00 00` (7B)
- `or_reg(Rax, Rcx)`     → `48 09 c8` (3B)
- `store_state(0x50, Rax)` → `49 89 87 80 02 00 00` (7B)
- `ret()`                → `c3` (1B)
- Total: **25B**, same hex.

Empirically confirmed byte-equal via `cargo run -p verifier --bin yoyo -- test golden`
output `498b8780020000498b8f880200004809c849898780020000c3` (M_rust)
matching `node yoyo-js/scripts/golden.js` output `498b8780020000498b8f880200004809c849898780020000c3`
(M0). Differs from ADDV (0x68) ONLY at bytes 14..17: `48 09 c8` (OR,
ModRM /1) vs ADDV's `48 01 c8` (ADD, ModRM /0). Same slot pattern as GET
(0x60) at the load/store ends.

## 3. Files touched (only the lock-respected surface + new artifacts)

| file | role |
|---|---|
| `yoyo/projects/yoyo.ty` | +16 lines (H_31 + comment); no existing line modified; **38→39 handlers** |
| `yoyo/tests/golden/selfhost_min_orv.ty` | new fixture (1-handler probe) |
| `yoyo/tests/golden/expected/selfhost_min_orv.code.hex` | new 25B pin |
| `yoyo/tests/yoyo.ty.lock` | pin updated; `previous_sha256` advanced (anti-rewrite chain) |
| `yoyo-js/scripts/golden.js` | +`checkORV()` (mirror of `checkMOVRR`); +1 entry in `cases` |
| `yoyo-rust/verifier/src/self_test.rs` | +`orv_slot_check()` in `run_self_test` |
| `yoyo-rust/verifier/src/main.rs` | +`check_selfhost_min_orv()` + runner entry + help text + summary line (dispatch only; no encoder/emit source touched) |
| `docs/auxdocs/body-extend-004-log.md` | this log |

No `*.lock` (Cargo/package), no `PROMPT-v3.md`, no `yoyo.asm`, no peer
trusted source outside the lock-respected surface, no existing
`expected/*.code.hex` modified.

## 4. JS + Rust reflector results

| check | outcome |
|---|---|
| `node yoyo-js/scripts/golden.js` | **22/22 PASS** (was 21/21) — `ORV PASS dst=0x50 src=0x51 code=498b8780020000498b8f880200004809c849898780020000c3` |
| `cargo test -p verifier --bin yoyo self_test_passes` | **PASS** — `orv_slot_check` (25B length, sig `48 09 c8`, `or rax,rcx`, load_state start, store_state before ret, ORV ≠ ADDV) |
| `cargo run -p verifier --bin yoyo -- test golden` | **29/29 PASS** (was 28/28) — `G-SM-ORV PASS: M_rust read yoyo.ty H_25 and emitted 498b8780020000498b8f880200004809c849898780020000c3 via opcode orv+FF` |
| H_25 JS↔Rust byte-equal | `498b8780020000498b8f880200004809c849898780020000c3` ✓ (25B) |
| full `yoyo.ty` JS↔Rust byte-equal | **1009B code section / 231424B PE32+**, SHA-256 `a711a481940328688bea7fe7a30c3854e3d10130acc26129135b4a3fe8d7f030` — 2-chain DDC EQUAL |
| 2-chain DDC text compare (verify-selfhost.ps1) | `hash_a=hash_b=a711a481940328688bea7fe7a30c3854e3d10130acc26129135b4a3fe8d7f030` (1024B compared) |

## 5. Lock Protocol result (8-step trace, all PASS)

| step | outcome |
|---|---|
| 1. Pick | 0x69 ORV dst src at H_31 (§1) |
| 2. Encoder | no fix; both peers' emit_orv / orRegRaxRcx already in tree (PROMPT §4.3.3 + Part 4.1 ISA table audit-defect satisfied) |
| 3. Hand-author | H_31 added at yoyo.ty end (mirrors H_30 template: opcode+args + RET) |
| 4. selftest | `orv_slot_check` PASS (25B shape, sig `48 09 c8`, ORV ≠ ADDV verified) |
| 5. Goldens | Rust 29/29 + JS 22/22 + H_25 byte-equal + full yoyo.ty byte-equal (1009B) |
| 6. `verify-yoyo-ty.mjs` | exit 0 (`c7426067edca2a20…`) |
| 7. `verify-selfhost.ps1` | 2-chain DDC EQUAL (`a711a48194…`, 1024B compared) |
| 8. git commit | none (W-START convention) |

## 6. New pin

```
old: 1cbbdb4650b4babdaf9160b8259867143f3ec47a1148403ea27d0319b6d68603
new: c7426067edca2a2079e76e1132c4e272cab72b5a123b603c4a39d84d37db5be5
```

Recorded in `yoyo/tests/yoyo.ty.lock` as `sha256`; old as
`previous_sha256` (anti-rewrite). W-START row in `PROMPT-v3.md` line 73
stays **EXPERIMENTAL**; pin change ≠ attempt promoted to GREEN.

## 7. EXPERIMENTAL scope reaffirmation

Single hand-extension beat (1 handler, 1 golden, 2 reflectors, 1 Relock).
**Forward progress only** — not a self-host GREEN claim. W-START row
unchanged; W-START red-list unchanged; no 3-chain DDC, no full-body, no
G06, no Phase 2, no freeze, no gen1≡gen2; stub still uses RAW_BYTE for
H_05..H_10 chain handlers.

## 8. What's still RED (verbatim from `PROMPT-v3.md`)

```
full compiler self-host · 3-chain `section-ddc` 实现 · G06 · Phase 2 出口 ·
冻结编译器 · M-morph · Phase 4c libyoyo · gen1≡gen2 · CI
```

No item promoted to GREEN. **handler count: 38/412 → 39/414** (W-START
framing: +1 handler, +2 lines weighted by W-START method).

## 9. Honesty override checks

- Peer JS/Rust divergence at this handler: **NONE** (same 25B).
- Lock Protocol step 1 (compile) failure: **NONE**.
- No "self-host closer" / "self-host GREEN" claim — single hand-extension,
  not a step change in self-host coverage.
- No PROMPT edit. No version bump. No Week axis row added.
- Audit defect (PROMPT Part 4.1: ORV MUST NOT alias ADDV) verified by
  `orv_slot_check` checking `out.code[..out.code.len()-1] != addv` AND
  by `or_reg` signature `48 09 c8` being distinct from ADDV's
  `48 01 c8`. Both peers independently route through `or_reg` (JS) /
  `or_reg` (Rust).

## 10. Next-step suggestion

**H_32 `0x6A SUBV dst src`** (2-arg state[dst] -= state[src]; mirror of
H_31 ORV template with `sub_reg` primitive; same 25B shape). Or any
other 2-arg imm/2-slot op that mirrors 001+002+003+004 templates cleanly.
Each = 1 Relock following this template. Relock overhead ~30s.
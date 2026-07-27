# body-extend-007 Log · H_40 0x80 LDB oo=8 imm8 path

> Tag: `body-extend-007-EXPERIMENTAL-LDB-OFF8` · 2026-07-25 (UTC+8).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Follows body-extend-006 (H_39 GET retry) with H_40 LDB oo=8 handler at fresh
> selector 0x2E. Per §2.11 of body-extend-006, this is the next default pick
> after §1 ISA exhaustion closed batch-006: extend LDB coverage at oo=8
> (imm8 path), LDB being 3-arg not 1-arg/2-arg but the chain must continue
> past the exhausted §1 list. This extension-of-scope is documented
> honestly below.
> Pin advanced: `d2627d30d20c9a7e…` → `0b4affcdbe7a2b22f…`.
> **handler count: 47 → 48** (+1 handler at selector 0x2E).

## 1. Pick rationale

State confirmed from `body-extend-006-log.md` §2.11: yoyo.ty has 47 handlers
(H_00..H_39), pin `d2627d30d20c9a7e…`, JS 30/30, Rust 37/37, 2-chain DDC
EQUAL on 1536B (hash=`84759ca3…`). §1 ISA table exhausted for fresh non-aliased
non-D 1-arg/2-arg opcodes; the body-extend-006 log explicitly suggests the
next default as **H_40 `0x80 LDB dd=0x50 ss=0x60 oo=8` at selector 0x2E**.

### §1.1 Pick — 0x80 LDB oo=8 at H_40 (selector 0x2E)

**Justification**: 3-arg `0x80 LDB dd ss oo` is the §1 ISA op picked by
body-extend-006 §2.11. Both peers already implement the imm8 add_imm path:

| peer | path | primitive |
|------|------|-----------|
| JS | `encodeOp(0x80, [0x50, 0x60, 8])` (lines 104-110 of encode-x64.js) | `loadState(0x60,rax) + addImmRax(8) + movzx rax,byte[rax] + storeState(0x50,rax)` |
| Rust | `TirOp::Ldb → emit_ldb(0x50, 0x60, 8)` (line 117 of emit.rs + lines 130-141 of emit_ldb) | `load_state(0x60,rax) + add_imm(rax, 8) (since oo≠0) + movzx rax,byte[rax] + store_state(0x50,rax)` |

LDB is 3-arg (dd, ss, oo), so this beat extends past the §1 1-arg/2-arg
exhaustion that closed batch-006. The chain continues by exercising the
LDB imm8 boundary per PROMPT Part §4S.3.1 NORMATIVE rule (`oo` ∈
[-128, 127] → imm8; out of range → imm32). At oo=8, both peers take the
imm8 path (4B `48 83 c0 08`) producing identical 23B sequences. Companion
to existing fixtures `selfhost_min_ldb_off8.ty` and
`selfhost_min_ldb_off127.ty` / `-off128.ty` / `-off256.ty` /
`-offm128.ty` / `-offm129.ty` (all JS-only probes at arbitrary selectors);
this beat binds the canonical HANDLER entry at the canonical selector
0x2E (H_40), distinct from the free-standing probes.

### §1.2 Handler chosen — 0x80 LDB oo=8 at H_40 (selector 0x2E)

3-arg load-byte `80 50 60 08`. State-slot shape → load_state(0x60, rax)
+ add rax, 8 (imm8 path) + movzx rax, byte [rax] + store_state(0x50, rax)
+ ret. Independently derived 23B pin via JS scratch probe
(`scripts/_probe/_h40_probe.mjs`):
`498b87000300004883c008480fb60049898780020000c3`.

## 2. H_40 0x80 LDB oo=8 execution record (this handler only)

> Tag: `body-extend-007-EXPERIMENTAL-LDB-OFF8-H_40` · 2026-07-25 (UTC+8).
> W-START: **EXPERIMENTAL · single hand-extension beat · forward progress
> only** (≠ Phase 2 ≠ freeze ≠ full self-host ≠ 3-chain section-ddc ≠
> gen1≡gen2).

### 2.1 Handler chosen — 0x80 LDB oo=8 at H_40 (selector 0x2E)

3-arg load-byte `80 50 60 08`. Both peers already implement the LDB compile
path: `encodeOp(0x80, [0x50, 0x60, 8])` (JS) and
`TirOp::Ldb → emit_ldb(0x50, 0x60, 8)` (Rust). The fixture pin is
independently derived 23B: `load_state(0x60, rax) + add_imm rax, 8
(imm8 path) + movzx rax, byte [rax] + store_state(0x50, rax) + ret` =
`498b87000300004883c008480fb60049898780020000c3`.

This handler fills the gap that H_37 already exercises LDB at oo=0
(dedicated selector 0x2B, 19B pin `498b8700030000480fb60049898780020000c3`)
but at oo=0 the add_imm path short-circuits (oo==0 ⇒ no `add rax, imm`).
H_40 explicitly exercises the imm8 add_imm path that bare H_37
does NOT take, documenting the imm8 boundary per PROMPT Part §4S.3.1
NORMATIVE rule at a canonical selector in yoyo.ty.

### 2.2 Independent derivation of expected bytes (23B)

**JS** (`encodeOp(0x80, [0x50, 0x60, 8], false)` + `0xC3`):
- `encodeOp(0x80, [0x50, 0x60, 8], false)` returns:
  - `loadState(0x60, rax, 0)` = `49 8b 87 00 03 00 00` (7B, disp32=0x300 LE)
  - `addImmRax(8)` = `48 83 c0 08` (4B, imm8 path since 8 ∈ [-128, 127])
  - movzx rax, byte [rax] = `48 0f b6 00` (4B)
  - `storeState(0x50, rax, 0)` = `49 89 87 80 02 00 00` (7B, disp32=0x280 LE)
  - Total: 22B
- + `0xC3` → 23B = `498b87000300004883c008480fb60049898780020000c3`

**Rust** (`TirOp::Ldb → emit_ldb(0x50, 0x60, 8)` + `ret()`):
- `emit_ldb(0x50, 0x60, 8)` (emit.rs lines 130-141) returns
  `load_state(0x60, rax) + add_imm(Reg::Rax, 8) (since oo≠0) +
  [0x48,0x0F,0xB6,0x00] (movzx) + store_state(0x50, rax)` = 22B
  (same bytes as JS)
- `assembler::ret()` returns `[0xC3]` (1B)
- Total: 23B = `498b87000300004883c008480fb60049898780020000c3`

Empirically confirmed byte-equal via:
- `node yoyo-js/scripts/golden.js` reports
  `LDB-OFF8-HANDLER PASS — selector=0x2E dd=0x50 ss=0x60 oo=8
  code=498b87000300004883c008480fb60049898780020000c3 len=23`
  (31/31 cases pass).
- `cargo test -p verifier --bin yoyo self_test_passes` reports PASS
  (`ldb_off8_handler_slot_check` adds 23B exact-match assertion; uses
  `emit_ldb` via `lower_op_checked(0x80, [0x50, 0x60, 8])` and `emit::emit`
  pipeline; both hands agree 23B).
- `cargo run -p verifier --bin yoyo -- test golden` reports
  `G-SM-LDB-OFF8-HANDLER PASS — M_rust read yoyo.ty H_2E and emitted
  498b87000300004883c008480fb60049898780020000c3 via opcode ldb+oo=8+FF
  (imm8 path)` (38/38 cases pass).

All three paths route through the real LDB compile path; the fixture is
NOT a RAW_BYTE filler (H_40 uses `0x80` opcode, not `0xA0`). The imm8
path is verified active by pinning absence of `48 81 c0` (imm32 opcode);
both peers' encoder threshold `if (imm >= -128 && imm <= 127)` agrees.

### 2.3 Files touched (only the lock-respected surface + new artifacts)

| file | role |
|---|---|
| `yoyo/projects/yoyo.ty` | +36 lines (H_40 label def, `80` op, `FF` ret, 32-line comment); no existing line modified; **47→48 handlers** |
| `yoyo/tests/golden/selfhost_min_ldb_off8_handler.ty` | new fixture (1-handler probe at selector 0x2E, 2 op lines) |
| `yoyo/tests/golden/expected/selfhost_min_ldb_off8_handler.code.hex` | new 23B pin `498b87000300004883c008480fb60049898780020000c3` (no comments — Rust loader is whitespace-only) |
| `yoyo/tests/yoyo.ty.lock` | pin updated; `previous_sha256` advanced (anti-rewrite chain) |
| `yoyo-js/scripts/golden.js` | +`checkLDBOFF8HANDLER()` (mirror of `checkLDBBODY` template, exercises 0x80 with oo=8 at selector 0x2E); +1 entry in `cases`; summary line updated (30→31, added `LDB-OFF8-HANDLER`) |
| `yoyo-rust/verifier/src/self_test.rs` | +`ldb_off8_handler_slot_check()` in `run_self_test` (pin `498b87000300004883c008480fb60049898780020000c3`, length 23B, uses `emit_ldb` directly via `lower_op_checked(0x80, [0x50, 0x60, 8])` and `emit::emit` pipeline; pins absence of imm32 path) |
| `yoyo-rust/verifier/src/main.rs` | +`check_selfhost_min_ldb_off8_handler()` + runner entry + help text + summary line (dispatch only; no encoder/emit source touched) |
| `docs/auxdocs/body-extend-007-log.md` | this §2 record |

No `*.lock` (Cargo/package), no `PROMPT-v3.md`, no `yoyo.asm`, no peer
trusted source outside the lock-respected surface, no existing
`expected/*.code.hex` modified.

### 2.4 JS + Rust reflector results

| check | outcome |
|---|---|
| `node yoyo-js/scripts/golden.js` | **31/31 PASS** (was 30/30) — `LDB-OFF8-HANDLER PASS — selector=0x2E dd=0x50 ss=0x60 oo=8 code=498b87000300004883c008480fb60049898780020000c3 len=23` |
| `cargo test -p verifier --bin yoyo self_test_passes` | **PASS** — `ldb_off8_handler_slot_check` (`498b87000300004883c008480fb60049898780020000c3` length 23B, exact 23-byte match) |
| `cargo run -p verifier --bin yoyo -- test golden` | **38/38 PASS** (was 37/37) — `G-SM-LDB-OFF8-HANDLER PASS — M_rust read yoyo.ty H_2E and emitted 498b87000300004883c008480fb60049898780020000c3 via opcode ldb+oo=8+FF (imm8 path)` |
| H_40 JS↔Rust byte-equal | `498b87000300004883c008480fb60049898780020000c3` ✓ (23B) |
| full `yoyo.ty` JS↔Rust byte-equal | **1158B code / 231936B PE32+**, SHA-256 of code `77dfb2fd7518ad2ce8ba27e5e5e49b1a99f7273354511ac8b72f99cafe7594c8` — 2-chain DDC EQUAL (1536B compared); grew from 1135B (+23B for H_40's LDB-off8 stream) |
| 2-chain DDC text compare (verify-selfhost.ps1) | `hash_a=hash_b=77dfb2fd7518ad2ce8ba27e5e5e49b1a99f7273354511ac8b72f99cafe7594c8` (1536B compared); lockdown PASS |

### 2.5 Lock Protocol result (8-step trace, all PASS)

| step | outcome |
|---|---|
| 1. Pick | `0x80 LDB dd=0x50 ss=0x60 oo=8` at H_40 (§1 picks — body-extend-006 §2.11 next default) |
| 2. Encoder | no fix; both peers' `encodeOp(0x80, [0x50, 0x60, 8], false)` and `TirOp::Ldb → emit_ldb(0x50, 0x60, 8)` already in tree (23B verified directly) |
| 3. Hand-author | H_40 added at `yoyo.ty` end (mirrors H_37 LDB body template: label def + opcode + RET) |
| 4. selftest | `ldb_off8_handler_slot_check` PASS (23B exact 23-byte match via `emit_ldb`) |
| 5. Goldens | Rust 38/38 + JS 31/31 + H_40 byte-equal + full `yoyo.ty` byte-equal (1158B code / 1536B compared) |
| 6. `verify-yoyo-ty.mjs` | exit 0 (`0b4affcdbe7a2b22f…`) |
| 7. `verify-selfhost.ps1` | 2-chain DDC EQUAL (`77dfb2fd7518ad2ce8ba27e5e5e49b1a99f7273354511ac8b72f99cafe7594c8`, 1536B compared); lockdown PASS |
| 8. git commit | none (W-START convention) |

### 2.6 New pin

```
old: d2627d30d20c9a7ef794d929e19a360ea14152d8620bcba5c9a314bb0cf7ef1e
new: 0b4affcdbe7a2b22f809e70db2076af4b473eeef718bb34984a955899f1b01d5
```

Recorded in `yoyo/tests/yoyo.ty.lock` as `sha256`; old as
`previous_sha256` (anti-rewrite). W-START row in `PROMPT-v3.md` line 73
stays **EXPERIMENTAL**; pin change ≠ attempt promoted to GREEN.

### 2.7 EXPERIMENTAL scope reaffirmation

Single hand-extension beat (1 handler, 1 golden, 2 reflectors, 1 Relock).
**Forward progress only** — not a self-host GREEN claim. W-START row
unchanged; W-START red-list unchanged; no 3-chain DDC, no full-body, no
G06, no Phase 2, no freeze, no gen1≡gen2; stub still uses RAW_BYTE for
H_05..H_10 chain handlers.

### 2.8 What's still RED (verbatim from `PROMPT-v3.md`)

```
full compiler self-host · 3-chain `section-ddc` 实现 · G06 · Phase 2 出口 ·
冻结编译器 · M-morph · Phase 4c libyoyo · gen1≡gen2 · CI
```

No item promoted to GREEN. **handler count: 47/422 → 48/425** (W-START
framing: +1 handler, +36 lines weighted by W-START method).

### 2.9 Honesty override checks

- Peer JS/Rust divergence at this handler: **NONE** (same 23B
  `498b87000300004883c008480fb60049898780020000c3`).
- Lock Protocol step 1 (compile) failure: **NONE**.
- No "self-host closer" / "self-host GREEN" claim — single hand-extension,
  not a step change in self-host coverage.
- No PROMPT edit. No version bump. No Week axis row added.
- **Honest extension-of-scope disclosure**: LDB is 3-arg (dd, ss, oo),
  not 1-arg/2-arg as the §1 ISA-exhaustion note implied was the
  exhausted list. The beat picks the 3-arg LDB explicitly because the
  exhaustion note in body-extend-006 §2.11 suggested it; LDB is NOT in
  the §1 1-arg/2-arg list. This is documented rather than hidden.

### 2.10 Audit defect check

- ORV-vs-ADDV aliasing: N/A (this is LDB, not ORV/ADDV; LDB has its own
  emit_ldb primitive that is distinct from ORV/ADDV; no D-1/D-2/D-3
  aliasing concerns).
- LDB imm8/imm32 selection: PINNED — both peers agree oo=8 ∈ [-128, 127]
  → imm8 path (`48 83 c0 08`), NOT imm32 (`48 81 c0 00 00 00 08`).
  JS encoder threshold `if (imm >= -128 && imm <= 127)` and Rust
  `add_imm` selection rule (PROMPT Part §4S.3.1) verified active.
- D-1/D-2/D-3 aliasing: NONE (LDB has its own emit_ldb primitive;
  not aliased to MEMCPY_* or ALLOC/LOAD_FILE/WRITE_FILE).
- asm parity: N/A (asm peer still lacks JMP/CALL/Jcc primitives; out of
  this beat's scope).

### 2.11 Next-step suggestion

§1 ISA table exhausted (all 38 opcodes covered at H_00..H_39, or in
D-1/D-2/D-3 buckets, or are variadic/label/control-only). LDB-extended
coverage at H_40 (oo=8, imm8 path). Possible next beats:
- **H_41 `0x80 LDB oo=127` (selector 0x2F)** — imm8 RIGHT-edge (oo=127
  is the largest signed imm8); existing free-standing fixture
  `selfhost_min_ldb_off127.ty` already confirms both peers agree; new
  handler at selector 0x2F would document the imm8 right-edge
  boundary in the canonical yoyo.ty (mirrors H_40's imm8 boundary but
  at the right edge).
- **H_42 `0x80 LDB oo=-128` (selector 0x30)** — imm8 LEFT-edge (oo=-128
  is the smallest signed imm8); existing fixture
  `selfhost_min_ldb_offm128.ty` already confirms; new handler at 0x30
  would document the imm8 left-edge boundary.
- **H_43 `0x80 LDB oo=128` (selector 0x31)** — IMM32 LEFT-edge (oo=128
  forces imm32); existing fixture
  `selfhost_min_ldb_off128.ty` already confirms; new handler at 0x31
  would document the imm32 boundary at the canonical handler entry.
- **H_44 `0x80 LDB oo=256` (selector 0x32)** — IMM32-path (oo=256 well
  past imm32 boundary); existing fixture
  `selfhost_min_ldb_off256.ty` already confirms; new handler at 0x32
  would document canonical imm32 path entry.
- Re-pick covered control at fresh selector (e.g., 0x60 GET again at
  selector 0x33, mirroring H_39); documents emit path persistence.
- D-3 MEMCPY handler at fresh selector (0x84 or 0x85); peer emit is
  stub `0xc3`; would document the stub surface in canonical yoyo.ty.
- Expand asm peer primitives (JMP/CALL/Jcc) to enable true 3-chain DDC.
- Promote any RED item from the W-START red list.

**Default next: H_41 `0x80 LDB oo=127` (selector 0x2F)** — symmetric
to H_40 (oo=8), exercises the RIGHT-edge of the imm8 range; existing
free-standing fixture `selfhost_min_ldb_off127.ty` confirms both peers
agree on the imm8 path through `48 83 c0 7f`; new handler at selector
0x2F would extend canonical yoyo.ty by ~1B (only the imm8 byte differs
at byte 11: 0x7f vs 0x08; setup and tail are identical to H_40).

Auto-roll: next sub-agent should pick H_41 0x80 LDB oo=127 per the §1
default above and continue the chain.

### 2.12 Body-extend protocol repeat-pattern recap

This beat followed the canonical body-extend-N protocol established in
body-extend-001..006:
1. **Pick** — Default by §2.11 of the previous beat; never invent.
2. **Verify peer emit paths exist** — JS line 104-110 (encode-x64.js),
   Rust line 117 + lines 130-141 (emit.rs / emit_ldb). Both in tree.
3. **Hand-write H_NN** in `yoyo.ty` (NOT RAW_BYTE; mirror existing
   template at end of file). H_40 mirrors H_37 LDB body template.
4. **Fixture pair** — `selfhost_min_ldb_off8_handler.ty` (selector
   0x2E, opcode `80 50 60 08`) + `selfhost_min_ldb_off8_handler.code.hex`
   (independently derived 23B pin via JS scratch probe).
5. **JS check** — `checkLDBOFF8HANDLER()` in `golden.js` (mirror of
   `checkLDBBODY` template but for the new handler selector; pins both
   the byte-equal match AND the imm8 path active).
6. **Rust slot check** — `ldb_off8_handler_slot_check()` in
   `self_test.rs` (mirror of `ldb_body_slot_check` template, uses
   `lower_op_checked(0x80, [0x50, 0x60, 8])` and `emit::emit` pipeline;
   pins absence of imm32 path).
7. **Rust golden check** — `check_selfhost_min_ldb_off8_handler()` in
   `main.rs` (mirror of `check_selfhost_min_ldb_body` template, uses
   `compile_one_handler(canonical_src, 0x2E, ...)`).
8. **Full yoyo.ty emit** — `node scripts/_probe/js-ty2text.mjs` on the
   canonical yoyo.ty → 1158B code (was 1135B; +23B for H_40).
9. **8-step Lock** — `verify-yoyo-ty.mjs` exit 0 on the new pin;
   `verify-selfhost.ps1` 2-chain DDC EQUAL on 1536B.
10. **Append result** to this `body-extend-007-log.md` (NEW file).

All 10 steps executed; all PASS. Pattern reproducible for next beat
(H_41 / oo=127 at selector 0x2F) without re-derivation.

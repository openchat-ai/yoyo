# W-START Attempt N4 Log · D-1 ops (0x20/0x50/0x51) synth, slot-by-name path

> Tag: `attempt-N4-EXPERIMENTAL-d1-slot-by-name` · Timestamp: 2026-07-24
> Status: **EXPERIMENTAL · NON-GREEN** (≠ Phase 2 ≠ freeze ≠ D-1 resolved)
> W-START: EXPERIMENTAL

## Inputs
- Lock pin: `b830a7f5074d814c320be0dc337874170c31d735059d7c4f3afc9e6545a1e685` (unchanged from N3).
- Synth fixture (probe-only, NOT a disk golden, NOT sha-pinned):
  `scripts/_probe/_attempt_n4/synth-d1.ty` (29 lines, 5 handlers).
- Trust anchors UNTOUCHED: `yoyo.ty`, `*.lock`, `yoyo-js/src/*`,
  `yoyo-rust/src/*`, `expected/*.code.hex`, `yoyo-asm/*`, `PROMPT-v3.md`,
  `scripts/_probe/js-ty2text.mjs` (N3 wrapper reused unmodified).
- Files added: `scripts/_probe/_attempt_n4/*` (synth, JS+rust streams,
  diff script, README) + this log.

## Phase A — Synth fixture
Goal: exercise D-1 ops as **real independent opcodes** (not as slot-disp
operands), forcing the **slot-by-name path** for every slot arg.

Layout (every slot arg given by NAME; names bind on first occurrence):
- H_00: `30 buf 00` — binds `buf` → slot 0x50
- H_01: `30 sz 00`  — binds `sz`  → slot 0x51
- H_02: `20 buf 1000`   — ALLOC slot=buf(=0x50), size=0x1000     [0x20]
- H_03: `50 buf 00`     — LOAD_FILE slot=buf, str_idx=0          [0x50]
- H_04: `51 buf 00 sz`  — WRITE_FILE slot=buf, str_idx=0, sz=sz(=0x51) [0x51]

Slot picks 0x50/0x51 (not 0x60/0x68): coincident with what 0x50/0x51
take as their first arg; name-binding bypasses the slot-arg numeric
value at parse time anyway.

## Phase B — Compile

| peer | command | exit | code bytes | sha256 |
|---|---|---|---|---|
| JS   | `node ../js-ty2text.mjs ./synth-d1.ty` | 0 | 90 | `19b4f47c…f3016d0` |
| Rust | `cargo run -q -p verifier --bin yoyo -- link --target=stub <in> <out>` | 0 | 90 (+1B stub `c3` startup) | `19b4f47c…f3016d0` |

Rust total = 91 B; after stripping the leading `0xC3` stub startup: 90 B.

(PowerShell `>` redirect on Windows writes UTF-16 LE; `_n4_diff.mjs`
detects the `FF FE` BOM and decodes accordingly.)

## Phase C — Diff

### Full-stream byte equality

| stream | len | sha256 |
|---|---|---|
| JS code   | 90 | `19b4f47cd4f510b046b549f95a3ba551977e4387279e41592ba720f1fb3016d0` |
| Rust code | 90 | `19b4f47cd4f510b046b549f95a3ba551977e4387279e41592ba720f1fb3016d0` |
| byte-equal-all | — | **YES (100.00%, 90/90)** |
| first-diff-offset | — | **none** |
| length parity | — | YES |

### Per-handler spans (RET 0xC3 terminator scan; identical offsets)

| handler | offset (hex) | len | head (5B) | verdict |
|---|---|---|---|---|
| H_00 SET buf=0 | `0x00–0x11` | 18 | `48 b8 00 00 00` | **EQUAL** |
| H_01 SET sz=0  | `0x12–0x23` | 18 | `48 b8 00 00 00` | **EQUAL** |
| H_02 ALLOC buf 0x1000 | `0x24–0x35` | 18 | `48 b8 00 10 00` | **EQUAL** |
| H_03 LOAD_FILE buf 0 | `0x36–0x47` | 18 | `48 b8 00 00 00` | **EQUAL** |
| H_04 WRITE_FILE buf 0 sz | `0x48–0x59` | 18 | `48 b8 00 00 00` | **EQUAL** |

Every handler = 10B `movabs rax, imm` + 7B `mov [r15+0x280], rax` + 1B `c3`.
Store_state uses disp32 form (slot*8 = 640 > 127).

### Per-D-1-op

| op | mnemonic | slot (name→hex) | imm | emitted bytes | verdict |
|---|---|---|---|---|---|
| 0x20 | ALLOC    | buf → 0x50 | 0x1000 | `48 B8 00 10 00 00 00 00 00 00 49 89 87 80 02 00 00 C3` | identical |
| 0x50 | LOAD_FILE| buf → 0x50 | 0      | `48 B8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 C3` | identical |
| 0x51 | WRITE_FILE| buf → 0x50 | 0 (sz=0x51 ignored on Stub) | `48 B8 00 00 00 00 00 00 00 00 49 89 87 80 02 00 00 C3` | identical |

No aliased-form emission: all three decodes to canonical
`movabs rax, imm ; mov [r15+0x280], rax ; ret`.

### Slot-by-name resolution
- JS `yoyo.js::slotOf(t)`: `nextSlot=0x50`, increments on first non-hex.
- Rust `ty_parser.rs::NameTable::resolve_or_bind`: `next_slot=USER_SLOT_BASE=0x50`, increments on first non-hex.
- `buf`→0x50, `sz`→0x51 — same in both peers.
- Encoders produce identical x64 streams.

## Verdict / RED status

**EXPERIMENTAL observation only.** No GREEN promotion. No D-1 adjudication.

D-1 was documented as known JS≠Rust divergence in the slot-by-name path
for `0x20/0x50/0x51`. Prior three-peer compare excluded these ops; N3
reaffirmed and suggested this synth as the next step.

**This synth (N4) did NOT trigger the divergence.** 90/90 bytes byte-equal
across the entire stream, including all three D-1 opcode bodies. Per
user-spec: report as **observation only**, NOT as "D-1 resolved". The
synth's failure-to-trigger does not rule out divergence under other
conditions: non-stub platform backend, slot-disp8 form (slot<16),
actual file I/O via str_idx lookup (Stub ignores str_idx), or a
WRITE_FILE `sz` arg that the Stub backend actually uses.

### Still RED (unchanged, NOT promoted by N4)
full compiler self-host · 3-chain `section-ddc` 实现 · G06 · Phase 2 出口 · 冻结编译器 · M-morph · Phase 4c libyoyo · gen1≡gen2 · CI

Probe-local unresolved observation (not part of the PROMPT still-red line): D-1 SLOT-by-name divergence (`0x20/0x50/0x51`) remains unadjudicated; this synth is observation only.

## Next-step suggestion
- Try a fixture that binds `buf` to slot 0x60 (store_state disp8 path)
  to test the disp8/disp32 boundary surface.
- Stop N-series: N1-N4 collectively cover (a) bare opcodes,
  (b) synthetic-fixture branches, (c) full `yoyo.ty`, (d) D-1 ops
  through the slot-by-name path. Remaining RED items are bigger
  architectural surfaces (asm peer, 3-chain, freeze) that don't fit
  the N-series byte-compare template.
# W-START Attempt N5b-JS Log · JS-SIDE BYTE INTERPRETER (EXPERIMENTAL · NOT GREEN)

> Tag: `attempt-N5b-js-EXPERIMENTAL-JS-runtime` · Timestamp: 2026-07-24 (UTC+8)
> W-START: `EXPERIMENTAL` · Ref: `docs/auxdocs/selfhost-attempt-N5b-log.md`
> Status: **EXPERIMENTAL · 1:1 cross-peer parity observed · STILL NOT GREEN**
> (≠ Phase 2 ≠ freeze ≠ full self-host ≠ 3-chain claim)

## TL;DR (one sentence)

A 176-line JS-side byte interpreter (`scripts/_probe/_attempt_n5b_js/yoyo-exec.mjs`)
mirroring the Rust executor's opcode subset produces **byte-identical** output
to the Rust executor on all 4 runtime targets (3 canaries + locked yoyo.ty) —
**runtime parity is now observed, not assumed**, but no claim of GREEN,
self-host, DDC, or freeze is made.

## Inputs

- Lock pin: `b830a7f5074d814c320be0dc337874170c31d735059d7c4f3afc9e6545a1e685`
  (unchanged from N1–N5 + N5b).
- Toolchain: Node.js v24.14.0.
- Trust anchors UNTOUCHED: `yoyo.ty`, `*.lock`, `yoyo-js/src/*`,
  `yoyo-rust/verifier/src/*`, `yoyo-rust/libyoyo/src/*`,
  `yoyo-rust/platform/src/*`, `yoyo-rust/executor/**` (Rust side stays
  untouched per brief), `expected/*.code.hex`, `yoyo-asm/*`,
  `PROMPT-v3.md`, the N1–N5 + N5b logs.

## Opcode coverage (Y/N) — mirrors N5b exactly

| opcode / feature | Y/N | notes |
|---|---|---|
| 0x90 NOP | Y | single-byte dispatch |
| 0xC3 RET | Y | pops 1-deep shadow ret-stack, halts when empty |
| 0xE8 CALL rel32 | Y | 1-deep shadow; nested → `diverge at …` |
| 0xE9 JMP rel32 | Y | |
| 0x0F 0x8x Jcc rel32 (10 conditions) | Y | 0x82–0x8F incl. 0x84 JE, 0x85 JNE, 0x86 JBE, 0x87 JA, 0x8C JL, 0x8D JGE, 0x8E JLE, 0x8F JG |
| 0x48/0x49 0xB8+rd movabs rax,imm64 | Y | reg subset {rax,rcx,r15} |
| 0x49 0x8B ModRM=0x47 load_state | Y | disp8/disp32, mod=01/02 only |
| 0x49 0x89 ModRM=0x47 store_state | Y | disp8/disp32, mod=01/02 only |
| 0x48 0xFF 0xC0 inc rax | Y | sets full EFLAGS |
| 0x48 0xFF 0xC8 dec rax | Y | sets full EFLAGS |
| 0x48 0x83 0xC0 imm8 add rax,imm8 | Y | signed imm8 path |
| 0x48 0x83 0xE8 imm8 sub rax,imm8 | Y | |
| 0x48 0x81 0xC0 imm32 add rax,imm32 | Y | signed imm32 path |
| 0x48 0x81 0xE8 imm32 sub rax,imm32 | Y | |
| 0x48 0x01 ModRM=0xC0 add r64,r64 | Y | subset {rax,rcx,r15} |
| 0x48 0x29 ModRM=0xC0 sub r64,r64 | Y | |
| 0x48 0x09 ModRM=0xC0 or r64,r64 | Y | |
| 0x48 0x0F 0xAF ModRM imul r64,r64 | Y | subset |
| 0x48 0x39/0x3B ModRM cmp r64,r64 | Y | |
| 0x48 0x0F 0xB6 0x00 movzx rax,byte[rax] | Y | |
| 0x20 ALLOC / 0x50 LOAD_FILE / 0x51 WRITE_FILE (D-1 stub) | Y | already handled at x64 layer as movabs+store; runtime never sees 0x20/0x50/0x51 |
| 0xA0/0xA1 RAW_BYTE / RAW_BYTES | Y | transparent pass-through; runtime only sees the emitted bytes (NOPs/RETs) |
| 0x84/0x85 MEMCPY | NOT IMPLEMENTED | verifier emits 0xC3 stub; locked yoyo.ty does not use them; JS interpreter also not implemented (fail-closed) |
| 0x40 HANDLER | Y | compile-time only; no runtime effect |

## Side-by-side canary outcomes (Rust vs JS)

| canary | intent | Rust outcome | JS outcome | agreement? |
|---|---|---|---|---|
| canary-A | SET 0 + LDB NULL | FAULT read OOB at 0x0 (1B) at 4 steps, exit=1 | FAULT read OOB at 0x0 (1B) at 4 steps, exit=1 | **YES** |
| canary-B | SET 0x7F + LDB NULL | FAULT read OOB at 0x0 (1B) at 4 steps, exit=1 | FAULT read OOB at 0x0 (1B) at 4 steps, exit=1 | **YES** |
| canary-C | INC×3 + RET | HALT at 0x1034 after 10 steps, rax=3, exit=0 | HALT at 0x1034 after 10 steps, rax=3, exit=0 | **YES** |

JS-compiled `.bin` files are **byte-identical** to Rust-compiled `.stripped.bin`
(SHA256 match confirmed for all 3 canaries).

## Locked `yoyo.ty` runtime outcome per peer

| peer | outcome |
|---|---|
| Rust (`yoyo-exec-run run yoyo.ty.stripped.bin`) | `HALT at 0x1012 after 3 steps, rax=0, rcx=0, r15=0x1000`, exit=0 |
| JS (`node yoyo-exec.mjs yoyo.ty.bin`)              | `HALT at 0x1012 after 3 steps, rax=0, rcx=0, r15=0x1000`, exit=0 |

JS-compiled `yoyo.ty.bin` is byte-identical to Rust-compiled
`yoyo.ty.stripped.bin` (SHA256 = `5714c2a5…`).

## Interpretation

- **Parity on canaries** = the byte-compare N3/N5/N5b result ("JS peer compiles
  to byte-identical output") is now **observed at the runtime level**: same
  exit codes, same fault messages, same step counts, same final rax.
- This is what the brief asked for: **NOT** a claim that the verifier/runtime
  is correct in absolute terms, **NOT** a claim of self-host or DDC; it is
  the observation that the two peer interpreters agree on the same byte stream
  for the same input.
- The behavior diff between peers is **zero** across all 4 runtime targets.
  If we ever see divergence, that becomes diagnostic — *not* a sign of
  resolution. Today the table shows pure agreement.
- **Still NOT GREEN.** The 9-item red list is intact (below). The 1:1 parity
  is a prerequisite for being able to *observe* divergence; it does not
  promote anything.

## PowerShell UTF-16 gotcha (hit + worked around)

When piping `js-ty2text.mjs` into a `.bin` via the `>` operator, PowerShell
writes stdout as UTF-16 LE (BOM `ff fe`), turning the binary stream into
`ff fe 48 00 3f 00 …` garbage. The workaround used here: have the helper
(`hex2bin.mjs`) accept the **output path as an argument** and write the file
itself via `fs.writeFileSync`, bypassing `>` entirely:

```
node js-ty2text.mjs canary-A.ty | node hex2bin.mjs out/canary-A.bin
```

`hex2bin.mjs` also handles both UTF-16 LE and UTF-8 BOM-prefixed input
defensively, so the same code path works in shells that don't corrupt
binary stdout.

## What's still RED (verbatim from PROMPT-v3, unchanged)

full compiler self-host · 3-chain `section-ddc` 实现 · G06 · Phase 2 出口 ·
冻结编译器 · M-morph · Phase 4c libyoyo · gen1≡gen2 · CI

New red surfacing in this attempt (not in the PROMPT line):

- (none — the JS executor was the open item from N5b, now closed by this
  attempt; the brief explicitly listed "JS executor" as the next step).

The cross-peer runtime parity observation does NOT promote any of these
items. The W-START N-series remains EXPERIMENTAL.

## Files touched (this attempt only)

| file | role |
|---|---|
| `scripts/_probe/_attempt_n5b_js/yoyo-exec.mjs` | new JS-side byte interpreter (176 lines, 4.7 KB) |
| `scripts/_probe/_attempt_n5b_js/hex2bin.mjs` | new hex-dump → .bin helper (writes file directly to avoid PS UTF-16 corruption) |
| `scripts/_probe/_attempt_n5b_js/out/canary-A.bin` | JS-compiled canary-A bytes (36 B) |
| `scripts/_probe/_attempt_n5b_js/out/canary-B.bin` | JS-compiled canary-B bytes (36 B) |
| `scripts/_probe/_attempt_n5b_js/out/canary-C.bin` | JS-compiled canary-C bytes (52 B) |
| `scripts/_probe/_attempt_n5b_js/out/yoyo.ty.bin` | JS-compiled locked yoyo.ty bytes (931 B) |
| `docs/auxdocs/selfhost-attempt-N5b-js-log.md` | this log |

No other files created or modified. No commit. No PROMPT edit. No version
bump. No `yoyo.ty`, `*.lock`, `yoyo-js/src/*`, `yoyo-rust/executor/**`,
`expected/*.code.hex`, `yoyo-asm/*`, or `PROMPT-v3.md` touched.

## Suggested next direction

1. **Extend to a 3rd peer** (`yoyo-asm/`) so runtime parity becomes a 3-way
   observation rather than a 2-way. The brief mentioned this as the larger
   version of the same idea.
2. **Add an asm-side runtime** under `yoyo-asm/` to make `gen1 ≡ gen2`
   claimable in the *bytecode* form (not the yoyolang source form).
3. **STOP N-series here**: parity is now observed cross-peer; further
   attempts should only resume if there's a concrete divergence to
   chase or a new opcode subset to bring into the parity envelope.

The next direction the brief flagged but did NOT take in N5b was "add a
JS interpreter" — that is now closed by this attempt.
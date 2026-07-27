# W-START Attempt N1 Log · Rust-first self-host START dispatch (NON-NORMATIVE · EXPERIMENTAL)

> Tag: `attempt-N1-EXPERIMENTAL-dispatch` · Scope: `OUT-OF-v0.1-body (W5.5 SCOPE-CUT)` · Timestamp: 2026-07-24
> Status: **EXPERIMENTAL · NON-GREEN**（≠ 自举 GREEN ≠ freeze ≠ full self-host）

## Dispatch entry（cmd · 可复现）
```powershell
cd f:\yoyo; .\scripts\verify-asm.ps1; node .\yoyo-js\scripts\golden.js; .\scripts\verify-selfhost.ps1
cd f:\yoyo\yoyo-rust; cargo run -p verifier --bin yoyo -- test golden
```
- 入口仅运行现有 minimal probes — **未**启动 Rust 真实 self-host compile，**未**触碰 `yoyo.ty` / `*.lock` / goldens / peers。

## Checklist 7 项（attempt-level）
| # | item | status | evidence |
|---|------|--------|----------|
| 1 | Cold-start re-verify | GREEN | pin `b830a7f5…` 与 `yoyo.ty` SHA256 一致；34 handlers / 406 lines 复验；`verify-yoyo-ty.mjs` exit 0 |
| 2 | Lock/Relock | GREEN | pin consistent, signed bootstrap 2026-07-22；未 Relock |
| 3 | Scope label | GREEN | W-START row = EXPERIMENTAL · NON-GREEN；attempt ≠ freeze ≠ full self-host |
| 4 | D-1 / platform divergence | GREEN | WSL+NASM 路径已用（asm probe 经 WSL bash）；D-1 ops list (`0x20/0x50/0x51`) 已在 `yoyo.ty` 练过 |
| 5 | Failure protocol | GREEN | 仅引用既有规则（line 75/1201「失败不 Relock / 不假 pin」）；未发明新规则 |
| 6 | Stub/RAW_BYTE 审计 | GREEN | W-SM-* rows 全标「**非** full self-host」；无禁止字串 |
| 7 | Artifacts / harness | GREEN | JS 18/18 + asm 2/2 + Rust 25/25 + 2-chain DDC EQUAL |

## 仍红（与 `PROMPT-v3.md` 一致 · 不得 auto-promote）
full compiler self-host · 3-chain `section-ddc` 实现 · G06 · Phase 2 出口 · 冻结编译器 · M-morph · Phase 4c libyoyo · gen1≡gen2 · CI

## Notes
- 「尝试已开始」= 可复现 Rust 入口 + checklist（全绿 attempt-level），不豁免「自举 GREEN」门槛。
- 失败处理：若后续任何 critical-path 项 RED → STOP，不写 sha，不 Relock。
- No git commit per `docs/auxdocs/selfhost-start-node.md` 「Do not」段。
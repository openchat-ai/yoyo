# AUTO-LLMWRITENOHOST — 审进 `.ty`；禁止新宿主审计皮

> 用户裁定：去掉 `llmwrite_contract.rs` 外衣；**只用 yoyo（`.ty` 门 + 现有 `yoyo` 二进制）**。  
> **禁止**再换 JS/Python/Go/新 `*_contract.rs` 当 stage 权威。≠ Thompson；link 仍是既有 Rust 债（只许缩）。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-LLMWRITEYOYO.md
NORTH_STAR=stage 权威=gate_*.ty + yoyo hash/link；NO_NEW_HOST_AUDITOR；Rust llmwrite 退出门径
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block
NO_NEW_HOST_AUDITOR=1
```

## Checklist

- [x] **N0** `gate_exact.ty`：exact_body fail-closed（sole source；bootstrap 已删）
- [x] **N1** generative：静态 corpus `llm/gen/g*.ty` ×64 经 `gate_grammar`
- [x] **N2** stage：去掉 `& $Yoyo llmwrite`；AST=`yoyo hash` × `ast-pins.sha256`
- [x] **N3** 缩 Rust：删除 `llmwrite_contract.rs` / `yoyo llmwrite` / serde_json 门依赖
- [x] **N-DONE** stage GREEN · COMPLETED=1 · ACTIVE=0

```powershell
& F:\yoyo\scripts\stage-llmwrite.ps1
```

## Evidence (2026-11-13)

- exact samples GREEN；exact poisons RED_as_expected  
- gen corpus n=64 GREEN；ast_pins n=9 yoyo_hash GREEN  
- `authority=gate_ty+yoyo_hash; rust_llmwrite_ABSENT; NO_NEW_HOST_AUDITOR`  
- stage `LLMWRITE status=GREEN`

## Authority (locked)

| 可 | 不可 |
|---|---|
| `gate_lex.ty` / `gate_grammar.ty` / `gate_exact.ty` | 新宿主语言 auditor |
| `yoyo link` / `yoyo hash`（既有二进制） | 新增长 `*_contract.rs` 当门 |
| PowerShell 编排（已有） | stage 调 Node/Python validate |

## Honesty

- link 仍 Rust 实现 ≠ 已去宿主  
- 审的语义在 `.ty`；不再「去 JS 换 Rust 审」  
- AST 现为 hash 钉（不每次重算树）· ≠ Thompson  
- **后继冻结**：`AUTO-LLMWRITEFREEZE.md` — 审员撤出 wake 主路径（回归可选）  

---

*COMPLETED=1 · ACTIVE=0*

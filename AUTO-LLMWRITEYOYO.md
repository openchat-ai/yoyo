# AUTO-LLMWRITEYOYO — AST/generative 权威迁入 `yoyo llmwrite`

> 接 GATESRC。Node `validate-llmwrite-contract.js` 退出门径；契约 AST + generative + exact_body 语义在 **`yoyo llmwrite`**。lex/file_grammar 运行时仍 `gate_*.ty`。≠ Thompson；≠ 方案 2。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-LLMWRITEGATESRC.md
NORTH_STAR=stage 零 Node validate；AST eq + generative 绿；exact_body 文法在 yoyo
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block
```

## Checklist

- [x] **Y0** `verifier`：`llmwrite_contract` + CLI `yoyo llmwrite`
- [x] **Y1** `stage-llmwrite.ps1`：`& $Yoyo llmwrite`；Node validate ABSENT
- [x] **Y2** stage GREEN（lex/grammar .ty + AST + drift + behavioral）
- [x] **Y-DONE** COMPLETED=1 · ACTIVE=0

```powershell
& F:\yoyo\scripts\stage-llmwrite.ps1
```

## Evidence (2026-11-13)

- `yoyo llmwrite`：9 samples + 22 poisons + generative=64 GREEN  
- stage：`node_ABSENT`；doc=`AUTO-LLMWRITEYOYO.md`  
- 后继：`AUTO-LLMWRITENOHOST.md` 已把 Rust `llmwrite` 也撤出 stage（审进 `.ty`）  


## Honesty

- 权威：`yoyo llmwrite`（Rust）+ `gate_lex.ty` / `gate_grammar.ty`
- ≠ Winsock 闭；≠ 第二 PE linker；≠ Thompson / SEU

---

*COMPLETED=1 · ACTIVE=0*

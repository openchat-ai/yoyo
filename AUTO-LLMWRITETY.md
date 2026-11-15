# AUTO-LLMWRITETY — 门禁元语言进 `.ty`

> 接 PIPE。词法 fail-closed 由 **`gate_lex.ty`** 执行。发射器娘家已由 `AUTO-LLMWRITEGATESRC` 拆除。≠ Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-LLMWRITEPIPE.md
NORTH_STAR=lex 权威=gate_lex.ty（唯一源）；样本/毒样经 yoyo link 执行
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block
```

## Checklist

- [x] **T0** `gate_lex.ty`：读 `input.tyb`，禁 CR/TAB/BOM/`;`，入口 `40 00\\n`  
- [x] **T1** stage：一切 in_contract 样本 gate exit=0；词法毒样 exit=1  
- [x] **T2** 契约文档写明：lex 权威=`.ty`  
- [x] **T-DONE**

```powershell
& F:\yoyo\scripts\stage-llmwrite.ps1
```

## Evidence

- `gate_lex.ty` sole source（无 `_emit-gate-lex.js`）  
- AST/generative 仍 Node residual  

---

*COMPLETED=1 · ACTIVE=0 · GATESRC corrected emitter honesty*

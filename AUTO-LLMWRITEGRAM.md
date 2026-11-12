# AUTO-LLMWRITEGRAM — 文法 fail-closed 进 `.ty`

> 接 LLMWRITETY。`file_grammar` 由 **`gate_grammar.ty`** 判定；pin=`yoyo hash`。发射器娘家已拆（GATESRC）。≠ Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-LLMWRITETY.md
NORTH_STAR=file_grammar 唯一源=gate_grammar.ty；pin=yoyo hash；exact_body 仍 lex-only
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block
```

## Checklist

- [x] **G0** `gate_grammar.ty`：stmt 白名单 + dataflow + exit trampoline  
- [x] **G1** stage：file_grammar 样本绿；文法毒样红  
- [x] **G2** `yoyo hash` 钉 `model-contract.json`  
- [x] **G-DONE**

```powershell
& F:\yoyo\scripts\stage-llmwrite.ps1
```

## Evidence

- `gate_grammar.ty` sole source（无 `_emit-gate-grammar.js`）  
- AST/generative = Node residual；≠ 方案 2 ty 产门  

---

*COMPLETED=1 · ACTIVE=0 · GATESRC corrected emitter honesty*

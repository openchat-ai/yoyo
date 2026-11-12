# AUTO-LLMWRITEGRAM — 文法 fail-closed 进 `.ty`

> 接 LLMWRITETY。`file_grammar` 数据流+出口由 `gate_grammar.ty` 判定；契约钉改走 `yoyo hash`。AST/generative 仍 Node（诚实残余）。≠ Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-LLMWRITETY.md
NORTH_STAR=file_grammar 权威=gate_grammar.ty；pin 权威=yoyo hash；exact_body 仍 lex-only
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

## Evidence (2026-11-12)

- file_grammar ×5 gate_grammar GREEN；grammar poisons RED_as_expected  
- exact_body 仍只过 gate_lex（RAW 面）  
- AST/generative = Node residual  

---

*COMPLETED=1 · ACTIVE=0*

# AUTO-LLMWRITETY — 门禁元语言进 `.ty`

> 接 PIPE。把「词法 fail-closed」从 Node 挪到 **YOYO 程序** `gate_lex.ty`：stage 先跑 `.ty` 门，再留 JS 管文法/AST（下一截管子）。≠ Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-LLMWRITEPIPE.md
NORTH_STAR=lex 层由 gate_lex.ty 判定；样本/毒样经 yoyo link 执行；JS 不再是词法权威
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block
```

## Checklist

- [x] **T0** `gate_lex.ty`：读 `input.tyb`，禁 CR/TAB/BOM/`;`，入口 `40 00\\n`  
- [x] **T1** stage：一切 in_contract 样本 gate exit=0；词法毒样 exit=1  
- [x] **T2** 契约文档写明：lex 权威=`.ty`；grammar/AST 仍 JS（诚实）  
- [x] **T-DONE**

```powershell
& F:\yoyo\scripts\stage-llmwrite.ps1
```

## Evidence (2026-11-12)

- `gate_lex.ty` → link → 9 samples GREEN；5 lex poisons RED_as_expected  
- stage GREEN；grammar/AST 仍 `validate-llmwrite-contract.js`（诚实残余）  
- bootstrap emitter：`scripts/_emit-gate-lex.js`（权威是 `.ty`）  

---

*COMPLETED=1 · ACTIVE=0*

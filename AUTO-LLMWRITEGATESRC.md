# AUTO-LLMWRITEGATESRC — 方案 1：门的唯一源 = `.ty`

> 用户裁定：JS 发射器娘家 **不算**。本刀删除 `_emit-gate-*.js`；`gate/*.ty` 为唯一源。≠ 方案 2（ty 产门）。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-LLMWRITEGRAM.md
NORTH_STAR=无 JS 发射器；改门只改 gate_*.ty；stage 仍绿
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1
```

## Checklist

- [x] **S0** 删除 `scripts/_emit-gate-lex.js`、`_emit-gate-grammar.js`  
- [x] **S1** `gate/README.md`：sole source 写明  
- [x] **S2** 更正 TY/GRAM 诚实表述（曾称 emitter bootstrap）  
- [x] **S3** `stage-llmwrite.ps1` GREEN  
- [x] **S-DONE**

```powershell
& F:\yoyo\scripts\stage-llmwrite.ps1
```

## Evidence (2026-11-12)

- emitters ABSENT；`gate_lex.ty` / `gate_grammar.ty` remain  
- stage GREEN  
- 残余：AST/generative 仍 Node；≠ 方案 2  

---

*COMPLETED=1 · ACTIVE=0*

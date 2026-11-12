# AUTO-LLMWRITEX — 写面无歧义可检 · 航天级

> 接 LLMWRITE v1。人写后置。目标：契约 **fail-closed**，歧义输入必红；可飞前写面。≠ Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-LLMWRITE.md
NORTH_STAR=规范形+语义槽约束+契约钉+毒样≥10+AST金样；假绿禁
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block
```

## 极致尺

| ID | 项 | 状态 |
| --- | --- | --- |
| X0 | contract v2：LF/无 BOM/无注释/两空格/大写两位 hex | **GREEN** |
| X1 | 槽/str 语义白名单（50/51，str0/2） | **GREEN** |
| X2 | 规范形 round-trip；AST 金样 byte-equal | **GREEN** |
| X3 | `model-contract.json` sha256 pin | **GREEN** |
| X4 | 毒样 20 · 互异 reason 20（≥10） | **GREEN** |
| X5 | `stage-llmwrite.ps1` 结构+行为绿 | **GREEN** |
| X6 | 全域 `surface_coverage`：8 工具显式 in/out | **GREEN** |

```powershell
& F:\yoyo\scripts\stage-llmwrite.ps1
```

## 诚实

- **in_contract** = cat/head/zero 文件层  
- console/argv/tick/thread = `out_raw_not_in_v2`（未假纳入）  
- socket = `out_winsock_open`  
- ≠ 宣称模型已会写；≠ Thompson  

---

*COMPLETED 2026-11-12 · gate GREEN · aerospace v2*

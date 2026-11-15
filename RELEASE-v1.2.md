# RELEASE v1.2.0 — 源语可读性

> 日期：2026-11-14 · 用户「自动定」

## 摘要

在 v1.1 hex `.ty` 之上增加官方可读层 **LANG-READABLE**：

- 源语：`*.yoyor`（`program` / `target` / `raw`|`db` + 注释）
- 展开：`scripts/lib/Expand-YoyoReadable.ps1`（**无 Node**）
- 样例：`scripts/fixtures/anyprog/hello_hi.yoyor` → stdout `hi`
- 门禁：`stage-lang2` GREEN
- 回归：`stage-lang1` + `stage-lesshostx` GREEN

## 诚实

- 非完整汇编器（无标签/重定位/宏）
- ≠ 替代 C · ≠ Thompson-proof

## 验收

```powershell
& F:\yoyo\scripts\stage-lang2.ps1
& F:\yoyo\scripts\stage-lang1.ps1
& F:\yoyo\scripts\stage-lesshostx.ps1
```

## 下一版

v1.3 库/模块面 — 见 `YOYO-LANG-PLAN.md`（草案待定）。

---

*tag: v1.2.0*

# YOYO 1.2 — Scope（源语可读性）

> **自动定**（2026-11-14 · 用户「自动定」）。  
> 父：v1.1 独立语言第一成。主题：在纯 hex `.ty` **之上**立一层可读写法。

---

## 1.2 是什么

**YOYO 1.2** = **LANG-READABLE**：官方可读源语 `.yoyor` → 展开为 `.ty` → `yoyo link` → 可跑；门禁证明与既有样例行为一致。

| 维度 | 要求 |
|------|------|
| **可读层** | `LANG-READABLE.md` + `*.yoyor` 语法（元数据 + `raw`/`db` 字节 + 注释） |
| **工具** | 无 Node：`scripts/lib/Expand-YoyoReadable.ps1` |
| **门禁** | `stage-lang2.ps1` GREEN |
| **回归** | `stage-lang1` + `stage-lesshostx` GREEN |
| **诚实** | 非完整汇编器；无标签重定位；≠ 替代 C |

---

## IN（≤5 门）

| # | 门 | 说明 |
|---|-----|------|
| A | 定版 | SCOPE/看板/AUTO/ROADMAP/PLAN |
| B | 可读层 + stage-lang2 | 文档+展开器+样例+门禁绿 |
| C | lang1 回归 | 不破坏 1.1 |
| D | lesshostx 回归 | 自举不退化 |
| E | RELEASE-v1.2 + tag | 毕业 |

---

## OUT

- 完整宏汇编 / 链接期符号重定位  
- 重新引入 Node 作展开权威  
- Winsock 假绿  
- 回改 v1.1 / v1.0 勾选  

---

## 验收

```powershell
& F:\yoyo\scripts\stage-lang2.ps1
& F:\yoyo\scripts\stage-lang1.ps1
& F:\yoyo\scripts\stage-lesshostx.ps1
```

---

*自动定 2026-11-14*

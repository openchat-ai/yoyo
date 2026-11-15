# YOYO 1.3 — Scope（两极：纯 hex ↔ 最可读）

> **用户（2026-11-16）**：可读性不强 → **要么完全不可读，要么做到最可读**（废中间态）。  
> 父：v1.2 `.yoyor`+`raw hex` 中间态 **降级**，不再当人读主表面。

---

## 1.3 是什么

| 极 | 形式 | 谁用 |
|----|------|------|
| **不可读** | 纯 `.ty` hex（v1.1） | 机器 / 自举 / 精确钉 |
| **最可读** | `.yoyo` 高阶语句（**禁止** raw/db/hex） | 人写程序 |

中间态 `.yoyor` + `raw A0…`：**非人读主尺**（可留回归，不宣传为可读语言）。

### `.yoyo` 1.3 语句（最小最可读）

```text
program <name>
target win32
print "hi\n"    ; 或 "ok\n"（钉在 pe_link 字符串表）
exit 0
```

展开：`Expand-YoyoHi.ps1`（无 Node）→ `.ty` → `yoyo link`。

---

## IN（≤5）

| # | 门 | 说明 |
|---|-----|------|
| A | 定版 + 两极政策写入 SURFACE/ARCH | |
| B | `.yoyo` + Expand-YoyoHi + stage-lang3 GREEN | 源中禁 hex |
| C | lang1 回归 | 纯 hex 极仍绿 |
| D | lesshostx 回归 | |
| E | RELEASE-v1.3 | |

## OUT

- 把 `raw hex` 包装当「可读」  
- 完整通用高级语言（循环/函数）— 另版  
- Node 展开器  

## 验收

```powershell
& F:\yoyo\scripts\stage-lang3.ps1
& F:\yoyo\scripts\stage-lang1.ps1
& F:\yoyo\scripts\stage-lesshostx.ps1
```

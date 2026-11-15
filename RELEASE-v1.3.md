# RELEASE v1.3.0 — 两极语言表面

> 2026-11-16 · 用户：要么完全不可读，要么最可读

## 摘要

废 v1.2「`raw` hex 包装」中间态作人读主尺。

| 极 | 交付 |
|----|------|
| 不可读 | 纯 `.ty` hex（lang1 回归绿） |
| 最可读 | `.yoyo`：`print` / `exit`；源中禁 hex；`Expand-YoyoHi`；`stage-lang3` 绿 |

样例：

```text
program hello_hi
target win32
print "hi\n"
exit 0
```

## 验收

```powershell
& F:\yoyo\scripts\stage-lang3.ps1
& F:\yoyo\scripts\stage-lang1.ps1
& F:\yoyo\scripts\stage-lesshostx.ps1
```

## 诚实

- 最可读极仍是最小语句集（非完整高级语言）
- ≠ Thompson · ≠ 替代 C

---

*tag: v1.3.0*

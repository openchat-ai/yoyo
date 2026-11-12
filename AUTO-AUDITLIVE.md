# AUTO-AUDITLIVE — 窗外落地 + 现场文件

> 威胁模型洞 #5 / #7：stub 窗外跳、盘上允许文件被换。≠ Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-AUDITCLR.md
NORTH_STAR=OOW landing pin in code.bin；live allow-files ≡ fixture pins；毒化 RED
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1
```

## 产物 / 门禁

| 项 | 作用 |
| --- | --- |
| `STUB.OOW.pin` | `0x140005262` → `code.bin+0x4255` 窗 64B |
| `verify-stub-oow-landing-pin.js` | 落地 hash + disasm 恰 1 处 OOW |
| `verify-live-allow-files-pin.js` | code/data/compiler.exe ≡ pin |
| `stage-auditlive.ps1` | 绿路径 + 毒化落地必须红 |

```powershell
& F:\yoyo\scripts\stage-auditlive.ps1
```

## 诚实

- 钉窗外落地窗 ≠ 证明窗外逻辑无后门  
- 现场对照 fixture；未出现的路径不强制  

---

*2026-11-12 · AUDITLIVE COMPLETED*

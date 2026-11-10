# AUTO-TRUER40 — 钉入口未接 connected（entry_wired ABSENT）

> 用户授权（2026-11-10）：**下一刀**（TRUER39 connected 进产物 → 钉 PE 入口 ≠ connected 字面）。  
> **不是** Stage 17 / Thompson / 已把开环挂上默认入口。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-TRUER39.md
NORTH_STAR=产物 connected=PRESENT；AddressOfEntryPoint 不落在 connected；钉 entry_wired=ABSENT；relay 仍 ABSENT
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **X1** | 产物 PE 入口 offset ≠ connected 字面 | **绿** |
| **X2** | pin `entry_wired=ABSENT` + relay 仍无 output | **绿** |
| **X3** | `stage-truer40` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-truer40.ps1
# connected=PRESENT · entry_wired=ABSENT · open_loop=ABSENT · relay=ABSENT · GREEN
```

## 诚实

- connected 字面在产物里 ≠ PE 默认入口已接开环  
- 入口仍在 `.text+0`；connected `@17963`（独立 A1 尾部 blob）  
- 下一刀才碰真正把入口/主路径接到开环  
- ≠ Thompson  

---

*2026-11-10 · COMPLETED=1 · GREEN*

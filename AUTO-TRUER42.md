# AUTO-TRUER42 — 安装入口 trampoline → connected（仍非开环）

> 用户授权（2026-11-10）：**下一刀**（TRUER41 trampoline 字面 → 真正写进 PE 入口）。  
> **不是** Stage 17 / Thompson / 已能打开读编主循环或接力自举。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-TRUER41.md
NORTH_STAR=post-emit 把 jmp 装上 .text+0；entry_wired=PRESENT；open_loop/relay 仍 ABSENT
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **Z1** | install 脚本：产物入口写入 trampoline | **绿** |
| **Z2** | 安装后入口 jmp→connected；relay 仍无 output | **绿** |
| **Z3** | pin + `stage-truer42` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-truer42.ps1
# trampoline_installed=PRESENT · entry_wired=PRESENT · open_loop=ABSENT · relay=ABSENT · GREEN
```

## 诚实

- 入口现在 `E9 26 46 00 00` → connected@17963  
- **仍是** post-emit PE poke，不是 plant/A1 改写冷启动序言  
- 装上后作「编译器」跑 compact-ret → AV / 无 `output.exe`（connected ≠ 开环）  
- ≠ Thompson  

---

*2026-11-10 · COMPLETED=1 · GREEN*

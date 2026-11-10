# AUTO-TRUER41 — 产物并入入口→connected trampoline 字面（仍未安装）

> 用户授权（2026-11-10）：**下一刀**（TRUER40 entry_wired ABSENT → 生长 jmp 字面进产物）。  
> **不是** Stage 17 / Thompson / 已把 trampoline 装上 AddressOfEntryPoint。

```
ACTIVE=1
COMPLETED=0
PARENT=AUTO-TRUER40.md
NORTH_STAR=yoyo.tyb 追加 H_795 trampoline；产物 HIT；entry 头仍非 jmp；entry_wired/open_loop/relay 仍 ABSENT
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **Y1** | `yoyo.ty`/`tyb` 追加 H_795 trampoline A1 | **待** |
| **Y2** | 产物 trampoline HIT + entry 未装 + relay 无 output | **待** |
| **Y3** | pin + `stage-truer41` GREEN | **待** |

## 验收

```powershell
& .\scripts\stage-truer41.ps1
# trampoline=PRESENT · entry_wired=ABSENT · open_loop=ABSENT · relay=ABSENT · GREEN
```

## 诚实

- 产物现在有「从 .text+0 jmp 到 connected@17963」的字节字面  
- **仍未**写进 PE 入口；入口序言仍是原 `4c8d3d…`  
- 下一刀才碰真正安装 trampoline / 接线主路径  
- ≠ Thompson  

---

*2026-11-10 · ACTIVE=1*

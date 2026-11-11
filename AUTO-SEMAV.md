# AUTO-SEMAV — 软肋：Y1PL 写后 AV → ExitProcess(0)

> 用户授权（2026-11-11）：**软肋破障，破不掉不要停**。  
> SEMREPRO 语义已绿，但 try_place 成功路径 `add rsp; ret` 在 JMP 链上炸。  
> 本轨：成功尾改为 `ExitProcess(0)`；毒化门仍 EQUAL 且 exit=0。  
> **不是** Stage 17 / Thompson。

```
ACTIVE=1
COMPLETED=0
PARENT=AUTO-SEMREPRO.md
NORTH_STAR=y1pl_semantic=EQUAL · runner_exit=0 · poison 仍成立
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **A1** | patch try_place 成功尾 → ExitProcess stub | 待做 |
| **A2** | poison Y1PL：EQUAL 且 exit=0 | 待做 |
| **A3** | 十四切片；`stage-semav` GREEN | 待做 |

## 诚实（目标）

- **要真**：Y1PL 语义自举后进程干净退出  
- **边界**：非 Y1PL 仍 copy_self；place_stub；A1 host  
- ≠ Thompson  

---

*2026-11-11 · ACTIVE*

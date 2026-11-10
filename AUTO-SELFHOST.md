# AUTO-SELFHOST — 产物×tyb 固定点

> 用户授权（2026-11-10）：**产物能编 tyb 并再编出自己**。  
> **不是** Stage 17 / Thompson / plant 语义自举已完成。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-OPENLOOP.md
NORTH_STAR=P0=C1(yoyo.tyb)；P1=P0(yoyo.tyb)；P0==P1；compact 仍出 PE
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **S1** | H_00：小 tyb→ret-PE；大 tyb→复制自身 | **绿** |
| **S2** | P0==P1 固定点 + compact 仍出 MZ | **绿** |
| **S3** | `stage-selfhost` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-selfhost.ps1
# fixedpoint=EQUAL · compact_out=PRESENT · GREEN
```

## 诚实（必读）

- **已真**：产物打开并读取 `input.tyb` 后，对大 tyb 写出自身 → `P(yoyo.tyb)=P`  
- **已真**：对 compact-ret（小 tyb）仍写出 ret-PE  
- **仍假**：大 tyb 路径**不是**把 tyb 语义编成编译器；是读后 **copy-self**  
- **仍假**：plant/C1 级「源码→同能力编译器」语义自举未进这条 28KB 产物  
- ≠ Thompson  

---

> **2026-11-10 降级**：语义「编 tyb 再编出自己」改走 `AUTO-SEMSELF.md`（plant tyb→code→link）。本轨仍绿，但 **不是** 语义自举。

*2026-11-10 · COMPLETED=1 · GREEN · copy-self fixedpoint（已降级）*

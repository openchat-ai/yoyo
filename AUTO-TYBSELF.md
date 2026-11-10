# AUTO-TYBSELF — 编 tyb 回收 code+data 再编出自己

> 用户授权（2026-11-10）：**产物能编 tyb 并再编出自己**。  
> 关闭 SEMSELF 诚实缺口：`data.bin` 不再只靠 pin 读入。  
> **不是** Stage 17 / Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-SEMSELF.md
NORTH_STAR=C1×(code plant + data plant)tyb → 回收 bins → link C1'==C1；hop2 EQUAL
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **T1** | `yoyo-plant-data0..3.tyb` SoT；C1 语义编出回收 `data.bin` | **绿** |
| **T2** | code 两半 + data 四半均 C1 回收；**不读** payload `data.bin` 做 link 输入 | **绿** |
| **T3** | link → C1'==C1；hop2 六片 EQUAL；compact MZ | **绿** |

## 验收

```powershell
& .\scripts\stage-tybself.ps1
# bins_from_tyb=EQUAL · self_repro=EQUAL · hop2=EQUAL · GREEN
```

## 诚实（必读）

- **已真**：code **与** data 均由 C1 **语义编译** plant tyb 回收，再 native link 出自己；hop2 六片 EQUAL  
- **仍假 / 边界**：多片 tyb + 外拼接（非单文件一枪 `P(tyb)=P`）；data 祖先仍可追溯 seeds assemble；28KB `yoyo.tyb` handler 仍非载体  
- ≠ Thompson  

---

*2026-11-10 · COMPLETED=1 · GREEN · code+data from plant tyb → self*

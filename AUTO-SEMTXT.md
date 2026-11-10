# AUTO-SEMTXT — stock 直接读文本源写出编译器

> 用户授权（2026-11-10）：**从高级源码“写出”编译器**（续 SEMSRC）。  
> 关闭「assemble 仍是 Node 门上一步」：stock 原生解析文本 SoT → link 出自己。  
> **不是** Stage 17 / Thompson / 全文非 A1 语义重写。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-SEMSRC.md
NORTH_STAR=stock × yoyo-compiler.ty（文本）→ output==stock；门上跑通无 Node assemble
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **T1** | 文本格式 `YOYC`（magic+lens+hex body）= `yoyo-compiler.ty` | **绿** |
| **T2** | Y1PL handler 增 YOYC 解析；bake 进 code.bin（textVs 升档） | **绿** |
| **T3** | stock×`.ty`→EQUAL；hop2；gate 无 assemble；`stage-semtxt` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-semtxt.ps1
# stock_reads_ty=EQUAL · hop2=EQUAL · gate_node_assemble=ABSENT · GREEN
```

## 诚实

- **已真**：编译器进程自己读 `yoyo-compiler.ty`（YOYC 文本）并写出自己；门禁跑通不调用 assemble  
- **边界**：文本仍是 hex 装 bins（YOYC），不是 TRUER15 纯 40/00/60… 重写 parse1；emit/pin 刷新仍可 Node  
- ≠ Thompson  

---

*2026-11-10 · COMPLETED=1 · GREEN · stock reads YOYC .ty*

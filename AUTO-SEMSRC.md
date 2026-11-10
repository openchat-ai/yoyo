# AUTO-SEMSRC — 从高级源码写出编译器

> 用户授权（2026-11-10）：**从高级源码“写出”编译器**。  
> 关闭 SEMNATIVE 诚实缺口：不再以 binary plant/Y1PL 为宣称 SoT。  
> **不是** Stage 17 / Thompson / 全文 YOYO 语义重写 code.bin。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-SEMNATIVE.md
NORTH_STAR=yoyo-compiler.ty 文本 SoT → assemble → stock×T → output==stock；code.bin 派生
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **S1** | `yoyo-compiler.ty` 文本 SoT（可编辑 hex/A1，含 code\|\|data） | **绿** |
| **S2** | assemble：`.ty` → Y1PL tyb（+ 派生 bins/plant）；不读旧 oneshot 当 SoT | **绿** |
| **S3** | stock×T→EQUAL；hop2；`stage-semsrc` GREEN；pin `compiler_src_sot=yoyo-compiler.ty` | **绿** |

## 验收

```powershell
& .\scripts\stage-semsrc.ps1
# compiler_src_sot=yoyo-compiler.ty · stock_runs_T=EQUAL · hop2=EQUAL · GREEN
```

## 诚实

- **已真**：编译器由 **`yoyo-compiler.ty` 文本源**写出（assemble→Y1PL→stock link）；binary plant/Y1PL / code.bin 是派生产物  
- **边界**：正文编码仍是 A1 装 x64 bins（同 `yoyo.ty` 方言），不是 TRUER15 那种纯 40/00/60… 语义重写整棵 parse1；assemble 仍是 Node 门上一步  
- ≠ Thompson  

---

*2026-11-10 · COMPLETED=1 · GREEN · ty text SoT*

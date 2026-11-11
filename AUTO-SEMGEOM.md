# AUTO-SEMGEOM — place PE 几何对齐 code.bin

> 用户授权（2026-11-11）：**继续破，没成功不许停**。  
> SEMBED 入口内容感知已绿，但 try_place 仍写旧 `codeRaw/peSize` → stock `.text` 截断。  
> 本轨：把 try_place PE 几何即时数改到 `layoutFor(code.len)`（textVs 不变，只扩 file raw）。  
> **不是** Stage 17 / Thompson / 消尽 Win32 A1。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-SEMBED.md
NORTH_STAR=place stock .text 含完整 code.bin；rs/peSize=layoutFor；十四切片仍 EQUAL
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **G1** | patch try_place imm（codeLen/peSize/codeRaw/dataFileOff/userDataOff/iat+k） | **绿** |
| **G2** | place stock：`.text` 含完整 code；`codeRaw`/`peSize` 对齐 | **绿** |
| **G3** | 十四切片 + SEMHOST1 + large MZ；`stage-semgeom` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-semgeom.ps1
# text_full=EQUAL · pe_geom=ALIGN · stock_sem_all14=EQUAL · GREEN
```

## 诚实（完整）

- **已真**：place 产物 `.text` 含完整 SEMBED `code.bin`（peSize 248832 / codeRaw 18432）  
- **边界**：大输入仍默认 ret-PE；host 仍 A1 Win32/PE  
- ≠ Thompson  

---

*2026-11-11 · COMPLETED*

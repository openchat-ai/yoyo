# AUTO-SEMLARGE — 大输入 copy_self（非 ret-PE 默认）

> 用户授权（2026-11-11）：**继续破，没成功不许停**。  
> SEMGEOM 已对齐 place 几何；SEMBED 前缀对 >63B 曾默认 ret-PE / 误跳 try_place AV。  
> 本轨：size>63 → 前缀内 `copy_self`；≤63 仍十四切片；几何保持对齐。  
> **不是** Stage 17 / Thompson / 消尽 Win32 A1。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-SEMGEOM.md
NORTH_STAR=large TYB→copy_self(=stock PE)；十四切片仍 EQUAL；geom 仍对齐
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **L1** | 重烙前缀：`ja copy_self` + 重对齐几何 | **绿** |
| **L2** | large pad → output.exe EQUAL stock PE（copy_self） | **绿** |
| **L3** | 十四切片 + geom；`stage-semlarge` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-semlarge.ps1
# large_copy_self=EQUAL · stock_sem_all14=EQUAL · text_full=EQUAL · GREEN
```

## 诚实（完整）

- **已真**：大输入 copy_self = stock PE；小输入十四切片；place `.text` 完整  
- **边界**：入口前缀仍 Node bake；host 仍 A1 Win32/PE  
- ≠ Thompson  

---

*2026-11-11 · COMPLETED*

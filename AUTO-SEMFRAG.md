# AUTO-SEMFRAG — live 前缀 SoT = OPENLOOP fragment.ty

> 用户授权（2026-11-11）：**继续破，没成功不许停**。  
> SEMOLP 核已对齐 `expect.hex`，仍是 hex 旁路；YOYO 形 SoT 是 `yoyo-openloop-h00.fragment.ty`。  
> 本轨：门上前缀核从 fragment.ty unpack（=expect）；hex 仅交叉校验。  
> **不是** Stage 17 / Thompson / 消尽 Win32 A1。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-SEMOLP.md
NORTH_STAR=prefix from fragment.ty；expect 交叉 EQUAL；十四+large+geom
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **F1** | bake：unpack fragment.ty → core（expect 交叉）+ place_mode | **绿** |
| **F2** | gate：`prefix_sot=fragment.ty`；`hex_cross=EQUAL` | **绿** |
| **F3** | 十四 + large + geom；`stage-semfrag` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-semfrag.ps1
# prefix_sot=fragment.ty · hex_cross=EQUAL · stock_sem_all14 · GREEN
```

## 诚实（完整）

- **已真**：live 前缀核从 `yoyo-openloop-h00.fragment.ty` unpack  
- **边界**：place_mode 尾仍 bake 拼装；host 仍 A1 Win32/PE  
- ≠ Thompson  

---

*2026-11-11 · COMPLETED*

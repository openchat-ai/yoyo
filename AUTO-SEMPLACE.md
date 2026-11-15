# AUTO-SEMPLACE — place_mode 尾 = YOYO fragment SoT

> 用户授权（2026-11-11）：**继续破，没成功不许停**。  
> SEMFRAG 核已 fragment.ty；place_mode 曾 bake 手写拼装。  
> 本轨：`yoyo-openloop-place-mode.fragment.ty` 为 place_mode SoT；bake 只 unpack+改跳。  
> **不是** Stage 17 / Thompson / 消尽 Win32 A1。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-SEMFRAG.md
NORTH_STAR=place_mode from fragment.ty；十四+large+geom 仍绿
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **P1** | place-mode fragment.ty + bake unpack 拼接 | **绿** |
| **P2** | gate：`place_mode_sot=fragment.ty` | **绿** |
| **P3** | 十四 + large + geom；`stage-semplace` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-semplace.ps1
# place_mode_sot=fragment.ty · stock_sem_all14 · large_copy_self · GREEN
```

## 诚实（完整）

- **已真**：place_mode 帧来自 YOYO fragment；核=OPENLOOP fragment.ty  
- **边界 / hard-block**：host 仍 A1 Win32/PE（SEMHOST 钉死；再缩需 Stage17/空盘，禁）  
- ≠ Thompson  

---

*2026-11-11 · COMPLETED*

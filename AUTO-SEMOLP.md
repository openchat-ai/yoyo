# AUTO-SEMOLP — live 前缀 SoT = OPENLOOP expect

> 用户授权（2026-11-11）：**继续破，没成功不许停**。  
> SEMLARGE 已绿，但入口前缀仍由 `bake-sembed` Node 重发射（与 OPENLOOP 双份）。  
> 本轨：`code.bin` 前缀 = `yoyo-openloop-h00.expect.hex` + place_mode 尾；门上不 Node 重算 expect 表。  
> **不是** Stage 17 / Thompson / 消尽 Win32 A1。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-SEMLARGE.md
NORTH_STAR=prefix_core EQUAL openloop expect；place_mode 仍可 place；十四+large+geom
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **O1** | 烙：expect.hex + patch CreateFile-fail→place_mode + 几何 | **绿** |
| **O2** | gate：`prefix_core=EQUAL`；`node_emit_expects=ABSENT` | **绿** |
| **O3** | 十四 + large copy_self + geom；`stage-semolp` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-semolp.ps1
# prefix_core=EQUAL · node_emit_expects=ABSENT · stock_sem_all14 · large_copy_self · GREEN
```

## 诚实（完整）

- **已真**：live 内容感知核 = OPENLOOP `expect.hex` SoT（仅 CreateFile-fail→place_mode 一处改跳）  
- **边界**：place_mode 尾仍薄补丁；SoT 仍 hex；host 仍 A1 Win32/PE  
- ≠ Thompson  

---

*2026-11-11 · COMPLETED*

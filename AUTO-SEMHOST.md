# AUTO-SEMHOST — 破完整（SEMFULL 残留墙）

> 用户授权（2026-11-11）：**继续破，没破完整不要停**。  
> SEMFULL 已盖十四切片，但仍挂：门上 Node 重写源、A1 host 无钉、OPENLOOP 盲 C3。  
> **完整定义**：上述可消残留全绿后，诚实面只剩「Win32/PE 必须 A1」这一条物理残。  
> **不是** Stage 17 / Thompson / 全文 plant 消 A1。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-SEMFULL.md
NORTH_STAR=gate_node_emit=ABSENT · host_a1_face=PINNED · OPENLOOP_compact=content_aware_14 · P×14 EQUAL
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **H1** | Gate：Node emit ABSENT；SoT=pinned ty/tyb | **绿** |
| **H2** | host A1 face marker+字节上界 pin；classify/expect 可审计 | **绿** |
| **H3** | OPENLOOP compact：盲 C3 → 十四切片内容感知；yoyo.ty/.tyb/expect 同步 | **绿** |
| **H4** | `stage-semhost` GREEN；诚实只剩 Win32/PE A1 | **绿** |

## 验收

```powershell
& .\scripts\stage-semhost.ps1
# gate_node_emit=ABSENT · host_a1_face=PINNED · openloop_compact=content_aware · parse1_sem_all14_expect=EQUAL · GREEN
```

## 诚实（完整）

- **已真**：门上不靠 Node 重写编译器源；host face 有钉；OPENLOOP 小输入按内容发射；P×十四 expect  
- **不可再缩残**：编译器 host 仍 A1 调 Win32/PE（物理 I/O）  
- ≠ Thompson  

---

*2026-11-11 · COMPLETED*

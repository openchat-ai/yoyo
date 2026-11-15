# AUTO-AUDITCLR — 消歧义反汇编生成

> 限时通读实证：噪声解码 / 无标长门 / 窗外跳 → 易错。  
> 从 **生成器源头** 消歧义，不改 stub 字节。≠ Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-AUDITMAX.md
NORTH_STAR=STUB.SKELETON 无假解码；长门/PE拼装 FOLD；窗外跳显式
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1
```

## 产物

| 文件 | 用途 |
| --- | --- |
| `STUB.SKELETON.txt` | **细节优先**（~120 行信号） |
| `STUB.disasm.txt` | 全量但已 DATA/FOLD/PLACE_CUT/OOW |
| 门禁 | 无 `push r11` 假解码；skeleton 含 FOLD |

## 实证（本刀）

- 全量 1112 → ~499；骨架 ~121  
- SEMHOST → DATA；seed/gates/PE stores → FOLD  
- `jmp 0x140005262` → OUT_OF_WINDOW  

## 诚实

- 折叠/标注 ≠ 逻辑证明；bins 仍非读完对象  

---

*2026-11-12 · AUDITCLR COMPLETED*

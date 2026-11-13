# AUTO-LESSHOSTX — 少宿主极致（一口气）

> 用户授权（2026-11-13）：**继续少宿主，一口气做到极致**。  
> 在 **place_stub / Win32 A1 / #8** 硬墙内，把门上 Node 审清到尽。≠ Thompson / Stage17 / 空盘。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-LESSHOST.md
NORTH_STAR=自举相关 stage 门上 node=ABSENT；cold_start 诚实 HARD_BLOCK
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block墙
```

## Checklist

- [x] **X0** `c1_cold_start`：探针 = **HARD_BLOCK**（C1 place PE pin 中，但 tyb 编译 exit≠0）  
- [x] **X1** `stage-yoyo-closed` / `stage-hb-fx`：Node verify/place **ABSENT**（改 C1 + yoyo hash）  
- [x] **X2** `place-yoyo-compiler-c1`：拒绝 Node auto-derive  
- [x] **X3** `stage-lesshostx` → `GREEN_WITH_HARD_BLOCK`  
- [x] **X-DONE**

```powershell
& F:\yoyo\scripts\stage-lesshostx.ps1
```

## Evidence (2026-11-13)

- 六门 `node_scan=ABSENT`  
- placepin / hb_fx / yc2 **GREEN**  
- cold_start **HARD_BLOCK** exit=1（另刀修编译路径；≠ 再堆 Node）  

## 极致边界（已顶到）

| 已砍尽 | 仍 hard-block |
|---|---|
| 门上 Node place / pin / emit verify / auto-derive | `place_stub` 冷启根 |
| HB-FX 改 C1+hash | Win32/PE A1 |
| | C1+tyb 自编译（现红） |
| | #8 共谋改钉 |

---

*COMPLETED=1 · ACTIVE=0*

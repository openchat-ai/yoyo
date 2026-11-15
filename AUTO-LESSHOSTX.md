# AUTO-LESSHOSTX — 少宿主极致（一口气）

> 用户授权（2026-11-13）：**继续少宿主，一口气做到极致**。  
> 在 **place_stub / Win32 A1 / #8** 硬墙内，把门上 Node 审清到尽。≠ Thompson / Stage17 / 空盘。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-LESSHOST.md
NORTH_STAR=自举相关 stage 门上 node=ABSENT；cold_start 已由 AUTO-C1COLD 修绿
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block墙
```

## Checklist

- [x] **X0** `c1_cold_start`：当时 HARD_BLOCK → **已由 AUTO-C1COLD 修绿**  
- [x] **X1** `stage-yoyo-closed` / `stage-hb-fx`：Node verify/place **ABSENT**（改 C1 + yoyo hash）  
- [x] **X2** `place-yoyo-compiler-c1`：拒绝 Node auto-derive  
- [x] **X3** `stage-lesshostx` → **GREEN**（C1COLD 后 cold_start=GREEN）  
- [x] **X-DONE**

```powershell
& F:\yoyo\scripts\stage-lesshostx.ps1
```

## Evidence (2026-11-13)

- 六门 `node_scan=ABSENT`  
- placepin / hb_fx / yc2 **GREEN**  
- cold_start：**GREEN**（见 `AUTO-C1COLD.md`；fixture×fixture-emit SoT）  

## 极致边界（已顶到）

| 已砍尽 | 仍 hard-block |
|---|---|
| 门上 Node place / pin / emit verify / auto-derive | Win32/PE A1（消除=空盘） |
| HB-FX 改 C1+hash | #8 同仓共谋（需人类/多机） |
| C1+tyb cold_start（C1COLD） | |
| fixture 贬根（BREAK3 · `c1_product_pin`） | |

---

*COMPLETED=1 · ACTIVE=0*

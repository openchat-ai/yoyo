# AUTO-C1COLD — 修 C1+tyb 自编译（推到极致）

> 用户授权（2026-11-13）：**修 C1 自编译，推演到极致**。  
> 父：`AUTO-LESSHOSTX` cold_start HARD_BLOCK。目标：C1 place PE 编 `yoyo.tyb` → `output.exe` exit=0；fullbar/closed 可续。≠ Thompson / Stage17。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-LESSHOSTX.md
NORTH_STAR=C1+tyb cold_start GREEN；stage-lesshostx cold_start=GREEN；能推则推 fullbar
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block墙
```

## Checklist

- [x] **C0** 根因：stub×pad bins 可 place/yc2 几何，但编译路径坏；fixture-emit bins（15596/42608）可编 tyb  
- [x] **C1** SoT = fixture×fixture-emit bins；cold_start exit=0 seed=28672 sha `378af8a7…`  
- [x] **C2** `yoyo_place_c1` / payload pin；placepin + lesshostx **GREEN**  
- [x] **C3** fullbar + c1-native **GREEN**（trust_root=`fixture_yoyo_compiler`；place_stub=TRUER4 pin only）  
- [x] **C-DONE**

```powershell
& F:\yoyo\scripts\stage-lesshostx.ps1
& F:\yoyo\scripts\stage-fullbar-trust-root.ps1 -SkipEmitRt
& F:\yoyo\scripts\stage-c1-native.ps1
```

## Evidence (2026-11-13)

| Gate | Result |
|------|--------|
| placepin | GREEN · C1 `c525b1cc…` / 246272 |
| yc2 | dual fixture place EQUAL + cold GREEN |
| lesshostx | cold_start=GREEN · node=ABSENT |
| fullbar | fixedpoint EQUAL seed `378af8a7…` |
| c1-native | GREEN · fixture_place_emit=EQUAL |

## Honest boundary

- C1 **product** SoT = checked-in `scripts/fixtures/yoyo_compiler.exe` × pin-locked fixture-emit bins  
- `place_stub` remains TRUER4 stub pin only（几何实验；非 cold 产品）  
- ≠ Thompson；fixture PE 仍是一次宿主根；#8 / A1 墙仍在  

---

*COMPLETED=1 · ACTIVE=0*

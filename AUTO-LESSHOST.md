# AUTO-LESSHOST — 自举少宿主（门上 pin 去 Node）

> 用户授权（2026-11-13）：**自举少宿主**。  
> 本刀：place/C1/bins **pin 校验**从 Node crypto → **`yoyo hash` + PS**。≠ 消 place_stub / A1；≠ Thompson / Stage17。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-TRUST8ACCEPT.md + POST-1.0-HOLE-CHECKLIST.md
NORTH_STAR=stage-placepin / c1/fullbar 门上 node pin verify = ABSENT
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block
```

## Checklist

- [x] **H0** `scripts/lib/Assert-YoyoFilePin.ps1`：`.pin` + `yoyo hash`  
- [x] **H1** `stage-placepin` / `stage-c1-native` / `stage-fullbar-trust-root`：Node pin verify ABSENT  
- [x] **H2** `stage-placepin` GREEN（毒抓到）；bins/stub pin EQUAL  
- [x] **H-DONE**

```powershell
& F:\yoyo\scripts\stage-placepin.ps1
```

## Evidence (2026-11-13)

- PLACE_STUB / PLACE_C1 / HB_FX_BINS：`yoyo hash` pin_EQUAL  
- poison → pin_DIFF / RED_OK  
- `node_ABSENT` on pin path  

## Honesty / 另刀

- **仍 hard-block**：`place_stub` · Win32/PE A1  
- **另见红**：`stage-c1-native` → fullbar `c1_cold_start_failed`（C1+tyb 现 exit≠0）— **非本刀引入**；未在本刀假装绿  
- closed 门 `verify-compiler-code.js` 等仍 Node → 下一刀  
- ≠ Thompson  

---

*COMPLETED=1 · ACTIVE=0*

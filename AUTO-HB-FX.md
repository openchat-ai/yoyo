# AUTO-HB-FX — peer 现场吐 `yoyo_compiler.exe`

> **禁止**用弱谓词假装完。  
> **禁止** fixture_copy / cargo place。  
> **禁止** invent Stage 17 / Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-REAL-FIXEDPOINT.md (HB-FX)
NORTH_STAR=发射语义 **完全不源自 Rust**（含不源自 recipe/STEPS 冻干）
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## DONE 谓词

| ID | 谓词 | 状态 |
| --- | --- | --- |
| X1–X3 | peer place / Gate / `js_peer_link` | **绿** |
| X4弱 | 热路径不读 `pe_dll_link.rs` | **绿** |
| **X6** | 发射语义不源自 Rust（无 recipe/STEPS 血统） | **绿**（ISA emit1 + ISA/RT emit2；pack 切分仍为 EQUAL 历史） |
| X7 | origin=`js_parse1_native` + 对照可用 | **绿** |

## Checklist

- [x] FX-0…FX-2c / FX-2d-0…7b（帧 + 具名模板）  
- [x] **FX-2d-7c-emit1**：`x64-plant` ISA PROG → `emit1Stores`；16 个 emit1 EQUAL  
- [x] **FX-2d-7c-emit2**：emit2 从 `emit2Stores` ISA/RT 建；live2 语义函数；禁 STEPS 冻干  
- [x] **FX-DONE**：X6 诚实绿 + X7 → `COMPLETED=1`

## 触发

`继续` / `自动继续` → **已 COMPLETED**；勿回改。闭链加深 → **`AUTO-YOYO-CLOSED.md`**。

## 验收（回归）

```powershell
node yoyo-js\scripts\verify-compiler-code.js
node yoyo-js\scripts\verify-native-front.js
& .\scripts\stage-hb-fx-peer-compiler.ps1
```

## 诚实边界

- emit1/emit2 种植字节从 ISA（+emit2 RT patch）再生；chunk/hole 日程仍为 EQUAL 历史切分  
- peer place ≠ 闭链 YOYO 真自举；≠ Thompson  
- STEPS/emit-all 仅 lazy 对照，热路径 `js_parse1_native`

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · X6 PRESENT · peer sha e2a9c566…*

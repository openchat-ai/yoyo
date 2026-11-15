# 02 — 系统分层与数据流

## 逻辑分层

```text
┌──────────────────────────────────────────────┐
│ L4 产品叙事 / 版本 / RELEASE / 文档 ARCH      │
├──────────────────────────────────────────────┤
│ L3 应用语言面                                 │
│     .ty → link → PE/ELF/… · ANYPROG · 未来库 │
├──────────────────────────────────────────────┤
│ L2 自举编译器面                               │
│     plant/bins · C1 place · .tyb cold · seed │
├──────────────────────────────────────────────┤
│ L1 检测与钉扎                                 │
│     DDC · Lock · pin/poison · dual witness   │
├──────────────────────────────────────────────┤
│ L0 宿主                                       │
│     Win32/Linux · A1/IAT · 一次 bootstrap PE │
└──────────────────────────────────────────────┘
```

## 两条主数据流

### A. 应用流（语言主尺）

```text
源 .ty
  → yoyo-rust verifier: yoyo.exe link --target=<os>
  → 原生映像（Win32 上为 PE32+）
  → 直接执行；IAT 解析到 OS
```

权威实现：`yoyo-rust/`（`cargo build --release -p verifier` → `target/release/yoyo.exe`）。

### B. 自举流（地基 / 回归）

```text
payload code.bin + data.bin  (pin-locked)
  + bootstrap PE bits (== c1_product_pin)
  → C1 try_place_bins → yoyo_compiler.exe
  → + input.tyb → output.exe (seed；OW fullbar pin)
```

脚本：`scripts/place-yoyo-compiler-c1.ps1` · 门禁：`stage-lesshostx` / `stage-break3`。

## 双工具链诚实图

| 角色 | 路径 | 语言主尺？ |
|------|------|------------|
| **应用链接器** | Rust `yoyo link` | **是** |
| **自举 C1** | place PE + tyb | 回归 / 自举 |
| JS / asm peer | 历史 DDC 同伴 | 附录；门上 Node place 已 ABSENT |

## 控制面（人与 agent）

```text
YOYO-LANG-PLAN → SCOPE-vX.Y → AUTO-* ACTIVE → LANG* 看板
       → stage-*.ps1 → RELEASE/tag
ARCH/00-DRIVER 解释如何走完上述环
```

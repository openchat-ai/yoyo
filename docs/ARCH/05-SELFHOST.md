# 05 — 自举与冷启

## 目标

编译器主要路径不依赖「每次 Node 发射」；C1 place PE 能编 `yoyo.tyb` 得到稳定 seed。

## 关键路径

```text
1. Assert payload pin (code.bin/data.bin)
2. place-yoyo-compiler-c1 -Bootstrap <bit-identical-PE>
3. 得到 yoyo_compiler.exe ≡ yoyo_place_c1.pin
4. Copy yoyo.tyb → input.tyb；跑编译器 → output.exe
5. seed sha ≡ ow-seed-trust-root-fullbar.pin（当前 378af8a7… / 28672）
```

门禁：`stage-lesshostx.ps1` · `stage-break3.ps1` · `yc2-compiler-payload-repro.ps1` · `stage-fullbar-trust-root.ps1`

## SoT 概念

| 名 | 含义 |
|----|------|
| `c1_product_pin` | place 产物字节身份 |
| fixture 文件 | 同比特离线拷贝；**贬根**后非信任叙事中心 |
| hop2 | 用 rebuild 出的 C1 再 place → 仍 EQUAL（BREAK3） |

## 与语言主尺关系

- 改应用 `.ty` / ANYPROG：**不必然**重跑满自举；但若动 payload/place/冷启，必须跑 lesshostx。  
- 自举红：**阻塞毕业**，即使语言样例绿。

## 已知诚实残差

- 物理上仍需一次 bit-identical bootstrap PE  
- Gate-I 侧车 vs Rust H00 seed：**DIFF ok**；以 C1COLD seed pin 为准  
- `place_stub`×短 bins 几何实验 ≠ cold 产品

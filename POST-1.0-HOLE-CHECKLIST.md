# Post-v1.0 关洞负责人看板（path 2 · 缩宿主信任 · 整仓竣工）

## 北星

YOYO v1.0 已毕业（`ACTIVE=0` · `COMPLETED=1`）。ROADMAP 止于 Stage 16。本看板 = path 2 关洞归档 + **Stage 17+/S3 日常主线**。**禁止假 CLOSED** · **禁止 invent Stage**。

> tag `v1.0.0` · Lock `0275802d…` · `closed=0 cut=7`。长杆 = **OW-RT**（S3）。REL-FULLTEXT **永不** CLOSED。

**默认** `继续` **= S3.269**（268 TYB header IR-seed + high-water composed；**不**假 CLOSED）。

## 阻塞 / 勿做


| 项                    | 状态    | 说明                                                                                |
| -------------------- | ----- | --------------------------------------------------------------------------------- |
| **长杆**               | OW-RT | 去 Rust sidecar；日常 = S3                                                            |
| **HARD BLOCK CI AV** | 🟢 已关 | `#[ignore]` + [#37](https://github.com/openchat-ai/yoyo/issues/37)；诊断长文已折         |
| **勿做**               | —     | 勿 fake CLOSED；勿 `AUTO ACTIVE=1`；勿 invent Stage；勿定点/oracle/hop 当 CLOSED；勿用 CI 当调试器 |


**相关**：`SCOPE-CUT-v1.0-hole-inventory.md` · `SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md` · `SCOPE-CUT-v1.0-ow-seed-observe.md` · `AUTO_TO_1.0.md` · `ci-anti-thrash.mdc`

---



## path 2 归档（洞 + 门 · 终态）

权威 disposition：`SCOPE-CUT-v1.0-hole-inventory.md`。门禁明细 / CI → `git log`。


| 洞                | 门           | 诚实                                 |
| ---------------- | ----------- | ---------------------------------- |
| **OW-H00**       | C           | CUT · full `.text` EQUAL 才谈 CLOSED |
| **OW-STUB**      | A/C         | CUT · `stub_tail_nonzero==0`       |
| **OW-RT**        | D–F · **I** | CUT · **长杆** → S3                  |
| **OW-IAT**       | A/C         | CUT · 依赖 OW-RT 去 sidecar           |
| **OW-SEED**      | G/H/J       | CUT · J=bootstrap 例外（不再待关）；I→OW-RT |
| **REL-FULLTEXT** | C           | CUT · **设计上不毕业**                   |
| **REL-STUBOS**   | C           | CUT · 非本长杆优先                       |



| 门         | pin / 一句话                                              |
| --------- | ------------------------------------------------------ |
| **A–C** ✅ | `f8eb429` / Linux tramp / `11a2cea` FINAL · path 2 里程碑 |
| **D–F** ✅ | OW-RT DLL emit / origin / YOYO-built · 仍 CUT           |
| **G–H** ✅ | OW-SEED 操作发射 + trust-root · 仍 CUT                      |
| **I** —   | → OW-RT 长杆（实验 RED · 预期）                                |
| **J** ✅   | bootstrap 例外 · 不再待关                                    |


> 勿 invent 重开关洞门。`stage17-*` = 门禁文件名 ≠ ROADMAP Stage 17。

---



## Stage 17+ / S3（日常主线）


| 阶段        | 状态  | 一句话                                                     |
| --------- | --- | ------------------------------------------------------- |
| **S1**    | ✅   | in-DLL mini codegen（TYB→slim PE）                        |
| **S2**    | ✅   | slim 真 I/O 钩子 · StubPlatform **仍 stub** · pin `2de5ff6` |
| **S3**    | ⏳   | meta-emit 近端 subset；真自举 = 半年+                           |
| **S4/S5** | 地图  | 全 ISA / 形式化 · **不排假工期**                                 |




### 近端子集合同（`S3-subset-fixed10`）


| 可 GREEN                                              | **不算**毕业                          |
| ---------------------------------------------------- | --------------------------------- |
| 固定模板 10 op · meta-emit / `emit_x86` `.text` **字节对拍** | 任意 slot/imm、全 mini-ISA            |
| 宿主固件只剩 DATA `0x10` pack；subset 证明不走 `pack_meta_emit` | 零 Rust 独写完整 codegen               |
| slim 真 I/O（S3.99–108）；StubPlatform **stub**          | StubPlatform 变真 OS                |
| —                                                    | 任何 OW-* **CLOSED** 话术（`closed=0`） |




### S3 进度索引

明细 / 单刀正文 → `git log`。下表只留波段 tip pin。


| 波段                 | pin         | 一句话                                              |
| ------------------ | ----------- | ------------------------------------------------ |
| **S3.1–29** ✅      | `d310710`   | RAW_BYTES + 固定模板 10 op host 表                    |
| **S3.30–52** ✅     | `673297e`   | bake 表 + multi bake                              |
| **S3.53–76** ✅     | `290679b`   | JMP/CALL/JCC + ALU/MEM/平台 stub                   |
| **S3.77–98** ✅     | `6b6756e`   | any-slot + LOAD/WRITE→slim IAT                   |
| **S3.99–108** ✅    | `a10c20d`   | host+in-DLL 真 I/O e2e                            |
| **S3.109–111** ✅   | `0fa5d9e`   | 近端合同 + fixed10 对拍；证明去掉 `pack_meta_emit`         |
| **S3.112–135** ✅   | `e431341`   | RAW/lea/pack 宿主面收拢；剩余 Rust 面清单                   |
| **S3.136–152** ✅   | `8186680`   | pack via_ir / OnceLock / scratch 地板              |
| **S3.153–163** ✅   | `cce0b51`   | STB pack + 自举地图门禁（done/floor/cliff）             |
| **S3.164–176** ✅   | `7d9e782`   | A1 OnceLock + TYB STB 发射 + 真自举近端清单               |
| **S3.177–203** ✅   | `6ff580b`   | 混合/两阶段/run 折叠；生产路径无 bytewise DATA              |
| **S3.204–223** ✅   | `2b24667`   | empty-DATA in-DLL；1024 帽；short-lea 37B 地板        |
| **S3.224–241** ✅   | `128fe7d`   | op→shape + scan-fill；控制模板收成字节种子                  |
| **S3.242–253** ✅   | `a98ddad`   | one/argc flat 冻结；展开只扫 argc；e2e I/O 离 cwd        |
| **S3.254–255** ✅   | `20bd92d`   | YOYO IR 展开 flat→wire；生产 one/many ctrl 走 via_yoyo     |
| **S3.256–259** ✅   | `791936a`   | 展开 ctrl flat/哨兵/自再编码；剩余 Rust 面钉死（bootstrap/A1） |
| **S3.260–262** ✅   | `94f3e77`   | argc-poke 并入展开种子；many-job 128；命名 bootstrap 唯一生产点 |
| **S3.263** ✅       | `e6d9362`   | sole host wire→records；expand layout 钉死                   |
| **S3.264** ✅       | `8534834`   | expand argc IR-seed；ctrl flat 92→41；kill pack alias          |
| **S3.265** ✅       | `c95a46c`   | expand ONE assembler=312；coalesce 劣于 two-seed；A1 三块不动 |
| **S3.266** ✅       | `a8ddd93`   | host wire→records = parse_tyb header wrap（无第二解析器）     |
| **S3.267** ✅       | `c7ccad6`   | expand 发射完整 TYB；via_yoyo 直 parse_tyb；ctrl 41→57        |
| **S3.268** ✅       | pending     | TYB header IR-seed；ctrl 57→47；sim composed high-water        |
| **S3.269** ⏳       | —           | 继续缩 wire-parse / A1 宿主面 · **本步**                   |



**缺口**：lea→.text 仍唯一靠 A1 · 展开种子仍靠 Rust bootstrap · pack wire 仍宿主 header wrap · 真自举。逐步正文 → `git log`。

---



## 验收命令

**日常（S3）**

```powershell
cd F:\yoyo
& .\scripts\stage17-s2-runtime-effect-slim.ps1
cd F:\yoyo\yoyo-rust\verifier
cargo test --lib -- s3_subset_fixed10_meta_emit_parity --test-threads=1
```

**path 2 归档（回归才跑）**

```powershell
cd F:\yoyo
& .\scripts\stage17-ow-iat-wireup.ps1
& wsl -e bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
& .\scripts\stage16-scope-cut-finalize.ps1 -SkipBuild
& .\scripts\stage15-hole-inventory.ps1 -SkipBuild
& .\scripts\stage17-ow-rt-yoyo-runtime.ps1
& .\scripts\stage16-v09-regress.ps1 -SkipBuild
cd F:\yoyo\yoyo-rust\verifier; cargo run -- test ddc
```

---



## 对 AI / 负责人

- `继续` → **S3.269**；做完直接下一刀；红了先修再进。path 2 已勾勿重开。fixed10 golden / 布局不要再折。
- OW-* GREEN ≠ CLOSED；`closed=0`；本地先绿；2 次红 CI 停推。
- AUTO `ACTIVE=0`；每刀可回滚；证伪则停并记根因。

---

*创建：2026-08-31 · post-v1.0 · 对齐 STAGE16_OWNER_CHECKLIST 模板*

**快照 2026-10-20**：header IR-seed + high-water · 默认 **S3.269** · StubPlatform stub · OW-RT **CUT** · `closed=0 cut=7` · 无假 CLOSED · 无 tag · 长文已折（git）

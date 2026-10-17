# Post-v1.0 关洞负责人看板（path 2 · 缩宿主信任 · 整仓竣工）

## 北星

YOYO v1.0 已毕业（`ACTIVE=0` · `COMPLETED=1`）。ROADMAP 止于 Stage 16。本看板 = path 2 关洞归档 + **Stage 17+/S3 日常主线**。**禁止假 CLOSED** · **禁止 invent Stage**。

> tag `v1.0.0` · Lock `0275802d…` · `closed=0 cut=7`。长杆 = **OW-RT**（S3）。REL-FULLTEXT **永不** CLOSED。

**默认** `继续` **= S3.163**（162 bootstrap bytewise 定为地板；下一刀短 lea / IR 自举地图；仍不改 golden；**不**假 CLOSED）。

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


| 波段              | pin       | 一句话                                |
| --------------- | --------- | ---------------------------------- |
| **S3.1–29** ✅   | `d310710` | RAW_BYTES + 固定模板 10 op host 表      |
| **S3.30–52** ✅  | `673297e` | bake 表 + multi bake                |
| **S3.53–76** ✅  | `290679b` | JMP/CALL/JCC + ALU/MEM/平台 stub     |
| **S3.77–98** ✅  | `6b6756e` | any-slot + LOAD/WRITE→slim IAT     |
| **S3.99–108** ✅ | `a10c20d` | host+in-DLL 真 I/O e2e              |
| **S3.109** ✅    | `3e3dec9` | 近端合同 + 剩余 op meta-emit             |
| **S3.110** ✅    | `7dcaa8d` | `s3_subset_fixed10_meta_emit_parity` 10 op 字节对拍 |
| **S3.111** ✅    | `0fa5d9e` | fixed10 证明去掉 `pack_meta_emit`；mini=YOYO 表 |
| **S3.112** ✅    | `0d593d3` | 删宿主固件三段 RAW `0xA1` 序言（sim 本就跳过） |
| **S3.113–115** ✅ | `05c8eba` | 打包 / 行模板 / SET body 收成一份；in-DLL RAW 留给 PE |
| **S3.116–117** ✅ | `40de19f` | 一套 prog-loop sim + 一份循环指令尾 |
| **S3.118–119** ✅ | `53cd9d8` | scan/compose/fulltext 走同一 sim；host 直调 |
| **S3.120–122** ✅ | `1f3b4f9` | 共用 DATA 前缀；模板体改由 `emit_x86` 生成 |
| **S3.123** ✅    | `8fa5693` | RET/NOP 单字节改走 `emit_x86` |
| **S3.124–128** ✅ | `fcb497e` | fixed10 一份；`lea` 由 disp/slot 生成；SET/ADD 记录一份 |
| **S3.129–130** ✅ | `432040f` | `s3_fixed_rec` 对拍；10 例由同一张表拼出 |
| **S3.131–132** ✅ | `c3ad6e4` | 短 `lea` 共用；compose→`append_scan_memcpy`；in-DLL 仍有 A1 |
| **S3.133–134** ✅ | `3f1e2a7` | IR 记录一份；scan→`append_scan_byte_out` |
| **S3.135** ✅    | `e431341` | 剩余 Rust 面清单（DATA `0x10` / 短 `lea` / in-DLL A1） |
| **S3.136** ✅    | `60052d6` | YOYO 表 DATA 统一走 `s3_pack_data`（仍 Rust） |
| **S3.137–140** ✅ | `3596c40` | pack↔unpack；body OnceLock；host 最后一英里；111 例穷尽对拍 |
| **S3.141** ✅    | `6010384` | `S3_PACK_CHUNK=3` 规格 + 记录数门禁 |
| **S3.142** ✅    | `c4a4ff0` | YOYO IR pack 原型（unroll SET/ADD/MEMCPY）对拍 wire |
| **S3.143–144** ✅ | `91b4dcd` | loop pack + via_ir records == `s3_pack_data` |
| **S3.145–146** ✅ | `3ff56ce` | 动态布局；host 固件 DATA 改走 via_ir |
| **S3.147–148** ✅ | `a85b062` | 表路径 via_ir；`s3_pack_data` 仅参照；scratch=`host_pack_bytes` |
| **S3.149** ✅    | `e6a48db` | scratch 仅 const+payload；sim MEMCPY 增长 outbuf |
| **S3.150** ✅    | `85aa57f` | `s3_append_raw_a1`；in-DLL 三 lea 种子共用 |
| **S3.151–152** ✅ | `8186680` | scratch 前缀 OnceLock；pack 后剩余 Rust 面清单 |
| **S3.153** ✅    | `a9135cd` | bootstrap scratch bytewise；chunk-3 仅 IR 输出 |
| **S3.154–155** ✅ | `7b58cb5` | sim STB (0x81)；LDB→STB pack 对拍 wire |
| **S3.156** ✅    | `8d85ec2` | STB 进 emit_x86 + mini 校验 |
| **S3.157** ✅    | `a0d224f` | loop STB pack IR 1..=64 wire 对拍 |
| **S3.158** ✅    | `25eae12` | via_ir 改走 loop STB（host/表 DATA） |
| **S3.159** ✅    | `54e4023` | STB copy slim PE e2e（Windows） |
| **S3.160** ✅    | `f2fae2c` | pack+STB 后剩余 Rust 面再钉 |
| **S3.161** ✅    | `37aa36d` | via_ir 路径强制 STB loop 门禁 |
| **S3.162** ✅    | —         | bootstrap bytewise = 地板（无 YOYO 写 TYB 记录前不可消） |
| **S3.163** ⏳    | —         | 短 lea / IR 自举地图 · **本步** |


**缺口**：bootstrap bytewise 地板 · 短 lea 必留 · 真自举（YOYO 写 TYB）。逐步正文 → `git log`。

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

- `继续` → **S3.162**；做完直接下一刀；红了先修再进。path 2 已勾勿重开。fixed10 golden / 布局不要再折。
- OW-* GREEN ≠ CLOSED；`closed=0`；本地先绿；2 次红 CI 停推。
- AUTO `ACTIVE=0`；每刀可回滚；证伪则停并记根因。

---

*创建：2026-08-31 · post-v1.0 · 对齐 STAGE16_OWNER_CHECKLIST 模板*

**快照 2026-10-17**：S3.142–161 pack/STB 轴收 · 默认 **S3.162** · StubPlatform stub · OW-RT **CUT** · `closed=0 cut=7` · 无假 CLOSED · 无 tag · 长文已折（git）
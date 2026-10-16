# Post-v1.0 关洞负责人看板（path 2 · 缩宿主信任 · 整仓竣工）

## 北星

YOYO v1.0 已毕业（`ACTIVE=0` · `COMPLETED=1`）。ROADMAP 止于 Stage 16。本看板 = post-v1.0 **path 2 关洞** + **Stage 17+/S3 日常主线**：**禁止假 CLOSED** · **禁止 invent Stage 功能**。

> **基线**：tag `v1.0.0` · Lock pin `0275802d…` · `closed=0 cut=7`（OW-H00 full DIFF 回 CUT）。  
> **整仓**：path 2 门 A–H/J 已勾；**I** → **OW-RT 长杆**（S3）；REL-FULLTEXT **永不** CLOSED；REL-STUBOS 非本长杆优先。

## 进度总览

```text
[x] A B C     path 2 里程碑（无 tag）
[x] D E F     OW-RT DLL emit / origin / YOYO-built effect（仍 CUT）
[x] G H       OW-SEED 操作发射 + trust-root（仍 CUT）
-   I         已移出 OW-SEED → OW-RT 长杆
[x] J         bootstrap 例外分类 · OW-SEED 不再待关
日常          Stage 17+ / S3 → 默认 继续 = S3.110
```

明细见下文「path 2 归档」。`stage17-*` = 门禁文件名，≠ ROADMAP Stage 17。

## 阻塞 / 勿做

| 项 | 状态 | 说明 |
|----|------|------|
| **长杆** | **OW-RT** | 去 Rust sidecar；日常 = S3；**禁止**假 CLOSED |
| **HARD BLOCK CI AV** | 🟢 已关闭 | `#[ignore]` + issue [#37](https://github.com/openchat-ai/yoyo/issues/37)；run `35553987325`/`35703768045` 绿 |
| **勿做** | — | 勿 fake OW-* CLOSED；勿 `AUTO ACTIVE=1`；勿 invent Stage；勿定点/oracle/multi-hop 当 CLOSED；勿空测 hop3+；勿用 CI 当调试器 |

> HARD BLOCK 诊断矩阵 / `__fastfail` / linux-m4 E0601 等长文已折；见 issue #37 与 `git log`（`146b58b` 等）。

## 相关文件

`SCOPE-CUT-v1.0-hole-inventory.md` · `SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md` · `SCOPE-CUT-v1.0-ow-seed-observe.md` · `STAGE16_OWNER_CHECKLIST.md`（历史） · `RELEASE-v1.0.md` · `BACKEND_SUPPORT.md` · `AUTO_TO_1.0.md` · `.cursor/rules/ci-anti-thrash.mdc`

---

## path 2 归档（洞清单 + 关洞门 · 已收拢）

**终态**：`closed=0 cut=7` · A–H/J 已勾 · **无假 CLOSED**。权威 disposition 全文：`SCOPE-CUT-v1.0-hole-inventory.md`。

| 洞 | 门 | 诚实一句话 |
|---|---|---|
| **OW-H00** | C | CUT · full `.text` EQUAL 才谈 CLOSED（Gate A 后 DIFF） |
| **OW-STUB** | A/C | CUT · 需 `stub_tail_nonzero==0` |
| **OW-RT** | D–F · **I** | CUT · **长杆**（YOYO-built · 无 Rust sidecar）→ S3 |
| **OW-IAT** | A/C | CUT · 依赖 OW-RT 去 sidecar |
| **OW-SEED** | G/H/J | CUT · J=bootstrap 例外（不再待关）；I 已移出→OW-RT |
| **REL-FULLTEXT** | C | CUT · **设计上不毕业 CLOSED** |
| **REL-STUBOS** | C | CUT · 生产 I/O（非本长杆优先） |

| 门 | 状态 | 一句话 |
|---|---|---|
| **A** ✅ | PR #26 `f8eb429` | Win OW-IAT wire-up · 仍 CUT |
| **B** ✅ | Linux tramp | H_00 gen1→gen4 不退化 · 仍 CUT |
| **C** ✅ | tip `11a2cea` | 洞清单 FINAL · smoke ≠ CLOSED |
| **D–F** ✅ | OW-RT spike | DLL emit / origin / YOYO-built · 仍 CUT |
| **G–H** ✅ | OW-SEED | 操作发射 + trust-root pin · 仍 CUT |
| **I** — | → OW-RT | 去 Rust sidecar（实验 RED · 预期） |
| **J** ✅ | 分类重述 | bootstrap 例外 · 不再待关 |

> 日常主线 = 下文 **Stage 17+ / S3**；勿把关洞门 invent 重开。门禁明细 / CI 见 `git log`。
---

## 后续路线（Stage 17+ / S3）

path 2 诚实终态 · **OW-RT 是关键节点**。扩 oracle = 陷阱；真突破 = in-DLL **真 codegen**（任意 H_00 IR）。

| 阶段 | 目标 | 量级 |
|------|------|------|
| **S1** ✅ | in-DLL mini codegen（手写机器码） | 已收口 |
| **S2** ✅ | slim 真 I/O 验收钩子 | 已收口 · StubPlatform **仍 stub** |
| **S3** | codegen 用 YOYO 表达（meta-emit） | 近端 = subset；真自举 = 半年+ |
| **S4/S5** | 全 ISA / 形式化 | 年级地图 · **不排假工期** |

### S3 近端子集合同（S3.109+）

| 项 | 近端可 GREEN | **不算**近端毕业 |
|---|---|---|
| **op** | 固定模板 10 op（`fixed_template_ops`） | 任意 slot/imm、全 mini-ISA |
| **对拍** | meta-emit · `emit_x86` · `.text` byte-identical | 「行为像」无字节对拍 |
| **驱动** | 允许 Rust 辅助打包/调度 | 零 Rust 独写完整 codegen |
| **I/O** | slim 真 I/O（S3.99–108）；StubPlatform **stub** | StubPlatform 变真 OS |
| **CLOSED** | **禁止** · OW-RT 仍 CUT · `closed=0` | 任何 OW-* CLOSED 话术 |

**近端毕业名**：`S3-subset-fixed10`。其后扩 op / 减 Rust / 真自举 = 另开刀号。

### 进度索引

| 波段 | 状态 | 一句话 | pin |
|---|---|---|---|
| **S1.1–S1.7.f** | ✅ | TYB→slim PE mini-ISA + host↔in-DLL parity | 见 git |
| **S2.1–S2.6** | ✅ | stub 固件 + slim 真 I/O 钩子 | S2.6=`2de5ff6` |
| **S3.1–S3.29** | ✅ | RAW_BYTES + 固定模板 10 op host 表 | `d310710` |
| **S3.30–S3.52** | ✅ | bake 表 + multi bake | `673297e` |
| **S3.53–S3.76** | ✅ | JMP/CALL/JCC + ALU/MEM/平台 stub | `290679b` |
| **S3.77–S3.98** | ✅ | any-slot + LOAD/WRITE→slim IAT | `6b6756e` |
| **S3.99–S3.108** | ✅ | host+in-DLL 真 I/O e2e | `a10c20d` |
| **S3.109** | ✅ | 近端合同 + fixed-template 剩余 op meta-emit | `3e3dec9` |
| **S3.110+** | ⏳ | `S3-subset-fixed10` 收口 / 减 Rust 辅助 | 本步 |

**诚实缺口（至 S3.109）**：近端合同与 6 op meta-emit 已进；仍缺合同毕业标记、减 Rust 辅助、真自举。

#### S3.110 — 近端毕业标记或减 Rust 辅助（本步 · 未开始）

| 项 | 内容 |
|---|---|
| 切片 | 钉 `S3-subset-fixed10` 验收脚本/一行总测，或去掉一层 Rust pack；**不**假 CLOSED |
| 验收 | cargo/script GREEN；marker TBD |
| 诚实 | OW-RT 仍 CUT · `closed=0` |

**默认 `继续` = S3.110 / subset 收口**。

> 写法：已完成波段 = 索引；逐步正文见 git。**禁止**把 GREEN 写成 OW-* CLOSED。S1/S2 GREEN ≠ OW-RT CLOSED。

---

## 验收命令

**两层别混**：日常 = S3；path 2 归档 = 关洞回归时才跑。

### 日常（S3）

```powershell
cd F:\yoyo
& .\scripts\stage17-s2-runtime-effect-slim.ps1
cd F:\yoyo\yoyo-rust\verifier
cargo test --lib -- s3_meta_emit_autopack --test-threads=1
```

### path 2 归档（非日常）

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

- `继续` → 读「默认继续」刀号；path 2 已勾门勿 invent 重开。
- **禁止**假 CLOSED；OW-* GREEN ≠ CLOSED；`closed=0`。
- 本地先绿；连续 2 次红 CI → 停推；WIP 用 `[skip ci]`。
- AUTO：`ACTIVE=0` · `COMPLETED=1`；本路线是 S3 主线，非关洞延续。
- 每刀 commit 可回滚；中途证伪则停并记根因。

---

*创建：2026-08-31 · post-v1.0 path 2 · 模板对齐 STAGE16_OWNER_CHECKLIST.md*

**当前分支诚实快照（2026-10-16 · + S3.109 · 文档收拢）**：近端子集合同 + meta-emit 矩阵 GREEN · StubPlatform 仍 stub · 默认 `继续`=**S3.110/subset 收口** · OW-RT **仍 CUT** · `closed=0 cut=7` · **无假 CLOSED** · **无 tag** · 历史长文已折（见 git）

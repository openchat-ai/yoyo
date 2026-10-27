# Post-v1.0 关洞负责人看板（path 2 · 缩宿主信任 · 整仓竣工）

## 北星

YOYO v1.0 已毕业（`ACTIVE=0` · `COMPLETED=1`）。ROADMAP 止于 Stage 16。本看板 = path 2 关洞归档 + **Stage 17+/S3 日常主线**。**禁止假 CLOSED** · **禁止 invent Stage**。

> tag `v1.0.0` · Lock `0275802d…` · `closed=0 cut=7`。长杆 = **OW-RT**（S3）。REL-FULLTEXT **永不** CLOSED。

**默认** 继续 **= S3.466**（465 post-LOAD/WRITE map；下一刀=YOYO-emit dispatch stub / OW-RT；**不**假 CLOSED）。

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

明细 / 单刀正文 → `git log --grep=S3.`。下表只留**波段 tip** + **近窗**（421+）。更早单刀已折。

| 波段 | pin | 一句话 |
| --- | --- | --- |
| **S3.1–29** ✅ | `d310710` | RAW_BYTES + 固定模板 10 op host 表 |
| **S3.30–52** ✅ | `673297e` | bake 表 + multi bake |
| **S3.53–76** ✅ | `290679b` | JMP/CALL/JCC + ALU/MEM/平台 stub |
| **S3.77–98** ✅ | `6b6756e` | any-slot + LOAD/WRITE→slim IAT |
| **S3.99–108** ✅ | `a10c20d` | host+in-DLL 真 I/O e2e |
| **S3.109–111** ✅ | `0fa5d9e` | 近端合同 + fixed10 对拍 |
| **S3.112–223** ✅ | `2b24667` | pack/lea/空DATA/short-lea 收拢（详见 git） |
| **S3.224–306** ✅ | `8ccdaf6` | shape-fill / expand→asm22 地板 |
| **S3.307–358** ✅ | `34436b5` | pack/cfg(test) 收拢 + 真自举能力图 |
| **S3.359–406** ✅ | `a39a2a2` | 目标码种子链；Win PE 去 wrap |
| **S3.407–420** ✅ | `5b425df` | OW-RT 切片 + densify/skip-mini **拒羊** |
| **S3.421–428** ✅ | `2bc6fbc` | s1_loop op-handler poke-from-seed |
| **S3.429–464** ✅ | dc4a5cd | seed+collapse+lift + shared + YOYO dispatch + LOAD/WRITE |

#### 近窗（本波 · 单刀）

| 刀 | pin | 一句话 |
| --- | --- | --- |
| **S3.429** ✅ | 0b714e3 | CALL/JMP/JCC handler seed（bake+301/306/311） |
| **S3.430** ✅ | 5fac607 | ALLOC prefix seed（bake+317） |
| **S3.431** ✅ | f6a85b5 | ALLOC store seed（bake+351/355） |
| **S3.432** ✅ | 58701d2 | multi CALL/JMP/JCC seed reuse |
| **S3.433** ✅ | f2ec117 | mt SET/ALLOC seed reuse |
| **S3.434** ✅ | e530198 | post-seed 能力图 + 拒 one-op mt densify |
| **S3.435** ✅ | ac888a2 | batch mt GET+INC+DEC disp8→bake |
| **S3.436** ✅ | 7efdd35 | batch mt ADD+SUB+MOVRR disp8→bake |
| **S3.437** ✅ | 462bbf4 | batch mt CMP+IMUL+ALUV disp8→bake |
| **S3.438** ✅ | a34856f | batch mt LDB+MCPD+MCPS disp8→bake |
| **S3.439** ✅ | 90df71f | post-disp8 map + 拒 disp32 densify 羊 |
| **S3.440** ✅ | 72d53e0 | collapse shared GET/MOVRR disp8 seed thunk |
| **S3.441** ✅ | 168453c | collapse shared INC/DEC disp8 seed thunk |
| **S3.461** ✅ | 6c9006b | collapse shared ADD/SUB disp8 seed thunk |
| **S3.443** ✅ | e25f9c6 | collapse shared CMP disp8 seed thunk |
| **S3.444** ✅ | 73d700d | collapse shared IMUL+ALUV disp8 seed thunk |
| **S3.445** ✅ | 6ffce38 | collapse shared LDB+MCPD disp8 seed thunk |
| **S3.446** ✅ | 75636c5 | collapse shared MCPS pair seed thunk |
| **S3.447** ✅ | a26dd4a | post-shared-collapse map + 拒 densify hand-disp32 |
| **S3.448** ✅ | 0145ebe | lift s1 GET/MOVRR any-slot（disp8 shared + hand mix） |
| **S3.449** ✅ | 88ce21f | lift s1 INC/DEC any-slot（disp8 shared + hand d32） |
| **S3.450** ✅ | 7f5d6a3 | lift s1 ADD/SUB any-slot（disp8 shared + hand d32） |
| **S3.451** ✅ | 4c9228a | lift s1 CMP any-slot（disp8 shared + hand mix） |
| **S3.452** ✅ | 39e680a | lift s1 IMUL+ALUV any-slot（disp8 shared + hand mix） |
| **S3.453** ✅ | 2fab1bc | lift s1 LDB+MCPD any-slot（disp8 shared + hand mix） |
| **S3.454** ✅ | a237361 | lift s1 MCPS any-slot（disp8 shared + hand mix） |
| **S3.455** ✅ | 30246b7 | post-s1-lift map（GET..MCPS shared+hand done） |
| **S3.456** ✅ | af5bb08 | collapse shared SET seed（mt+s1 slot→d8/d32） |
| **S3.457** ✅ | d2436f9 | collapse shared ALLOC seed（mt+s1 prefix+store） |
| **S3.458** ✅ | b7af3a2 | post-SET/ALLOC-collapse map |
| **S3.459** ✅ | 64fc271 | OW-RT entry cliff map（dispatcher still Rust） |
| **S3.460** ✅ | c9884af | collapse shared CALL/JMP/JCC seeds（mt+s1） |
| **S3.461** ✅ | 5ee8732 | YOYO-own fixed10 dispatcher slice（index+tramp） |
| **S3.462** ✅ | c8ff4a6 | extend YOYO-own full op dispatch（29 tramp） |
| **S3.463** ✅ | 9639089 | post-dispatch map（YOYO index done；OW-RT CUT） |
| **S3.464** ✅ | dc4a5cd | collapse shared LOAD/WRITE seeds（mt+s1） |
| **S3.465** ✅ | b45daca | post-LOAD/WRITE-collapse map |
| **S3.466** ⏳ | — | **本步**：YOYO-emit dispatch stub / OW-RT |

**缺口**：shared IAT LOAD/WRITE 完成。下一刀=YOYO-emit dispatch stub · OW-RT。Densify=羊。DATA/LABEL=meta。无假 CLOSED。ake=362 · s3_=275。

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

- `继续` → **S3.461**（YOYO-own dispatcher slice / OW-RT；禁 densify）。做完直接下一刀；红了先修再进。path 2 已勾勿重开。fixed10 golden / 布局不要再折。
- OW-* GREEN ≠ CLOSED；`closed=0`；本地先绿；2 次红 CI 停推。
- AUTO `ACTIVE=0`；每刀可回滚；证伪则停并记根因。

---

*创建：2026-08-31 · post-v1.0 · 对齐 STAGE16_OWNER_CHECKLIST 模板*

**快照 2026-10-26**：ctrl 2 · asm 2 · assembler 22 · many 2 · one 2 · host 173 · 默认 **S3.466** · bake=362 · StubPlatform stub · OW-RT **CUT** · closed=0 cut=7 · 无假 CLOSED · 看板已再折（波段+近窗）

# Post-v1.0 关洞负责人看板（path 2 · 缩宿主信任 · 整仓竣工）

## 北星

YOYO v1.0 已毕业（`ACTIVE=0` · `COMPLETED=1`）。ROADMAP 止于 Stage 16。本看板 = path 2 关洞归档 + **Stage 17+/S3 日常主线**。**禁止假 CLOSED** · **禁止 invent Stage**。

> tag `v1.0.0` · Lock `0275802d…` · `closed=0 cut=7`。长杆 = **OW-RT**（S3）。REL-FULLTEXT **永不** CLOSED。

**默认** 继续 **= S3.514**（513 STB oo>127；下一刀=slot≥16 / OW-RT）。

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
| **S3.429–469** ✅ | b17499c | seed+collapse+lift + shared + YOYO dispatch/tramp + RET/NOP |

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
| **S3.466** ✅ | e2c1d20 | YOYO-emit dispatch probe object seed（45B） |
| **S3.467** ✅ | 889efba | mt→same-index ABI cliff reject |
| **S3.468** ✅ | 8981eee | YOYO-own handler trampoline table seed（29×8） |
| **S3.469** ✅ | b17499c | collapse shared RET/NOP bake-emit seed |
| **S3.470** ✅ | eb89525 | RAW_BYTE emit body seed + shared（LABEL=meta） |
| **S3.471** ✅ | 0a8a2c2 | RAW_BYTES emit body seed + shared（48B） |
| **S3.472** ✅ | 0a89943 | post-raw map（RAW_* seeds done） |
| **S3.473** ✅ | 719a992 | push_patch object seed（41B + poke） |
| **S3.474** ✅ | 01c8183 | post-push_patch map（push_patch seed done） |
| **S3.475** ✅ | 2605bdd | pass-2 object seed（95B + poke） |
| **S3.476** ✅ | c757a92 | trail-C3 object seed（44B + poke） |
| **S3.477** ✅ | a0d7b6a | memcpy-code object seed（39B + poke） |
| **S3.478** ✅ | 9f88418 | memcpy-data object seed（36B + poke） |
| **S3.479** ✅ | 6c08b80 | post-finish-memcpy map（finish memcpy seeds done） |
| **S3.480** ✅ | 96a4d19 | template-post-lea object seed（18B + poke） |
| **S3.481** ✅ | dca3af0 | write_out setup object seed（19B + poke） |
| **S3.482** ✅ | f62cb02 | post-s1-finish map（finish body seeds done） |
| **S3.483** ✅ | 07ebcee | handler guard densify = sheep reject |
| **S3.484** ✅ | c66f2fd | post-finish/guard residual map |
| **S3.485** ✅ | 3a27089 | OW-RT mini→YOYO entry cliff reject |
| **S3.486** ✅ | 2c0df5b | DATA append object seed（39B shared mt+s1） |
| **S3.487** ✅ | 649ab96 | post-data-append map（DATA append seed done） |
| **S3.488** ✅ | ff3dd13 | s1 LABEL table-push object seed（62B） |
| **S3.489** ✅ | b649c1d | post-label map（push_label seed done） |
| **S3.490** ✅ | 27b20a9 | mt→s1 push_label ABI cliff reject |
| **S3.491** ✅ | 9f5893a | OW-RT residual after finish/table-seed wave |
| **S3.492** ✅ | 1968e60 | YOYO-built compile cliff reject（≠ seed densify） |
| **S3.493** ✅ | 4907afc | YOYO-IR compile slice map（fixed10 owned；扩表禁折 golden） |
| **S3.494** ✅ | 71f36cc | IMUL+RET YOYO object-seed compose（slots 0,1；不折 FIXED10） |
| **S3.495** ✅ | b052e70 | ALUV+RET YOYO object-seed compose（poke 01/09/29；slots 0,1） |
| **S3.496** ✅ | 91d1c8e | LDB+RET YOYO object-seed compose（OO0/OO poke；dd=ss=0） |
| **S3.497** ✅ | 9814bc7 | MEMCPY_DATA+RET YOYO object-seed compose（disp8 poke） |
| **S3.498** ✅ | 7c70f08 | MEMCPY_STATE+RET YOYO object-seed compose（loop PAIR；n∈1..=4） |
| **S3.499** ✅ | ccf0f02 | post-non-fixed10 seed compose map（IMUL..MCPS owned；CF/I/O 仍 emit_x86） |
| **S3.500** ✅ | a57dde7 | control-flow +RET YOYO object-seed compose（CALL/JMP/JCC；rel32=0） |
| **S3.501** ✅ | 25b7888 | ALLOC+RET YOYO object-seed compose（PREFIX+STORE_DISP32） |
| **S3.502** ✅ | 0daf55a | STB+RET YOYO object-seed compose（OO0/imm8；bake=362 未扩） |
| **S3.503** ✅ | 704a3c6 | RAW_BYTE/RAW_BYTES+RET YOYO literal payload compose |
| **S3.504** ✅ | 5a6f831 | LOAD_FILE+RET YOYO object-seed compose（160B poke；bake=362） |
| **S3.505** ✅ | ae48400 | WRITE_FILE+RET YOYO object-seed compose（139B poke；bake=362） |
| **S3.506** ✅ | 24e0c01 | DATA/STR/RAW+RET YOYO compose（text=C3；.data=payload） |
| **S3.507** ✅ | cc44d40 | multi DATA|STR|RAW+RET YOYO compose（concat≤384） |
| **S3.508** ✅ | 8075b4a | host YOYO compose gap inventory（剩 LABEL/pass-2 等） |
| **S3.509** ✅ | 86c715c | LABEL+CF+RET YOYO compose（pass-2 backward） |
| **S3.510** ✅ | 59818f6 | CF+NOP+LABEL+RET YOYO compose（forward pass-2） |
| **S3.511** ✅ | ae97c12 | IMUL/ALUV+RET disp8 wrong-slot lift |
| **S3.512** ✅ | 5fc6695 | RAW_BYTES+RET ops-path（default NOP‖C3） |
| **S3.513** ✅ | 8df4158 | STB+RET oo>127 YOYO compose（IMM32 seed） |
| **S3.514** ⏳ | — | **本步**：slot≥16 (disp32) lift / OW-RT |

**缺口**：STB oo 全档已 YOYO；slot≥16 / 混合仍 emit_x86。下一刀=slot≥16。bake=362 · s3_=323。

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

- `继续` → **S3.514**（slot≥16 / OW-RT；禁 densify / 禁折 FIXED10）。做完直接下一刀；红了先修再进。path 2 已勾勿重开。fixed10 golden / 布局不要再折。
- OW-* GREEN ≠ CLOSED；`closed=0`；本地先绿；2 次红 CI 停推。
- AUTO `ACTIVE=0`；每刀可回滚；证伪则停并记根因。

---

*创建：2026-08-31 · post-v1.0 · 对齐 STAGE16_OWNER_CHECKLIST 模板*

**快照 2026-10-28**：ctrl 2 · asm 2 · assembler 22 · many 2 · one 2 · host 173 · 默认 **S3.514** · bake=362 · StubPlatform stub · OW-RT **CUT** · closed=0 cut=7 · 无假 CLOSED · 看板已再折（波段+近窗）

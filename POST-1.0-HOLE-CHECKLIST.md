# Post-v1.0 关洞负责人看板（path 2 · 缩宿主信任 · 整仓竣工）

## 北星

YOYO v1.0 已毕业（`ACTIVE=0` · `COMPLETED=1`）。ROADMAP 止于 Stage 16。本看板 = path 2 关洞归档 + **Stage 17+/S3 日常主线**。**禁止假 CLOSED** · **禁止 invent Stage**。

> tag `v1.0.0` · Lock `0275802d…` · `closed=0 cut=7`。长杆 = **OW-RT**（S3）。REL-FULLTEXT **永不** CLOSED。

**默认** 继续 **= S3.575**（574=emit1 NOP seed；下一刀=emit1 widen / M4）。

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

明细 / 单刀正文 → `git log --grep=S3.`。下表只留**波段 tip** + **近窗**（本步）。更早单刀已折。

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
| **S3.429–469** ✅ | `b17499c` | seed+collapse+lift + shared + YOYO dispatch/tramp + RET/NOP |
| **S3.470–493** ✅ | `4907afc` | finish/table seeds + cliff reject + YOYO-IR map |
| **S3.494–515** ✅ | `bd5d6bc` | host YOYO compose（non-fixed10+RET · any-slot lift） |
| **S3.516–521** ✅ | `5dd7651` | CMP/multi-op · LABEL/bare · DATA surface（禁折 FIXED10） |

#### 近窗（本波 · 单刀）

| 刀 | pin | 一句话 |
| --- | --- | --- |
| **S3.522–525** ✅ | 7c12713 | Gate F mini → sidecar bake → ABSENT cliff → **M1** mutate-after-load |
| **S3.526–528** ✅ | b6c570a | **M2/M3** H_00+ALT → **M4-prep** flip ABSENT |
| **S3.529–533** ✅ | 68d7edf | beyond-mini oracle → yoyo.tyb VirtualAlloc → gen4 e2e → live ABSENT cliff → **ALT gate-i** |
| **S3.534–538** ✅ | 5160946 | live-compile map → slice_ready=false → sheep-reject → CreateProcess ABSENT → **IAT CreateProcessA** |
| **S3.539–541** ✅ | 48784ef | call-site design → Wait IAT → **compiler PE bytes ABSENT** |
| **S3.542** ✅ | 1946a35 | CreateProcessA+Wait+GetExitCode call-site （无 PE→oracle）；PE bytes 仍 None |
| **S3.543** ✅ | b11733e | first PE bytes=pe_link(exit0) smoke；place+CreateProcess e2e；≠R→C→W |
| **S3.544** ✅ | 6635294 | passthrough input.tyb→output.exe；place+CreateProcess e2e；C ABSENT |
| **S3.545** ✅ | 14415ff | RET-only compile-in-child→slim RET PE；beyond-RET ABSENT；M4-prep |
| **S3.546** ✅ | 3c0b114 | RET+SET+RET compile-in-child；beyond-SET+RET/yoyo.tyb ABSENT |
| **S3.547** ✅ | 3fdd9c2 | mini-table yoyo_tbl_c（RET+SET+RET rows）；broader fixed10 ABSENT |
| **S3.548** ✅ | 25feff7 | fixed10 op+RET compile-in-child（hand RET/SET + emit extras；yoyo_f10_c） |
| **S3.549** ✅ | 25feff7 | in-DLL CreateProcess fixed10 smoke（NOP+RET / GET+RET） |
| **S3.550** ✅ | 25feff7 | M4-prep — fixed10 child GREEN ≠ flip；blockers 仍 non-empty |
| **S3.551** ✅ | 57c3e91 | multi-op SET+ADD/SET+GET+RET emit + in-DLL；yoyo_multi_c；yoyo.tyb ABSENT |
| **S3.552** ✅ | 6aa33d2 | longer chains SET+ADD+SUB / NOP+INC+DEC +RET；in-DLL；yoyo.tyb ABSENT |
| **S3.553** ✅ | b1db4dd | 4-op SET+ADD+SUB+NOP+RET emit + in-DLL |
| **S3.554** ✅ | b1db4dd | child rejects yoyo.tyb（EXIT_COMPILE_FAIL；honest ABSENT） |
| **S3.555** ✅ | 8ea1ee5 | SET+MOVRR / SET+CMP+MOVRR chains；M4 flip 仍 ABSENT |
| **S3.556** ✅ | ef910cd | child live-compose design（memcmp≠parse+emit+link；ready=false） |
| **S3.557** ✅ | c1b1fc6 | live1 NOP+RET field-match（yoyo_live1_c；PE 仍 bake） |
| **S3.558** ✅ | 454221a | GET+RET field-match（live1；PE 仍 bake；in-DLL） |
| **S3.559** ✅ | 958be74 | INC+ADD field-match（live1；PE 仍 bake） |
| **S3.560** ✅ | 02ac94b | DEC+RET field-match；SUB/MOV/CMP 仍 memcmp |
| **S3.561** ✅ | 05eef60 | MOVRR+CMP field-match；SUB 仍 memcmp |
| **S3.562** ✅ | b2440e3 | SUB field-match — all fixed10 op+RET structured（PE 仍 bake） |
| **S3.563** ✅ | b26a40b | live-emit cliff（write bake ≠ emit_x86；ready=false） |
| **S3.564** ✅ | 3e9f137 | live2 NOP pe_patch-in-child（RET 骨架+.text 90 C3；yoyo_live2_c） |
| **S3.565** ✅ | f593880 | live2 NOP/INC/DEC pe_patch（shared RET 骨架+.text blobs） |
| **S3.566** ✅ | 05a1603 | live2 GET/ADD/SUB/SET pe_patch |
| **S3.567** ✅ | f081f08 | live2 MOVRR/CMP — all fixed10 op+RET pe_patch |
| **S3.568** ✅ | fe8f569 | multi live2 pe_patch ABSENT cliff（chains 仍 memcmp） |
| **S3.569** ✅ | d2bb04f | SET+ADD live2 pe_patch（memcmp→blob；其它链仍 memcmp） |
| **S3.570** ✅ | 47dfc3e | SET+GET live2 pe_patch（memcmp→blob） |
| **S3.571** ✅ | 814f57b | SET+ADD+SUB / NOP+INC+DEC / SET+MOVRR live2 |
| **S3.572** ✅ | c8282b2 | all CHAINS multi live2 pe_patch（4-op+CMP+MOV） |
| **S3.573** ✅ | 61403e8 | emit_x86-in-child cliff（pe_patch≠parse+emit；ready=false） |
| **S3.574** ✅ | d913d1f | emit1 NOP inline seed（90 C3；yoyo_emit1_c；≠blob） |
| **S3.575** ⏳ | — | **本步**：emit1 widen / M4 （禁 densify / 假 CLOSED） |

**缺口**：live in-DLL compile **ABSENT**（见 `yoyo_live_in_dll_compile_map`）。生产 flip 仍 ABSENT。bake=362。

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

- 继续 → **S3.575**（emit1 widen / M4；禁 densify / 假 CLOSED）。S3.574=emit1 NOP seed。红了记根因停。
- OW-* GREEN ≠ CLOSED；`closed=0`；本地先绿；2 次红 CI 停推。
- AUTO `ACTIVE=0`；每刀可回滚；证伪则停并记根因。

---

*Created：2026-08-31 · post-v1.0 · 对齐 STAGE16_OWNER_CHECKLIST 模板*

**快照 2026-10-29**：ctrl 2 · asm 2 · assembler 22 · many 2 · one 2 · host 173 · 默认 **S3.568** · bake=362 · StubPlatform stub · OW-RT **CUT** · closed=0 cut=7 · fixed10+multi + yoyo.tyb-reject cliff GREEN · 近窗自动收拢

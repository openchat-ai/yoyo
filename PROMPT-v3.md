# YOYO: Engineering Specification (v3.3.10)

## Master Table of Contents / 总目录（按阅读路径）

### BOOK I — Identity & Norms / 身份与规范
- [Part N: Normative Conventions (v3.3.5)](#part-n-normative-conventions-v335)
- [Part L: Language Positioning (One Page)](#part-l-language-positioning-one-page)

> **Work queue** ≠ this ToC. See 文首『当前进度 · Week 轴』.

### BOOK II — Orientation / 入门导向
- [Part 0: Quick Start](#part-0-quick-start)
- [Part 1: 4-Project Architecture](#part-1-4-project-architecture)
- [Part 2: Context and Goals](#part-2-context-and-goals)
- [Part 3: Thompson Honesty (half-page)](#part-3-thompson-honesty-half-page)

### BOOK III — Core Language / 核心语言
- [Part 4: Core Architecture (ISA / State Machine / Primitives)](#part-4-core-architecture)
- [Part 4S: Operational Semantics (NORMATIVE)](#part-4s-operational-semantics-normative)
- [Part G: Formal `.ty` Grammar (NORMATIVE)](#part-g-formal-ty-grammar-normative)

### BOOK IV — Toolchain & Verification / 工具链与验证
- [Part 5: Self-Hosting Chain](#part-5-self-hosting-chain)
- [Part 5B: Cold-Start, Bootstrap & LOCKED Lifecycle](#part-5b-cold-start-bootstrap--locked-lifecycle-normative)
- [Part 6: DDC Verification (3-Chain)](#part-6-ddc-verification-3-chain)
- [Part 7: Platform Abstraction](#part-7-platform-abstraction)
- [Part 8: Variable / Name Layer](#part-8-variable--name-layer)
- [Part 9: Safety Architecture (4 Properties + 13 Decisions)](#part-9-safety-architecture-4-properties--13-decisions)

### BOOK V — Protocol Extension (Morph) / 协议扩展（形态）
- [Part E: Morphological Adaptation (Protocol)](#part-e-morphological-adaptation-protocol-normative)

### BOOK VI — Satellite Specs / 卫星规格
- [Part F: Foundations](#part-f-foundations-normative)
- [Part Deduce: Plan-first Deduction](#part-deduce-plan-first-deduction-normative-v01)
- [Part Gnd: Physical Cite Registry](#part-gnd-physical-cite-registry-normative-short)
- [Part S: v0.1 Scope Table](#part-s-v01-scope-table-normative)

### BOOK VII — Delivery / 交付与阶段
- [Part 10: 6-Phase Execution Plan](#part-10-6-phase-execution-plan)
- [Part 11: Cross-Project Comparison](#part-11-cross-project-comparison)
- [Part 12: SIMD Extensions (NON-NORMATIVE ROADMAP)](#part-12-simd-extensions)
- [Part 13: Decision History + Anti-Patterns](#part-13-decision-history--anti-patterns)
- [Part 14: Maintainer Role + Custody Workflow](#part-14-maintainer-role--custody-workflow)
- [Part 15: Demos & Use Cases (mostly NON-NORMATIVE)](#part-15-demos--use-cases)
- [Part 16: Master Roadmap (NON-NORMATIVE)](#part-16-master-roadmap-extensions)

### BOOK VIII — Appendices / 附录
- [Appendix A: libyoyo API + 3-Platform Implementation](#appendix-a-libyoyo-api--3-platform-implementation)
- [Appendix B: yoyo-asm Third Implementation](#appendix-b-yoyo-asm-third-implementation)
- [Appendix C: Cross-Platform Story (Why libyoyo)](#appendix-c-cross-platform-story-why-libyoyo)
- [Appendix D: Anti-Patterns Catalog](#appendix-d-anti-patterns-catalog)
- [Appendix E: Build & Test + Reference Documents](#appendix-e-build--test--reference-documents)
- [Appendix F: Conformance Suite](#appendix-f-conformance-suite-normative-definition)
- [Appendix G: Trust Roots Inventory](#appendix-g-trust-roots-inventory-normative-disclosure)
- [Appendix H: Future Deduction Substrate](#appendix-h-future-deduction-substrate-non-normative-roadmap--out-of-v01)
- [Appendix T: Thompson 1984 Background](#appendix-t-thompson-1984-background-non-normative)
- [Appendix Bib: FACT Bibliography](#appendix-bib-fact-bibliography-non-normative-for-fact-use)
- [Appendix CH: Prior Changelog Archive](#appendix-ch-prior-changelog-archive-non-normative)

---

## 当前进度 · Week 轴（NON-NORMATIVE · 日常只看这里）

> **日常入口。** 规格语义仍在下方 Parts；施工顺序 = 本 Week 轴。Status 仅允许：**GREEN (DONE)** / **RED** / **SCOPE-CUT** / **HOLD**。勿再维护 `STATUS.md` / `docs/PROGRESS-MAP.md`（已并入此处）。

### 你现在在哪
`yoyo.ty` = **788 handlers / 4170 lines**（850 行注释已恢复）；Rust golden **739/739 PASS**· executor **8/8 PASS**；JS==Rust==Python 三端字节级相等（3-chain DDC EQUAL，SHA-256: `4fb8b87f`）。W-START body-extend-001..106 全部 GREEN；MEMCPY real emit + LEA scale fix + executor expand + DDC fix；P2 imm 边界 + P1 多 slot 变体补齐；`.tyb` 纸带格式（8B 记录）就绪；`--selfhost` HOT 自举框架就绪；pin `0275802d2b4459e6…`（Decision #25）。

### 仍红（big list）
full compiler self-host · 冻结编译器

### W-START NODE（EXPERIMENTAL · body-extend 扩写完成 · 2026-07-24 点火 · 2026-07-28 收束 · ≠ freeze）
`EXPERIMENTAL · NON-GREEN · Rust-first · OUT-OF-v0.1-body（SCOPE-CUT 边界外点火）` — 详表 `docs/auxdocs/selfhost-start-node.md`
- **attempt ≠ freeze ≠ full self-host**；开火≠仍红翻绿；失败不 Relock / 不假 pin；产物仅 `EXPERIMENTAL`（不自动仍红→绿）
- **Workflow Hard Rule (non-normative; behavior, not law)** — default-first: 下一拍明显时直接执行默认 + 上一个子代理参数，**不再列选项问 A/B/C**；仅在 (a) 工具链缺、(b) 观测到 peer 分叉、(c) lock pin 想改但无既有 log、(d) PROMPT 要改 NORMATIVE（如 bump version）时才停下问；每拍成功仍产 `docs/auxdocs/<attempt|topic>-N-log.md`；不复述 dashboard/审计汇总，只接上一拍摘要 + 1 行下一拍。
- Checklist（压缩）：冷启复验文首+pin · Lock/Relock 一致（无 LOCKED 不谈 freeze）· scope 标签 · D-1/平台分叉 fail-closed · stub/RAW_BYTE 不宣称 C-ddc / Morph / freeze / gen1≡gen2
- 「尝试已开始」= 可复现 Rust 入口（cmd+log+scope tag）+ checklist（**attempt-level 全绿**；见 `docs/auxdocs/selfhost-attempt-N1-log.md`）
- 「自举 GREEN」= 仍红项（full body · Freeze+Lock）— **START NODE 一律不豁免**；Freeze = end gate（Part 5）；full body 仍在 W5.5 **SCOPE-CUT**；3-chain section-ddc 已达成（Python asm peer EQUAL）；gen1≡gen2 已达成（`.ty`==`.tyb` 三端一致）
- body-extend 连续扩写（EXPERIMENTAL · ≠ stub 34）：控制面 `docs/auxdocs/body-extend-queue.md` — scratch≤8 并发 / Relock 单写 / **矩阵满即停（matrix coverage gate）**；现 **788 handlers** · pin `0275802d2b4459e6…`（Decision #25）· body-extend-106 DONE（P2 imm 边界 + P1 多 slot）
- 入口（最小，不真编）：`cd f:\yoyo; .\scripts\verify-asm.ps1; node .\yoyo-js\scripts\golden.js; .\scripts\verify-selfhost.ps1; cd f:\yoyo\yoyo-rust; cargo run -p verifier --bin yoyo -- test golden`
- attempt-level 4 critical-path：#1 pin re-verify ✅ / #4 D-1+WSL 路径 ✅ / #5 不假 pin ✅ / #7 harness 18+25+2 DDC EQUAL ✅

### 3-Peer 对照（三家的规矩）
| peer | 覆盖 | 验证命令 |
|------|------|---------|
| JS (M0) | G00–G05 + INC/DEC/JMP + body-extend 全套 | `node .\yoyo-js\scripts\golden.js` |
| Rust | **739/739 golden**（G00–G05 + G-SM 全量 + JCC-ALL + IO + MEMCPY） | `cargo run -p verifier --bin yoyo -- test golden` |
| Python (asm peer) | 788 handlers, 3-chain DDC EQUAL (SHA-256: 4fb8b87f) | `python yoyo-asm\asm.py yoyo\projects\yoyo.ty out.exe` |
| Rust `.tyb` | 788 handlers, paper-tape DDC EQUAL (SHA-256: 4fb8b87f) | `yoyo.exe link --target=win32 yoyo.tyb out.exe` |
| asm | INC/DEC（经 WSL 编译+运行） | `.\scripts\verify-asm.ps1` |

比对方式：各 peer 对同一 opcode 序列 emit raw x64 bytes → hex text diff。平台无关 opcode 三家字节**必须一致**；平台相关 opcode（ALLOC/LOAD/WRITE）允许分叉。

### ISA / cross-peer gaps（NON-NORMATIVE · 自 W4.1 收纳）
body 今日 = **788 handlers / 4170 lines**（W-START 扩写后）。扩写勿静默碰下列面（非 Week 红，但是诚实缺口）：
- **D-1** `0x20/0x50/0x51`：JS 三码合流 movabs+store；Rust 走 `PlatformBackend`（Stub=movabs+store / Win=movabs+store）→ **peer 字节可分叉**；`yoyo.ty` 已练（H_2B-H_2D）。
- **D-2** `0x64 MOVRR`：两端今日等于 GET（JS load+store；Rust `emit_get`）；规范独立语义未强制 — Phase 2 cleanup。
- **D-3** `0x84/0x85`：两端真实 `rep movsb` emit；DDC EQUAL；JS REX.R + Rust 参数顺序 + pin byte-17 均已修平（body-extend-105）。
- **D-4**：gen1≡gen2 — 三端 DDC EQUAL，`.ty`==`.tyb`（SHA-256: 4fb8b87f），**GREEN**

### 下一拍待决（Next ops · 2026-07-28）
| # | pick | rationale（1 行） | Status |
|---|------|-------------------|--------|
| 1 | `0x66 INC slot` | H_17 + G-SM-INC | **GREEN (DONE)** |
| 2 | `0x67 DEC slot` | H_18 + G-SM-DEC | **GREEN (DONE)** |
| 3 | `0x70 JMP hh` | H_19 + G-SM-JMP | **GREEN (DONE)** |
| 4 | `0x41 CALL hh` | H_20 + G-SM-CALL | **GREEN (DONE)** |
| 5 | `0x71-7A Jcc hh` | H_21..H_2A + G-SM-JE + G-SM-JCC-ALL | **GREEN (DONE)** |
| 6 | `0x20/0x50/0x51` I/O | H_2B-H_2D + G-SM-IO | **GREEN (DONE)** |
| 7 | asm INC/DEC | `verify-asm.ps1` exit 0 | **GREEN (DONE)** |
| 8 | body-extend-001..106 | W-START 连续扩写；788 handlers · 739/739 golden · P2+P1 补齐 | **GREEN (DONE)** |
| 9 | yoyo.ty 注释恢复 | 850 行注释从 golden fixture 恢复 | **GREEN (DONE)** |
| 10 | MEMCPY real emit + executor expand | D-3 语义缺口关闭 · RSI/RDI/FC | **GREEN (DONE)** |
|| 11 | P2 imm 边界补齐 | LDB/ADD/SUB imm8/imm32 边界 handler 填充 | **GREEN (DONE)** |
|| 12 | P1 多 slot 变体 | INC/DEC/JMP/CALL 多目标 handler | **GREEN (DONE)** |
|| 13 | 3-chain section-ddc 实现 | Python asm peer — JS==Rust==Python EQUAL (SHA-256: 4fb8b87f) | **GREEN (DONE)** |
| 14 | `.tyb` 纸带格式 | 8B 记录，argc-dep 布局，Rust tyb_parser，DDC EQUAL | **GREEN (DONE)** |
| 15 | `--selfhost` HOT 自举框架 | emit.rs handler_offsets + pe_link selfhost + selfhost.rs | **GREEN (DONE)** |
| 16 | gen1≡gen2 | `.ty`==`.tyb` 产出一致 (SHA-256: 4fb8b87f)，三端 DDC EQUAL | **GREEN (DONE)** |
| 17 | selfhost startup 完整实现 | M1.exe 运行时读 .tyb → 复制 handler → 写 PE | **HOLD** |

### DDC 修复备注
#D-1 修复：Rust Win32Platform `emit_alloc`/`emit_load_file`/`emit_write_file` 从 VirtualAlloc 参数设置改为 movabs+store，与 JS M0 编译器匹配。DDC 恢复到 EQUAL。真实平台实现（VirtualAlloc IAT / syscall）延迟到 Phase 2。
#D-3 修复（body-extend-105）：MEMCPY_DATA/MEMCPY_STATE 三伤修平 — JS REX.R 0x4D→0x49、JS LEA scale 0x40→0xC0、Rust emit.rs 参数顺序 swap + pin byte-17 0xC8→0x00。JS==Rust==pin EQUAL。

### 最终 pin
`0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb`（`yoyo/tests/yoyo.ty.lock`，Decision #25）

### Quick verify
```powershell
cd f:\yoyo\yoyo-rust; cargo build -p verifier
cd f:\yoyo\yoyo-rust; cargo test -p verifier --bin yoyo
cd f:\yoyo; .\scripts\verify-selfhost.ps1
node .\scripts\verify-yoyo-ty.mjs
node .\yoyo-js\scripts\golden.js
cd f:\yoyo\yoyo-rust; cargo run -p verifier --bin yoyo -- test golden
.\scripts\verify-asm.ps1           # 需 WSL
node .\scripts\check-foundations.mjs
node .\scripts\check-plans.mjs
node .\scripts\check-cites.mjs
node .\scripts\check-sugar.mjs
```

---

### Week 0 — bootstrap 绿灯
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W0.1 | L-lock：`verify-yoyo-ty.mjs` → 0 | **GREEN (DONE)** |
| 2 | W0.2 | G00 + S-selfhost：`golden.js` + `verify-selfhost.ps1` → 0 | **GREEN (DONE)** |
| 3 | W0.3 | F-foundations + D-plan：`check-foundations` + `check-plans` → 0 | **GREEN (DONE)** |
| 4 | W0.4 | sugar-hello：`check-sugar.mjs` → 0 | **GREEN (DONE)** |
| 5 | W0.5 | Bib cites + bootstrap 骨架：`check-cites.mjs` → 0 | **GREEN (DONE)** |

### Week 1 — 稳定复验 + 诚实绑定
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W1.1 | Quick verify 全块可复验 | **GREEN (DONE)** |
| 2 | W1.2 | Phase 0 / 1b 命名命令写入；Part 10 保持 `[ ]` | **GREEN (DONE)** |
| 3 | W1.3 | `quarantine-gen.ps1` 冒烟 exit 0（≠ LOCKED） | **GREEN (DONE)** |
| 4 | W1.4 | stub 口径：不宣称 C-ddc / Morph / 冻结 | **GREEN (DONE)** |
| 5 | W1.5 | 本周唯一代码拍 = 文档绑定 | **GREEN (DONE)** |

### Week 2 — Golden 阶梯（G01–G02）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W2.1 | G01 fixture 绿 | **GREEN (DONE)** |
| 2 | W2.2 | G02 fixture 绿 | **GREEN (DONE)** |
| 3 | W2.3 | G01–G02 所需 emit（含 JS ORV 修复） | **GREEN (DONE)** |
| 4 | W2.4 | 信任锚 / hash 记录复验 | **GREEN (DONE)** |
| 5 | W2.5 | G-golden 诚实标「部分」；L-lock / selfhost 绿 | **GREEN (DONE)** |

### Week 3 — Golden 继续（G03–G05）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W3.1 | G03 CMP+JE 绿 | **GREEN (DONE)** |
| 2 | W3.2 | G04 CALL/RET 绿 | **GREEN (DONE)** |
| 3 | W3.3 | G05 named slots + stock_gui 冒烟 | **GREEN (DONE)** |
| 4 | W3.4 | lock + selfhost 仍 0（未 Relock） | **GREEN (DONE)** |
| 5 | W3.5 | 不宣称 G-golden 全套 / G06 | **GREEN (DONE)** |

### Week 4 — `yoyo.ty` 手扩（H_03）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W4.1 | ISA 缺口盘点（已收纳文首） | **GREEN (DONE)** |
| 2 | W4.2 | H_03 blank-init 手扩（21→33 lines） | **GREEN (DONE)** |
| 3 | W4.3 | 8-step Relock；`verify-yoyo-ty.mjs` → 0 | **GREEN (DONE)** |
| 4 | W4.4 | `verify-selfhost.ps1` → 0（2-chain） | **GREEN (DONE)** |
| 5 | W4.5 | 禁止广告冻结 / 全量自举 | **GREEN (DONE)** |

### Week 5 — Phase 2 继续 / gen 诚实
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W5.1 | H_04 手扩（仍非 Phase 2 完成） | **GREEN (DONE)** |
| 2 | W5.2 | `OUTPUT_DATA_NEED=0x38000` 口径；D4 仍阻塞 gen parity | **GREEN (DONE)** |
| 3 | W5.3 | PE ledger + quarantine Q3 inventory（≠ LOCKED） | **GREEN (DONE)** |
| 4 | W5.4 | D-plan 未广告（plan=`planned`） | **GREEN (DONE)** |
| 5 | W5.5 | full body + libyoyo migration 移出 v0.1 | **SCOPE-CUT** |

### Amendment W-SM — W-selfhost-min mechanical（H_05–H_16）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM | W-SM mechanical CLOSED at H_16 · Rust 18/18 · JS 6/6 · pin `c697b4b7…`（H_05–H_16 RAW_BYTE NOP/RET 链；scoped Rust seed；**非** full self-host） | **GREEN (DONE)** |

### Amendment W-SM-INC — arithmetic first beat（H_17）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-INC | H_17 `0x66 INC S[0x50]` + G-SM-INC · Rust **19/19** · JS 6/6 · 2-chain DDC EQUAL · pin `1042740e…`（emit `498b878002000048ffc049898780020000c3`；**非** JMP/DEC/G06/full self-host） | **GREEN (DONE)** |

### Amendment W-SM-DEC — arithmetic second beat（H_18）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-DEC | H_18 `0x67 DEC S[0x50]` + G-SM-DEC · Rust **20/20** · JS 6/6 · 2-chain DDC EQUAL · pin `c1930e22…`（emit `498b878002000048ffc849898780020000c3`；**非** JMP/G06/full self-host） | **GREEN (DONE)** |

### Amendment W-SM-JMP — control flow first beat（H_19）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-JMP | H_19 `0x70 JMP H_00` + G-SM-JMP · Rust **21/21** · JS 6/6 · 2-chain DDC EQUAL · pin `df280937…`（fixture `48b8000000000000000049898780020000c3e9e9ffffffc3`；canonical JMP@285 rel32=-290 target=H_00@0；**非** G06/full self-host） | **GREEN (DONE)** |

### Amendment W-SM-CALL — control flow second beat（H_20）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-CALL | H_20 `0x41 CALL H_00` + G-SM-CALL · Rust **22/22** · JS 6/6 · 2-chain DDC EQUAL · pin `ae31182d…`（fixture `48b8000000000000000049898780020000c3e8e9ffffffc3`；canonical CALL@291 rel32=-296 target=H_00@0；**非** G06/full self-host） | **GREEN (DONE)** |

### Amendment W-SM-JE — conditional branch first beat（H_21）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-JE | H_21 `0x71 JE H_00`（SET0 CMP JE） + G-SM-JE · Rust **23/23** · JS 6/6 · 2-chain DDC EQUAL · pin `59367665…`（fixture 76B；canonical JE@348 rel32=-354 target=H_00@0；**非** G06/full self-host） | **GREEN (DONE)** |

### Amendment W-SM-JCC-ALL — all 9 Jcc batch（H_22-H_2A）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-JCC-ALL | H_22..H_2A `0x72-0x7A 9 Jcc` + G-SM-JCC-ALL · Rust **24/24** · JS 6/6 · 2-chain DDC EQUAL（1024B）· pin `b8fd3dbd…`（fixture 540B all 9 Jcc to H_00；**非** G06/full self-host） | **GREEN (DONE)** |

### Amendment W-SM-IO — platform ops（H_2B-H_2D）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-IO | H_2B `0x20 ALLOC` + H_2C `0x50 LOAD_FILE` + H_2D `0x51 WRITE_FILE` · Rust **25/25** · JS 6/6 · 2-chain DDC DIFFER（code: Rust 961 vs JS 931, gap from startup/emit parity, see D-1）· pin `b830a7f5…`（fixture 72B） | **GREEN (DONE)** |

### Amendment asm INC/DEC — 3rd peer bootstrap
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | ASM-INC-DEC | asm `emit_inc_rax` + `emit_dec_rax` + `emit_store_state_rax` 原语；`yoyo-asm.s` 经 WSL 编译运行，emit INC/DEC S[0x50] 字节与 JS/Rust 完全一致（`verify-asm.ps1` exit 0） | **GREEN (DONE)** |

> 三家（JS/Rust/asm）INC/DEC 字节一致。下一步：asm 扩 JMP/CALL/Jcc 等，以实现真正的 3-chain DDC。

### Amendment W-START — Rust-first self-host START NODE（SCOPE-CUT 边界外）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-START | Rust-first self-host START NODE · **EXPERIMENTAL · 尝试已开始 2026-07-24**（attempt-N1 dispatch；minimal probes；**非** freeze / **非** 自举 GREEN；详表 `docs/auxdocs/selfhost-attempt-N1-log.md`） | **EXPERIMENTAL** |

---

# ═══ BOOK I — Identity & Norms / 身份与规范 ═══

> Reading path: gates, Forbidden, claim-class, N.6–N.8, then Part L.

---

## Part N: Normative Conventions (v3.3.5)

> **Status**: NORMATIVE. This Part defines how to read the rest of this document.

### N.1 Requirement Levels

The key words **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** are to be interpreted as in RFC 2119.

| Marker | Meaning |
|--------|---------|
| **NORMATIVE** | Binding on conforming implementations and agents |
| **NON-NORMATIVE** | Explanatory, historical, or aspirational; MUST NOT be treated as acceptance criteria |
| **ROADMAP** | Future work; acceptance criteria absent unless explicitly stated |

### N.2 Document Role Hierarchy

1. **This document (`PROMPT-v3.md`)** — single source of truth for *intent and acceptance*
2. **Pinned artifacts** (`yoyo/projects/yoyo.ty`, `yoyo/tests/yoyo.ty.lock`, golden hashes) — source of truth for *locked bytes*
3. **Implementation trees** (`yoyo-js/`, `yoyo-rust/`, `yoyo-asm/`) — MUST converge toward this spec; when they diverge, the divergence is a **defect**, not a redefinition of the language
4. **文首『当前进度 · Week 轴』** — NON-NORMATIVE daily progress dashboard; MUST NOT override this spec
5. ~~`STATUS.md` / `docs/PROGRESS-MAP.md`~~ — **deprecated** (merged into Week 轴); MUST NOT be maintained as a second hub

### N.3 What Success Means (Machine-Checkable)

**Documentation complete** ≠ **phase validated**.

| Class | Gate | Pass condition |
|-------|------|----------------|
| **D-docs** | Spec sections exist and cross-link | Human review of this file |
| **P-phase** | Phase acceptance checklist | Every `[ ]` item is an executable check (script or test name) |
| **L-lock** | Decision #13 | `scripts/verify-yoyo-ty.mjs` exit 0 |
| **S-selfhost** | M1≡M2≡M3 (JS path) | `scripts/verify-selfhost.ps1` exit 0 on *same* compare algorithm as Part 6.9 |
| **C-ddc** | 3-chain | `yoyo diff` / Part 6.9 extract-normalize equality across JS/Rust/asm peers |
| **G-golden** | Trust anchors | SHA-256 of pinned files match `docs/GOLDEN_HASH_*.txt` |
| **M-morph** | Part E Morph-Lock | Named morphology profile pinned + journal + Prove gates for mutation class (required when advertising non-default morph) |
| **M-posture** | Part E.19 | Energy↔perf pole/blend pins + transition interfaces; required when advertising env-driven posture switching. **Stub interfaces do not green this gate** |
| **F-foundations** | Part F | When advertising foundation-backed Plans: pinned `*.fdn` exists; every `maps_to` resolves; morph lists `foundation_ids` |
| **D-plan** | Part Deduce | When advertising Plan-derived morph/posture claims: `*.plan.*` present; steps use only allowed kinds; `ReplayRecord` hashes match before Relock |
| **M-gnd** | Part Gnd | When advertising physical units/constants/posture *scores with physical meaning*: pinned cite bundle + cite-resolution check. Default E0 with no physical claims MAY omit |

A phase is **validated** only when its **P-phase** row is green. Checkbox `[x]` in Part 10 means "validated in this bootstrap tree per 文首 Week 轴" only when the listed command is named and currently passes; otherwise it MUST be `[ ]` with a blocker note.

**Stub rule (v3.3)**: named interfaces MAY exist as stubs for scaffolding, but stubs MUST be labeled `NON-CONFORMING` / `STUB-FAIL` and **MUST NOT** make M-* / F-* / D-* / C-ddc gates pass.

### N.4 Forbidden Claims (Honesty Guard) 【class:FORBIDDEN】

Agents and docs MUST NOT claim:

- "Provable Thompson-attack resistance" / **"DDC / 3-chain ⇒ output provably correct"** (YOYO provides **detection under independence assumptions**, not a proof) 【class:FORBIDDEN】
- Independence probability literally `p³` without stating the independence hypothesis (Part 6.2)
- That YOYO is a general-purpose replacement for C/Rust
- That zero dynamic allocation holds for host toolchains (Node/V8, rustc) or for non-emit paths that still use `Vec` during bootstrap — Goals rows MUST scope to **emit/AM paths claiming EQUAL** (see N.4.1)
- That morphological adaptation (Part E) is Thompson-proof, or that a morph silently rewrites kernel invariants **K**
- Cross-profile `EQUAL` without declaring both morphology profile ids (Part E.5 / E.12)
- **"Self-evolving ⇒ Thompson-proof"**; **"Morph completed ⇒ K changed safely"** without document revision of K
- **"Unknown env handled"** without `morph.unknown-fallback` (Part E.9 / E.4 Eu)
- **"Radiation-hardened"** without Part 15.8 / 16.5 ROADMAP caveats
- That a **DANGEROUS** self-rewrite was **SAFE** because DDC was green on the *old* profile only
- **Silent EQUAL across posture poles** without scoped posture id / re-prove (Part 6.9.7 / §E.19.6)
- Silent drop of Appendix G trust-root disclosure when entering `energy-extreme`
- Silent ISA semantic break when flipping poles (subset fail-closed ≠ redefining opcode meaning)
- Silent drop of `foundation_ids` or ReplayRecord while advertising prior Plan-derived claims (Parts F / Deduce)
- **"YOYO trades away native codegen" / "no native codegen"** as a product trade (default emit is x86-64; see Part 3 honesty)
- Calling a posture switch **"seamless"** without meeting **§E.19.3** (corruption-free, bounded interruption, journal continuity)

#### N.4.1 Joint veto list (Round 1 B↔C + Role C scrub + Prompt-opt C2) — MUST

| Forbidden phrase / claim | Why |
|--------------------------|-----|
| **under any environment** / any-env universal morph | Marketing; Eu is degrade, not omniscience |
| **seamless** without citing §E.19.3 conditions | Marketing; engineering definition only |
| **stub / empty registry ⇒ green / conforming** | Fake acceptance |
| **asm ground truth** / **Gnd = physical ground truth** | asm is independent peer; Gnd is cite registry |
| **Gnd/CODATA proves `perf-extreme` or energy pole** | Preference pin ≠ physics endorsement |
| **YOYO implements / obeys \<physics law\>** | NON-CLAIM; cite only |
| **天人合一 / 推演天机 / 惊为天人 / 吃透一切** (and synonyms) | Non-operational mysticism |
| Using Gnd to shrink/delete Appendix G trust roots | Honesty break |
| **TheoryManifest + DeriveTick continuous daemon + SemanticDigest + ProofObject + TheoremRegistry** as v0.1 NORMATIVE CDS | Debate rejected as theater / second lifecycle |
| **Radiation / space-mission-ready as Phase 0–6 Final deliverable** | Contradicts Part L; Part 15/16 remain ROADMAP vision |
| **DDC / 3-chain ⇒ output provably correct** (and synonyms) | Same class as N.4 proof-grade claims |
| **Space-Ready / first mission** as a standalone title sentence without ROADMAP + not-flight-software caveats | Screenshot-grade marketing |
| **YOYO trades away native codegen** / **no native codegen** | Contradicts default x64 emit |
| **Goals row: Zero dynamic allocation** without emit/AM + N.4 scope | Host toolchains exempt |

#### N.4.2 Deduction / foundation honesty (Round 1 A↔D)

- Silent delete of `foundation_ids` while advertising prior claims
- Cross-foundation-set `EQUAL` without re-prove
- Natural-language prose as Prove evidence
- EMIT of unregistered artifact types as Relock evidence
- Advertising derived claims without `ReplayRecord` (plan hash + EMIT payload hash + foundation pins)

### N.5 Claim Classes (v3.3.2) — MUST

> **Status**: NORMATIVE. Replaces “cite every sentence like a paper.” Classes reduce hallucination by naming *what kind of claim* a sentence is — not by stacking decorative footnotes.

| Class | What it is | Required evidence |
|-------|------------|-------------------|
| **FACT** | Externally verifiable fact (constant, ABI, RFC keyword semantics, published threat paper *as history*) | Real `cite_id` from **Appendix Bib** (+ Gnd pin when physical meaning is advertised) |
| **DECISION** | This-repo resolution (slots, opcode count, lock steps, PPMPR, etc.) | Rationale + date + decision/id (Part 13 / N.4.1 row / debate doc) — **not** a fake paper |
| **INTERFACE** | Named gate, test, or schema surface | Gate/test id (Part N.3); stub ⇒ **NON-CONFORMING** / red |
| **ROADMAP** | Future work | Marker only — **no** cite theater |
| **FORBIDDEN** | Must-not-claim (honesty) | Points at N.4 / N.4.1 |

**【class:…】 tag rule (low-tier LLM readable)** — every **new** MUST / MUST NOT sentence (and every new Goals / Forbidden table row) MUST carry one of:

- inline tag: `【class:FACT】` / `【class:DECISION】` / `【class:INTERFACE】` / `【class:ROADMAP】` / `【class:FORBIDDEN】`
- **or** a table column named `Class` with the same token

Progressive only: existing 5k lines are **not** reclassified in one go. High-risk rows in §2.3 / N.4 / Part 3 MAY be tagged first.

#### N.5.1 Advertise-time cite rules — MUST

1. **Physical** claims (units/constants/scores with physical meaning) → Part **Gnd** pin; stay silent if unpinned.
2. **Thompson / DDC capability** claims → honest **detection ≠ proof** + `cite_id` (`Thompson84`, `WheelerDDC`); cite MUST NOT imply proven resistance.
3. **FACT** ads without a resolvable Bib entry / pin → **FORBIDDEN** to advertise.
4. Bib / Gnd are **not** a third truth source, not CDS theater, and MUST NOT expand Gnd into a footnote encyclopedia.

#### N.5.2 Cite checker — MUST

`scripts/check-cites.mjs` (or an extension of `scripts/check-*.mjs`) MUST:

- fail on **unknown** `cite_id`
- fail / stay red if Bib is empty while FACT ads are claimed green
- **MUST NOT** require every MUST sentence to carry a paper cite (DECISION / INTERFACE / ROADMAP / FORBIDDEN need class tags, not Thompson)

Stub rule applies until the checker is real: labeled NON-CONFORMING; MUST NOT fake-green.

### N.6 Landing diagnosis pointer (NON-NORMATIVE)

Encyclopedic single SoT + ambition ratchet + plan-over-delivery induce agent hallucination and rebuild loops. Vote record deleted; landing in git history / 文首 Week 轴. Do not expand mythology here.

### N.7 Gen quarantine / anti-rewrite pointer (NON-NORMATIVE)

When DDC/lock are green, “not good enough” alone MUST NOT authorize tearing down the repo — default **Morph→Prove→Relock**. Contamination authorizes quarantine and honest red, **not** Relock exemption or invent-green after a clean-slate myth. Essentials: script **`scripts/quarantine-gen.ps1`** (fail-closed); detail in git history. Do not dump full quarantine into this SoT; do not weaken N.4 / N.4.1.

### N.8 Authoring convenience vs Layer-S law 【class:DECISION】

> **Status**: NORMATIVE *pointer* + DECISION. Detail lives outside this SoT.

**Authoring convenience** (optional `.tys`: mnemonics, named slots, labels; Part 8 thin names on `.ty`; pretty/disasm views) is **not** the product identity and **MUST NOT** be compared as DDC/lock truth. **Layer-S** `.ty` (Part 4.0) remains the auditable source of law for lock pins and 3-chain peer inputs after deterministic desugar. 【class:DECISION】

Agents and docs MUST NOT:

- Sell “LLMs prefer machine/hex as authoring” as a design pillar or acceptance criterion 【class:FORBIDDEN】
- Treat NL prose as ISA, delete emit, panic-rewrite YOYO into a general-purpose PL, or silently regenerate LOCKED sources “for readability” 【class:FORBIDDEN】
- Advertise C-ddc / L-lock green from sugar sketches alone; `scripts/check-sugar.mjs` is ROADMAP and **fail-closed** until real 【class:INTERFACE】

Essentials: Part N.8 + `scripts/check-sugar.mjs` (fail-closed). Part L / Part 8 identity unchanged — sugar thickens the *surface*, not the *law*.

---

## Part L: Language Positioning (One Page)

> **Status**: NORMATIVE. Read this before treating YOYO as a programming language product.

### L.1 What YOYO Is

YOYO is a **verifiable, compiler-specialized ISA and toolchain**:

- A small **opcode set** (38 core ops) targeting **x86-64** code emission by default
- A **256-slot state machine** (u64 cells) as the primary abstract machine
- A **self-hosting compiler** expressed in that ISA (`yoyo.ty`) once the full body exists
- A **3-chain Diverse Double-Compiling (DDC)** verification method across JS / Rust / asm peers
- **Morphological adaptation** (Part E) — for a **named** env class with a **declared** degrade/profile, the stack MAY reconfigure representation, ISA surface, codegen host, resource model, I/O/ABI, and verification posture via **PPMPR**, without silently mutating kernel invariants **K**

YOYO's product questions are: **"Can I detect a compiler-level Thompson-style backdoor across diverse implementations?"** and **"Can the toolchain take a declared morphology under a named env class while remaining auditable?"** — not "Can I write applications comfortably?"

### L.2 What YOYO Is Not

| Not | Why out of scope |
|-----|------------------|
| C / Rust / Go replacement | No types, no libc, no package ecosystem |
| General-purpose PL | ISA is emit-oriented; ergonomics are secondary (Part 8 is thin sugar) |
| Formally verified compiler (CompCert-class) | No machine-checked semantics / proof artifacts in-tree |
| Portable "write once, run anywhere" | Default normative emit: **x86-64** on **Windows + Linux** (macOS MAY). Cross-arch / freestanding / tiny-RAM forms are **morphologies** (Part E), not a silent universal binary |
| Unaudited self-modifying "AI language" | Part E morphs are protocol-gated; FORBIDDEN to silently mutate **K** |
| Space-grade flight software | Part 15.8 / 16.5 are **ROADMAP / NON-NORMATIVE**; Part E.9 / E.4 E9 defines the interface only |
| Full SIMD language | Part 12 is **NON-NORMATIVE ROADMAP** |
| Native ternary CPU | Part 4.6 Trit is a **convention library**, not a CPU mode |

### L.3 In-Scope Deliverables

1. Spec-complete ISA + `.ty` grammar + operational semantics (Parts 4, 4S, G)
2. Three peer compilers/verifiers that can compile the same locked `yoyo.ty`
3. Normative DDC compare (Part 6.9) + reflective determinism check
4. Lock protocol for `yoyo.ty` (Part 9.4 — **8 steps**) + Morph-Lock when morphology changes (Part E.10)
5. Conformance suite (Appendix F)
6. Morphological adaptation architecture (Part E) — profiles, PPMPR, mutation classes; ROADMAP backends behind stable interfaces; **energy↔perf posture continuum** (Part E.19)
7. Pinned foundations + Plan-first deduction (Parts F / Deduce) + optional Physical Cite Registry (Part Gnd) when physical claims are advertised

### L.4 Explicit Out-of-Scope (Do Not Implement Under This Spec)

- New general-purpose language features (structs, GC, async, modules beyond handlers)
- Cross-architecture multi-target in one binary without a declared morphology profile (Mode 2 multi-segment / new ISA lowers are ROADMAP **A** modules under Part E)
- Regenerating `yoyo.ty` with an automated generator while LOCKED (DANGEROUS self-rewrite under unlock ≠ silent regenerator)
- Claiming DDC success while peers share emit source or platform stubs
- Claiming a morph is still LOCKED-release while in **MORPHING** (Part E.14 / 5B.2)

---

# ═══ BOOK II — Orientation / 入门导向 ═══

> Quick Start → architecture → goals → Thompson honesty.

---

## Part 0: Quick Start

> **If you read nothing else, read this.**

### Bootstrap commands (6 steps; hosted x64)

> Portability reality: normative CPU is **x86-64**; CI/bootstrap is validated on **Windows and/or Linux**. macOS MAY work via the same PE/ELF split but is not a Phase-0 gate. “Any platform” marketing is **NON-NORMATIVE**.

```bash
# 1. Enter the monorepo (4 projects)
cd yoyo-org   # or this workspace root

# 2. Build Rust verifier + libyoyo (PROJECT 3)
cd yoyo-rust && cargo build --release -p verifier -p libyoyo

# 3. Install JS compiler deps (PROJECT 2)
cd ../yoyo-js && npm install

# 4. Confirm golden hashes for trust anchors (when files exist)
cat ../docs/GOLDEN_HASH_js.txt
cat ../docs/GOLDEN_HASH_libyoyo.txt
cat ../docs/GOLDEN_HASH_asm.txt      # may be TBD until Phase 4d

# 5. Build M1 from locked yoyo.ty (PROJECT 1)
cd ..
node yoyo-js/src/yoyo.js yoyo/projects/yoyo.ty yoyo-js/build/M1.exe

# 6. Self-host / DDC check (profile depends on maturity — Part 6.9)
# Stub bootstrap: full-file SHA may be used (profile fullfile-bootstrap).
# Release claim: section-ddc on .text+.data via the same compare function.
scripts/verify-selfhost.ps1
# and/or: yoyo-rust peer link + ddc_compare(M_js, M_rust)
```

**Freeze rule**: Only after **full** `yoyo.ty` (not the stub) reaches M1≡M2≡M3 under `section-ddc` **and** Lock Protocol completes is the compiler “frozen at M3”. Stub trees MUST NOT claim freeze.

### Guarantees (conditional)

1. **If LOCKED**: `yoyo.ty` changes only via the **8-step** Lock Protocol (Part 9.4)
2. **If C-ddc green**: peers agree under Part 6.9 (`M_js≡M_rust≡M_asm` on compared sections) under a **declared morphology profile** (Part E)
3. **If morphing**: PPMPR + Morph-Lock (Part E); no LOCKED release claims while **MORPHING**
4. **Always**: trust roots in Appendix G remain irreducible (morphs do not delete them)

If a required gate fails → halt (Part 13).

---

## Part 1: 4-Project Architecture

|> v3 reorganizes YOYO around **4 independent projects**: canonical `yoyo` language + 3 implementations (yoyo-js / yoyo-rust / yoyo-asm).

### 1.1 The 4 Projects

| # | Project | Role |
|---|---------|------|
| 1 | **yoyo** | Canonical Language — `projects/yoyo.ty` (locked), ISA spec, format, libyoyo API, goldens |
| 2 | **yoyo-js** | Compiler (M0 seed 162 lines, JS portion of platform-emit) |
| 3 | **yoyo-rust** | Verifier + Stdlib (verifier/ + libyoyo/, Rust portion of platform-emit) |
| 4 | **yoyo-asm** | Independent DDC peer (`yoyo-asm.s` ~500 lines x64, asm portion of platform-emit) |

### 1.2 Why 4, Not 3 or 6

- **Why not 3:** Without `yoyo` as its own project, `projects/yoyo.ty` would live inside one impl; the other two would `git submodule` — destroying the independent input promise of 3-chain DDC.
- **Why not 6:** The earlier "6 entities" framing conflated role with org. `yoyo.js` lives inside `yoyo-js/`, `libyoyo` shares release cadence with the verifier inside `yoyo-rust/`, `platform-emit` is split 3 ways. **Project organization ≠ DDD role. Roles are 6; projects are 4.**

### 1.3 Project Layout (canonical, authoritative)

```
yoyo/                          # PROJECT 1: language (canonical)
  projects/yoyo.ty             # 🔒 locked source
  tests/yoyo.ty.lock           # Decision #13 lock
  tests/golden/                # post-freeze golden tests
  isa/  format/  api/libyoyo/  foundations/  plans/  gnd/
yoyo-js/                       # PROJECT 2: JS compiler (M0 + platform-emit portion)
yoyo-rust/                     # PROJECT 3: Rust verifier + libyoyo + platform-emit portion
yoyo-asm/                      # PROJECT 4: asm independent peer
tests/                         # repo-root pins (morph / foundations / plans / gnd)
```

### 1.4 Project Forbidden

| Project | Forbidden |
|---------|-----------|
| `yoyo` | platform-specific bytes (0xE8 / 0xE9 / FF 15 / IAT / syscall numbers / PE-ELF magic) |
| `yoyo-js` | emit algo different from `yoyo.ty`; holding `projects/yoyo.ty`; regenerating `yoyo.ty` while LOCKED |
| `yoyo-rust` | duplicating emit in verifier; compiling `yoyo.ty` for **distribution** (DDC peer only) |
| `yoyo-asm` | reusing yoyo-js's `platform/*` files; cleverness diverging from `yoyo.ty` |

Violation → reject + reset. See Part 9 Decision #13.
---

## Part 2: Context and Goals

### 2.1 What is YOYO

State-machine-based, self-hosting compiler toolchain (Part L) producing x64 binaries:
- **38 core instructions** (integer, control flow, memory, syscalls); source = hex token stream; opcode ids = u8 (Part 4.0)
- **256-slot state machine** (8 bytes/slot, accessed via R15)
- **Three diverse implementations** (JS / Rust / asm) compared by 3-chain DDC (Part 6.9)
- **Freeze-at-M3** only after full-body lock + DDC (stub trees are NOT frozen compilers)

### 2.2 Why YOYO Exists

Answer one question: **"Can I trust my compiler?"** Ken Thompson (1984) showed a compiler can hide a backdoor that (1) is not in source, (2) survives recompilation from clean source, (3) persists through the build chain. YOYO's design is a direct response — see Part 3 (honesty half-page) + Appendix T.

### 2.3 Goals

| Goal | Class | How Achieved |
|------|-------|--------------|
| **Divergence-detectable / auditable compilation** | FACT | 3-chain DDC (Part 6) — detection ≠ proof 【cite:Thompson84;WheelerDDC】 |
| **Small audit surface** | DECISION | 38-line ISA, 162-line seed (`yoyo.js`) |
| **Self-hosting** | DECISION | M0→M1→M2→M3 chain, frozen at M3 |
| **Cross-platform** | DECISION | libyoyo abstracts syscalls; `.tyo` + platform backends |
| **Deterministic (scoped)** | DECISION | Zero dynamic allocation **in emit/AM paths claiming EQUAL**; host toolchains exempt (N.4) |
| **Reliable** | DECISION | Full Result chain, no panics, budget-limited |
| **Morphological adaptation** | INTERFACE | Part E — PPMPR / Morph-Lock for **named** env class (not "any env") |
| **Energy↔perf posture continuum** | INTERFACE | Part E.19 — pole/blend switching; "seamless" only if §E.19.3 |
| **Pinned foundations + Plan-first deduction** | INTERFACE | Parts F / Deduce — `.fdn` + `*.plan.*` + ReplayRecord (no CDS daemon) |
| **Physical cite pins (optional)** | FACT | Part Gnd — pin units/constants when advertising physical meaning 【cite:CODATA2018】 |

### 2.4 Non-Goals

- **Performance as a product goal** — YOYO prioritizes auditability over speed. Does **not** forbid `posture.perf-extreme` **preference pin** (Part E.19); that pin is an auditable strategy, **not** a claim of throughput leadership.
- **Type safety** — ISA is untyped u64; types are application-level.
- **Rich ecosystem** — No package manager; libyoyo is a thin syscall façade, not a libc.
- **Cross-architecture compilation** — Single architecture per build (x86-64 normative).
- **User-friendly ergonomics** — Designed for auditors, not typical developers.
- **C/Rust replacement** — see Part L (explicit).
- **Provable absence of backdoors** — DDC detects divergence; it does not prove purity.

### 2.5 Project Layout (single authority)

**Part 1.3** is the single Project Layout source of truth. Do not maintain a second full tree here.
---

## Part 3: Thompson Honesty (half-page)

> **Status**: NORMATIVE for *honesty claims below*. Long Thompson narrative = **Appendix T** (NON-NORMATIVE).

Ken Thompson (1984, *Reflections on Trusting Trust*) showed a compiler backdoor can survive clean-source recompilation. YOYO’s response is **diverse double-compiling (DDC)** across independent peers — a **detection** method under an independence hypothesis, **not** a formal proof of purity (Part N.4). 【class:FACT】【cite:Thompson84;WheelerDDC】

| Claim YOYO MAY make | Claim YOYO MUST NOT make |
|---------------------|--------------------------|
| Peers diverge → attack/defect signal under Part 6.9 【class:FACT】 | “Output is provably correct” / “Thompson-proof” 【class:FORBIDDEN】 |
| Shrinking audit surface (ISA table, small seed) 【class:DECISION】 | Host OS / CPU / auditors are gone 【class:FORBIDDEN】 |
| DDC raises attacker cost if peers are independent 【class:FACT】 | Bare `p³` as a theorem without Part 6.2 hypothesis 【class:FORBIDDEN】 |

**Trade-off (honest)**: YOYO prioritizes auditability over feature richness and performance-as-product. Default path uses a state-machine AM; **native x86-64 emit exists** and is normative for hosted builds — do **not** claim “no native codegen.” Zero-dynamic-allocation claims are scoped to **emit/AM EQUAL paths** (Part N.4 / §2.3).

Irreducible trust roots remain in **Appendix G**. Background narrative: **Appendix T**.

---

# ═══ BOOK III — Core Language / 核心语言 ═══

> ISA → opsem → `.ty` grammar. (Part **G** = grammar; Part **Gnd** = cite registry.)

---

## Part 4: Core Architecture

### 4.0 Encoding Honesty (NORMATIVE)

YOYO has **two distinct encodings**. Conflating them is a spec defect.

| Layer | Name | Unit | Where used |
|-------|------|------|------------|
| **S** | **Source token stream** | Whitespace-separated hex / name tokens | `.ty` files (NORMATIVE for all peers) |
| **M** | **Machine opcode id** | Single `u8` in `0x00`–`0xFF` for core ISA | First token of each instruction; TIR `TirOp` |
| **X** | **x64 byte sequence** | Variable-length native code | Emit output / PE `.text` |
| **R** | **Roadmap 24-bit opcode space** | Conceptual `0x000000`–`0xFFFFFF` | Mode 2 multi-segment / SIMD ranges — **NON-NORMATIVE** until a phase acceptance test exists |

#### 4.0.1 Source Form (Layer S) — authoritative

A `.ty` program is a sequence of **instructions**. Each instruction is one logical line after comment stripping:

```
OPCODE_TOKEN ARG_TOKEN...
```

Examples (NORMATIVE):

```
40 20          ; HANDLER hh=0x20
30 50 00       ; SET slot=0x50 imm=0
68 50 51       ; ADDV dst=0x50 src=0x51
FF             ; RET
```

**MUST NOT** require a leading `00 00` pad before the opcode. Historical notes referring to executor form `00 00 <opcode>` are **obsolete** and non-normative.

#### 4.0.2 What “24-bit instruction encoding” means

Older prose called the ISA “24-bit”. In v3.1 that phrase means **only**:

1. The **roadmap opcode namespace** (Layer R) is large enough for future CPU_TYPE∥OPCODE packing; **or**
2. Informally, some x64 encodings emitted by primitives are multi-byte.

It does **NOT** mean that `.ty` files store packed 3-byte instruction words. Core opcodes are **8-bit ids**. Upper ranges `0x100+` in Part 4.1 Mode 2 / Part 12 are ROADMAP identifiers, not current `.ty` tokens (tokens are still hex bytes; multi-byte opcodes are not defined for Layer S in v3.1).

#### 4.0.3 Operand width in source

- Slot ids: hex integer in `0x00`–`0xFF` (or a name resolving to that range)
- Immediate values: hex integer; SET imm is u64 truncated per emit rules
- Handler ids `hh`: hex `0x00`–`0xFF`
- `RAW_BYTES` / `DATA` / `STR` / `RAW`: variadic byte tokens until end of line

---

### 4.1 Opcode Allocation (NORMATIVE Core Table)

All **38** core instructions use Layer-M ids in `0x00`–`0xFF`.

| Op | Mnemonic | Arity | Category | Emit summary |
|----|----------|-------|----------|--------------|
| 0x00 | NOP | 0 | Other | `90` |
| 0x10 | DATA | ≥0 var | Data | append args as bytes to `.data`; no `.text` |
| 0x12 | STR | ≥0 var | Data | same as DATA (string bytes) |
| 0x13 | RAW | ≥0 var | Data | same as DATA |
| 0x20 | ALLOC | 2 | Platform | `PlatformBackend::emit_alloc(slot,size)` |
| 0x30 | SET | 2 | Move | `movabs rax,imm` + `store_state slot` |
| 0x40 | HANDLER | 1 | Label | define label `hh` at current `.text` offset |
| 0x41 | CALL | 1 | Control | `E8 rel32` to handler `hh` |
| 0x50 | LOAD_FILE | 2 | Platform | `emit_load_file(slot,str_idx)` |
| 0x51 | WRITE_FILE | 3 | Platform | `emit_write_file(slot,str_idx,sz)` |
| 0x60 | GET | 2 | Move | `state[dst] ← state[src]` |
| 0x61 | SUB | 2 | Arith | `state[slot] ← state[slot] - imm` (imm as add/sub encoding rules) |
| 0x62 | ADD | 2 | Arith | `state[slot] ← state[slot] + imm` |
| 0x63 | IMUL | 2 | Arith | `state[dst] ← state[dst] * state[src]` (signed imul) |
| 0x64 | MOVRR | 2 | Move | alias of GET |
| 0x65 | CMP | 2 | Flags | `cmp state[a], state[b]` → sets EFLAGS; **no store** |
| 0x66 | INC | 1 | Arith | `state[slot]++` |
| 0x67 | DEC | 1 | Arith | `state[slot]--` |
| 0x68 | ADDV | 2 | Arith | `state[dst] ← state[dst] + state[src]` |
| 0x69 | ORV | 2 | Logic | `state[dst] ← state[dst] \| state[src]` (**bitwise OR**; MUST NOT emit ADD) |
| 0x6A | SUBV | 2 | Arith | `state[dst] ← state[dst] - state[src]` |
| 0x70 | JMP | 1 | Control | `E9 rel32` |
| 0x71 | JE | 1 | Control | `0F 84 rel32` (ZF=1) |
| 0x72 | JNE | 1 | Control | `0F 85 rel32` |
| 0x73 | JL | 1 | Control | `0F 8C rel32` (SF≠OF) |
| 0x74 | JGE | 1 | Control | `0F 8D rel32` |
| 0x75 | JLE | 1 | Control | `0F 8E rel32` |
| 0x76 | JG | 1 | Control | `0F 8F rel32` |
| 0x77 | JB | 1 | Control | `0F 82 rel32` (CF=1) |
| 0x78 | JAE | 1 | Control | `0F 83 rel32` |
| 0x79 | JBE | 1 | Control | `0F 86 rel32` |
| 0x7A | JA | 1 | Control | `0F 87 rel32` |
| 0x80 | LDB | 3 | Memory | see §4S.3 |
| 0x84 | MEMCPY_DATA | 3 | Memory | see §4S.3 |
| 0x85 | MEMCPY_STATE | 3 | Memory | see §4S.3 |
| 0xA0 | RAW_BYTE | 1 | Escape | emit 1 absolute byte |
| 0xA1 | RAW_BYTES | ≥1 var | Escape | emit N absolute bytes |
| 0xFF | RET | 0 | Control | `C3` |

**Retired / non-opcodes**: `0x82`–`0x83` as YOYO opcodes are **forbidden**. Those bytes are x64 Jcc second bytes inside `0F 8x` encodings, not Layer-M ids. Docs that listed JB/JAE at 0x82/0x83 were wrong.

**ISA table audit rule**: `yoyo-rust/verifier/src/isa_table.txt` MUST match this table. Known historical defect: `ORV => … add_reg …` is **incorrect**; MUST be `or_reg` / bitwise OR.

#### Mode 2 / SIMD ranges

The tables formerly under “24-bit Mode 2” and Part 12 opcode maps are **NON-NORMATIVE ROADMAP**. They MUST NOT be required for Phase 0–4d acceptance.

---

### 4.2 State Machine & Slot Map (ONE authoritative rule)

- Base: hosted = PE/ELF data section mapping; bare-metal = `0x9000` (Part 7.7)
- Access: `R15 + slot*8`
- Encoding: disp8 for slot 0–15; disp32 for 16–255

#### Authoritative reserved slot map

| Range | Purpose | Named-slot allocator |
|-------|---------|----------------------|
| `0x00` | RES_STARTUP_LEN (system) | **MUST NOT** auto-bind names here |
| `0x01`–`0x0F` | yoyo system / startup scratch | forbidden for names |
| `0x10`–`0x1F` | data pointers / data_base | forbidden |
| `0x20`–`0x3F` | string / data refs | forbidden |
| `0x40`–`0x4F` | handler-id scratch (optional use) | forbidden |
| **`0x50`–`0xCF`** | **User variables** | **first-occurrence names bind here** (`USER_SLOT_BASE=0x50`, `USER_SLOT_MAX=0xCF`) |
| `0xD0`–`0xFF` | Reserved future | forbidden for names; hex OK only with documented layout |

**Conflict resolution (v3.1)**: Earlier text that said user vars are only `0x80–0xCF` while names bind at `0x50+` is **void**. The single rule is **`0x50`–`0xCF` inclusive** for user/named slots (128 slots). Hex programs MAY still write system slots deliberately.

---

## Part 4S: Operational Semantics (NORMATIVE)

> Every conforming emitter MUST implement these transforms. Where a host path is a stub, the stub MUST be documented as **non-conforming until complete**, and DDC against a complete peer MUST fail closed.

### 4S.0 Abstract Machine State

```
State:
  S[0..255] : u64          # state slots; base pointer live in R15 at runtime
  CodePos   : u32          # emit cursor in .text (compile-time)
  Labels[hh]: Option<u32>  # handler entry offsets
  DataSeg   : byte[]       # .data contents
  Flags     : EFLAGS       # only meaningful after CMP / arith that sets flags
  Stack     : x64 stack    # used by CALL/RET only (native ABI)
```

**Compile-time** vs **run-time**: HANDLER/labels are compile-time; S[] updates describe the **emitted program’s run-time effect** unless noted.

### 4S.1 Conventions: HANDLER / CALL / RET / Stack

| Op | Preconditions | Effect | UB / Failure |
|----|---------------|--------|--------------|
| **HANDLER hh** | `hh ∈ 0..255`; first definition wins or duplicate → `IsaError::DuplicateLabel` (implementations MUST reject duplicates) | `Labels[hh] := CodePos`; emits **no** bytes | Duplicate hh → hard error |
| **CALL hh** | `Labels[hh]` defined after pass-2 | Emit `call rel32`; run-time: push return rip, jump to handler | Undefined hh → `LabelOutOfRange`; stack overflow is host UB outside YOYO budget |
| **JMP/Jcc hh** | label defined after pass-2; Jcc reads EFLAGS from **last flag-writing op** | Emit jmp/jcc; run-time branch | Same label error; **reading flags without prior CMP/arith is UNSPECIFIED** (programs MUST CMP before Jcc) |
| **RET** | — | Emit `C3`; run-time pop rip | Returning with empty stack is host UB |

**Calling convention (YOYO handlers)**:

- Handlers are **labels**, not C functions: no mandatory prologue/epilogue beyond what the handler body emits
- `CALL` uses the **native x64** `call`/`ret` pair
- Callee MAY freely clobber `rax`/`rcx` as used by primitives; programs that need preservation MUST save to slots
- `R15` MUST remain the state base for the duration of YOYO-emitted code (startup blob establishes it)

### 4S.2 Arithmetic / Logic / Flags → x64 condition codes

Flag-writing ops (SET does **not** set flags; INC/DEC/ADD/SUB/IMUL/ADDV/SUBV/ORV/CMP do per x64):

| YOYO op | x64 sequence (conceptual) | Flags |
|---------|---------------------------|-------|
| CMP a b | load a→rax, b→rcx; `cmp rax,rcx` | ZF/SF/OF/CF as cmp |
| ADD/SUB/INC/DEC/ADDV/SUBV | corresponding add/sub/inc/dec | per x64 |
| ORV | `or rax, rcx` | ZF/SF cleared OF/CF; SF from result |
| IMUL | `imul rax, rcx` | OF/CF per imul; others undefined on x64 |

Jcc mapping (Layer M → x64 cc byte in `0F cc rel32`):

| Op | Mnemonic | cc | Condition |
|----|----------|-----|-----------|
| 0x71 | JE | 0x84 | ZF=1 |
| 0x72 | JNE | 0x85 | ZF=0 |
| 0x73 | JL | 0x8C | SF≠OF |
| 0x74 | JGE | 0x8D | SF=OF |
| 0x75 | JLE | 0x8E | ZF=1 ∨ SF≠OF |
| 0x76 | JG | 0x8F | ZF=0 ∧ SF=OF |
| 0x77 | JB | 0x82 | CF=1 |
| 0x78 | JAE | 0x83 | CF=0 |
| 0x79 | JBE | 0x86 | CF=1 ∨ ZF=1 |
| 0x7A | JA | 0x87 | CF=0 ∧ ZF=0 |

### 4S.3 Memory: LDB / MEMCPY (address space & OOB)

**Address space (hosted)**:

- `LDB dd ss oo`: let `addr = S[ss] + oo` (oo zero-extended u16/u64 per emitter; normative: oo is unsigned offset added to pointer in `S[ss]`). Then `S[dd] ← zero_extend_u64( byte_load(addr) )`.
- Pointers in `S[ss]` are **host virtual addresses** produced by ALLOC / LOAD_FILE / startup — not YOYO-slot indices.
- **OOB**: if `addr` is not a readable mapping, behavior is **host fault** (process abort). Conforming compilers MUST still emit the `movzx` load; they MUST NOT insert bounds checks unless a future safety phase defines them. YOYO does **not** define a panic path for OOB.
- **Bare-metal**: same semantics against physical/identity map established by startup; OOB is likewise unchecked hardware fault.

**MEMCPY_DATA dst src n** (normative intent):

- `dst`, `src`, `n` are **slot ids**
- Copy `S[n]` bytes from address `S[src]` to address `S[dst]` (data/memory space)
- Overlap: UNSPECIFIED (programs MUST NOT overlap); recommend `memmove`-like only if all peers agree in conformance tests
- Failure: host fault on bad addresses; if `S[n]` exceeds remaining mapping → host fault
- **Bootstrap stub status**: an emitter MAY temporarily emit `ret` as a stub; such a peer is **non-conforming** for DDC against a full peer until memcpy is real

**MEMCPY_STATE dst src n**:

- Copy `S[n]` **slots** (each 8 bytes) from state region starting at slot `S[src]`? **No** — normative clarification:
  - Args `dst`,`src`,`n` are slot ids holding: destination **slot index**, source **slot index**, **byte count**
  - Copy `S[n]` bytes from `&S[ S[src] ]` to `&S[ S[dst] ]` within the 256×8 state blob
- **OOB**: if `S[dst]*8 + S[n] > 256*8` or same for src → emit-time error if statically known; else **UNSPECIFIED** at run-time (MUST be covered by conformance once implemented)
- Stub rule identical to MEMCPY_DATA

#### 4S.3.1 Encoder selection rule for signed immediates (NORMATIVE)

When emitting `add <reg>, imm` or `sub <reg>, imm` style sequences (used by `LDB` §4S.3 offset addition and by `add_imm` / `sub_imm` §4.3.2 primitives), emitters MUST select the operand width as follows:

- If the immediate value, **interpreted as a signed integer**, lies in `[-128, 127]` (i.e. fits in signed int8), emit the **imm8** encoding (`48 83 + ModRM(reg) + imm8`, 4 bytes).
- Otherwise emit the **imm32** encoding (`48 81 + ModRM(reg) + imm32` little-endian, 7 bytes). The imm32 value MUST be the same signed integer sign-extended to 32 bits (NOT zero-extended from a u16 source).

**Cross-peer ground truth** — the four-corner probe table `LDB-off{127,−128,128,−129,256}` in `docs/aux/three-peer-bytes.md` confirms this rule across JS / Rust / asm peers. The probe results **confirm** the rule; the rule itself is **normative** and applies to all current and future emitters independent of probe state.

**Conformance**: emitting imm8 for a value outside `[-128, 127]` (silent truncation) or imm32 for a value inside that range (3 wasted bytes + peer-equality break) is **non-conforming**.

See also: docs/aux/three-peer-bytes.md (LDB-off127, LDB-offm128, LDB-off128, LDB-offm129, LDB-off256) — confirms boundary rule across asm / JS / Rust.

### 4S.4 Platform ops failure behavior

| Op | Success | Failure |
|----|---------|---------|
| ALLOC | `S[slot]` ← pointer (or size marker in stub backends) | Backend returns `IsaError` at emit-time if unsupported; run-time failure is platform-specific (NULL / abort) — programs SHOULD check |
| LOAD_FILE | `S[slot]` ← buffer ptr / handle per backend contract | emit-time unsupported → error; run-time I/O fail → backend-defined |
| WRITE_FILE | bytes written | same |

Stub backends that store constants MUST be labeled `PlatformKind::Stub` and MUST NOT be used as a DDC peer for release claims.

### 4S.5 RAW_BYTE / RAW_BYTES

- Emit exact bytes into `.text`
- **No** validation of whether bytes form legal x64
- Auditors MUST grep `A0`/`A1` occurrences in locked sources
- UB: jumping into the middle of a multi-byte sequence is programmer responsibility

### 4S.6 Per-opcode compact semantics table

For each op: **Pre** = preconditions; **Δ** = state/memory/flag transform; **Fail** = error class.

| Op | Pre | Δ | Fail |
|----|-----|---|------|
| NOP | — | no architectural effect (`90`) | — |
| DATA/STR/RAW | — | append to DataSeg | — |
| SET | slot≤255 | S[slot]←imm | SlotOutOfRange |
| GET/MOVRR | slots≤255 | S[dst]←S[src] | SlotOutOfRange |
| ADD/SUB | slot≤255; imm fits add/sub encoding | S[slot]±←imm; flags | ImmOutOfRange / Slot |
| ADDV/SUBV | — | S[dst]±←S[src]; flags | Slot |
| ORV | — | S[dst]←S[dst]\|S[src]; flags | Slot |
| IMUL | — | S[dst]←S[dst]*S[src]; flags | Slot |
| INC/DEC | — | S[slot]±1; flags | Slot |
| CMP | — | flags←cmp(S[a],S[b]); no store | Slot |
| ALLOC/LOAD/WRITE | backend supports | platform Δ | IsaError / host |
| LDB | — | S[dd]←zx(mem[S[ss]+oo]) | host OOB fault |
| MEMCPY_* | n etc. | see §4S.3 | stub / host / OOB |
| HANDLER/CALL/JMP/Jcc/RET | §4S.1 | §4S.1 | LabelOutOfRange |
| A0/A1 | — | emit bytes | — |

---

### 4.3 The 13 Primitives

The 13 primitives are the **building blocks** of all 38 ISA instructions. Each emits a single x64 sequence and returns `Result<Vec<u8>, IsaError>`. All 13 emit identically across platforms (Part 7.5).

#### 4.3.1 State Machine Primitives (2)

```rust
/// Emits: `mov <dest>, [r15 + slot*8]`
/// Size: 4 bytes (slot ≤ 15, disp8) or 7 bytes (slot ≥ 16, disp32)
/// Always uses REX.WB (0x49 0x8B) because state base is R15.
pub fn load_state(slot: u16, dest: Reg) -> IsaResult<Vec<u8>> {
    if slot > 255 { return Err(IsaError::SlotOutOfRange { slot }); }
    let disp = (slot as u32) * 8;
    let modrm_reg = dest.modrm_bits();
    if disp <= 127 {
        Ok(vec![0x49, 0x8B, modrm_reg | 0x40, disp as u8])
    } else {
        let mut b = vec![0x49, 0x8B, modrm_reg | 0x80];
        b.extend_from_slice(&disp.to_le_bytes());
        Ok(b)
    }
}

/// Emits: `mov [r15 + slot*8], <src>`
/// Same encoding as load_state but opcode 0x89 (store).
/// Size: 4 bytes (slot ≤ 15) or 7 bytes (slot ≥ 16).
pub fn store_state(slot: u16, src: Reg) -> IsaResult<Vec<u8>>;
```

#### 4.3.2 Register-Immediate Primitives (3)

```rust
/// Emits: `movabs <reg>, imm64`
/// Size: 10 bytes (always — 0x48 + 0xB8+rd + 8-byte LE imm64)
pub fn movabs(reg: Reg, imm: u64) -> IsaResult<Vec<u8>>;

/// Emits: `add <reg>, imm`
/// Size: 4 bytes (imm fits in i8) or 7 bytes (imm fits in i32)
///         e.g. `48 83 C0 imm8` (4B) or `48 81 C0 imm32` (7B)
pub fn add_imm(reg: Reg, imm: u64) -> IsaResult<Vec<u8>>;

/// Emits: `sub <reg>, imm`
/// Size: 4 bytes (imm ∈ [-128, 127]) or 7 bytes (imm ∈ [-2³¹, 2³¹-1])
pub fn sub_imm(reg: Reg, imm: u64) -> IsaResult<Vec<u8>>;
```

#### 4.3.3 Register-Register Primitives (3)

```rust
/// Emits: `add <dst>, <src>` (3 bytes, ModRM encoded)
pub fn add_reg(dst: Reg, src: Reg) -> IsaResult<Vec<u8>>;

/// Emits: `or <dst>, <src>` (3 bytes) — used by ORV (MUST NOT alias add_reg)
pub fn or_reg(dst: Reg, src: Reg) -> IsaResult<Vec<u8>>;

/// Emits: `sub <dst>, <src>` (3 bytes)
pub fn sub_reg(dst: Reg, src: Reg) -> IsaResult<Vec<u8>>;

/// Emits: `imul <dst>, <src>` (4 bytes: `48 0F AF C2` style)
pub fn mul_reg(dst: Reg, src: Reg) -> IsaResult<Vec<u8>>;
```

#### 4.3.4 Comparison Primitive (1)

```rust
/// Emits: `cmp <a>, <b>` (3 bytes, ModRM encoded)
/// Sets EFLAGS; jcc_* primitives read EFLAGS to branch.
pub fn cmp_reg(a: Reg, b: Reg) -> IsaResult<Vec<u8>>;
```

#### 4.3.5 Control Flow Primitives (4)

```rust
/// Emits: `call <rel32>` (5 bytes: `E8 imm32`)
pub fn call_rel32(offset: i32) -> IsaResult<Vec<u8>>;

/// Emits: `jmp <rel32>` (5 bytes: `E9 imm32`)
pub fn jmp_rel32(offset: i32) -> IsaResult<Vec<u8>>;

/// Emits: `j<cc> <rel32>` (6 bytes: `0F 8x imm32`)
/// cc is x86 condition code byte (0x84=je, 0x85=jne, 0x8C=jl, 0x8D=jge, ...).
/// v3 standardizes JCC_TABLE generation in isaproc (Part 4.4).
pub fn jcc_rel32(cc: u8, offset: i32) -> IsaResult<Vec<u8>>;

/// Emits: `ret` (1 byte: `C3`)
pub fn ret() -> Vec<u8>;
```

#### 4.3.6 Encoding Constraints (compile-time enforced)

| Primitive | Constraint | Failure Mode |
|-----------|-----------|--------------|
| load_state / store_state | `slot: u16 ≤ 255` | `IsaError::SlotOutOfRange` |
| movabs | `imm: u64` always fits (u64 is 64-bit) | none |
| add_imm / sub_imm | `imm ∈ [i32::MIN, i32::MAX]` | `IsaError::ImmOutOfRange` |
| jcc_rel32 | `cc ∈ {0x84, 0x85, 0x8C, 0x8D, 0x8E, 0x8F, 0x82, 0x83, 0x86, 0x87}` | `IsaError::InvalidConditionCode` |
| call_rel32 / jmp_rel32 / jcc_rel32 | offset computable at emit time (rel32 = target - (current + 5)) | `IsaError::LabelOutOfRange` |

#### 4.3.7 Type: `Reg` Enum

```rust
pub enum Reg {
    Rax, Rcx, Rdx, Rbx, Rsp, Rbp, Rsi, Rdi,  // 8 legacy regs
    R8, R9, R10, R11, R12, R13, R14, R15,    // 8 extended regs
}
```

#### Type: `IsaError` Enum

```rust
pub enum IsaError {
    SlotOutOfRange { slot: u16 },
    ImmOutOfRange { value: u64, max: u64 },
    InvalidConditionCode { cc: u8 },
    InvalidRegister { reg: u8 },
    LabelOutOfRange { hh: u8 },
    BufferOverflow { needed: usize, available: usize },
    ArgCountMismatch { op: u8, expected: usize, got: usize },
    UndefinedName { name: String },
    DuplicateOpcode { op: u8 },
    BudgetExceeded { used: u64, max: u64 },
}

pub type IsaResult<T> = Result<T, IsaError>;
```

### 4.4 The isaproc Proc-Macro

`isaproc` is a Rust proc-macro crate in the yoyo-rust workspace (lives at `yoyo-rust/verifier/isa-proc/`). It reads `src/isa.rs` (38-line instruction table) and generates the entire dispatch, lower, and emit layer at compile time.

#### 4.4.1 Crate Structure

```
yoyo-rust/
└── verifier/
    └── isa-proc/
        ├── Cargo.toml              # proc-macro = true
        └── src/
            ├── lib.rs              # main proc-macro entry (300 lines)
            ├── isa_parser.rs       # parses src/isa.rs syntax (100 lines)
            └── codegen.rs          # generates Rust code (200 lines)
```

#### 4.4.2 Cargo.toml (isa-proc)

```toml
[package]
name = "isa-proc"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = { version = "2", features = ["full", "extra-traits"] }
quote = "1"
proc-macro2 = "1"
```

#### 4.4.3 ISA Syntax (v3 grammar)

`src/isa.rs` is **not** valid Rust by itself. It's parsed by `isaproc` at proc-macro expansion time:

```
0x30 SET slot imm => movabs rax imm store_state slot rax
```

| Token | Meaning |
|-------|---------|
| `0x30` | opcode (hex, 2-4 digits) |
| `SET` | mnemonic (ASCII identifier) |
| `slot imm` | parameter names (space-separated, becomes `Arg` enum variants) |
| `=>` | separator (required) |
| `movabs rax imm store_state slot rax` | emission pattern (reference to primitives + params by name) |

**Comments**: `;` or `#` to end of line.
**Multi-line**: Use `+` at end of line for continuation.

#### 4.4.4 Generated Code Surface

The proc-macro emits, at macro expansion time:

1. **`TirOp` enum** (~38 variants, one per ISA row)
2. **`lower_op(op, args) -> IsaResult<TirInst>`** — per-opcode dispatcher (lower source args → typed TIR)
3. **`emit_one(op, &mut FixedBuf, &EmitContext) -> IsaResult<()>`** — per-instruction x64 emission (calls primitives in spec order)
4. **`render_one(op, source) -> String`** — human-readable SOURCE / TIR / X86 column (Part 4.5)
5. **`instr_name(op) -> &'static str`** — mnemonic lookup
6. **`instr_branch_kind(op) -> BranchKind`** — branch metadata for fixup pass
7. **`opcode_from_u8(b: u8) -> Option<u8>`** — reverse lookup
8. **`JCC_TABLE: [u8; 10]`** — 10 condition codes for `0x71`-`0x7A` (kycc 71=je, 72=jne, 73=jl, 74=jge, 75=jle, 76=jg, 77=jb, 78=jae, 79=jbe, 7A=ja)
9. **`JCC_MNEMONIC: [&'static str; 10]`** — paired human names

#### 4.4.5 Invocation

```rust
// In src/tir.rs:
use isa_proc::isa;

// Two supported forms:
isa!(include_str!("isa_table.txt"));    // file path

isa! { r#"
    0x30 SET slot imm => movabs rax imm store_state slot rax
    0x60 GET dst src  => load_state src rax load_state dst rax
    ...
"# }                                     // inline string literal
```

#### 4.4.6 Failure Modes

| Failure | Trigger | Emit Error |
|---------|---------|------------|
| Duplicate opcode | Same hex on two lines | `compile_error!(...)` |
| Bad mnemonic (not Rust ident) | `0x30 3SET ...` | `compile_error!(...)` |
| Primitive not in registry | `mov_ri` typo | `compile_error!(...)` |
| Param referenced in pattern but not declared | `set foo x` with no `foo` in args | `compile_error!(...)` |

### 4.5 Emit Pipeline

```
.ty source → ty_parser::parse() → SourceLine[]
           → tir::lower() → TirInst[]
           → emit::emit() → x64 bytes
           → disasm::disasm() → disassembly
           → render::render() → three-column output
```

### 4.6 Ternary Data Model (Trit) — NON-NORMATIVE CONVENTION

|> Trit patterns are application-level convention on `u64` slots. **Not** part of core ISA acceptance criteria. Phase gates MUST NOT require Trit. Full content (4.6.1–4.6.7, decision patterns, anti-patterns) → `docs/aux/build.md` §"Trit conventions".

| Code | Meaning |
|------|---------|
| `0` | Sell / Negative / Oppose |
| `1` | Hold / Neutral / Wait |
| `2` | Buy / Positive / Support |

**Why u64 not a real ternary type**: (1) compiler internals use u64; (2) 38 ops suffice; (3) packing is paper advantage; (4) type safety misses logic bugs; (5) audit surface growth; (6) ISA frozen post-Phase 1. Trit is convention, not type.

# YOYO: Engineering Specification (v3.3.10)

> A self-hosting **compiler-specialized ISA / DDC toolchain** with **auditable** Thompson-style backdoor *detection* (not a formal proof), **4-project architecture**, **3-chain DDC verification**, **yoyo.ty lockdown**, and **protocol-gated morphology** (Part E). This document is the **single source of truth** for rebuilding YOYO from scratch.
>
> **Week axis (v3.3.10)** — **日常进度只看文首『当前进度 · Week 轴』**；下文 Parts 是规范参考，不是阅读/施工顺序。 Do **not** treat BOOK I–VIII or Part order as the work queue. Agents: one Week at a time — not “run whole PROMPT.”
>
> **v3.3.10 changelog** (**nav / progress dashboard only** — **no** NORMATIVE semantic change):
> - W-START NODE HOLD → **EXPERIMENTAL · 尝试已开始**（attempt-N1 dispatch；minimal probes；≠ 自举 GREEN；attempt-level checklist 全绿；`docs/auxdocs/selfhost-attempt-N1-log.md`）
>
> **v3.3.9 changelog** (**nav / progress dashboard only** — **no** NORMATIVE semantic change):
> - W-START NODE **HOLD** added（Rust-first self-host START；≠ freeze / ≠ 自举 GREEN）
>
> **v3.3.8 changelog** (**nav / progress dashboard only** — **no** NORMATIVE semantic change):
> - Folded durable W4.1 cross-peer gap notes (D-1…D-4) into 文首；deleted `docs/W4-GAP-INVENTORY.md`
>
> Markers **NORMATIVE** / **NON-NORMATIVE** / **ROADMAP** follow Part N. Debate CLOSED → git history.

---
### BOOK I — Identity & Norms / 身份与规范
- [Part N: Normative Conventions (v3.3.5)](#part-n-normative-conventions-v335)
- [Part L: Language Positioning (One Page)](#part-l-language-positioning-one-page)

> **Work queue** ≠ this ToC. See 文首『当前进度 · Week 轴』.

### BOOK II — Orientation / 入门导向
- [Part 0: Quick Start](#part-0-quick-start)
- [Part 1: 4-Project Architecture](#part-1-4-project-architecture)
- [Part 2: Context and Goals](#part-2-context-and-goals)
- [Part 3: Thompson Honesty (half-page)](#part-3-thompson-honesty-half-page)

### BOOK III — Core Language / 核心语言
- [Part 4: Core Architecture (ISA / State Machine / Primitives)](#part-4-core-architecture)
- [Part 4S: Operational Semantics (NORMATIVE)](#part-4s-operational-semantics-normative)
- [Part G: Formal `.ty` Grammar (NORMATIVE)](#part-g-formal-ty-grammar-normative)

### BOOK IV — Toolchain & Verification / 工具链与验证
- [Part 5: Self-Hosting Chain](#part-5-self-hosting-chain)
- [Part 5B: Cold-Start, Bootstrap & LOCKED Lifecycle](#part-5b-cold-start-bootstrap--locked-lifecycle-normative)
- [Part 6: DDC Verification (3-Chain)](#part-6-ddc-verification-3-chain)
- [Part 7: Platform Abstraction](#part-7-platform-abstraction)
- [Part 8: Variable / Name Layer](#part-8-variable--name-layer)
- [Part 9: Safety Architecture (4 Properties + 13 Decisions)](#part-9-safety-architecture-4-properties--13-decisions)

### BOOK V — Protocol Extension (Morph) / 协议扩展（形态）
- [Part E: Morphological Adaptation (Protocol)](#part-e-morphological-adaptation-protocol-normative)

### BOOK VI — Satellite Specs / 卫星规格
- [Part F: Foundations](#part-f-foundations-normative)
- [Part Deduce: Plan-first Deduction](#part-deduce-plan-first-deduction-normative-v01)
- [Part Gnd: Physical Cite Registry](#part-gnd-physical-cite-registry-normative-short)
- [Part S: v0.1 Scope Table](#part-s-v01-scope-table-normative)

### BOOK VII — Delivery / 交付与阶段
- [Part 10: 6-Phase Execution Plan](#part-10-6-phase-execution-plan)
- [Part 11: Cross-Project Comparison](#part-11-cross-project-comparison)
- [Part 12: SIMD Extensions (NON-NORMATIVE ROADMAP)](#part-12-simd-extensions)
- [Part 13: Decision History + Anti-Patterns](#part-13-decision-history--anti-patterns)
- [Part 14: Maintainer Role + Custody Workflow](#part-14-maintainer-role--custody-workflow)
- [Part 15: Demos & Use Cases (mostly NON-NORMATIVE)](#part-15-demos--use-cases)
- [Part 16: Master Roadmap (NON-NORMATIVE)](#part-16-master-roadmap-extensions)

### BOOK VIII — Appendices / 附录
- [Appendix A: libyoyo API + 3-Platform Implementation](#appendix-a-libyoyo-api--3-platform-implementation)
- [Appendix B: yoyo-asm Third Implementation](#appendix-b-yoyo-asm-third-implementation)
- [Appendix C: Cross-Platform Story (Why libyoyo)](#appendix-c-cross-platform-story-why-libyoyo)
- [Appendix D: Anti-Patterns Catalog](#appendix-d-anti-patterns-catalog)
- [Appendix E: Build & Test + Reference Documents](#appendix-e-build--test--reference-documents)
- [Appendix F: Conformance Suite](#appendix-f-conformance-suite-normative-definition)
- [Appendix G: Trust Roots Inventory](#appendix-g-trust-roots-inventory-normative-disclosure)
- [Appendix H: Future Deduction Substrate](#appendix-h-future-deduction-substrate-non-normative-roadmap--out-of-v01)
- [Appendix T: Thompson 1984 Background](#appendix-t-thompson-1984-background-non-normative)
- [Appendix Bib: FACT Bibliography](#appendix-bib-fact-bibliography-non-normative-for-fact-use)
- [Appendix CH: Prior Changelog Archive](#appendix-ch-prior-changelog-archive-non-normative)

---

## 当前进度 · Week 轴（NON-NORMATIVE · 日常只看这里）

> **日常入口。** 规格语义仍在下方 Parts；施工顺序 = 本 Week 轴。Status 仅允许：**GREEN (DONE)** / **RED** / **SCOPE-CUT** / **HOLD**。勿再维护 `STATUS.md` / `docs/PROGRESS-MAP.md`（已并入此处）。

### 你现在在哪
`yoyo.ty` = **788 handlers / 4170 lines**（850 行注释已恢复）；Rust golden **739/739 PASS**· executor **8/8 PASS**；JS==Rust==Python 三端字节级相等（3-chain DDC EQUAL，SHA-256: `4fb8b87f`）。W-START body-extend-001..106 全部 GREEN；MEMCPY real emit + LEA scale fix + executor expand + DDC fix；P2 imm 边界 + P1 多 slot 变体补齐；`.tyb` 纸带格式（8B 记录）就绪；`--selfhost` HOT 自举框架就绪；pin `0275802d2b4459e6…`（Decision #25）。

### 仍红（big list）
full compiler self-host · 冻结编译器

### W-START NODE（EXPERIMENTAL · body-extend 扩写完成 · 2026-07-24 点火 · 2026-07-28 收束 · ≠ freeze）
`EXPERIMENTAL · NON-GREEN · Rust-first · OUT-OF-v0.1-body（SCOPE-CUT 边界外点火）` — 详表 `docs/auxdocs/selfhost-start-node.md`
- **attempt ≠ freeze ≠ full self-host**；开火≠仍红翻绿；失败不 Relock / 不假 pin；产物仅 `EXPERIMENTAL`（不自动仍红→绿）
- **Workflow Hard Rule (non-normative; behavior, not law)** — default-first: 下一拍明显时直接执行默认 + 上一个子代理参数，**不再列选项问 A/B/C**；仅在 (a) 工具链缺、(b) 观测到 peer 分叉、(c) lock pin 想改但无既有 log、(d) PROMPT 要改 NORMATIVE（如 bump version）时才停下问；每拍成功仍产 `docs/auxdocs/<attempt|topic>-N-log.md`；不复述 dashboard/审计汇总，只接上一拍摘要 + 1 行下一拍。
- Checklist（压缩）：冷启复验文首+pin · Lock/Relock 一致（无 LOCKED 不谈 freeze）· scope 标签 · D-1/平台分叉 fail-closed · stub/RAW_BYTE 不宣称 C-ddc / Morph / freeze / gen1≡gen2
- 「尝试已开始」= 可复现 Rust 入口（cmd+log+scope tag）+ checklist（**attempt-level 全绿**；见 `docs/auxdocs/selfhost-attempt-N1-log.md`）
- 「自举 GREEN」= 仍红项（full body · Freeze+Lock）— **START NODE 一律不豁免**；Freeze = end gate（Part 5）；full body 仍在 W5.5 **SCOPE-CUT**；3-chain section-ddc 已达成（Python asm peer EQUAL）；gen1≡gen2 已达成（`.ty`==`.tyb` 三端一致）
- body-extend 连续扩写（EXPERIMENTAL · ≠ stub 34）：控制面 `docs/auxdocs/body-extend-queue.md` — scratch≤8 并发 / Relock 单写 / **矩阵满即停（matrix coverage gate）**；现 **788 handlers** · pin `0275802d2b4459e6…`（Decision #25）· body-extend-106 DONE（P2 imm 边界 + P1 多 slot）
- 入口（最小，不真编）：`cd f:\yoyo; .\scripts\verify-asm.ps1; node .\yoyo-js\scripts\golden.js; .\scripts\verify-selfhost.ps1; cd f:\yoyo\yoyo-rust; cargo run -p verifier --bin yoyo -- test golden`
- attempt-level 4 critical-path：#1 pin re-verify ✅ / #4 D-1+WSL 路径 ✅ / #5 不假 pin ✅ / #7 harness 18+25+2 DDC EQUAL ✅

### 3-Peer 对照（三家的规矩）
| peer | 覆盖 | 验证命令 |
|------|------|---------|
| JS (M0) | G00–G05 + INC/DEC/JMP + body-extend 全套 | `node .\yoyo-js\scripts\golden.js` |
| Rust | **739/739 golden**（G00–G05 + G-SM 全量 + JCC-ALL + IO + MEMCPY） | `cargo run -p verifier --bin yoyo -- test golden` |
| Python (asm peer) | 788 handlers, 3-chain DDC EQUAL (SHA-256: 4fb8b87f) | `python yoyo-asm\asm.py yoyo\projects\yoyo.ty out.exe` |
| Rust `.tyb` | 788 handlers, paper-tape DDC EQUAL (SHA-256: 4fb8b87f) | `yoyo.exe link --target=win32 yoyo.tyb out.exe` |
| asm | INC/DEC（经 WSL 编译+运行） | `.\scripts\verify-asm.ps1` |

比对方式：各 peer 对同一 opcode 序列 emit raw x64 bytes → hex text diff。平台无关 opcode 三家字节**必须一致**；平台相关 opcode（ALLOC/LOAD/WRITE）允许分叉。

### ISA / cross-peer gaps（NON-NORMATIVE · 自 W4.1 收纳）
body 今日 = **788 handlers / 4170 lines**（W-START 扩写后）。扩写勿静默碰下列面（非 Week 红，但是诚实缺口）：
- **D-1** `0x20/0x50/0x51`：JS 三码合流 movabs+store；Rust 走 `PlatformBackend`（Stub=movabs+store / Win=movabs+store）→ **peer 字节可分叉**；`yoyo.ty` 已练（H_2B-H_2D）。
- **D-2** `0x64 MOVRR`：两端今日等于 GET（JS load+store；Rust `emit_get`）；规范独立语义未强制 — Phase 2 cleanup。
- **D-3** `0x84/0x85`：两端真实 `rep movsb` emit；DDC EQUAL；JS REX.R + Rust 参数顺序 + pin byte-17 均已修平（body-extend-105）。
- **D-4**：gen1≡gen2 — 三端 DDC EQUAL，`.ty`==`.tyb`（SHA-256: 4fb8b87f），**GREEN**

### 下一拍待决（Next ops · 2026-07-28）
| # | pick | rationale（1 行） | Status |
|---|------|-------------------|--------|
| 1 | `0x66 INC slot` | H_17 + G-SM-INC | **GREEN (DONE)** |
| 2 | `0x67 DEC slot` | H_18 + G-SM-DEC | **GREEN (DONE)** |
| 3 | `0x70 JMP hh` | H_19 + G-SM-JMP | **GREEN (DONE)** |
| 4 | `0x41 CALL hh` | H_20 + G-SM-CALL | **GREEN (DONE)** |
| 5 | `0x71-7A Jcc hh` | H_21..H_2A + G-SM-JE + G-SM-JCC-ALL | **GREEN (DONE)** |
| 6 | `0x20/0x50/0x51` I/O | H_2B-H_2D + G-SM-IO | **GREEN (DONE)** |
| 7 | asm INC/DEC | `verify-asm.ps1` exit 0 | **GREEN (DONE)** |
| 8 | body-extend-001..106 | W-START 连续扩写；788 handlers · 739/739 golden · P2+P1 补齐 | **GREEN (DONE)** |
| 9 | yoyo.ty 注释恢复 | 850 行注释从 golden fixture 恢复 | **GREEN (DONE)** |
| 10 | MEMCPY real emit + executor expand | D-3 语义缺口关闭 · RSI/RDI/FC | **GREEN (DONE)** |
|| 11 | P2 imm 边界补齐 | LDB/ADD/SUB imm8/imm32 边界 handler 填充 | **GREEN (DONE)** |
|| 12 | P1 多 slot 变体 | INC/DEC/JMP/CALL 多目标 handler | **GREEN (DONE)** |
|| 13 | 3-chain section-ddc 实现 | Python asm peer — JS==Rust==Python EQUAL (SHA-256: 4fb8b87f) | **GREEN (DONE)** |
| 14 | `.tyb` 纸带格式 | 8B 记录，argc-dep 布局，Rust tyb_parser，DDC EQUAL | **GREEN (DONE)** |
| 15 | `--selfhost` HOT 自举框架 | emit.rs handler_offsets + pe_link selfhost + selfhost.rs | **GREEN (DONE)** |
| 16 | gen1≡gen2 | `.ty`==`.tyb` 产出一致 (SHA-256: 4fb8b87f)，三端 DDC EQUAL | **GREEN (DONE)** |
| 17 | selfhost startup 完整实现 | M1.exe 运行时读 .tyb → 复制 handler → 写 PE | **HOLD** |

### DDC 修复备注
#D-1 修复：Rust Win32Platform `emit_alloc`/`emit_load_file`/`emit_write_file` 从 VirtualAlloc 参数设置改为 movabs+store，与 JS M0 编译器匹配。DDC 恢复到 EQUAL。真实平台实现（VirtualAlloc IAT / syscall）延迟到 Phase 2。
#D-3 修复（body-extend-105）：MEMCPY_DATA/MEMCPY_STATE 三伤修平 — JS REX.R 0x4D→0x49、JS LEA scale 0x40→0xC0、Rust emit.rs 参数顺序 swap + pin byte-17 0xC8→0x00。JS==Rust==pin EQUAL。

### 最终 pin
`0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb`（`yoyo/tests/yoyo.ty.lock`，Decision #25）

### Quick verify
```powershell
cd f:\yoyo\yoyo-rust; cargo build -p verifier
cd f:\yoyo\yoyo-rust; cargo test -p verifier --bin yoyo
cd f:\yoyo; .\scripts\verify-selfhost.ps1
node .\scripts\verify-yoyo-ty.mjs
node .\yoyo-js\scripts\golden.js
cd f:\yoyo\yoyo-rust; cargo run -p verifier --bin yoyo -- test golden
.\scripts\verify-asm.ps1           # 需 WSL
node .\scripts\check-foundations.mjs
node .\scripts\check-plans.mjs
node .\scripts\check-cites.mjs
node .\scripts\check-sugar.mjs
```

---

### Week 0 — bootstrap 绿灯
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W0.1 | L-lock：`verify-yoyo-ty.mjs` → 0 | **GREEN (DONE)** |
| 2 | W0.2 | G00 + S-selfhost：`golden.js` + `verify-selfhost.ps1` → 0 | **GREEN (DONE)** |
| 3 | W0.3 | F-foundations + D-plan：`check-foundations` + `check-plans` → 0 | **GREEN (DONE)** |
| 4 | W0.4 | sugar-hello：`check-sugar.mjs` → 0 | **GREEN (DONE)** |
| 5 | W0.5 | Bib cites + bootstrap 骨架：`check-cites.mjs` → 0 | **GREEN (DONE)** |

### Week 1 — 稳定复验 + 诚实绑定
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W1.1 | Quick verify 全块可复验 | **GREEN (DONE)** |
| 2 | W1.2 | Phase 0 / 1b 命名命令写入；Part 10 保持 `[ ]` | **GREEN (DONE)** |
| 3 | W1.3 | `quarantine-gen.ps1` 冒烟 exit 0（≠ LOCKED） | **GREEN (DONE)** |
| 4 | W1.4 | stub 口径：不宣称 C-ddc / Morph / 冻结 | **GREEN (DONE)** |
| 5 | W1.5 | 本周唯一代码拍 = 文档绑定 | **GREEN (DONE)** |

### Week 2 — Golden 阶梯（G01–G02）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W2.1 | G01 fixture 绿 | **GREEN (DONE)** |
| 2 | W2.2 | G02 fixture 绿 | **GREEN (DONE)** |
| 3 | W2.3 | G01–G02 所需 emit（含 JS ORV 修复） | **GREEN (DONE)** |
| 4 | W2.4 | 信任锚 / hash 记录复验 | **GREEN (DONE)** |
| 5 | W2.5 | G-golden 诚实标「部分」；L-lock / selfhost 绿 | **GREEN (DONE)** |

### Week 3 — Golden 继续（G03–G05）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W3.1 | G03 CMP+JE 绿 | **GREEN (DONE)** |
| 2 | W3.2 | G04 CALL/RET 绿 | **GREEN (DONE)** |
| 3 | W3.3 | G05 named slots + stock_gui 冒烟 | **GREEN (DONE)** |
| 4 | W3.4 | lock + selfhost 仍 0（未 Relock） | **GREEN (DONE)** |
| 5 | W3.5 | 不宣称 G-golden 全套 / G06 | **GREEN (DONE)** |

### Week 4 — `yoyo.ty` 手扩（H_03）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W4.1 | ISA 缺口盘点（已收纳文首） | **GREEN (DONE)** |
| 2 | W4.2 | H_03 blank-init 手扩（21→33 lines） | **GREEN (DONE)** |
| 3 | W4.3 | 8-step Relock；`verify-yoyo-ty.mjs` → 0 | **GREEN (DONE)** |
| 4 | W4.4 | `verify-selfhost.ps1` → 0（2-chain） | **GREEN (DONE)** |
| 5 | W4.5 | 禁止广告冻结 / 全量自举 | **GREEN (DONE)** |

### Week 5 — Phase 2 继续 / gen 诚实
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W5.1 | H_04 手扩（仍非 Phase 2 完成） | **GREEN (DONE)** |
| 2 | W5.2 | `OUTPUT_DATA_NEED=0x38000` 口径；D4 仍阻塞 gen parity | **GREEN (DONE)** |
| 3 | W5.3 | PE ledger + quarantine Q3 inventory（≠ LOCKED） | **GREEN (DONE)** |
| 4 | W5.4 | D-plan 未广告（plan=`planned`） | **GREEN (DONE)** |
| 5 | W5.5 | full body + libyoyo migration 移出 v0.1 | **SCOPE-CUT** |

### Amendment W-SM — W-selfhost-min mechanical（H_05–H_16）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM | W-SM mechanical CLOSED at H_16 · Rust 18/18 · JS 6/6 · pin `c697b4b7…`（H_05–H_16 RAW_BYTE NOP/RET 链；scoped Rust seed；**非** full self-host） | **GREEN (DONE)** |

### Amendment W-SM-INC — arithmetic first beat（H_17）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-INC | H_17 `0x66 INC S[0x50]` + G-SM-INC · Rust **19/19** · JS 6/6 · 2-chain DDC EQUAL · pin `1042740e…`（emit `498b878002000048ffc049898780020000c3`；**非** JMP/DEC/G06/full self-host） | **GREEN (DONE)** |

### Amendment W-SM-DEC — arithmetic second beat（H_18）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-DEC | H_18 `0x67 DEC S[0x50]` + G-SM-DEC · Rust **20/20** · JS 6/6 · 2-chain DDC EQUAL · pin `c1930e22…`（emit `498b878002000048ffc849898780020000c3`；**非** JMP/G06/full self-host） | **GREEN (DONE)** |

### Amendment W-SM-JMP — control flow first beat（H_19）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-JMP | H_19 `0x70 JMP H_00` + G-SM-JMP · Rust **21/21** · JS 6/6 · 2-chain DDC EQUAL · pin `df280937…`（fixture `48b8000000000000000049898780020000c3e9e9ffffffc3`；canonical JMP@285 rel32=-290 target=H_00@0；**非** G06/full self-host） | **GREEN (DONE)** |

### Amendment W-SM-CALL — control flow second beat（H_20）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-CALL | H_20 `0x41 CALL H_00` + G-SM-CALL · Rust **22/22** · JS 6/6 · 2-chain DDC EQUAL · pin `ae31182d…`（fixture `48b8000000000000000049898780020000c3e8e9ffffffc3`；canonical CALL@291 rel32=-296 target=H_00@0；**非** G06/full self-host） | **GREEN (DONE)** |

### Amendment W-SM-JE — conditional branch first beat（H_21）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-JE | H_21 `0x71 JE H_00`（SET0 CMP JE） + G-SM-JE · Rust **23/23** · JS 6/6 · 2-chain DDC EQUAL · pin `59367665…`（fixture 76B；canonical JE@348 rel32=-354 target=H_00@0；**非** G06/full self-host） | **GREEN (DONE)** |

### Amendment W-SM-JCC-ALL — all 9 Jcc batch（H_22-H_2A）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-JCC-ALL | H_22..H_2A `0x72-0x7A 9 Jcc` + G-SM-JCC-ALL · Rust **24/24** · JS 6/6 · 2-chain DDC EQUAL（1024B）· pin `b8fd3dbd…`（fixture 540B all 9 Jcc to H_00；**非** G06/full self-host） | **GREEN (DONE)** |

### Amendment W-SM-IO — platform ops（H_2B-H_2D）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-SM-IO | H_2B `0x20 ALLOC` + H_2C `0x50 LOAD_FILE` + H_2D `0x51 WRITE_FILE` · Rust **25/25** · JS 6/6 · 2-chain DDC DIFFER（code: Rust 961 vs JS 931, gap from startup/emit parity, see D-1）· pin `b830a7f5…`（fixture 72B） | **GREEN (DONE)** |

### Amendment asm INC/DEC — 3rd peer bootstrap
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | ASM-INC-DEC | asm `emit_inc_rax` + `emit_dec_rax` + `emit_store_state_rax` 原语；`yoyo-asm.s` 经 WSL 编译运行，emit INC/DEC S[0x50] 字节与 JS/Rust 完全一致（`verify-asm.ps1` exit 0） | **GREEN (DONE)** |

> 三家（JS/Rust/asm）INC/DEC 字节一致。下一步：asm 扩 JMP/CALL/Jcc 等，以实现真正的 3-chain DDC。

### Amendment W-START — Rust-first self-host START NODE（SCOPE-CUT 边界外）
| # | id | 任务 | Status |
|---|-----|------|--------|
| 1 | W-START | Rust-first self-host START NODE · **EXPERIMENTAL · 尝试已开始 2026-07-24**（attempt-N1 dispatch；minimal probes；**非** freeze / **非** 自举 GREEN；详表 `docs/auxdocs/selfhost-attempt-N1-log.md`） | **EXPERIMENTAL** |

---

# ═══ BOOK I — Identity & Norms / 身份与规范 ═══

> Reading path: gates, Forbidden, claim-class, N.6–N.8, then Part L.

---


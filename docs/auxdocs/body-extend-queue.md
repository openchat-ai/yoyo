# body-extend 连续队列控制面（NON-NORMATIVE · EXPERIMENTAL）

> 2026-07-26。回答「能并发吗 / 有没有终结点」。  
> 不宣称 freeze / 自举 GREEN / W5.5 full body。用户 STOP 永远优先。

## 现状快照

| 项 | 值 |
|----|-----|
| pin | `514ff62ce8663a15…`（body-extend-101 Relock） |
| handlers | **747**（H_00..H_740） |
| 下一拍 | `parallel-batch-96-SPAWN.md`（scratch-only）→ 未跑 |
| LABEL_CAP | 1024（硬顶；consolidate 不得突破 `LABEL_CAP - 8`） |
| Relock 写法 | per-beat `scripts/_probe/_tmp_beNNN_relock.mjs` + `scripts/verify-yoyo-ty.mjs` |

PROMPT 文首 Week 轴仍停在 stub **34 handlers**（v0.1）；本队列是 SCOPE-CUT 边界外的 EXPERIMENTAL 扩写，**二者不要混称 GREEN**。

## 并发模型（默认 · 唯一）

```
[scratch pool ≤8] ──PASS 清单──▶ [consolidator ×1] ──Relock──▶ 下一拍
     │                                    │
     │ 只写 _scratch_*                    │ 独占写 yoyo.ty / golden /
     │ 与 JS↔Rust probe                   │   golden.js / self_test /
     │                                    │   main.rs / yoyo.ty.lock
```

1. **可并行**：同一 `parallel-batch-N` 内最多 **8** 个 scratch worker（独立 `_scratch_*.ty` + JS↔Rust byte-eq）。共享 runner：`scripts/_probe/parallel-batch-scratch-lib.mjs`（`runScratchPicks` / `mapPool`，默认 concurrency≤8；优先 `yoyo.exe` 避免 cargo lock）。模板：`parallel-batch-94-run.mjs`。**新批必须用此 lib**，勿再复制旧串行 for-loop。**不要**开 100 个 Relock writer。
2. **必须串行**：**恰好 1** 个 consolidator（`body-extend-N`）。单 Relock 链；禁止并行 append / 并行写 lock。
3. **流水线重叠（允许）**：consolidator 做 beat N 的 append+Relock 时，scratch pool 可按已发布的 `parallel-batch-(N+1)-SPAWN` pick 列表预跑；**不得**在 Relock 完成前改 canonical 树。未发 SPAWN 时禁止抢写下一拍 pick（避免重名 / 撞已锁变体）。
4. **批大小**：每拍 6–8 PASS（与现 SPAWN 一致）。REJECT 不进 Relock。

## AUTO-STOP（终结点 · 原默认主门 — 见 §矩阵覆盖取代）

PROMPT **没有**声明 body-extend 自动滚的目标机身尺寸 /「够自举」条数。W5.5 full body = **SCOPE-CUT**；Freeze = Part 5 另一扇门。  
故本控制面立一条 **NORMATIVE-light** 停机规则（auxdocs 级，不改 ISA）：

| 优先级 | 条件 | 动作 |
|--------|------|------|
| **主门** | Relock 后 `handlers ≥ 800` | **停止** auto-spawn 下一 `parallel-batch`；log 写 `AUTO-STOP @800` + HOLD |
| 次门 | 用户明确 STOP / HOLD | 立即停；保留 SPAWN 可人工 resume |
| 硬顶 | 本拍 append 后将 `handlers > 1016`（`LABEL_CAP-8`） | consolidator **拒绝** Relock |

停机 ≠ freeze ≠ 自举 GREEN ≠ W-START 翻绿。到 800 只是**停止无限 imm 阶梯空转**；若要继续扩写，用户须显式「resume body-extend past 800」。

## §矩阵覆盖取代 imm 随机策略（2026-07-26 建议，未改默认）

`docs/auxdocs/selfhost-emit-matrix.md` 给出了 (opcode, shape) 的 64 行覆盖矩阵，其中 **14 行 YES+MISSING/PARTIAL** 是自举 GREEN 真正缺口。`scripts/_probe/coverage_scan.mjs` 可复现。

**新建议（替代随机 8 imm 变体）**：
- 下一拍起，scratch picks 按矩阵 **优先级缺口排序** (P0→P1→P2→P3)，不按随机 imm 阶梯。
- 主门 800 保留为硬上限（防止失控），但**新语义主门** = 矩阵满（所有 YES 行 DONE）。
- imm 阶梯（230/232 等 LDB/ADD/SUB）只覆盖 imm32 编码变体，**自举不需要**（P3）。当前 213× ADD-IMM / 213× SUB-IMM / 215× LDB imm32 是变体膨胀，不是能力增量。
- **P0 缺口（当前最大）**：`84 MEMCPY_DATA`、`85 MEMCPY_STATE`（D-3 stub=C3 不通过 DDC）。

**优先级缺口排序（next batch target set）**：

| 优先级 | op | shape | 理由 |
|--------|----|-------|------|
| P0 | 84 MEMCPY_DATA | dst src n (3-arg) | 自举真实 body 必须；stub=C3 不通过 DDC |
| P0 | 85 MEMCPY_STATE | dst src n (3-arg) | 同上 |
| P0 | 60 GET | dst src 多 slot 变体 | 自举循环必须频繁 GET |
| P0 | 30 SET | slot imm 多 imm 变体 | 自举初始化大量 SET |
| P1 | 68 ADDV/6A SUBV/69 ORV/63 IMUL | dst src 多组合 | 自举算术密集 |
| P1 | 65 CMP | a b 多 slot | 配合 Jcc 自举条件循环 |
| P1 | 66 INC/67 DEC | 多 slot | 自举循环计数器 |
| P2 | LDB/ADD/SUB imm 边界 | imm8/imm32 (§4S.3.1) | 三 peer ground truth |
| P2 | 70 JMP/41 CALL | 多目标 hh | 自举分发 |
| P3 | imm 阶梯/64 MOVRR/A0/A1/NOP | - | 仅变体实验，自举不需要 |

**矩阵满 ≠ freeze**（freeze 仍是 Part 5 end gate: full body + 3-chain + Lock）。矩阵满 = "body-extend phase DONE" 的语义结束点，比 800 句柄数更好的终结门。

> 默认主门 800 保留（未改）；本建议需用户一句 "switch to matrix" 才改批策略。**已 switch (2026-07-26)**: batch-96 起 picks 取自矩阵 P0→P1 缺口排序，不再用 imm 阶梯。

## Resume

1. 读最新 `body-extend-NNN-log.md`（或未跑的 `*-SPAWN.md`）与 `yoyo.ty.lock` pin。  
2. 若停在 AUTO-STOP：用户一句 resume 后，把主门临时视为暂停（log 注明），从下一 `parallel-batch` SPAWN 继续；**不**自动改主门数字。  
3. 若停在 body-extend-100 SPAWN ready：先跑 consolidator（serialize + Relock），再按上表决定是否 spawn batch-95。  
4. 缺 SPAWN 时：从最新 log 的 Deferred / PASS 列表重建，禁止猜 pin。

## 禁区（不变）

- 不碰 D-1 `0x20/0x50/0x51`、D-2 `0x64`、D-3 MEMCPY `0x84/0x85` body opcodes。  
- 不 invent-green；DDC EQUAL/DIFFER 如实记。  
- 不自动 git commit / 不擅自改 PROMPT NORMATIVE。

## 指针

- 待 scratch：`docs/auxdocs/parallel-batch-96-SPAWN.md`  
- 上一拍 Relock：`docs/auxdocs/body-extend-101-log.md`  
- 上一拍 scratch：`docs/auxdocs/parallel-batch-95-log.md`  
- scratch 并发 lib：`scripts/_probe/parallel-batch-scratch-lib.mjs`（smoke：`node scripts/_probe/parallel-batch-scratch-lib-smoke.mjs`）

# W-selfhost-min closure memo (anti-rewrite pointer)

> 辅助性历史说明；不是规范、GREEN、P-phase 勾选或 N.3 gate row，也不改动任何 LOCKED 链。

## 1. Purpose

记录 W-SM 阶段为何/如何被机械收尾；唯一作用是 anti-rewrite 上下文，不当绿门、不当 P-phase 勾选、不动 LOCKED 链。

## 2. What was W-SM

- W-SM 是 `W-selfhost-min mechanical`，范围止于 H_05…H_16。
- 当时的落点是 17 handlers / 267 lines。
- 增量主要是机械式 `RAW_BYTE` NOP/RET chains，而不是高层编译逻辑的自举实现。
- Rust 侧记录为 18/18 goldens。
- JS 侧记录为 G00–G05，6/6。
- 2-chain `fullfile-bootstrap` 比较记录为 EQUAL。
- 该阶段的既有 pin 上下文是 `c697b4b7…`。
- 诚实口径：这不是真的自举编译，而是把当时的形式语义锁住；上述记录只描述机械阶段的收尾状态，不替它作正确性或完整性辩护。

## 3. Why closed at H_16

- 继续追加 `RAW_BYTE` handler，只会增加绿色计数与链长度。
- 这种追加不再提供形态正确性的有效增量。
- N.4 禁止把 DDC 表述为 “provably correct”；`RAW_BYTE` 堆叠尤其不可能填补证明义务。
- 因此 H_16 是停止机械扩张、保留诚实边界的收口点，不是胜利证明。

## 4. What this phase did NOT do

W-SM 没有完成：

- full compiler self-host；
- 3-chain `section-ddc`；
- asm peer；
- G06；
- Phase 2 的 ≤1500 lines full body 出口；
- Morph Prove；
- compiler freeze；
- Phase 4c libyoyo migration（OUT of v0.1）；
- gen1≡gen2（D-4）。

## 5. Anti-rewrite lessons

- 历史上 gen2 曾经 “worked”，随后仅因 “not good enough” 被拆掉；N.7 现在显式禁止以此为由摧毁已经工作的绿色状态。
- 反复进行 GH rollback 会把工程推入第 4 次 restart；不得用清空、回滚或重建来制造自杀式 green state。
- 对已锁住且仍绿色的链，唯一合规的“补”路径是 **Morph→Prove→Relock**，而不是先拆后说。
- 发现 contamination 时，Q3 quarantine protocol 要求隔离并诚实标红；quarantine 不是跳过 Relock 的豁免。
- `scripts/quarantine-gen.ps1` 的 fail-closed 行为是护栏：证据不足时停止，不替污染状态编造通过。
- Part B 的 LOCKED lifecycle 不允许从 clean slate 叙事中 invent-green；锁的连续性、来源与重锁步骤都必须可追溯。
- DDC 的作用是 detection under assumptions，不是 proof；EQUAL 不能升级为“输出已被证明正确”。
- 本 memo 只保存这些反重写经验；它不把 W-SM 追认成真正自举，也不改变后续 gate 的红绿。

## 6. Honest red list carry-over

从 W-SM 自身视角，下列项目继续保持红色或未完成，未来读者不应从机械 EQUAL 重新推导为绿色：

- full compiler self-host；
- 3-chain `section-ddc`；
- asm peer；
- G06；
- Phase 2 出口（≤1500 lines full body）；
- 冻结编译器；
- M-morph / Morph Prove；
- Phase 4c libyoyo migration（OUT of v0.1）；
- 历史 gen1≡gen2 parity（D-4）；
- Part 10 CI 勾选。

## 7. Where to read next

依次阅读 `PROMPT-v3.md`：

1. 文首「当前进度 · Week 轴」：看当前施工状态，不把本 memo 当新 gate。
2. Part N.7：看 Gen quarantine / anti-rewrite pointer。
3. Part B（Part 5B）：看 Cold-Start、Bootstrap 与 LOCKED lifecycle。
4. 文首「仍红（big list）」：看尚未满足的诚实边界。

这些指针不新增 GREEN，不替代 N.3、P-phase 或 LOCKED 流程。

## 8. Changelog

- 2026-07-23：新增 W-SM 机械阶段 closure memo；仅文档、anti-rewrite 上下文，无代码或 gate 变更。

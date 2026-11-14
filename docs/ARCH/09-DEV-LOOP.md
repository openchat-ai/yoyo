# 09 — 全流程开发循环（可执行剧本）

> 目标：从「想法」到「绿门禁 / 可毕业」的标准动作。Agent 按此剧本执行。

---

## 剧本 A — 给语言加一个能力（主尺）

**例**：新文件工具 / 新控制台行为 / 新导入（应用面）。

1. **SCOPE 检查**  
   - 当前版 IN 是否覆盖？否则先走 `10-VERSIONING` 开版或 SCOPE 追加（用户授权）。  
2. **设计**  
   - 写/改 `scripts/fixtures/anyprog/<name>.ty`  
   - 更新 `API-HONEST.md`（若新导入）  
   - 冷启 A1 六名：**不得**为应用便利去改冷启集合，除非独立 SCOPE 刀。  
3. **实现链接侧**（若需新 opcode/IAT）  
   - 改 `yoyo-rust`；`cargo build --release -p verifier`  
   - 相关单元 / ddc  
4. **接入门禁**  
   - `stage-anyprog.ps1` 增加用例  
   - `stage-lang1.ps1` 仍包 anyprog  
5. **文档**  
   - `LANG-SURFACE.md` 能力表 + `03-LANGUAGE.md` 如需  
6. **验收**  
   ```powershell
   & F:\yoyo\scripts\stage-lang1.ps1
   ```  
7. **勾选 / commit**（按 AUTO 一刀一落）

---

## 剧本 B — 修自举 / cold

1. 读 `05-SELFHOST.md` + 失败 `reason=`  
2. 确认 bins / pin / bootstrap 哈希（防旧二进制）  
3. 最小复现：单次 place → 单次 cold  
4. 修 → `stage-lesshostx` →（毕业前）`stage-break3`  
5. 若 seed 变：更新 `ow-seed-trust-root-fullbar.pin`（需诚实说明；可能要 `-WritePin` 流程）  
6. 文档对齐 BREAK3/C1COLD 叙事；**勿**把 stub 产品化

---

## 剧本 C — 只改文档 / 规划

1. 改 `docs/ARCH/*` / PLAN / 机构图  
2. 若改变 DRIVER 行为：同步 `.cursor/rules/stage4-auto-owner.mdc` 与 `AGENTS.md`  
3. commit；无需假跑全量自举（除非文档声称门禁结果）

---

## 剧本 D — 回归怀疑「整条链」

```powershell
& F:\yoyo\scripts\stage-lang1.ps1
& F:\yoyo\scripts\stage-lesshostx.ps1
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test ddc
```

全绿才宣称「未退化」。

---

## 剧本 E — Hard block

1. 看板写「阻塞」：现象 / 已证伪 / 需要人类什么  
2. `ACTIVE` 可保持 1 但 **停执行新功能**  
3. 不勾完成；不发假 RELEASE

---

## 完成定义（Definition of Done）

| 级别 | DoD |
|------|-----|
| 单项 | 门禁绿 + 勾选 +（加深）commit |
| 版本 | SCOPE 全门绿 + RELEASE + tag（+ push/gh 按毕业协议） |
| 文档驱动 | 新人/新 agent **只读 ARCH+SCOPE** 能复现门禁 |

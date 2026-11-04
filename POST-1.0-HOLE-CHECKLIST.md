# Post-v1.0 关洞看板（path 2 · OW-RT / S3）

## 一眼进度（打开即看 · 勿另问）

| | |
| --- | --- |
| **tip** | **S3.707** ⏳ **inventory evidence pack**（stage17 跑证 · SCOPE 仍 CUT）或远可选 |
| **主线** | S1✅ S2✅ **S3⏳** · S4/S5 未开 |
| **已站住** | live-compose · parse1≡exec · Gate I≡self-host · single-shot CP · dual-path · **nested CP stress** · inventory CUT 诚实 |
| **硬闸** | M4 flip **ABSENT**（inventory CUT · `closed=0 cut=7`）· `ready=false` |
| **禁** | flip / densify / raise slim / retarget fixed10 ADD / 假 CLOSED / 空转 wait |
| **近刀** | 691–706 ✅（…→dual-path→**nested CP stress B**） |
| **git** | S706 收工 → tip **S3.707** · **未 push** · bake=362 |

---

## 北星

v1.0 已毕业（`ACTIVE=0`）。本板 = **OW-RT 长杆日常**（S3）。**禁假 CLOSED** · **禁 invent Stage** · `closed=0 cut=7`。

**默认** 继续 **= 从下方「自动任务队列」取下一刀**（有行为/证据增量；**禁**只写 wait）。

| 阻塞 | 说明 |
| --- | --- |
| **长杆** | OW-RT → S3（去 Rust sidecar） |
| **勿做** | fake CLOSED · `AUTO ACTIVE=1` · invent Stage · 定点/oracle 当 CLOSED · CI 当调试器 |

Disposition / 洞表 → `SCOPE-CUT-v1.0-hole-inventory.md`。门禁明细 → `git log`。

---

## 自动任务队列（继续 = 取首条未勾）

> 编排原则：预检齐后**禁止**把 tip 写成「等新红」空转。硬闸 CUT 仍在，但「继续」必须落到有增量的刀。

| # | 刀 | 类型 | 增量 | 状态 |
| --- | --- | --- | --- | --- |
| 1 | **S3.705 dual-path host ADD/SUB** | 代码 | `emit_*_compact`；fixed10 seed 仍 imm32；slim `0x45ae` Δ=+28 | ✅ |
| 2 | **S3.706 nested CP probe 实现** | 代码/探针 | serial×2 GREEN；concurrent same-cwd both-exit | ✅ |
| 3 | **S3.707 inventory evidence pack** | 证据 | `stage17-ow-*` 跑证 + SCOPE 仍 CUT；≠flip | ⏳ tip |
| 4 | **host-mini / child-cwd drop-in** | 可选远 | 非毕业；有行为才动 | 可选 |
| 5 | **M4 flip** | 禁 | while CUT | 🚫 |

---

## S3 主线

| | |
| --- | --- |
| **S1** ✅ | in-DLL mini codegen |
| **S2** ✅ | slim 真 I/O · StubPlatform **仍 stub** |
| **S3** ⏳ | child parse1 扩 ISA · 真自举不排假工期 |
| **S4/S5** | 全 ISA / 形式化 · 地图 |

**诚实底线**：CreateProcess≡live-compose；parse1 ADD/SUB≡exec；host ADD/SUB **dual-path**；nested CP stress pinned；Gate I≡self-host；M4 flip **禁** while CUT。

### 波段 tip（更早单刀已折）

| 波段 | tip | 一句话 |
| --- | --- | --- |
| **S3.1–630** ✅ | `3809e18` | fixed10→parse1→any-slot→text4KiB |
| **S3.631–638** ✅ | `3227356` | yoyo.tyb child→CreateProcess GREEN |
| **S3.639–655** ✅ | `7f27b15` | compact→mid-grow→**full-grow / yoyo≈27KiB** |
| **S3.656–683** ✅ | — | HOT→H00→**parse1≡exec + stage live** |
| **S3.684–690** ✅ | — | cliff→contract→map→prep→gates→LDB→**slim len** |
| **S3.691** ✅ | — | fixed10 expect **retarget**（parse1≡win32；in-DLL≡emit_x86） |
| **S3.692** ✅ | — | host ADD/SUB **imm32 floor** named；design pin wording |
| **S3.693** ✅ | — | **leave-floor**；S634/5 stage1≡seed stage2→**NO_INPUT** |
| **S3.694** ✅ | — | drop-in self-host **NO_INPUT** cliff dissected |
| **S3.695** ✅ | — | **pick A** Gate I≡self-host；child-cwd drop-in optional |
| **S3.696** ✅ | — | M4 prep/gates **刷新**（硬闸=inventory CUT） |
| **S3.697** ✅ | `d710e3b` | CreateProcess blockers/call-site 刷新 |
| **S3.698** ✅ | — | re-entrancy **pick A**（single-shot） |
| **S3.699** ✅ | — | compose/self-host **preflight complete** |
| **S3.700** ✅ | — | inventory-facing **CUT map** pick A |
| **S3.701** ✅ | — | dual-path probe pick A（后由 705 实现） |
| **S3.702** ✅ | — | nested CP probe pick A（后由 706 实现） |
| **S3.703** ✅ | — | truth-repair + SCOPE-CUT preflight 注记 |
| **S3.704** ✅ | — | stage17/SCOPE **evidence surface** |
| **S3.705** ✅ | — | **dual-path B**：slim **0x45ae** Δ=+28 |
| **S3.706** ✅ | — | **nested CP stress B**：serial×2 + concurrent both-exit |
| **S3.707** ⏳ | — | **本步**：inventory evidence pack（stage17 · 仍 CUT） |

**缺口**：inventory **CUT**（G CLOSED ABSENT）；host-mini/child-cwd 远可选；slim residual Δ=+28。`ready=false`。bake=362。

---

## 验收（日常）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo test --lib -- yoyo_nested_createprocess yoyo_host_addsub_dual_path yoyo_h00_writefile_seed_pin --test-threads=1
```

path 2 回归才跑：`stage17-ow-*` / `stage15-hole-inventory` / `stage16-*`（见 `scripts/`）。

---

## 对 AI

- **每刀结束 / 用户说继续**：先扫「一眼进度」+ **自动任务队列**；取首条未勾真增量刀；收工同步 tip·近刀·缺口·git。
- 继续 → **禁**只回「等新红/人审」；遇红**自动修**再推。
- 禁 flip / densify / 假 CLOSED / retarget fixed10 ADD golden / 无增量空转。
- OW-* GREEN ≠ CLOSED；本地先绿；2 次红 CI 停推；AUTO `ACTIVE=0`。

---

*post-v1.0 · 收纳板 · tip S3.707 · 一眼进度置顶 · 任务队列防空转*

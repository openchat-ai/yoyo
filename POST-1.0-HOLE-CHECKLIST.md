# Post-v1.0 关洞看板（path 2 · OW-RT / S3）

## 北星

v1.0 已毕业（`ACTIVE=0`）。本板 = **OW-RT 长杆日常**（S3）。**禁假 CLOSED** · **禁 invent Stage** · `closed=0 cut=7`。

**默认** 继续 **= S3.636**（635=seed_host raw-run NO_INPUT cliff；下一刀=pe_link shape / runnable self-host / M4）。

| 阻塞 | 说明 |
| --- | --- |
| **长杆** | OW-RT → S3（去 Rust sidecar） |
| **勿做** | fake CLOSED · `AUTO ACTIVE=1` · invent Stage · 定点/oracle 当 CLOSED · CI 当调试器 |

Disposition / 洞表 → `SCOPE-CUT-v1.0-hole-inventory.md`。门禁明细 → `git log`。

---

## S3 主线

| | |
| --- | --- |
| **S1** ✅ | in-DLL mini codegen |
| **S2** ✅ | slim 真 I/O · StubPlatform **仍 stub** |
| **S3** ⏳ | child parse1 扩 ISA · 真自举不排假工期 |
| **S4/S5** | 全 ISA / 形式化 · 地图 |

**诚实底线**：yoyo.tyb-child GREEN ≠ drop-in self-host（slim AV；seed_host raw→NO_INPUT）≠ OW-* CLOSED。

### 波段 tip（更早单刀已折）

| 波段 | tip | 一句话 |
| --- | --- | --- |
| **S3.1–521** ✅ | `5dd7651` | fixed10→host compose→DATA |
| **S3.522–630** ✅ | `3809e18` | CreateProcess→parse1→any-slot→text4KiB |
| **S3.631–632** ✅ | `827b853` | yoyo.tyb child + in-DLL CreateProcess GREEN |
| **S3.633–635** ✅ | — | seed≠wide；slim AV；**seed raw→NO_INPUT** |
| **S3.636** ⏳ | — | **本步**：pe_link shape / runnable self-host / M4 |

**缺口**：drop-in self-host ABSENT（slim AV / seed_host cwd-contract NO_INPUT）；host-mini 1024 禁抬；M4 / general emit **ABSENT**。`ready=false`。bake=362。

---

## 验收（日常）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo test --lib -- yoyo_compiler_pe_parse1_ --test-threads=1
```

path 2 回归才跑：`stage17-ow-*` / `stage15-hole-inventory` / `stage16-*`（见 `scripts/`）。

---

## 对 AI

- 继续 → **S3.636**（pe_link shape toward runnable；禁 densify / 假 CLOSED / 单抬 slim-cap）。S3.635=seed_host raw NO_INPUT。红了记根因停。
- OW-* GREEN ≠ CLOSED；本地先绿；2 次红 CI 停推；AUTO `ACTIVE=0`。

---

*post-v1.0 · 收纳板 · tip S3.636*

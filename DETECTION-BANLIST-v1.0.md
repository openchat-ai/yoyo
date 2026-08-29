# DETECTION BANLIST — v1.0（Stage 16-B · 对外话术）

> **Status:** ACTIVE（对外 detection-only 禁词 / 假宣称表）  
> **Gate:** `scripts/stage16-detection-wording.ps1`（alias `stage16-b.ps1`）  
> **Consumers:** `RELEASE-v1.0.md`（graduated）· `RELEASE-NOTES-v1.0.md` · 任何 v1.0 宣传文案  
> **Rule:** DDC = **detection bar**，不是证明。禁止把 SCOPE-CUT / DIFF 说成已关洞或 Thompson 免疫。

**Upstream:** `SCOPE-CUT-v1.0-hole-inventory.md`（Stage 16-A FINAL · closed=0 cut=7）。

---

## 机器行（gate 解析）

每行 `BAN id=… pattern=…` 为一条禁宣称。`RELEASE-v1.0.md` 与本文件中，命中 `pattern` 的行 **必须** 落在「禁止 / Forbidden / Misleading / MUST NOT / ❌」语境（含 not / 不得 / do not），否则 gate RED。

```
BAN id=THOMPSON pattern=(?i)thompson[- ]proof|thompson[- ]immune|immune to compiler backdoors
BAN id=FULLY_CLOSED pattern=(?i)fully closed|all holes? (are )?closed|holes? (are )?fully closed|洞已全关|七项已关|closed=7
BAN id=FAKE_EQUAL pattern=(?i)full\s+[`']?\.text[`']?[^\n]{0,48}three[- ]peer[^\n]{0,24}EQUAL|(?i)three[- ]peer[^\n]{0,24}full\s+[`']?\.text[`']?[^\n]{0,24}EQUAL|(?i)graduation[^\n]{0,40}full\s+[`']?\.text[`']?[^\n]{0,24}EQUAL
BAN id=FAKE_PROOF pattern=(?i)provably (correct|secure|pure)|mathematical(ly)? (proof|immune)|DDC\s*(⇒|=>|=)\s*prov
BAN id=YOYO_RUNTIME_DONE pattern=(?i)YOYO-built runtime (is |has |now )?(done|shipped|complete|ready)|no longer (uses |embeds )?Rust runtime
BAN id=IAT_GONE pattern=(?i)LoadLibrary(A)? (removed|gone|eliminated)|no (longer )?(uses )?LoadLibrary
BAN id=SEED_HOST_GONE pattern=(?i)seed (is )?(no longer|not) (Rust[- ])?host[- ]emitted|non-Rust seed emitter (shipped|done)
```

---

## 人类可读禁词表（对外）

| ID | 禁止宣称（肯定式） | 允许（诚实） |
|----|-------------------|--------------|
| **THOMPSON** | Thompson-proof / immune to compiler backdoors | DDC detects peer divergence — **not** Thompson proof |
| **FULLY_CLOSED** | fully closed / 洞已全关 / closed=7 | `HOLE_INVENTORY_V10 status=FINAL` · **closed=0 cut=7** |
| **FAKE_EQUAL** | full `.text` three-peer EQUAL 作为毕业话术 | selfhost-body window EQUAL only；full `.text` **DIFF** |
| **FAKE_PROOF** | DDC ⇒ provably correct / mathematical immunity | detection bar only |
| **YOYO_RUNTIME_DONE** | YOYO-built runtime 已落地 / 无 Rust runtime | **OW-RT CUT** — still Rust `yoyo_runtime.dll` embed |
| **IAT_GONE** | LoadLibrary removed | **OW-IAT CUT** — LoadLibraryA / libdl still present (GetProcAddress dropped; PE export walk) |
| **SEED_HOST_GONE** | seed 不再由 Rust host 发射 | **OW-SEED CUT** — still Rust `yoyo.exe` emit |

---

## 剩余 CUT 清单（必须写入 RELEASE 草案）

| ID | Disposition | 一句话 |
|----|-------------|--------|
| **OW-H00** | **CUT** | H_00 entry slot；body 跳过；full `.text` DIFF |
| **OW-STUB** | **CUT** | Rust extract stub_tail_nonzero |
| **OW-RT** | **CUT** | Embedded Rust runtime.dll |
| **OW-IAT** | **CUT** | LoadLibraryA / libdl host trampoline (GetProcAddress dropped; PE export walk) |
| **OW-SEED** | **CUT** | Seed still Rust-emitted |
| **REL-FULLTEXT** | **CUT** | full `.text` not graduation EQUAL |
| **REL-STUBOS** | **CUT** | Stub OS not production I/O |

**Inventory cite required:** `SCOPE-CUT-v1.0-hole-inventory.md` · `closed=0` · `cut=7` · `status=FINAL`.

---

## 机器验收（exit 0）

```powershell
cd F:\yoyo
.\scripts\stage16-detection-wording.ps1 -SkipBuild
```

Gate must:

1. **不退化 Stage 16-A** — nest `stage16-scope-cut-finalize.ps1 -SkipBuild` exit 0  
2. **本文件存在** — 含七条 `BAN id=` + Status ACTIVE  
3. **RELEASE-v1.0.md 草案** — Status DRAFT；引用本 banlist + SCOPE-CUT-v1.0；七 CUT ID；detection≠proof  
4. **禁词扫描** — RELEASE（及本文件 claim 语境）无裸奔肯定式 banned claim  
5. **打印** `DETECTION_WORDING status=DRAFT …` 汇总行  

Full graduation / tag = Stage 16-D（本门只钉话术边界草案）。

---

*Stage 16-B · 打破后门魔咒：少误宣称 Thompson / 假关洞；CUT 清单入 RELEASE 草案*

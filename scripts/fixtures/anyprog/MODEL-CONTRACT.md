# MODEL-CONTRACT v3 — multi-scale / LLM unambiguous write surface

Fail-closed. Human ergonomics deferred. Spacecraft-facing: **no silent ambiguity**.  
Parent rails: `AUTO-LLMWRITE` → `AUTO-LLMWRITEX` → `AUTO-LLMWRITESCALE`.

## Normative source

`model-contract.json` (pinned by `model-contract.sha256`).  
This markdown is explanatory; **json wins** on conflict.

## Lexical (must) — 行尺度

| Rule | Value |
| --- | --- |
| Encoding | UTF-8 **without BOM** |
| Newline | LF only (`\n`); no `\r` |
| Tabs | forbidden |
| Comments | **forbidden** (`;` anywhere → RED) |
| Blank lines | **forbidden** |
| Entry line | exact `40 00` (no indent) |
| Body lines | exact two spaces + tokens |
| Tokens | uppercase hex, width 2 (`05` not `5`) |

## Semantic (must)

| Rule | Value |
| --- | --- |
| Handlers | exactly one: `40 00` |
| `FF` RET | forbidden |
| Slots | only `50` (buffer) / `51` (size) |
| `20` ALLOC | `20 50 <sz>` |
| `30` SET | `30 51 <imm>` only |
| `50` LOAD | `50 50 00` only (input.tyb) |
| `51` WRITE | `51 50 02 51` only (output.exe, size slot) |
| `A0` | **only** trailing exit trampoline (exact 13 bytes) |
| Winsock | forbidden |

## Grammar (must) — 句·段尺度

Authoritative: `grammar.*` in json (not named shapes).

| Rule | Value |
| --- | --- |
| Program | `entry stmt+ exit` |
| Stmts | `SET_SIZE` / `ALLOC_BUF` / `LOAD_IN` / `WRITE_OUT` only |
| Dataflow | `WRITE_OUT` requires prior size + buf |
| Require write | body must include ≥1 `WRITE_OUT` |

Named `shapes.*` are **aliases only** (convenience labels for samples).

## Multi-scale AST — 任意尺度

Each sample `*.ast.json` must byte-equal:

- `lines` — every source line  
- `stmts` — `KIND:opcode…`  
- `program` — `{entry, body[], exit[]}`  

Generative: `generative.trials` random legal programs, canonicalize idempotent.

## 全域 coverage

`surface_coverage` in json: every ANYPROG tool is **in_contract** or explicit **out_***.  
Missing key → RED (no silent gap). File-tier in: cat/head/zero/copy08/resize.

## Gates

```text
node yoyo-js/scripts/validate-llmwrite-contract.js
scripts/stage-llmwrite.ps1
```

Pin: `model-contract.sha256`

≠ Thompson · ≠ 全 ISA · ≠ Winsock closed · ≠ “model already programs YOYO”

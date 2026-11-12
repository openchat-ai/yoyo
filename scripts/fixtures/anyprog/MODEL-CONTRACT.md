# MODEL-CONTRACT v2 — aerospace / LLM unambiguous write surface

Fail-closed. Human ergonomics deferred. Spacecraft-facing: **no silent ambiguity**.

## Normative source

`model-contract.json` (pinned by `model-contract.sha256`).  
This markdown is explanatory; **json wins** on conflict.

## Lexical (must)

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

## Shapes (file tier)

Named bodies in json `shapes.*` then exit trampoline. Samples `cat`/`head`/`zero` must match + `*.ast.json`.

## 全域 coverage

`surface_coverage` in json: every ANYPROG tool is **in_contract** or explicit **out_***.  
Missing key → RED (no silent gap).

## Gates

```text
node yoyo-js/scripts/validate-llmwrite-contract.js
scripts/stage-llmwrite.ps1
```

Pin: `model-contract.sha256`

≠ Thompson · ≠ “model already programs YOYO”

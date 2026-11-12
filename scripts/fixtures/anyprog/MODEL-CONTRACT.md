# MODEL-CONTRACT v5 — pipe / multi-scale + drift + RAW capabilities

Fail-closed. Human ergonomics deferred.  
**钢管**：唯一写法 → 唯一映像 → 唯一摘要 → 漂移必红。  
Rails: LLMWRITE → WRITEX → SCALE → DRIFT → PIPE → **TY**.

## Normative source

`model-contract.json` (pinned by `model-contract.sha256`). Json wins.

## Gate authority = `.ty` + `yoyo hash`

| Layer | Authority |
| --- | --- |
| Lex | `gate/gate_lex.ty` |
| file_grammar | `gate/gate_grammar.ty` (stmts + dataflow + exit) |
| Contract pin | `yoyo hash model-contract.json` |
| AST / generative | Node residual (honest) |

exact_body samples: lex only in ty gates (RAW bodies).

## Program kinds

| Kind | Meaning |
| --- | --- |
| `file_grammar` | compositional stmts + exit tail only |
| `exact_body` | pinned unique body (RAW allowed mid-body); still lex-canonical |

`program_kinds` maps every `in_contract` sample.

## Semantic / grammar / AST / generative

Inherited from v3/v4 for `file_grammar`. Exact bodies share trailing exit trampoline + multi-scale AST.

## Drift + dual + section

| Rule | Value |
| --- | --- |
| Rebuild | `drift.rebuild_times` ≥2 byte-equal |
| Artifact pin | `artifact_pins.*` sha256 |
| Section pin | `section_pins.*.text_sha256` (.text) |
| Dual emit | `canon_relink`: source link ≡ canonicalize→link |
| Bitflip | source/invalid/artifact detect (DRIFT) |
| 2nd PE linker | `HARD_BLOCK_single_pe_emitter` (honest) |

## 全域 coverage

Nine tools `in_contract`. `socket` = `out_winsock_open` only.

## Gates

```text
node yoyo-js/scripts/validate-llmwrite-contract.js
scripts/validate-llmwrite-drift.ps1
scripts/stage-llmwrite.ps1
```

≠ Thompson · ≠ 抗 SEU 硅 · ≠ 第二独立 PE 链已存在 · ≠ Winsock closed

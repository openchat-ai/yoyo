# LLM write gates (YOYO source)

**Sole source:** `gate_lex.ty`, `gate_grammar.ty`, `gate_exact.ty`.  
Edit these files directly. There is **no** JS/Python/Rust contract auditor in the stage path.

| Gate | Role |
| --- | --- |
| `gate_lex.ty` | lex fail-closed on `input.tyb` |
| `gate_grammar.ty` | `file_grammar` stmts + dataflow + exit |
| `gate_exact.ty` | `exact_body` (A0 mid-OK) + trailing exit |

**Also stage-checked (yoyo binary, not a new auditor):**
- `yoyo hash` on `model-contract.json` + `llm/ast-pins.sha256`
- static generative corpus `llm/gen/g*.ty` via `gate_grammar`

`NO_NEW_HOST_AUDITOR` — do not reintroduce Node/Python/`*_contract.rs` as stage authority.

**Status:** bootstrap-phase auditor · **frozen regression** (`AUTO-LLMWRITEFREEZE.md`).  
Not on wake / self-host critical path. Manual: `& scripts\stage-llmwrite.ps1`.

Build/run: `yoyo link --target=win32 <gate>.ty <out.exe>`（stage does this）.

≠ Thompson · link 仍是既有 Rust 债（只许缩）

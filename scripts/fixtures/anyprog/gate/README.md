# LLM write gates (YOYO source)

**Sole source:** `gate_lex.ty`, `gate_grammar.ty`.  
Edit these files directly. There is **no** JS/Python emitter in-tree.

| Gate | Role |
| --- | --- |
| `gate_lex.ty` | lex fail-closed on `input.tyb` |
| `gate_grammar.ty` | `file_grammar` stmts + dataflow + exit |

Build/run: `yoyo link --target=win32 <gate>.ty <out.exe>`（stage does this）.

≠ Thompson · ≠ “ty produced the gate” (that would be knife 2)

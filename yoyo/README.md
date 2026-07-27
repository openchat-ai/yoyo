# yoyo — Canonical Language (Project 1)

Domain layer: locked source, ISA table, format specs, libyoyo API surface, golden tests.

## Contents

- `projects/yoyo.ty` — 🔒 locked compiler source (Decision #13)
- `projects/ternary_signal.ty` — trit decision example (Part 4.6)
- `isa/` — 38-line ISA table
- `api/libyoyo/` — API names / signatures
- `format/` — `.ty` / `.tyo` format notes
- `tests/golden/` — immutable golden cases (post-freeze)

## Lock Protocol

Any change to `projects/yoyo.ty` requires the 5-step procedure in `PROMPT-v3.md` Part 9.4.
No automated regenerator exists in the LOCKED state.

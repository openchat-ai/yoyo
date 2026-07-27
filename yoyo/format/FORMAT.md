# .ty / .tyo format notes

## .ty (default, required) — Layer S (PROMPT-v3 Part 4.0 / Part G)

Whitespace-separated hex tokens (and optional named slots). Comments: `;` or `#` to EOL.

**Encoding**: first token is the opcode byte (`u8`). There is **no** required `00 00` prefix.

```
40 20          ; HANDLER H_20
  30 50 00     ; SET state[0x50]=0
  FF           ; RET
```

Named slots bind in `0x50`–`0xCF` (Part 4.2 / 8.4).

## .tyo (optional, Phase 4a)

Magic `TYO\x01`, 32-byte header — see PROMPT-v3.md Part 10.5 Phase 4a.
Default pipeline is Option A: `.ty` → platform binary (no `.tyo`).

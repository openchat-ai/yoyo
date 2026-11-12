# MODEL-CONTRACT — LLM write surface (unambiguous)

Audience: code-generating models. Humans deferred. Win32 app PE only (`yoyo link --target=win32`).

## Hard rules

1. One entry handler: first line `40 00` then body; no other `40 hh` unless contract revision says so.
2. End with **ExitProcess trampoline only** (never bare `FF` RET after PE `jmp` entry).
3. Use only **allowed opcodes** below with exact arity.
4. Prefer opcodes over `A0` RAW. RAW allowed **only** as the exit trampoline bytes listed in `model-contract.json` → `exit_trampoline_a0`.
5. Fixed slots: `0x50` = buffer/pointer, `0x51` = size (bytes).
6. Fixed paths (pe_link string table): str0=`input.tyb`, str1=`input.ky`, str2=`output.exe`.
7. Do not invent imports, DLL names, or IAT slots outside the app IAT table.
8. Winsock / network: **FORBIDDEN** this contract revision (OPEN host AV).

## Allowed opcodes (app file tier)

| Op | Name | Args | Notes |
| --- | --- | --- | --- |
| `40` | HANDLER | hh | only `40 00` |
| `20` | ALLOC | slot size | slot=`50` typical |
| `30` | SET | slot imm | size in `51` |
| `50` | LOAD_FILE | slot str | str=`00` → input.tyb |
| `51` | WRITE_FILE | slot str sz | str=`02` → output.exe；sz=`51` |
| `A0` | RAW_BYTE | byte | **exit trampoline only** |

## Exit trampoline (mandatory)

Exact byte sequence (see json). Meaning: `xor ecx,ecx; sub rsp,0x20; call [r15+0x28]` (ExitProcess).

## Minimal program shapes

### cat (copy 5-byte input.tyb → output.exe)

```
40 00
  30 51 5
  50 50 00
  51 50 02 51
  <exit trampoline A0 lines>
```

### head (first 3 bytes)

Same as cat with `30 51 3`.

### zero (ALLOC 8, write 4 zero bytes)

```
40 00
  20 50 8
  30 51 4
  51 50 02 51
  <exit trampoline>
```

## Gate

- Structural: `node yoyo-js/scripts/validate-llmwrite-contract.js`
- Behavioral: `scripts/stage-llmwrite.ps1` (includes file-tier run)

≠ Thompson · ≠ human ergonomics

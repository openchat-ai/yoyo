# ANYPROG API honesty (tier-1)

## In use now (A1 = 6, unchanged)

| Import | Via |
| --- | --- |
| VirtualAlloc | opcode `0x20` ALLOC |
| CreateFileA | `0x50` LOAD_FILE / `0x51` WRITE_FILE |
| ReadFile | `0x50` |
| WriteFile | `0x51` |
| CloseHandle | `0x50` / `0x51` |
| ExitProcess | RAW_BYTE trampoline (`stdlib_exit.inc.ty`) |

String table (pe_link): `input.tyb` / `input.ky` / `output.exe` at `r15+0x10000`.

## Console stand-in

Tier-1 “console” = **file `output.exe` contents** checked by gate.  
True stdout needs **GetStdHandle** (would grow A1 → must update `yoyo_a1_imports.pin` + honesty).

## Tier-2+ (not this knife)

| Need | Why |
| --- | --- |
| GetStdHandle | real console |
| GetCommandLineA / argv | rich CLI |
| sockets / threads / time | broader “any program” |
| higher source / macros | human writeability |

≠ Thompson · ≠ silent A1 expand

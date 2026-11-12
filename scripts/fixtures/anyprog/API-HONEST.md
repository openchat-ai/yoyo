# ANYPROG API honesty (endpoint)

## Cold-start A1 (unchanged · A1PIN)

`VirtualAlloc,CreateFileA,ReadFile,WriteFile,CloseHandle,ExitProcess` — **6**.  
Applies to place_stub / compiler place. ≠ silent grow.

## App PE IAT (plain `yoyo link --target=win32` only)

| Slot | Import | Used by |
| --- | --- | --- |
| 0 | VirtualAlloc | ALLOC / RX trampoline |
| 1 | CreateFileA | LOAD/WRITE_FILE |
| 2 | ReadFile | LOAD_FILE |
| 3 | WriteFile | WRITE_FILE / console |
| 4 | CloseHandle | I/O |
| 5 | ExitProcess | `stdlib_exit` `[r15+0x28]` |
| 6 | GetStdHandle | hello_console / cmdline / thread |
| 7 | GetCommandLineA | cmdline |
| 8 | GetTickCount64 | tick |
| 9 | CreateThread | thread |
| 10 | WaitForSingleObject | thread |

## Tier tools (gate GREEN)

| Tool | Proves |
| --- | --- |
| cat / head / zero | file I/O |
| hello_console | real stdout |
| cmdline | argv face |
| tick | time |
| thread | concurrency |

## OPEN residual

| Item | Why |
| --- | --- |
| **Winsock** | `WSAStartup` under handmade PE AVs in `ws2_32!OpenWinSockRegistryRoot` (stack/host). Fixture `socket.ty` retained; not in gate. |
| Higher source / macros | human deferred; see `AUTO-LLMWRITE` + `MODEL-CONTRACT` (LLM unambiguous file tier) |
| Linux app peer | Win32-first this knife |

≠ Thompson · ≠ C replacement · cold-start A1 not expanded

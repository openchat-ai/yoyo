# YOYO — Multi-Architecture Backend Instructions V2

## Codebase state (18 backends already exist)
`PlatformKind` enum currently has: Win32, Linux, BareMetal, Stub, Cuda, Android, Apple, Eight051, X86, Freedos, Riscv64, Mips, PowerPc64Le, Avr, Arm32, Wasm, MachoX64, Serenity.

`BinaryFormat` enum: Pe64, Elf64, FlatBinary, Multiboot, MachO64, Pe32, DosCom, Elf32BE, Elf32LE, MachO64X64, Serenity.

## Rules
1. **NEVER modify** emit.rs, assembler.rs, types.rs, tir.rs, elf_link.rs, pe_link.rs, arm64_elf_link.rs, apple_backend.rs, cuda_backend.rs, x86_link.rs, riscv_link.rs, mips_elf_link.rs, ppc_elf_link.rs, arm32_elf_link.rs, wasm_backend.rs, macho_x64_link.rs
2. Only edit **platform.rs**, **main.rs**, and **create new linker files**.
3. After all changes: `cd f:\yoyo\yoyo-rust\verifier; cargo build` then `cargo test`. ALL 161 existing tests must pass.
4. Smoke-test: `cargo run -- link --target=<target> "f:\yoyo\yoyo\tests\golden\00_nop_ret.ty" "f:\yoyo\yoyo-rust\build\test_<target>.out"`

## Each backend needs:
1. `PlatformKind::Xxx` variant
2. `BinaryFormat` variant if new format
3. `parse_platform` entry (lowercase aliases)
4. `select_platform` arm
5. `XxxPlatform` struct + `PlatformBackend` impl
6. Linker dispatch in `main.rs` cmd_link
7. Test in xxx_link.rs (2-3 unit tests)

## Stub pattern
emit_alloc/load_file/write_file: architecture NOP + marker bytes (same as all other backends)
emit_exit: architecture-native halt/exit

## Architecture reference

### Xtensa (ESP32 real ISA)
- 32-bit RISC, big-endian or little-endian mode (use LE for ESP32)
- NOP: `0x0000F0` (or `0x004000` - 3 bytes). Actually Xtensa NOP = `0x0000F0D7` (wait, no). Xtensa NOP = `0x0000F0` (or `0x00F000`, depends on encoding). Let's use: `0x0000F0D7` (NOP encoding). Actually simplest: `NOP.N` = `0x000000` (3 bytes). Or just use `0x000000F0` (4-byte aligned). 
  - **Simplest**: Use `0x000000F0` (le: `F0 00 00 00`) as NOP placeholder.
- Exit: `BREAK` = `0x004000` (3 bytes). Or just `ret` = `0x00F0D7` (3 bytes).
- Template: FlatBinary, stack 0x10000, data 0x1000
- Flat binary, no linker

### Z80 (8-bit CPU)
- 8-bit, registers: A, B, C, D, E, H, L, IX, IY, SP, PC
- NOP: `0x00` (1 byte)
- Exit: `HALT` = `0x76`. Or `RST 0` = `0xC7`. Use `0x76` (HALT).
- Template: FlatBinary, entry 0x0000, stack 0x4000, data 0x2000
- Flat binary, no linker

### 6502 (8-bit CPU)
- 8-bit, registers: A, X, Y, SP, PC, P
- NOP: `0xEA` (1 byte)
- Exit: `BRK` = `0x00`. Or infinite loop `JMP *` = `0x4C 0x00 0x80` (self-jump). Use `0x00` (BRK).
- Template: FlatBinary, entry 0x8000, stack 0x0100, data 0x0200
- Flat binary, no linker

### M68k (Motorola 68000 — 32-bit CISC)
- 32-bit CISC, registers: D0-D7, A0-A7, PC, SR
- NOP: `0x4E71` (2 bytes, le: `71 4E`)
- Exit: `STOP` = `0x4E72 0x2000` (4 bytes). Or `TRAP #0` = `0x4E40` (2 bytes). Use `0x4E40` (TRAP #0, le: `40 4E`).
- Template: FlatBinary, entry 0x1000, stack 0x10000, data 0x8000
- Flat binary, no linker

### LoongArch (龙芯 LA64)
- 64-bit RISC, registers: r0-r31 (r0=zero, r1=ra, r2=tp, r3=sp, r4-r11=a0-a7, r12-r20=t0-t8, r21=reserved, r22=fp, r23-r31=s0-s8)
- NOP: `andn r0, r0, r0` = `0x00000000` (4 bytes, le: `00 00 00 00`)
- Exit: `syscall 0` = `0x38000000` (le: `00 00 00 38`)
- ELF64, e_machine = 0x102 (EM_LOONGARCH), data2LSB
- Entry: 0x120000000, text 0x120000000, data 0x120010000
- Create `loongarch_elf_link.rs` (based on arm64_elf_link.rs, e_machine=0x102, startup: `lu12i.w r15, 0x12001; addi r15, r15, 0;` — actually LA64: `lu12i.w x15, 0x12001` then `addi.w x15, x15, 0` then `jirl x0, x15, 0` for jump to code)
  - `lu12i.w rd, imm20` = `0x14000000 | (imm20 << 5) | rd` (6 bytes! WAIT - LoongArch is 4-byte fixed encoding)
  - Actually LoongArch instructions are all 32-bit (4 bytes). `lu12i.w rd, si20` encoding: `0000 1010 si20[19:0] rd[4:0]` = `0x0A000000 | (si20 << 5) | rd`
  - `jirl rd, rj, offs16` = `0x4A000000 | (offs16 << 10) | (rj << 5) | rd`
  - Startup: `lu12i.w x15, 0x12001; addi.w x15, x15, 0; jirl x0, x15, 0` — but this is oversimplified. Actually just write the code directly at entry, data at data_va. No startup preamble needed (like MIPS/ARM32).

### SPARC v8 (32-bit)
- 32-bit RISC, registers: %g0-%g7, %o0-%o7, %l0-%l7, %i0-%i7, Y, PSR, WIM, TBR
- NOP: `sethi %g0, 0` = `0x01000000` (wait, SPARC NOP = `0x01000000` actually: `sethi 0, %g0`). Correct SPARC NOP: `0x01000000` (BE bytes: 01 00 00 00)
- Exit: `ta 0x00` (trap always) = `0x91D02000` (BE: 91 D0 20 00)
- ELF32, e_machine = 0x02 (EM_SPARC), **data2MSB** (big-endian, e_ident[5]=2), ELFCLASS32 (e_ident[4]=1)
- ALL multi-byte writes must use `to_be_bytes()`!
- Entry: 0x10000, text 0x10000, data 0x20000
- Create `sparc_elf_link.rs` (ELF32 big-endian, e_machine=0x02, based on mips_elf_link.rs)
- Startup: `sethi %hi(data_va), %o0; or %o0, %lo(data_va), %o0; sethi %hi(code_va), %o1; or %o1, %lo(code_va), %o1; jmpl %o1, %g0; nop` — or simpler: no startup preamble.
- SPARC `sethi rd, imm22` = `0x01000000 | (rd << 25) | (imm22 >> 10)` — actually: `op[1:0]=00 | rd[4:0] | op3[5:0]=100 | imm22[21:0]` = `0x01000000 | (rd << 25) | (imm22 << 0)`
  - Wait, SPARC sethi: bits 31:30 = 00, bits 29:25 = rd, bits 24:22 = 100, bits 21:0 = imm22. So: `0x00000000 | (rd << 25) | 0x04000000 | imm22` = `0x04000000 | (rd << 25) | imm22`
  - Actually bit 24:22 = 100 = 4 in bits 24:22. So base = 0x04000000. `sethi %o0, 0x10000` = `0x04000000 | (8 << 25) | 0x10000` = `0x04000000 | 0x1000000 | 0x10000` = `0x05010000` (BE: 05 01 00 00).

### RV32 (RISC-V 32-bit)
- 32-bit RISC-V, registers x0-x31, ELF32
- NOP: `0x00000013` (addi x0,x0,0) — same as RV64
- Exit: `ecall` = `0x00000073` (le: 73 00 00 00)
- ELF32, e_machine = 0xF3 (EM_RISCV), data2LSB, ELFCLASS32
- e_ident[4] = 1 (32-bit)
- Entry: 0x8001000, text 0x8001000, data 0x8002000
- Create `riscv32_elf_link.rs` (based on riscv_link.rs but 32-bit ELF). Startup: same as RV64 (auipc+addi+jal)
  - Or simpler: no startup preamble, write code at entry.

### ARM64 Windows (AArch64 PE32+)
- ARM64 AArch64, PE32+ format (same as x64 PE32+ but ARM64 machine)
- e_machine = 0xAA64 (ARM64) — actually PE: machine = 0xAA64 (ARM64)
- Identical structure to pe_link.rs but with ARM64 e_machine
- Exit: Windows ARM64 syscall works differently. Use stub: `ret` = `0xC0 0x03 0x5F 0xD6` (le bytes)
- NOP: `0x1F 0x20 0x03 0xD5` (ARM64 NOP, same as Android)
- Template: Pe64, entry 0x1000, data 0x2000, stack 0x100000
- Create `arm64_pe_link.rs` (based on pe_link.rs, but machine=0xAA64, and uses ARM64 NOP+stub)
  - Startup: ARM64 `adr x15, data_va; br x16` — no, simpler: `adr x15, data; b user_code`
  - Actually ARM64 startup stub: `adrp x15, data_va; add x15, x15, lo12(data_va); b user_code` (adrp 4B + add 4B + b 4B = 12B, pad to 16B)
  - `b imm26` = `0x14000000 | (imm & 0x03FFFFFF)`

### FreeBSD (x64 ELF, FreeBSD syscalls)
- x64 ELF64, same as Linux but syscall numbers differ
- e_machine = 0x3E (EM_X86_64), same as Linux
- Entry: 0x400000, text 0x401000, data 0x402000
- Exit: FreeBSD syscall `exit` = 1. Use `mov eax, 1; syscall` = `0xB8 01 00 00 00 0F 05`
- NOP: `0x90` (x64)
- Starter: same as Linux x64. Create `freebsd_elf_link.rs` (copy of elf_link.rs, same machine, but syscall comment)
- **Reuse elf_link.rs** for linking (same x64 ELF format). Just add backend with different exit syscall value.

### Haiku (x64 ELF, Haiku OS)
- x64 ELF64, same as Linux
- Haiku x86_64 syscall: uses `int 0x64` (opcode `0xCD 0x64`). Exit = syscall 0x5C01 (or similar). For stub: `mov eax, 0x5C01; int 0x64` = `0xB8 01 5C 00 00 CD 64`
- NOP: `0x90`
- **Reuse elf_link.rs** for linking (same x64 ELF format). Just add backend with Haiku syscall stub.

### Plan 9 (x64, Plan 9 OS)
- x64, Plan 9 uses its own binary format (not ELF). Flat binary style.
- Plan 9 x64 ABI: syscall via `SYSCALL` instruction (0F 05). Syscall number in AX.
- Exit: `mov eax, 8; syscall` (Plan 9 exits: `exits(nil)`) — syscall 8 is ... actually Plan 9 syscall for exit is 8. Or simpler: `mov eax, 8; int 0x80` (Plan 9 uses int 0x80 for syscalls on x86). For x64, Plan 9 uses `SYSCALL`.
  - Plan 9 x64 syscall: rax=sysnum, arg1=rbx, arg2=rcx, arg3=rdx, arg4=rsi, arg5=rdi, arg6=r8
  - Exit syscall: `exits(nil)` = syscall 0x8. So: `xor ebx,ebx; mov eax, 8; syscall` = `0x31 0xDB 0xB8 0x08 0x00 0x00 0x00 0x0F 0x05`
- NOP: `0x90`
- BinaryFormat: `Plan9Flat` (new) — flat binary with Plan 9 magic header
  - Plan 9 header: magic `0x00000000` (for x86-64)... actually Plan 9 executables have a magic number. For 64-bit: `0x00000000` (no header for x86-64) or `0x00000064`...
  - **Simplest**: FlatBinary with no header. Just output code bytes.
  - Template: FlatBinary, entry 0x0000, stack 0x10000, data 0x1000

## Sub-agent assignments

### Sub-agent D: Xtensa, Z80, 6502, M68k (all flat binary, embedded/retro)
- 4 backends, all flat binary, no linker needed
- Add to platform.rs + main.rs flat binary write path

### Sub-agent E: LoongArch, SPARCv8, RV32, ARM64 Windows (ELFs + PE)
- 4 backends, 3 need linker files (loongarch_elf_link.rs, sparc_elf_link.rs, riscv32_elf_link.rs, arm64_pe_link.rs)
- LoongArch + SPARC + RV32: ELF linkers
- ARM64 Windows: PE32+ linker

### Sub-agent F: FreeBSD, Haiku, Plan 9 (OS backends)
- 3 backends
- FreeBSD + Haiku: add to platform.rs, reuse elf_link.rs for linking, just different exit syscall
- Plan 9: flat binary, no linker

## Workflow
1. Read current platform.rs and main.rs first
2. Make all changes
3. `cargo build` — fix ALL errors
4. `cargo test` — all 161 existing tests must pass
5. Smoke-test each target
6. Report: files changed, build status, test results, smoke test outputs
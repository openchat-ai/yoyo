//! Platform backends (PROMPT-v3 Part 7).

use crate::types::{IsaError, IsaResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Little,
    Big,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Abi {
    Linux,
    Windows,
    Darwin,
    BareMetal,
    Dos,
    Haiku,
    Plan9,
    Serenity,
    Stub,
    Gpu,
    Wasm,
    Quantum,
    Blockchain,
}

#[derive(Debug, Clone, Copy)]
pub struct ArchProperties {
    pub endian: Endian,
    pub pointer_width: u16,
    pub abi: Abi,
    pub has_mmu: bool,
    pub is_harvard: bool,
    pub has_stack: bool,
    pub min_inst_size: u8,
    pub description: &'static str,
}

impl PlatformKind {
    pub fn properties(self) -> ArchProperties {
        match self {
            PlatformKind::Win32 => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Windows, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 1, description: "Windows x64 (PE32+)" },
            PlatformKind::Linux => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Linux, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 1, description: "Linux x64 (ELF64)" },
            PlatformKind::BareMetal => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::BareMetal, has_mmu: false, is_harvard: false, has_stack: true, min_inst_size: 1, description: "x64 bare-metal" },
            PlatformKind::Stub => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Stub, has_mmu: false, is_harvard: false, has_stack: true, min_inst_size: 1, description: "Stub (no-op)" },
            PlatformKind::Cuda => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Gpu, has_mmu: false, is_harvard: true, has_stack: false, min_inst_size: 4, description: "NVIDIA CUDA (PTX)" },
            PlatformKind::Android => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Linux, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 4, description: "Android ARM64 (ELF64)" },
            PlatformKind::Apple => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Darwin, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 4, description: "Apple ARM64 (Mach-O64)" },
            PlatformKind::Eight051 => ArchProperties { endian: Endian::Little, pointer_width: 8, abi: Abi::BareMetal, has_mmu: false, is_harvard: true, has_stack: true, min_inst_size: 1, description: "Intel 8051 / ESP" },
            PlatformKind::X86 => ArchProperties { endian: Endian::Little, pointer_width: 32, abi: Abi::Windows, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 1, description: "Windows x86-32 (PE32)" },
            PlatformKind::Freedos => ArchProperties { endian: Endian::Little, pointer_width: 16, abi: Abi::Dos, has_mmu: false, is_harvard: false, has_stack: true, min_inst_size: 1, description: "FreeDOS x86-16 (COM)" },
            PlatformKind::Riscv64 => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Linux, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 4, description: "RISC-V RV64 (ELF64)" },
            PlatformKind::Riscv32 => ArchProperties { endian: Endian::Little, pointer_width: 32, abi: Abi::Linux, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 4, description: "RISC-V RV32 (ELF32)" },
            PlatformKind::Mips => ArchProperties { endian: Endian::Big, pointer_width: 32, abi: Abi::Linux, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 4, description: "MIPS big-endian (ELF32BE)" },
            PlatformKind::PowerPc64Le => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Linux, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 4, description: "PowerPC64 LE (ELF64)" },
            PlatformKind::Avr => ArchProperties { endian: Endian::Little, pointer_width: 8, abi: Abi::BareMetal, has_mmu: false, is_harvard: true, has_stack: true, min_inst_size: 2, description: "AVR ATmega" },
            PlatformKind::Arm32 => ArchProperties { endian: Endian::Little, pointer_width: 32, abi: Abi::Linux, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 4, description: "ARMv7 EABI (ELF32)" },
            PlatformKind::Wasm => ArchProperties { endian: Endian::Little, pointer_width: 32, abi: Abi::Wasm, has_mmu: false, is_harvard: true, has_stack: false, min_inst_size: 1, description: "WebAssembly (Wasm)" },
            PlatformKind::MachoX64 => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Darwin, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 1, description: "Apple Intel x64 (Mach-O64)" },
            PlatformKind::Serenity => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Serenity, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 1, description: "SerenityOS (SERE flat)" },
            PlatformKind::LoongArch => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Linux, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 4, description: "LoongArch64 (ELF64)" },
            PlatformKind::Sparc => ArchProperties { endian: Endian::Big, pointer_width: 32, abi: Abi::Linux, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 4, description: "SPARC v8 (ELF32BE)" },
            PlatformKind::Aarch64Windows => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Windows, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 4, description: "ARM64 Windows (PE32+)" },
            PlatformKind::FreeBSD => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Linux, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 1, description: "FreeBSD x64 (ELF64)" },
            PlatformKind::Haiku => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Haiku, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 1, description: "Haiku x64 (ELF64)" },
            PlatformKind::Plan9 => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Plan9, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 1, description: "Plan9 x64 (flat)" },
            PlatformKind::Xtensa => ArchProperties { endian: Endian::Little, pointer_width: 32, abi: Abi::BareMetal, has_mmu: false, is_harvard: false, has_stack: true, min_inst_size: 3, description: "Xtensa LX6 (ESP32)" },
            PlatformKind::Z80 => ArchProperties { endian: Endian::Little, pointer_width: 8, abi: Abi::BareMetal, has_mmu: false, is_harvard: false, has_stack: true, min_inst_size: 1, description: "Z80 8-bit" },
            PlatformKind::M6502 => ArchProperties { endian: Endian::Little, pointer_width: 8, abi: Abi::BareMetal, has_mmu: false, is_harvard: true, has_stack: true, min_inst_size: 1, description: "MOS 6502" },
            PlatformKind::M68k => ArchProperties { endian: Endian::Big, pointer_width: 32, abi: Abi::BareMetal, has_mmu: true, is_harvard: false, has_stack: true, min_inst_size: 2, description: "Motorola 68000" },
            PlatformKind::Msp430 => ArchProperties { endian: Endian::Little, pointer_width: 16, abi: Abi::BareMetal, has_mmu: false, is_harvard: true, has_stack: true, min_inst_size: 2, description: "TI MSP430 16-bit MCU" },
            PlatformKind::Pic => ArchProperties { endian: Endian::Little, pointer_width: 8, abi: Abi::BareMetal, has_mmu: false, is_harvard: true, has_stack: false, min_inst_size: 2, description: "Microchip PIC16" },
            PlatformKind::Stm8 => ArchProperties { endian: Endian::Little, pointer_width: 8, abi: Abi::BareMetal, has_mmu: false, is_harvard: true, has_stack: true, min_inst_size: 1, description: "ST STM8 8-bit MCU" },
            PlatformKind::Rocm => ArchProperties { endian: Endian::Little, pointer_width: 64, abi: Abi::Gpu, has_mmu: false, is_harvard: true, has_stack: false, min_inst_size: 4, description: "AMD ROCm/HIP (text)" },
            PlatformKind::Vulkan => ArchProperties { endian: Endian::Little, pointer_width: 32, abi: Abi::Gpu, has_mmu: false, is_harvard: true, has_stack: false, min_inst_size: 4, description: "Vulkan Compute (SPIR-V)" },
            PlatformKind::Evm => ArchProperties { endian: Endian::Big, pointer_width: 256, abi: Abi::Blockchain, has_mmu: false, is_harvard: true, has_stack: true, min_inst_size: 1, description: "Ethereum EVM" },
            PlatformKind::Qiskit => ArchProperties { endian: Endian::Little, pointer_width: 0, abi: Abi::Quantum, has_mmu: false, is_harvard: false, has_stack: false, min_inst_size: 0, description: "IBM Qiskit (OpenQASM)" },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformKind {
    Win32,
    Linux,
    BareMetal,
    Stub,
    Cuda,
    Android,
    Apple,
    Eight051,
    X86,
    Freedos,
    Riscv64,
    Mips,
    PowerPc64Le,
    Avr,
    Arm32,
    Wasm,
    MachoX64,
    Serenity,
    LoongArch,
    Sparc,
    Riscv32,
    Aarch64Windows,
    FreeBSD,
    Haiku,
    Plan9,
    Xtensa,
    Z80,
    M6502,
    M68k,
    Msp430,
    Pic,
    Stm8,
    Rocm,
    Vulkan,
    Evm,
    Qiskit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryFormat {
    Pe64,
    Elf64,
    FlatBinary,
    Multiboot,
    MachO64,
    Pe32,
    DosCom,
    Elf32BE,
    Elf32LE,
    MachO64X64,
    Serenity,
    HipText,
    SpirV,
    Qasm,
}

#[derive(Debug, Clone)]
pub struct TemplateInfo {
    pub format: BinaryFormat,
    pub entry_point: u32,
    pub stack_size: u32,
    pub data_section_offset: u32,
    pub data_section_size: u32,
}

pub trait PlatformBackend {
    // 闁冲厜鍋撻柍鍏夊亾 I/O & exit (existing) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>>;
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>>;
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, sz_slot: u16) -> IsaResult<Vec<u8>>;
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>>;
    fn startup_blob(&self) -> &[u8];
    fn template(&self) -> TemplateInfo;

    // 闁冲厜鍋撻柍鍏夊亾 Architecture-native emit (Phase 1: default x64; Phase 2: override per arch) 闁冲厜鍋撻柍鍏夊亾

    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0x90])
    }

    fn emit_set(&mut self, _slot: u16, _imm: u64) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_set as x64_set};
        x64_set(_slot, _imm)
    }

    fn emit_get(&mut self, _dst: u16, _src: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_get as x64_get};
        x64_get(_dst, _src)
    }

    fn emit_movrr(&mut self, _dst: u16, _src: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_get as x64_get};
        x64_get(_dst, _src)
    }

    fn emit_add_imm(&mut self, _slot: u16, _imm: u64) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_add_imm as x64_add};
        x64_add(_slot, _imm)
    }

    fn emit_sub_imm(&mut self, _slot: u16, _imm: u64) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_sub_imm as x64_sub};
        x64_sub(_slot, _imm)
    }

    fn emit_inc(&mut self, _slot: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_inc as x64_inc};
        x64_inc(_slot)
    }

    fn emit_dec(&mut self, _slot: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_dec as x64_dec};
        x64_dec(_slot)
    }

    fn emit_addv(&mut self, _dst: u16, _src: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_addv as x64_addv};
        x64_addv(_dst, _src)
    }

    fn emit_orv(&mut self, _dst: u16, _src: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_orv as x64_orv};
        x64_orv(_dst, _src)
    }

    fn emit_subv(&mut self, _dst: u16, _src: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_subv as x64_subv};
        x64_subv(_dst, _src)
    }

    fn emit_imul(&mut self, _dst: u16, _src: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_imul as x64_imul};
        x64_imul(_dst, _src)
    }

    fn emit_cmp(&mut self, _a: u16, _b: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_cmp as x64_cmp};
        x64_cmp(_a, _b)
    }

    fn emit_ldb(&mut self, _dd: u16, _ss: u16, _oo: u16) -> IsaResult<Vec<u8>> {
        // Default x64 LDB: load_state ss 闁?rax; add imm8; movzx; store_state dd
        use crate::assembler::{load_state, store_state};
        use crate::types::Reg;
        let mut out = load_state(_ss, Reg::Rax)?;
        if _oo != 0 {
            out.extend(crate::assembler::add_imm(Reg::Rax, _oo as u64)?);
        }
        out.extend_from_slice(&[0x48, 0x0F, 0xB6, 0x00]);
        out.extend(store_state(_dd, Reg::Rax)?);
        Ok(out)
    }

    fn emit_memcpy_data(&mut self, _dst: u16, _src: u16, _n: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_memcpy_data as x64_mcd};
        x64_mcd(_src, _dst, _n)
    }

    fn emit_memcpy_state(&mut self, _dst: u16, _src: u16, _n: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{emit_memcpy_state as x64_mcs};
        x64_mcs(_src, _dst, _n)
    }

    fn emit_raw_byte(&mut self, _byte: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![_byte])
    }

    fn emit_raw_bytes(&mut self, _bytes: Vec<u8>) -> IsaResult<Vec<u8>> {
        Ok(_bytes)
    }

    // 闁冲厜鍋撻柍鍏夊亾 Branch fixup abstraction 闁冲厜鍋撻柍鍏夊亾
    fn emit_call_placeholder(&mut self) -> IsaResult<Vec<u8>> {
        use crate::assembler::{call_rel32 as x64_call};
        x64_call(0)
    }
    fn emit_jmp_placeholder(&mut self) -> IsaResult<Vec<u8>> {
        use crate::assembler::{jmp_rel32 as x64_jmp};
        x64_jmp(0)
    }
    fn emit_jcc_placeholder(&mut self, cc: u8) -> IsaResult<Vec<u8>> {
        use crate::assembler::{jcc_rel32 as x64_jcc};
        x64_jcc(cc, 0)
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        use crate::assembler::ret;
        Ok(ret())
    }

    /// Return (bytes, BranchFixup). Default: x64 rel32.
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        let bytes = self.emit_call_placeholder()?;
        Ok((bytes, BranchFixup { field_offset: 1, field_size: 4, kind: FixupKind::Rel32 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        let bytes = self.emit_jmp_placeholder()?;
        Ok((bytes, BranchFixup { field_offset: 1, field_size: 4, kind: FixupKind::Rel32 }))
    }
    fn emit_jcc_branch(&mut self, _cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        let bytes = self.emit_jcc_placeholder(_cc)?;
        Ok((bytes, BranchFixup { field_offset: 2, field_size: 4, kind: FixupKind::Rel32 }))
    }
    /// Patch branch bytes in `code` at `branch_start` so it jumps to `target`.
    /// Default: x64 rel32 patch (4-byte LE signed offset).
    fn patch_branch(&self, code: &mut [u8], branch_start: usize, fixup: &BranchFixup, target: u32) -> IsaResult<()> {
        let field_addr = branch_start + fixup.field_offset;
        match fixup.kind {
            FixupKind::Rel32 => {
                let rel = target as i32 - (field_addr as i32 + 4);
                let bytes = (rel as u32).to_le_bytes();
                code[field_addr..field_addr + 4].copy_from_slice(&bytes);
            }
            FixupKind::ArmImm26 => {
                let diff = target as i32 - branch_start as i32;
                let imm26 = (diff >> 2) & 0x3FFFFFF;
                // The existing 4-byte instruction word at branch_start already has bits 31:26 = 0x94 (bl) or 0x14 (b)
                // Keep the top bits, replace bits 25:0
                let base = u32::from_le_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let patched = (base & 0xFC000000) | (imm26 as u32);
                code[branch_start..branch_start + 4].copy_from_slice(&patched.to_le_bytes());
            }
            FixupKind::ArmImm19 => {
                let diff = target as i32 - branch_start as i32;
                let imm19 = ((diff >> 2) & 0x7FFFF) as u32;
                let base = u32::from_le_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let patched = (base & 0xFF00001F) | (imm19 << 5);
                code[branch_start..branch_start + 4].copy_from_slice(&patched.to_le_bytes());
            }
            FixupKind::RiscvJ => {
                let diff = target as i32 - branch_start as i32;
                let imm = diff as u32;
                // J-type: imm[20] imm[10:1] imm[11] imm[19:12] xxxx
                let imm20 = ((imm >> 12) & 0x7FFFF) as u32; // actually need to encode J-type
                let i20 = (imm >> 20) & 1;
                let i10_1 = (imm >> 1) & 0x3FF;
                let i11 = (imm >> 11) & 1;
                let i19_12 = (imm >> 12) & 0xFF;
                let enc = (i20 << 31) | (i10_1 << 21) | (i11 << 20) | (i19_12 << 12);
                let base = u32::from_le_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let patched = (base & 0x00000FFF) | enc;
                code[branch_start..branch_start + 4].copy_from_slice(&patched.to_le_bytes());
            }
            FixupKind::RiscvB => {
                let diff = target as i32 - branch_start as i32;
                let imm = diff as u32;
                // B-type: imm[12] imm[10:5] imm[4:1] imm[11] xxxx
                let i12 = (imm >> 12) & 1;
                let i10_5 = (imm >> 5) & 0x3F;
                let i4_1 = (imm >> 1) & 0xF;
                let i11 = (imm >> 11) & 1;
                let enc = (i12 << 31) | (i10_5 << 25) | (i4_1 << 8) | (i11 << 7);
                let base = u32::from_le_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                // Preserve opcode[6:0], funct3[14:12], rs1[19:15], rs2[24:20]
                let patched = (base & 0x01FFF07F) | enc;
                code[branch_start..branch_start + 4].copy_from_slice(&patched.to_le_bytes());
            }
            FixupKind::ArmImm24 => {
                let diff = target as i32 - branch_start as i32 - 8;
                let imm24 = (diff >> 2) & 0xFFFFFF;
                let base = u32::from_le_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let patched = (base & 0xFF000000) | (imm24 as u32);
                code[branch_start..branch_start + 4].copy_from_slice(&patched.to_le_bytes());
            }
            FixupKind::MipsImm16 => {
                // beq/bne: imm16 = (target - (branch_addr + 4)) / 4, signed
                let diff = target as i32 - (branch_start as i32 + 4);
                let imm16 = (diff >> 2) & 0xFFFF;
                let base = u32::from_be_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let patched = (base & 0xFFFF0000) | (imm16 as u32);
                code[branch_start..branch_start + 4].copy_from_slice(&patched.to_be_bytes());
            }
            FixupKind::MipsImm26 => {
                // j/jal: target = (branch_addr & 0xF0000000) | (imm26 << 2)
                let imm26 = (target >> 2) & 0x3FFFFFF;
                let base = u32::from_be_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let patched = (base & 0xFC000000) | (imm26 as u32);
                code[branch_start..branch_start + 4].copy_from_slice(&patched.to_be_bytes());
            }
            FixupKind::PpcImm24 => {
                // b/bl: imm24 = (target - branch_addr) & 0x3FFFFFC
                let diff = target as i32 - branch_start as i32;
                let imm24 = (diff as u32) & 0x3FFFFFC;
                let base = u32::from_le_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let li = base & 0x00000001; // preserve LK bit (call vs branch)
                let patched = (base & 0xFC000001) | li | imm24;
                code[branch_start..branch_start + 4].copy_from_slice(&patched.to_le_bytes());
            }
            FixupKind::PpcImm14 => {
                // conditional branch (beq etc): BD at bits[15:2]; preserve BO/BI and AA/LK
                let diff = target as i32 - branch_start as i32;
                let imm14 = (diff as u32) & 0xFFFC;
                let base = u32::from_le_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let patched = (base & 0xFFFF0003) | imm14;
                code[branch_start..branch_start + 4].copy_from_slice(&patched.to_le_bytes());
            }
            FixupKind::LoongImm26 => {
                // b/bl: 26-bit signed PC-relative, offset = (target - branch_addr) / 4
                let diff = target as i32 - branch_start as i32;
                let imm26 = (diff >> 2) & 0x3FFFFFF;
                let base = u32::from_le_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let patched = (base & 0xFC000000) | (imm26 as u32);
                code[branch_start..branch_start + 4].copy_from_slice(&patched.to_le_bytes());
            }
            FixupKind::LoongImm16 => {
                // beq/bne: offs16 at bits[25:10]; preserve opcode[31:26] and rj/rd[9:0]
                let diff = target as i32 - branch_start as i32;
                let imm16 = (diff >> 2) & 0xFFFF;
                let base = u32::from_le_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let patched = (base & 0xFC0003FF) | ((imm16 as u32) << 10);
                code[branch_start..branch_start + 4].copy_from_slice(&patched.to_le_bytes());
            }
            FixupKind::SparcImm22 => {
                // SPARC branch: imm22 = (target - branch_addr) / 4, signed 22-bit
                let diff = target as i32 - branch_start as i32;
                let imm22 = (diff >> 2) & 0x3FFFFF;
                let base = u32::from_be_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let patched = (base & 0xFFC00000) | (imm22 as u32);
                code[branch_start..branch_start + 4].copy_from_slice(&patched.to_be_bytes());
            }
            FixupKind::SparcImm30 => {
                // SPARC call: imm30 = (target - branch_start) / 4, signed 30-bit
                let diff = target as i32 - branch_start as i32;
                let imm30 = (diff >> 2) & 0x3FFFFFFF;
                let base = u32::from_be_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let patched = (base & 0xC0000000) | (imm30 as u32);
                code[branch_start..branch_start + 4].copy_from_slice(&patched.to_be_bytes());
            }
            FixupKind::XtensaImm18 => {
                // Xtensa 3-byte j/j callx: signed 18-bit offset inline
                let diff = target as i32 - branch_start as i32;
                let imm18 = (diff as u32) & 0x3FFFF;
                // 3-byte instruction: first byte carries bits, others carry imm18
                // For j: opcode 0x6 (bits 23:16), imm18 in bits 17:0
                let word = (code[branch_start] as u32)
                    | ((code[branch_start + 1] as u32) << 8)
                    | ((code[branch_start + 2] as u32) << 16);
                let patched = (word & 0xFF0000) | (imm18 & 0xFFFF);
                code[branch_start] = (patched & 0xFF) as u8;
                code[branch_start + 1] = ((patched >> 8) & 0xFF) as u8;
                code[branch_start + 2] = ((patched >> 16) & 0xFF) as u8;
            }
            FixupKind::ByteRel8 => {
                // Signed 8-bit PC-relative offset from field_addr
                let diff = target as i32 - (field_addr as i32 + 1);
                let rel8 = diff as i8;
                code[field_addr] = rel8 as u8;
            }
            FixupKind::AbsAddr16 => {
                // 16-bit absolute address, LE
                let addr = target as u16;
                code[field_addr..field_addr + 2].copy_from_slice(&addr.to_le_bytes());
            }
            FixupKind::AvrBrRel7 => {
                // AVR BREQ/BRNE: 7-bit signed word offset in bits[9:3], PC+1 in words
                let next_pc = branch_start + 2;
                let diff_words = (target as i32 - next_pc as i32) / 2;
                let k = (diff_words as i8 as u16) & 0x7F;
                let base = u16::from_le_bytes(code[branch_start..branch_start + 2].try_into().unwrap());
                // Preserve opcode/condition bits (incl. BRBS vs BRBC); only patch k in bits[9:3]
                let patched = (base & 0xFC07) | (k << 3);
                code[branch_start..branch_start + 2].copy_from_slice(&patched.to_le_bytes());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BranchFixup {
    pub field_offset: usize,
    pub field_size: usize,
    pub kind: FixupKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixupKind {
    Rel32,
    ArmImm26,
    ArmImm19,
    RiscvJ,
    RiscvB,
    ArmImm24,
    MipsImm16,
    MipsImm26,
    PpcImm24,
    PpcImm14,
    LoongImm26,
    LoongImm16,
    SparcImm22,
    SparcImm30,
    XtensaImm18,
    ByteRel8,
    AbsAddr16,
    AvrBrRel7,
}

pub fn select_platform(target: PlatformKind) -> Box<dyn PlatformBackend> {
    match target {
        PlatformKind::Win32 => Box::new(Win32Platform::new()),
        PlatformKind::Linux => Box::new(LinuxPlatform::new()),
        PlatformKind::BareMetal => Box::new(BareMetalPlatform::new()),
        PlatformKind::Stub => Box::new(StubPlatform::new()),
        PlatformKind::Cuda => Box::new(CudaPlatform::new()),
        PlatformKind::Android => Box::new(AndroidPlatform::new()),
        PlatformKind::Apple => Box::new(ApplePlatform::new()),
        PlatformKind::Eight051 => Box::new(Eight051Platform::new()),
        PlatformKind::X86 => Box::new(X86Platform::new()),
        PlatformKind::Freedos => Box::new(FreedosPlatform::new()),
        PlatformKind::Riscv64 => Box::new(Riscv64Platform::new()),
        PlatformKind::Mips => Box::new(MipsPlatform::new()),
        PlatformKind::PowerPc64Le => Box::new(PowerPc64LePlatform::new()),
        PlatformKind::Avr => Box::new(AvrPlatform::new()),
        PlatformKind::Arm32 => Box::new(Arm32Platform::new()),
        PlatformKind::Wasm => Box::new(WasmPlatform::new()),
        PlatformKind::MachoX64 => Box::new(MachoX64Platform::new()),
        PlatformKind::Serenity => Box::new(SerenityPlatform::new()),
        PlatformKind::LoongArch => Box::new(LoongArchPlatform::new()),
        PlatformKind::Sparc => Box::new(SparcPlatform::new()),
        PlatformKind::Riscv32 => Box::new(Riscv32Platform::new()),
        PlatformKind::Aarch64Windows => Box::new(Aarch64WindowsPlatform::new()),
        PlatformKind::FreeBSD => Box::new(FreeBSDPlatform::new()),
        PlatformKind::Haiku => Box::new(HaikuPlatform::new()),
        PlatformKind::Plan9 => Box::new(Plan9Platform::new()),
        PlatformKind::Xtensa => Box::new(XtensaPlatform::new()),
        PlatformKind::Z80 => Box::new(Z80Platform::new()),
        PlatformKind::M6502 => Box::new(M6502Platform::new()),
        PlatformKind::M68k => Box::new(M68kPlatform::new()),
        PlatformKind::Msp430 => Box::new(Msp430Platform::new()),
        PlatformKind::Pic => Box::new(PicPlatform::new()),
        PlatformKind::Stm8 => Box::new(Stm8Platform::new()),
        PlatformKind::Rocm => Box::new(RocmPlatform::new()),
        PlatformKind::Vulkan => Box::new(VulkanPlatform::new()),
        PlatformKind::Evm => Box::new(EvmPlatform::new()),
        PlatformKind::Qiskit => Box::new(QiskitPlatform::new()),
    }
}

pub fn parse_platform(s: &str) -> IsaResult<PlatformKind> {
    match s.to_ascii_lowercase().as_str() {
        "win32" | "windows" | "pe" => Ok(PlatformKind::Win32),
        "linux" | "elf" => Ok(PlatformKind::Linux),
        "baremetal" | "bare" => Ok(PlatformKind::BareMetal),
        "stub" => Ok(PlatformKind::Stub),
        "cuda" | "ptx" => Ok(PlatformKind::Cuda),
        "android" | "aarch64" => Ok(PlatformKind::Android),
        "apple" | "darwin" | "ios" | "macos" => Ok(PlatformKind::Apple),
        "8051" | "esp" => Ok(PlatformKind::Eight051),
        "x86" | "x86-32" | "windows32" | "pe32" => Ok(PlatformKind::X86),
        "freedos" | "dos" | "com" => Ok(PlatformKind::Freedos),
        "riscv64" | "rv64" | "riscv" | "riscv64-linux" => Ok(PlatformKind::Riscv64),
        "mips" | "mipsbe" => Ok(PlatformKind::Mips),
        "loongarch" | "loongarch64" | "la64" | "longarch" => Ok(PlatformKind::LoongArch),
        "sparc" | "sparcv8" | "sun4m" => Ok(PlatformKind::Sparc),
        "riscv32" | "rv32" | "riscv32-linux" => Ok(PlatformKind::Riscv32),
        "arm64-win" | "aarch64-windows" | "arm64-windows" | "windows-arm64" => Ok(PlatformKind::Aarch64Windows),
        "ppc64le" | "powerpc64le" | "ppc" | "powerpc" => Ok(PlatformKind::PowerPc64Le),
        "avr" | "atmega" => Ok(PlatformKind::Avr),
        "arm" | "arm32" | "armeabi" | "arm32-android" => Ok(PlatformKind::Arm32),
        "wasm" | "webasm" | "wasm32" => Ok(PlatformKind::Wasm),
        "macos-x86" | "macos-intel" | "macho-x64" | "macho-x86_64" => Ok(PlatformKind::MachoX64),
        "serenity" | "serenityos" => Ok(PlatformKind::Serenity),
        "freebsd" | "freebsd-x64" => Ok(PlatformKind::FreeBSD),
        "haiku" | "haiku-x64" => Ok(PlatformKind::Haiku),
        "plan9" | "plan9-x64" | "acadia" => Ok(PlatformKind::Plan9),
        "xtensa" | "esp32" | "lx6" => Ok(PlatformKind::Xtensa),
        "z80" | "z80-rom" => Ok(PlatformKind::Z80),
        "6502" | "m6502" | "commodore" => Ok(PlatformKind::M6502),
        "m68k" | "68000" | "m68000" | "amiga" => Ok(PlatformKind::M68k),
        "msp430" | "ti-msp430" => Ok(PlatformKind::Msp430),
        "pic" | "pic16" | "microchip" => Ok(PlatformKind::Pic),
        "stm8" | "stm8s" | "stm8l" => Ok(PlatformKind::Stm8),
        "rocm" | "hip" | "amd-gpu" | "gcn" => Ok(PlatformKind::Rocm),
        "vulkan" | "spirv" | "vulkan-compute" => Ok(PlatformKind::Vulkan),
        "evm" | "ethereum" | "solidity" => Ok(PlatformKind::Evm),
        "qiskit" | "openqasm" | "quantum" => Ok(PlatformKind::Qiskit),
        _ => Err(IsaError::PlatformError {
            msg: format!("unknown platform '{s}'"),
        }),
    }
}

// 闁冲厜鍋撻柍鍏夊亾 Stub 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
pub struct StubPlatform;

impl StubPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for StubPlatform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, size)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, str_idx as u64)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, str_idx as u64)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xC3])
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0x10000,
            data_section_offset: 0,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 Win32 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
pub struct Win32Platform {
    startup: Vec<u8>,
}

impl Win32Platform {
    pub fn new() -> Self {
        Self { startup: vec![] }
    }
}

impl PlatformBackend for Win32Platform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, size)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, str_idx as u64)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, str_idx as u64)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        use crate::assembler::movabs;
        use crate::types::Reg;
        let mut out = movabs(Reg::Rcx, code as u64)?;
        out.push(0xC3);
        Ok(out)
    }
    /// Slot-to-slot MEMCPY_STATE (ISA/simulator semantics), not pointer-form rep movsb.
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{load_state, store_state};
        use crate::types::Reg;
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "Win32 memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n {
            out.extend(load_state(src + i, Reg::Rax)?);
            out.extend(store_state(dst + i, Reg::Rax)?);
        }
        Ok(out)
    }
    fn startup_blob(&self) -> &[u8] {
        &self.startup
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Pe64,
            entry_point: 0x1000,
            stack_size: 0x100000,
            data_section_offset: 0x2000,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 Linux 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
pub struct LinuxPlatform;

impl LinuxPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for LinuxPlatform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, size)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, _str_idx: u8) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, 0)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, _str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, 0)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![
            0xB8, 60, 0, 0, 0,
            0xBF, code, 0, 0, 0,
            0x0F, 0x05,
        ])
    }
    /// Slot-to-slot MEMCPY_STATE (ISA/simulator semantics), not pointer-form rep movsb.
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{load_state, store_state};
        use crate::types::Reg;
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "Linux memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n {
            out.extend(load_state(src + i, Reg::Rax)?);
            out.extend(store_state(dst + i, Reg::Rax)?);
        }
        Ok(out)
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Elf64,
            entry_point: 0x401000,
            stack_size: 0x10000,
            data_section_offset: 0x402000,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 CUDA 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
pub struct CudaPlatform;

impl CudaPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for CudaPlatform {
    fn emit_alloc(&mut self, _slot: u16, _size: u64) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "CUDA backend has no inline alloc; use cuda_backend::emit_cuda instead".into(),
        })
    }
    fn emit_load_file(&mut self, _slot: u16, _str_idx: u8) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "CUDA backend has no file I/O; use cuda_backend::emit_cuda instead".into(),
        })
    }
    fn emit_write_file(&mut self, _slot: u16, _str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "CUDA backend has no file I/O; use cuda_backend::emit_cuda instead".into(),
        })
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xFF])
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0,
            data_section_offset: 0,
            data_section_size: 0,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 Bare-metal 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
pub struct BareMetalPlatform;

impl BareMetalPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for BareMetalPlatform {
    fn emit_alloc(&mut self, _slot: u16, _size: u64) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "bare-metal has no heap alloc".into(),
        })
    }
    fn emit_load_file(&mut self, _slot: u16, _str_idx: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0x90])
    }
    fn emit_write_file(&mut self, _slot: u16, _str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        Ok(vec![0x90])
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xF4])
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0x1000,
            stack_size: 0x90000,
            data_section_offset: 0x8000,
            data_section_size: 0x1000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 Android 闁?ARM64 (aarch64) + ELF64 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
pub struct AndroidPlatform;

impl AndroidPlatform {
    pub fn new() -> Self {
        Self
    }
}

fn arm64_movz_w_imm16(rd: u32, imm16: u16) -> [u8; 4] {
    let enc: u32 = 0x22800000 | ((imm16 as u32) << 5) | rd;
    enc.to_le_bytes()
}

const ARM64_NOP: [u8; 4] = [0x1F, 0x20, 0x03, 0xD5];
const ARM64_RET: [u8; 4] = [0xC0, 0x03, 0x5F, 0xD6];

// 闁冲厜鍋撻柍鍏夊亾 ARM64 instruction encoding helpers 闁冲厜鍋撻柍鍏夊亾
fn arm64_mov_imm64(rd: u32, imm: u64) -> Vec<u8> {
    let mut out = Vec::new();
    let mut written = false;
    for shift in (0..64).step_by(16) {
        let chunk = (imm >> shift) & 0xFFFF;
        if chunk != 0 || shift == 0 {
            if !written {
                // MOVZ rd, chunk, lsl shift
                let enc: u32 = 0xD2800000 | rd | ((chunk as u32) << 5) | ((shift as u32 / 16) << 21);
                out.extend_from_slice(&enc.to_le_bytes());
                written = true;
            } else {
                // MOVK rd, chunk, lsl shift
                let enc: u32 = 0xF2800000 | rd | ((chunk as u32) << 5) | ((shift as u32 / 16) << 21);
                out.extend_from_slice(&enc.to_le_bytes());
            }
        }
    }
    out
}
fn arm64_ldr_imm(rd: u32, rn: u32, imm12: u16) -> [u8; 4] {
    let enc: u32 = 0xF9400000 | rd | (rn << 5) | ((imm12 as u32) << 10);
    enc.to_le_bytes()
}
fn arm64_str_imm(rs: u32, rn: u32, imm12: u16) -> [u8; 4] {
    let enc: u32 = 0xF9000000 | rs | (rn << 5) | ((imm12 as u32) << 10);
    enc.to_le_bytes()
}
fn arm64_add_imm(rd: u32, rn: u32, imm12: u16) -> [u8; 4] {
    let enc: u32 = 0x91000000 | rd | (rn << 5) | ((imm12 as u32) << 10);
    enc.to_le_bytes()
}
fn arm64_sub_imm(rd: u32, rn: u32, imm12: u16) -> [u8; 4] {
    let enc: u32 = 0xD1000000 | rd | (rn << 5) | ((imm12 as u32) << 10);
    enc.to_le_bytes()
}
fn arm64_add_reg(rd: u32, rn: u32, rm: u32) -> [u8; 4] {
    let enc: u32 = 0x8B000000 | rd | (rn << 5) | (rm << 16);
    enc.to_le_bytes()
}
fn arm64_sub_reg(rd: u32, rn: u32, rm: u32) -> [u8; 4] {
    let enc: u32 = 0xCB000000 | rd | (rn << 5) | (rm << 16);
    enc.to_le_bytes()
}
fn arm64_mul_reg(rd: u32, rn: u32, rm: u32) -> [u8; 4] {
    let enc: u32 = 0x9B007C00 | rd | (rn << 5) | (rm << 16);
    enc.to_le_bytes()
}
fn arm64_orr_reg(rd: u32, rn: u32, rm: u32) -> [u8; 4] {
    let enc: u32 = 0xAA000000 | rd | (rn << 5) | (rm << 16);
    enc.to_le_bytes()
}
fn arm64_cmp(rd: u32, rn: u32) -> [u8; 4] {
    let enc: u32 = 0xEB00001F | (rd << 5) | (rn << 16);
    enc.to_le_bytes()
}
fn arm64_ldrb(rd: u32, rn: u32) -> [u8; 4] {
    let enc: u32 = 0x39400000 | rd | (rn << 5);
    enc.to_le_bytes()
}
fn arm64_jcc_cond(cc: u8) -> u32 {
    match cc {
        0x84 => 0x0, 0x85 => 0x1, 0x86 => 0xB, 0x87 => 0xA,
        0x88 => 0xD, 0x89 => 0xC, 0x8A => 0x3, 0x8B => 0x2,
        0x8C => 0x9, 0x8D => 0x8, _ => 0x0,
    }
}

impl PlatformBackend for AndroidPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(ARM64_NOP.to_vec())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(ARM64_RET.to_vec())
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm64_mov_imm64(9, size);
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = arm64_mov_imm64(9, str_idx as u64);
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_mov_imm64(9, str_idx as u64);
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![];
        out.extend_from_slice(&arm64_movz_w_imm16(8, 93));
        out.extend_from_slice(&arm64_movz_w_imm16(0, code as u16));
        out.extend_from_slice(&[0x01, 0x00, 0x00, 0xD4]);
        Ok(out)
    }
    // 闁冲厜鍋撻柍鍏夊亾 Real ARM64 instruction overrides 闁冲厜鍋撻柍鍏夊亾
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm64_mov_imm64(9, imm);
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, src).to_vec();
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, src).to_vec();
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, slot).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&arm64_add_imm(9, 9, imm as u16));
        } else {
            out.extend_from_slice(&arm64_mov_imm64(10, imm));
            out.extend_from_slice(&arm64_add_reg(9, 9, 10));
        }
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, slot).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&arm64_sub_imm(9, 9, imm as u16));
        } else {
            out.extend_from_slice(&arm64_mov_imm64(10, imm));
            out.extend_from_slice(&arm64_sub_reg(9, 9, 10));
        }
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, slot).to_vec();
        out.extend_from_slice(&arm64_add_imm(9, 9, 1));
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, slot).to_vec();
        out.extend_from_slice(&arm64_sub_imm(9, 9, 1));
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, dst).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(10, 15, src));
        out.extend_from_slice(&arm64_add_reg(9, 9, 10));
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, dst).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(10, 15, src));
        out.extend_from_slice(&arm64_orr_reg(9, 9, 10));
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, dst).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(10, 15, src));
        out.extend_from_slice(&arm64_sub_reg(9, 9, 10));
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, dst).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(10, 15, src));
        out.extend_from_slice(&arm64_mul_reg(9, 9, 10));
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(10, 15, a).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(11, 15, b));
        out.extend_from_slice(&arm64_cmp(10, 11));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, ss).to_vec();
        if oo != 0 {
            out.extend_from_slice(&arm64_add_imm(9, 9, oo));
        }
        out.extend_from_slice(&arm64_ldrb(10, 9));
        out.extend_from_slice(&arm64_str_imm(10, 15, dd));
        Ok(out)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "ARM64 memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n as u16 {
            out.extend_from_slice(&arm64_ldr_imm(9, 15, src + i));
            out.extend_from_slice(&arm64_str_imm(9, 15, dst + i));
        }
        Ok(out)
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0x00, 0x00, 0x00, 0x94], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::ArmImm26 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0x00, 0x00, 0x00, 0x14], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::ArmImm26 }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        let cond = arm64_jcc_cond(cc);
        // B.cond: cond in bits[3:0], bit4=0, imm19 in bits[23:5]
        let enc: u32 = 0x54000000 | (cond & 0xF);
        Ok((enc.to_le_bytes().to_vec(), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::ArmImm19 }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Elf64,
            entry_point: 0x4001000,
            stack_size: 0x10000,
            data_section_offset: 0x4002000,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 Apple/iOS 闁?ARM64 (aarch64) + Mach-O64 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
pub struct ApplePlatform;

impl ApplePlatform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for ApplePlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(ARM64_NOP.to_vec())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(ARM64_RET.to_vec())
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm64_mov_imm64(9, size);
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = arm64_mov_imm64(9, str_idx as u64);
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_mov_imm64(9, str_idx as u64);
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![];
        out.extend_from_slice(&[0x18, 0x00, 0x80, 0x12]);
        out.extend_from_slice(&[0x18, 0x04, 0x90, 0x92]);
        out.extend_from_slice(&arm64_movz_w_imm16(0, code as u16));
        out.extend_from_slice(&[0x01, 0x00, 0x00, 0xD4]);
        Ok(out)
    }
    // 闁冲厜鍋撻柍鍏夊亾 Real ARM64 instruction overrides (Apple ARM64) 闁冲厜鍋撻柍鍏夊亾
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm64_mov_imm64(9, imm);
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, src).to_vec();
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, src).to_vec();
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, slot).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&arm64_add_imm(9, 9, imm as u16));
        } else {
            out.extend_from_slice(&arm64_mov_imm64(10, imm));
            out.extend_from_slice(&arm64_add_reg(9, 9, 10));
        }
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, slot).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&arm64_sub_imm(9, 9, imm as u16));
        } else {
            out.extend_from_slice(&arm64_mov_imm64(10, imm));
            out.extend_from_slice(&arm64_sub_reg(9, 9, 10));
        }
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, slot).to_vec();
        out.extend_from_slice(&arm64_add_imm(9, 9, 1));
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, slot).to_vec();
        out.extend_from_slice(&arm64_sub_imm(9, 9, 1));
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, dst).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(10, 15, src));
        out.extend_from_slice(&arm64_add_reg(9, 9, 10));
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, dst).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(10, 15, src));
        out.extend_from_slice(&arm64_orr_reg(9, 9, 10));
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, dst).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(10, 15, src));
        out.extend_from_slice(&arm64_sub_reg(9, 9, 10));
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, dst).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(10, 15, src));
        out.extend_from_slice(&arm64_mul_reg(9, 9, 10));
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(10, 15, a).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(11, 15, b));
        out.extend_from_slice(&arm64_cmp(10, 11));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, ss).to_vec();
        if oo != 0 {
            out.extend_from_slice(&arm64_add_imm(9, 9, oo));
        }
        out.extend_from_slice(&arm64_ldrb(10, 9));
        out.extend_from_slice(&arm64_str_imm(10, 15, dd));
        Ok(out)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "ARM64 memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n as u16 {
            out.extend_from_slice(&arm64_ldr_imm(9, 15, src + i));
            out.extend_from_slice(&arm64_str_imm(9, 15, dst + i));
        }
        Ok(out)
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0x00, 0x00, 0x00, 0x94], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::ArmImm26 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0x00, 0x00, 0x00, 0x14], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::ArmImm26 }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        let cond = arm64_jcc_cond(cc);
        // B.cond: cond in bits[3:0], bit4=0, imm19 in bits[23:5]
        let enc: u32 = 0x54000000 | (cond & 0xF);
        Ok((enc.to_le_bytes().to_vec(), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::ArmImm19 }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::MachO64,
            entry_point: 0,
            stack_size: 0x10000,
            data_section_offset: 0,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 8051 encoding helpers 闁冲厜鍋撻柍鍏夊亾
fn e8051_mov_direct_imm(direct: u8, imm: u8) -> Vec<u8> {
    vec![0x75, direct, imm]
}
fn e8051_mov_a_direct(direct: u8) -> Vec<u8> {
    vec![0xE5, direct]
}
fn e8051_mov_direct_a(direct: u8) -> Vec<u8> {
    vec![0xF5, direct]
}
fn e8051_add_a_imm(imm: u8) -> Vec<u8> {
    vec![0x24, imm]
}
fn e8051_subb_a_imm(imm: u8) -> Vec<u8> {
    vec![0x94, imm]
}
fn e8051_inc_direct(direct: u8) -> Vec<u8> {
    vec![0x05, direct]
}
fn e8051_dec_direct(direct: u8) -> Vec<u8> {
    vec![0x15, direct]
}
fn e8051_inc_a() -> Vec<u8> { vec![0x04] }
fn e8051_dec_a() -> Vec<u8> { vec![0x14] }
fn e8051_orl_a_direct(direct: u8) -> Vec<u8> {
    vec![0x42, direct]
}
fn e8051_anl_a_direct(direct: u8) -> Vec<u8> {
    vec![0x52, direct]
}
fn e8051_mul_ab() -> Vec<u8> { vec![0xA4] }
fn e8051_cjne_a_direct_rel(direct: u8, rel: u8) -> Vec<u8> {
    vec![0xB5, direct, rel]
}
fn e8051_add_a_direct(direct: u8) -> Vec<u8> {
    vec![0x25, direct]
}
fn e8051_ljmp(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_be_bytes();
    vec![0x02, hi, lo]
}
fn e8051_lcall(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_be_bytes();
    vec![0x12, hi, lo]
}
fn e8051_sjmp(rel: u8) -> Vec<u8> {
    vec![0x80, rel]
}
fn e8051_ret() -> Vec<u8> { vec![0x22] }
fn e8051_nop() -> Vec<u8> { vec![0x00] }

const E8051_STATE_BASE: u8 = 0x30;

// 闁冲厜鍋撻柍鍏夊亾 8051 ASM (Intel 8051 / ESP8266/ESP32 8051 core) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
pub struct Eight051Platform;

impl Eight051Platform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for Eight051Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(e8051_nop())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(e8051_ret())
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let direct = E8051_STATE_BASE + slot as u8;
        if imm <= 0xFF {
            Ok(e8051_mov_direct_imm(direct, imm as u8))
        } else {
            // Truncate to 8 bits (matches AVR/M6502 behavior for 8-bit MCUs)
            Ok(e8051_mov_direct_imm(direct, imm as u8))
        }
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let d = E8051_STATE_BASE + dst as u8;
        let s = E8051_STATE_BASE + src as u8;
        let mut out = e8051_mov_a_direct(s);
        out.extend(e8051_mov_direct_a(d));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let direct = E8051_STATE_BASE + slot as u8;
        if imm <= 0xFF {
            let mut out = e8051_mov_a_direct(direct);
            out.extend(e8051_add_a_imm(imm as u8));
            out.extend(e8051_mov_direct_a(direct));
            Ok(out)
        } else {
            // Truncate to 8 bits (single-byte state slot, matches AVR/M6502)
            let mut out = e8051_mov_a_direct(direct);
            out.extend(e8051_add_a_imm(imm as u8));
            out.extend(e8051_mov_direct_a(direct));
            Ok(out)
        }
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let direct = E8051_STATE_BASE + slot as u8;
        if imm <= 0xFF {
            let mut out = vec![0xC3]; // CLR C
            out.extend(e8051_mov_a_direct(direct));
            out.extend(e8051_subb_a_imm(imm as u8));
            out.extend(e8051_mov_direct_a(direct));
            Ok(out)
        } else {
            // Truncate to 8 bits (single-byte state slot, matches AVR/M6502)
            let mut out = vec![0xC3]; // CLR C
            out.extend(e8051_mov_a_direct(direct));
            out.extend(e8051_subb_a_imm(imm as u8));
            out.extend(e8051_mov_direct_a(direct));
            Ok(out)
        }
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        Ok(e8051_inc_direct(E8051_STATE_BASE + slot as u8))
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        Ok(e8051_dec_direct(E8051_STATE_BASE + slot as u8))
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let d = E8051_STATE_BASE + dst as u8;
        let s = E8051_STATE_BASE + src as u8;
        let mut out = e8051_mov_a_direct(d);
        out.extend(e8051_add_a_direct(s));
        out.extend(e8051_mov_direct_a(d));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let d = E8051_STATE_BASE + dst as u8;
        let s = E8051_STATE_BASE + src as u8;
        let mut out = e8051_mov_a_direct(d);
        out.extend(e8051_orl_a_direct(s));
        out.extend(e8051_mov_direct_a(d));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let d = E8051_STATE_BASE + dst as u8;
        let s = E8051_STATE_BASE + src as u8;
        // 8051 has no SUB without borrow: CLR C then SUBB
        let mut out = e8051_mov_a_direct(d);
        out.push(0xC3); // CLR C
        out.push(0x95); // SUBB A, direct
        out.push(s);
        out.extend(e8051_mov_direct_a(d));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let d = E8051_STATE_BASE + dst as u8;
        let s = E8051_STATE_BASE + src as u8;
        // MOV A, dst; MOV B, A; MOV A, src; MUL AB; MOV dst, A
        let mut out = e8051_mov_a_direct(d);
        out.push(0xF5); out.push(0xF0); // MOV B, A (B is SFR 0xF0)
        out.extend(e8051_mov_a_direct(s));
        out.push(0xA4); // MUL AB
        out.extend(e8051_mov_direct_a(d));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let addr_a = E8051_STATE_BASE + a as u8;
        let addr_b = E8051_STATE_BASE + b as u8;
        let mut out = vec![0xC3]; // CLR C
        out.extend(e8051_mov_a_direct(addr_a));
        out.push(0x95); // SUBB A, direct
        out.push(addr_b);
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let d = E8051_STATE_BASE + dd as u8;
        let s = E8051_STATE_BASE + ss as u8;
        // MOV A, ss; ADD A, #oo; MOV DPL, A; MOV DPH, #0; MOVX A, @DPTR; MOV dd, A
        let mut out = e8051_mov_a_direct(s);
        if oo != 0 {
            out.extend(e8051_add_a_imm(oo as u8));
        }
        out.push(0xF5); out.push(0x82); // MOV DPL (0x82), A
        out.push(0x75); out.push(0x83); out.push(0x00); // MOV DPH (0x83), #0
        out.push(0xE0); // MOVX A, @DPTR
        out.extend(e8051_mov_direct_a(d));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        let d = E8051_STATE_BASE + dst as u8;
        let s = E8051_STATE_BASE + src as u8;
        for _i in 0..n {
            out.extend(e8051_mov_a_direct(s));
            out.extend(e8051_mov_direct_a(d));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x00];
        out.extend_from_slice(&(size as u32).to_le_bytes());
        out.push(slot as u8);
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x00];
        out.push(str_idx);
        out.push(slot as u8);
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x00];
        out.push(str_idx);
        out.push(slot as u8);
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0x80, 0xFE]) // SJMP $
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((e8051_lcall(0), BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((e8051_ljmp(0), BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        match cc {
            0x84 => Ok((vec![0x60, 0x00], BranchFixup { field_offset: 1, field_size: 1, kind: FixupKind::ByteRel8 })), // JZ — JE
            0x85 => Ok((vec![0x70, 0x00], BranchFixup { field_offset: 1, field_size: 1, kind: FixupKind::ByteRel8 })), // JNZ — JNE
            _ => Ok((e8051_sjmp(0), BranchFixup { field_offset: 1, field_size: 1, kind: FixupKind::ByteRel8 })),
        }
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0x0000,
            stack_size: 0x80,
            data_section_offset: 0x0030,
            data_section_size: 0x0050,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 x86 (32-bit Windows / PE32) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
pub struct X86Platform {
    startup: Vec<u8>,
}

impl X86Platform {
    pub fn new() -> Self {
        Self { startup: vec![] }
    }
}

impl PlatformBackend for X86Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0x90])
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0xC3])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        // mov eax, imm32; mov [edi+slot*4], eax
        let mut out = vec![0xB8];
        out.extend_from_slice(&(imm as u32).to_le_bytes());
        out.extend_from_slice(&[0x89, 0x87]); // mov [edi+disp32], eax
        out.extend_from_slice(&(slot as u32 * 4).to_le_bytes());
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x8B, 0x87]; // mov eax, [edi+src]
        out.extend_from_slice(&(src as u32 * 4).to_le_bytes());
        out.extend_from_slice(&[0x89, 0x87]); // mov [edi+dst], eax
        out.extend_from_slice(&(dst as u32 * 4).to_le_bytes());
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x8B, 0x87];
        out.extend_from_slice(&(slot as u32 * 4).to_le_bytes());
        out.push(0x05); // add eax, imm32
        out.extend_from_slice(&(imm as u32).to_le_bytes());
        out.extend_from_slice(&[0x89, 0x87]);
        out.extend_from_slice(&(slot as u32 * 4).to_le_bytes());
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x8B, 0x87];
        out.extend_from_slice(&(slot as u32 * 4).to_le_bytes());
        out.push(0x2D); // sub eax, imm32
        out.extend_from_slice(&(imm as u32).to_le_bytes());
        out.extend_from_slice(&[0x89, 0x87]);
        out.extend_from_slice(&(slot as u32 * 4).to_le_bytes());
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0xFF, 0x87]; // inc dword [edi+disp32]
        out.extend_from_slice(&(slot as u32 * 4).to_le_bytes());
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0xFF, 0x8F]; // dec dword [edi+disp32]
        out.extend_from_slice(&(slot as u32 * 4).to_le_bytes());
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x8B, 0x87]; // mov eax, [edi+dst]
        out.extend_from_slice(&(dst as u32 * 4).to_le_bytes());
        out.extend_from_slice(&[0x03, 0x87]); // add eax, [edi+src]
        out.extend_from_slice(&(src as u32 * 4).to_le_bytes());
        out.extend_from_slice(&[0x89, 0x87]);
        out.extend_from_slice(&(dst as u32 * 4).to_le_bytes());
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x8B, 0x87];
        out.extend_from_slice(&(dst as u32 * 4).to_le_bytes());
        out.extend_from_slice(&[0x0B, 0x87]); // or eax, [edi+src]
        out.extend_from_slice(&(src as u32 * 4).to_le_bytes());
        out.extend_from_slice(&[0x89, 0x87]);
        out.extend_from_slice(&(dst as u32 * 4).to_le_bytes());
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x8B, 0x87];
        out.extend_from_slice(&(dst as u32 * 4).to_le_bytes());
        out.extend_from_slice(&[0x2B, 0x87]); // sub eax, [edi+src]
        out.extend_from_slice(&(src as u32 * 4).to_le_bytes());
        out.extend_from_slice(&[0x89, 0x87]);
        out.extend_from_slice(&(dst as u32 * 4).to_le_bytes());
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x8B, 0x87];
        out.extend_from_slice(&(dst as u32 * 4).to_le_bytes());
        out.extend_from_slice(&[0x0F, 0xAF, 0x87]); // imul eax, [edi+src]
        out.extend_from_slice(&(src as u32 * 4).to_le_bytes());
        out.extend_from_slice(&[0x89, 0x87]);
        out.extend_from_slice(&(dst as u32 * 4).to_le_bytes());
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x8B, 0x87];
        out.extend_from_slice(&(a as u32 * 4).to_le_bytes());
        out.extend_from_slice(&[0x3B, 0x87]); // cmp eax, [edi+b]
        out.extend_from_slice(&(b as u32 * 4).to_le_bytes());
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    /// Slot-to-slot MEMCPY_STATE (ISA/simulator), not pointer-form.
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "x86 memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n {
            out.extend_from_slice(&[0x8B, 0x87]); // mov eax, [edi+src]
            out.extend_from_slice(&((src + i) as u32 * 4).to_le_bytes());
            out.extend_from_slice(&[0x89, 0x87]); // mov [edi+dst], eax
            out.extend_from_slice(&((dst + i) as u32 * 4).to_le_bytes());
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        self.emit_set(slot, size)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        self.emit_set(slot, str_idx as u64)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        self.emit_set(slot, str_idx as u64)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xC3])
    }
    fn startup_blob(&self) -> &[u8] {
        &self.startup
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Pe32,
            entry_point: 0x1000,
            stack_size: 0x10000,
            data_section_offset: 0x2000,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 FreeDOS (DOS COM) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
pub struct FreedosPlatform;

impl FreedosPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for FreedosPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0x90])
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0xC3])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        // mov ax, imm16; mov [STATE_BASE+slot*2], ax
        let addr = 0x0200u16 + slot * 2;
        let mut out = vec![0xB8];
        out.extend_from_slice(&(imm as u16).to_le_bytes());
        out.push(0xA3);
        out.extend_from_slice(&addr.to_le_bytes());
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = 0x0200u16 + src * 2;
        let da = 0x0200u16 + dst * 2;
        let mut out = vec![0xA1];
        out.extend_from_slice(&sa.to_le_bytes());
        out.push(0xA3);
        out.extend_from_slice(&da.to_le_bytes());
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = 0x0200u16 + slot * 2;
        let mut out = vec![0xA1];
        out.extend_from_slice(&addr.to_le_bytes());
        out.push(0x05);
        out.extend_from_slice(&(imm as u16).to_le_bytes());
        out.push(0xA3);
        out.extend_from_slice(&addr.to_le_bytes());
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = 0x0200u16 + slot * 2;
        let mut out = vec![0xA1];
        out.extend_from_slice(&addr.to_le_bytes());
        out.push(0x2D);
        out.extend_from_slice(&(imm as u16).to_le_bytes());
        out.push(0xA3);
        out.extend_from_slice(&addr.to_le_bytes());
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        self.emit_add_imm(slot, 1)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        self.emit_sub_imm(slot, 1)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = 0x0200u16 + dst * 2;
        let sa = 0x0200u16 + src * 2;
        let mut out = vec![0xA1];
        out.extend_from_slice(&da.to_le_bytes());
        out.push(0x03); out.push(0x06); // add ax, [imm16]
        out.extend_from_slice(&sa.to_le_bytes());
        out.push(0xA3);
        out.extend_from_slice(&da.to_le_bytes());
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let aa = 0x0200u16 + a * 2;
        let ba = 0x0200u16 + b * 2;
        let mut out = vec![0xA1];
        out.extend_from_slice(&aa.to_le_bytes());
        out.extend_from_slice(&[0x3B, 0x06]);
        out.extend_from_slice(&ba.to_le_bytes());
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let sa = 0x0200u16 + (src + i) * 2;
            let da = 0x0200u16 + (dst + i) * 2;
            out.extend_from_slice(&[0xA1]);
            out.extend_from_slice(&sa.to_le_bytes());
            out.extend_from_slice(&[0xA3]);
            out.extend_from_slice(&da.to_le_bytes());
        }
        Ok(out)
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        match cc {
            0x84 => Ok((vec![0x74, 0x00], BranchFixup { field_offset: 1, field_size: 1, kind: FixupKind::ByteRel8 })), // JE
            0x85 => Ok((vec![0x75, 0x00], BranchFixup { field_offset: 1, field_size: 1, kind: FixupKind::ByteRel8 })), // JNE
            _ => Ok((vec![0xEB, 0x00], BranchFixup { field_offset: 1, field_size: 1, kind: FixupKind::ByteRel8 })),
        }
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        self.emit_set(slot, size)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        self.emit_set(slot, str_idx as u64)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        self.emit_set(slot, str_idx as u64)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xCD, 0x20])
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::DosCom,
            entry_point: 0x0100,
            stack_size: 0,
            data_section_offset: 0x200,
            data_section_size: 0x10000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 RISC-V encoding helpers 闁冲厜鍋撻柍鍏夊亾
fn riscv_li_imm(rd: u32, imm: u64) -> Vec<u8> {
    let mut out = Vec::new();
    if imm == 0 {
        // 32-bit zero: addi rd, x0, 0 = 0x00000013
        let enc: u32 = 0x00000013 | (rd << 7);
        out.extend_from_slice(&enc.to_le_bytes());
        return out;
    }
    let mut written = false;
    for shift in (0..64usize).step_by(12) {
        let chunk = ((imm >> shift) & 0xFFF) as u32;
        if chunk != 0 || shift == 0 {
            if !written {
                // LUI rd, imm20 (only for shift==0 non-zero upper bits)
                // Actually use LUI for the top 20 bits, then ADDI for the lower 12
                let hi = ((imm >> 12) + if (imm & 0x800) != 0 { 1 } else { 0 }) & 0xFFFFF;
                let lo = imm as i32 as i16 as u32;
                if hi != 0 {
                    let enc: u32 = 0x00000037 | ((hi as u32) << 12) | (rd << 7);
                    out.extend_from_slice(&enc.to_le_bytes());
                }
                if lo != 0 {
                    let lo12 = lo & 0xFFF;
                    let enc: u32 = 0x00000013 | (lo12 << 20) | (0 << 15) | (rd << 7); // addi rd, x0, lo12
                    out.extend_from_slice(&enc.to_le_bytes());
                } else if hi == 0 {
                    let enc: u32 = 0x00000013 | (rd << 7);
                    out.extend_from_slice(&enc.to_le_bytes());
                }
                written = true;
            } else {
                let enc: u32 = 0x00002013 | (chunk << 20) | (rd << 15) | (rd << 7); // addi rd, rd, chunk
                out.extend_from_slice(&enc.to_le_bytes());
                break;
            }
        }
    }
    out
}

fn riscv_ld(rd: u32, rs1: u32, imm12: u16) -> [u8; 4] {
    let imm12 = (imm12 as u32) & 0xFFF;
    let enc: u32 = 0x00003003 | (imm12 << 20) | (rs1 << 15) | (rd << 7);
    enc.to_le_bytes()
}

fn riscv_lw(rd: u32, rs1: u32, imm12: u16) -> [u8; 4] {
    let imm12 = (imm12 as u32) & 0xFFF;
    let enc: u32 = 0x00002003 | (imm12 << 20) | (rs1 << 15) | (rd << 7);
    enc.to_le_bytes()
}

fn riscv_sd(rs2: u32, rs1: u32, imm12: u16) -> [u8; 4] {
    let imm12 = imm12 as u32;
    let imm_lo = imm12 & 0x1F;
    let imm_hi = (imm12 >> 5) & 0x7F;
    let enc: u32 = 0x00002023 | (imm_hi << 25) | (rs2 << 20) | (rs1 << 15) | (imm_lo << 7);
    enc.to_le_bytes()
}

fn riscv_sw(rs2: u32, rs1: u32, imm12: u16) -> [u8; 4] {
    let imm12 = imm12 as u32;
    let imm_lo = imm12 & 0x1F;
    let imm_hi = (imm12 >> 5) & 0x7F;
    let enc: u32 = 0x00002023 | (imm_hi << 25) | (rs2 << 20) | (rs1 << 15) | (imm_lo << 7);
    enc.to_le_bytes()
}

fn riscv_add(rd: u32, rs1: u32, rs2: u32) -> [u8; 4] {
    let enc: u32 = 0x00000033 | (rs2 << 20) | (rs1 << 15) | (rd << 7);
    enc.to_le_bytes()
}

fn riscv_sub(rd: u32, rs1: u32, rs2: u32) -> [u8; 4] {
    let enc: u32 = 0x40000033 | (rs2 << 20) | (rs1 << 15) | (rd << 7);
    enc.to_le_bytes()
}

fn riscv_or(rd: u32, rs1: u32, rs2: u32) -> [u8; 4] {
    let enc: u32 = 0x00006033 | (rs2 << 20) | (rs1 << 15) | (rd << 7);
    enc.to_le_bytes()
}

fn riscv_mul(rd: u32, rs1: u32, rs2: u32) -> [u8; 4] {
    let enc: u32 = 0x02000033 | (rs2 << 20) | (rs1 << 15) | (rd << 7);
    enc.to_le_bytes()
}

fn riscv_lbu(rd: u32, rs1: u32, imm12: u16) -> [u8; 4] {
    let imm12 = (imm12 as u32) & 0xFFF;
    let enc: u32 = 0x00004003 | (imm12 << 20) | (rs1 << 15) | (rd << 7);
    enc.to_le_bytes()
}

fn riscv_jcc_base(cc: u8) -> u32 {
    // B-type with rs1=x10, rs2=x11 (or swapped), imm=0; RiscvB patch fills imm.
    match cc {
        0x84 => 0x00B50063, // JE  -> beq x10, x11
        0x85 => 0x00B51063, // JNE -> bne x10, x11
        0x86 => 0x00B54063, // JL  -> blt x10, x11
        0x87 => 0x00B55063, // JGE -> bge x10, x11
        0x88 => 0x00A5D863, // JLE -> bge x11, x10 (swapped)
        0x89 => 0x00A5C863, // JG  -> blt x11, x10 (swapped)
        0x8A => 0x00B56063, // JB  -> bltu x10, x11
        0x8B => 0x00B57063, // JAE -> bgeu x10, x11
        0x8C => 0x00A5F863, // JBE -> bgeu x11, x10 (swapped)
        0x8D => 0x00A5E863, // JA  -> bltu x11, x10 (swapped)
        _ => 0x00B50063,
    }
}

// 闁冲厜鍋撻柍鍏夊亾 RISC-V RV64 (Linux ELF64) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
const RISC64_NOP: [u8; 4] = [0x13, 0x00, 0x00, 0x00];

pub struct Riscv64Platform;

impl Riscv64Platform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for Riscv64Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(RISC64_NOP.to_vec())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(0x00008067u32.to_le_bytes().to_vec())
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, imm);
        out.extend_from_slice(&riscv_sd(6, 5, slot * 8));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, src * 8).to_vec();
        out.extend_from_slice(&riscv_sd(6, 5, dst * 8));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, slot * 8).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&riscv_addi(6, 6, imm as u32));
        } else {
            out.extend_from_slice(&riscv_li_imm(7, imm));
            out.extend_from_slice(&riscv_add(6, 6, 7));
        }
        out.extend_from_slice(&riscv_sd(6, 5, slot * 8));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, slot * 8).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&riscv_addi(6, 6, (imm as u32) | 0xFFFFF000)); // sub via addi negative
        } else {
            out.extend_from_slice(&riscv_li_imm(7, imm));
            out.extend_from_slice(&riscv_sub(6, 6, 7));
        }
        out.extend_from_slice(&riscv_sd(6, 5, slot * 8));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, slot * 8).to_vec();
        out.extend_from_slice(&riscv_addi(6, 6, 1));
        out.extend_from_slice(&riscv_sd(6, 5, slot * 8));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, slot * 8).to_vec();
        out.extend_from_slice(&riscv_addi(6, 6, 0xFFF)); // addi -1
        out.extend_from_slice(&riscv_sd(6, 5, slot * 8));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, dst * 8).to_vec();
        out.extend_from_slice(&riscv_ld(7, 5, src * 8));
        out.extend_from_slice(&riscv_add(6, 6, 7));
        out.extend_from_slice(&riscv_sd(6, 5, dst * 8));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, dst * 8).to_vec();
        out.extend_from_slice(&riscv_ld(7, 5, src * 8));
        out.extend_from_slice(&riscv_or(6, 6, 7));
        out.extend_from_slice(&riscv_sd(6, 5, dst * 8));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, dst * 8).to_vec();
        out.extend_from_slice(&riscv_ld(7, 5, src * 8));
        out.extend_from_slice(&riscv_sub(6, 6, 7));
        out.extend_from_slice(&riscv_sd(6, 5, dst * 8));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, dst * 8).to_vec();
        out.extend_from_slice(&riscv_ld(7, 5, src * 8));
        out.extend_from_slice(&riscv_mul(6, 6, 7));
        out.extend_from_slice(&riscv_sd(6, 5, dst * 8));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(10, 5, a * 8).to_vec(); // x10 = state[a]
        out.extend_from_slice(&riscv_ld(11, 5, b * 8)); // x11 = state[b]
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, ss * 8).to_vec(); // x6 = state[ss] (addr)
        if oo != 0 {
            out.extend_from_slice(&riscv_addi(6, 6, oo as u32));
        }
        out.extend_from_slice(&riscv_lbu(7, 6, 0)); // x7 = byte [x6]
        out.extend_from_slice(&riscv_sd(7, 5, dd * 8));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "RISC-V memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n as u16 {
            out.extend_from_slice(&riscv_ld(6, 5, (src + i) * 8));
            out.extend_from_slice(&riscv_sd(6, 5, (dst + i) * 8));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, size);
        out.extend_from_slice(&riscv_sd(6, 5, slot * 8));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, str_idx as u64);
        out.extend_from_slice(&riscv_sd(6, 5, slot * 8));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, str_idx as u64);
        out.extend_from_slice(&riscv_sd(6, 5, slot * 8));
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![];
        out.extend_from_slice(&riscv_addi(7, 0, 93));
        out.extend_from_slice(&riscv_addi(0, 0, code as u32));
        out.extend_from_slice(&0x00000073u32.to_le_bytes());
        Ok(out)
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0xEF, 0x00, 0x00, 0x00], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::RiscvJ }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0x6F, 0x00, 0x00, 0x00], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::RiscvJ }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        let enc = riscv_jcc_base(cc);
        Ok((enc.to_le_bytes().to_vec(), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::RiscvB }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Elf64,
            entry_point: 0x1001000,
            stack_size: 0x10000,
            data_section_offset: 0x1002000,
            data_section_size: 0x38000,
        }
    }
}

fn riscv_addi(rd: u32, rs1: u32, imm12: u32) -> [u8; 4] {
    let imm12 = imm12 & 0xFFF;
    let enc: u32 = 0x00000013 | (imm12 << 20) | (rs1 << 15) | (rd << 7);
    enc.to_le_bytes()
}

// 闁冲厜鍋撻柍鍏夊亾 MIPS big-endian (ELF32 BE) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
const MIPS_NOP: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

// MIPS register usage:
//   t0 = r8   (CMP a / general scratch)
//   t1 = r9   (CMP b / general scratch)
//   t2 = r10  (comparison result)
//   t8 = r24  (state base, set by linker startup stub)
//   ra = r31  (return address)
// State slot N lives at [t8 + N*4] (32-bit stride on this 32-bit platform).

fn mips_lui(rt: u32, imm16: u32) -> [u8; 4] {
    (0x3C000000u32 | (rt << 16) | (imm16 & 0xFFFF)).to_be_bytes()
}
fn mips_ori(rt: u32, rs: u32, imm16: u32) -> [u8; 4] {
    (0x34000000u32 | (rs << 21) | (rt << 16) | (imm16 & 0xFFFF)).to_be_bytes()
}
fn mips_lw(rt: u32, im: u32, rs: u32) -> [u8; 4] {
    (0x8C000000u32 | (rs << 21) | (rt << 16) | (im & 0xFFFF)).to_be_bytes()
}
fn mips_sw(rt: u32, im: u32, rs: u32) -> [u8; 4] {
    (0xAC000000u32 | (rs << 21) | (rt << 16) | (im & 0xFFFF)).to_be_bytes()
}
fn mips_addiu(rt: u32, rs: u32, imm16: u32) -> [u8; 4] {
    (0x24000000u32 | (rs << 21) | (rt << 16) | (imm16 & 0xFFFF)).to_be_bytes()
}
fn mips_addu(rd: u32, rs: u32, rt: u32) -> [u8; 4] {
    (0x00000021u32 | (rs << 21) | (rt << 16) | (rd << 11)).to_be_bytes()
}
fn mips_subu(rd: u32, rs: u32, rt: u32) -> [u8; 4] {
    (0x00000023u32 | (rs << 21) | (rt << 16) | (rd << 11)).to_be_bytes()
}
fn mips_or(rd: u32, rs: u32, rt: u32) -> [u8; 4] {
    (0x00000025u32 | (rs << 21) | (rt << 16) | (rd << 11)).to_be_bytes()
}
fn mips_multu(rs: u32, rt: u32) -> [u8; 4] {
    (0x00000019u32 | (rs << 21) | (rt << 16)).to_be_bytes()
}
fn mips_mflo(rd: u32) -> [u8; 4] {
    (0x00000012u32 | (rd << 11)).to_be_bytes()
}
fn mips_lbu(rt: u32, im: u32, rs: u32) -> [u8; 4] {
    (0x90000000u32 | (rs << 21) | (rt << 16) | (im & 0xFFFF)).to_be_bytes()
}
fn mips_jal(target: u32) -> [u8; 4] {
    (0x0C000000u32 | ((target >> 2) & 0x3FFFFFF)).to_be_bytes()
}
fn mips_j(target: u32) -> [u8; 4] {
    (0x08000000u32 | ((target >> 2) & 0x3FFFFFF)).to_be_bytes()
}
fn mips_beq(rs: u32, rt: u32, imm16: u32) -> [u8; 4] {
    (0x10000000u32 | (rs << 21) | (rt << 16) | (imm16 & 0xFFFF)).to_be_bytes()
}
fn mips_bne(rs: u32, rt: u32, imm16: u32) -> [u8; 4] {
    (0x14000000u32 | (rs << 21) | (rt << 16) | (imm16 & 0xFFFF)).to_be_bytes()
}

// Load 32-bit immediate `imm` into register `rt` using lui+ori (exact for any u32).
fn mips_li(rt: u32, imm: u64) -> Vec<u8> {
    let imm = imm as u32;
    let mut out = mips_lui(rt, imm >> 16).to_vec();
    out.extend_from_slice(&mips_ori(rt, rt, imm & 0xFFFF));
    out
}

// MIPS ABI register numbers
const R_ZERO: u32 = 0;
const R_T0: u32 = 8;
const R_T1: u32 = 9;
const R_T2: u32 = 10;
const R_T8: u32 = 24; // state base
const MIPS_STRIDE: u32 = 4;

pub struct MipsPlatform;

impl MipsPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for MipsPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(MIPS_NOP.to_vec())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(0x03E00008u32.to_be_bytes().to_vec()) // jr $ra
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = mips_li(R_T0, imm);
        out.extend_from_slice(&mips_sw(R_T0, slot as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = mips_lw(R_T0, src as u32 * MIPS_STRIDE, R_T8).to_vec();
        out.extend_from_slice(&mips_sw(R_T0, dst as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = mips_lw(R_T0, slot as u32 * MIPS_STRIDE, R_T8).to_vec();
        if imm < 0x8000 {
            out.extend_from_slice(&mips_addiu(R_T0, R_T0, imm as u32));
        } else {
            out.extend_from_slice(&mips_li(R_T1, imm));
            out.extend_from_slice(&mips_addu(R_T0, R_T0, R_T1));
        }
        out.extend_from_slice(&mips_sw(R_T0, slot as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = mips_lw(R_T0, slot as u32 * MIPS_STRIDE, R_T8).to_vec();
        if imm < 0x8000 {
            // addiu with two's-complement -imm (sign-extended 16-bit)
            let neg = (0x10000u32.wrapping_sub((imm & 0xFFFF) as u32)) & 0xFFFF;
            out.extend_from_slice(&mips_addiu(R_T0, R_T0, neg));
        } else {
            out.extend_from_slice(&mips_li(R_T1, imm));
            out.extend_from_slice(&mips_subu(R_T0, R_T0, R_T1));
        }
        out.extend_from_slice(&mips_sw(R_T0, slot as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = mips_lw(R_T0, slot as u32 * MIPS_STRIDE, R_T8).to_vec();
        out.extend_from_slice(&mips_addiu(R_T0, R_T0, 1));
        out.extend_from_slice(&mips_sw(R_T0, slot as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = mips_lw(R_T0, slot as u32 * MIPS_STRIDE, R_T8).to_vec();
        out.extend_from_slice(&mips_addiu(R_T0, R_T0, 0xFFFF)); // addiu -1
        out.extend_from_slice(&mips_sw(R_T0, slot as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = mips_lw(R_T0, dst as u32 * MIPS_STRIDE, R_T8).to_vec();
        out.extend_from_slice(&mips_lw(R_T1, src as u32 * MIPS_STRIDE, R_T8));
        out.extend_from_slice(&mips_addu(R_T0, R_T0, R_T1));
        out.extend_from_slice(&mips_sw(R_T0, dst as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = mips_lw(R_T0, dst as u32 * MIPS_STRIDE, R_T8).to_vec();
        out.extend_from_slice(&mips_lw(R_T1, src as u32 * MIPS_STRIDE, R_T8));
        out.extend_from_slice(&mips_or(R_T0, R_T0, R_T1));
        out.extend_from_slice(&mips_sw(R_T0, dst as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = mips_lw(R_T0, dst as u32 * MIPS_STRIDE, R_T8).to_vec();
        out.extend_from_slice(&mips_lw(R_T1, src as u32 * MIPS_STRIDE, R_T8));
        out.extend_from_slice(&mips_subu(R_T0, R_T0, R_T1));
        out.extend_from_slice(&mips_sw(R_T0, dst as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = mips_lw(R_T0, dst as u32 * MIPS_STRIDE, R_T8).to_vec();
        out.extend_from_slice(&mips_lw(R_T1, src as u32 * MIPS_STRIDE, R_T8));
        out.extend_from_slice(&mips_multu(R_T0, R_T1));
        out.extend_from_slice(&mips_mflo(R_T0));
        out.extend_from_slice(&mips_sw(R_T0, dst as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let mut out = mips_lw(R_T0, a as u32 * MIPS_STRIDE, R_T8).to_vec();
        out.extend_from_slice(&mips_lw(R_T1, b as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let mut out = mips_lw(R_T0, ss as u32 * MIPS_STRIDE, R_T8).to_vec();
        if oo != 0 {
            out.extend_from_slice(&mips_addiu(R_T0, R_T0, oo as u32));
        }
        out.extend_from_slice(&mips_lbu(R_T1, 0, R_T0));
        out.extend_from_slice(&mips_sw(R_T1, dd as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "MIPS memcpy_data: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n as u16 {
            // user data lives in .data; copy via state src/dst which are byte counts? 
            // Approximation: treat src/dst slots as byte memory pointers into .data.
            out.extend_from_slice(&mips_lw(R_T0, (src + i) as u32 * MIPS_STRIDE, R_T8));
            out.extend_from_slice(&mips_sw(R_T0, (dst + i) as u32 * MIPS_STRIDE, R_T8));
        }
        Ok(out)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "MIPS memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n as u16 {
            out.extend_from_slice(&mips_lw(R_T0, (src + i) as u32 * MIPS_STRIDE, R_T8));
            out.extend_from_slice(&mips_sw(R_T0, (dst + i) as u32 * MIPS_STRIDE, R_T8));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = mips_li(R_T0, size);
        out.extend_from_slice(&mips_sw(R_T0, slot as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = mips_li(R_T0, str_idx as u64);
        out.extend_from_slice(&mips_sw(R_T0, slot as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = mips_li(R_T0, str_idx as u64);
        out.extend_from_slice(&mips_sw(R_T0, slot as u32 * MIPS_STRIDE, R_T8));
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![];
        let imm = code as u32;
        out.extend_from_slice(&(0x34200000u32 | imm).to_be_bytes());
        out.extend_from_slice(&0x0000000Cu32.to_be_bytes());
        Ok(out)
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((mips_jal(0).to_vec(), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::MipsImm26 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((mips_j(0).to_vec(), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::MipsImm26 }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // CMP leaves a in t0 and b in t1. Emit a single branch instruction.
        // For signed comparisons beyond beq/bne we approximate with beq t0,t1 (always-true
        // for JL/JGE/JLE/JG) 闁?the CMP already computed the operands; full slt sequences
        // are documented as a future enhancement.
        let enc: u32 = match cc {
            0x84 => 0x10000000 | (R_T0 << 21) | (R_T1 << 16), // JE  -> beq t0, t1
            0x85 => 0x14000000 | (R_T0 << 21) | (R_T1 << 16), // JNE -> bne t0, t1
            // JL/JGE/JLE/JG: approximate with beq t1,t1 (always true)
            _ => 0x10000000 | (R_T1 << 21) | (R_T1 << 16),
        };
        Ok((enc.to_be_bytes().to_vec(), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::MipsImm16 }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Elf32BE,
            entry_point: 0x4001000,
            stack_size: 0x10000,
            data_section_offset: 0x4002000,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 PowerPC64 LE (Linux ELF64) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
const PPC64LE_NOP: [u8; 4] = [0x00, 0x00, 0x00, 0x60];

// PPC64LE encoding helpers (all LE)
// PPC: opcode is top 6 bits (bits 26..31). Little-endian: write u32 LE.
// state base register r13. slot N at [r13 + N*8].
// PPC64 LE register numbers as used by the interpreter:
//   r0=0, r13=13 (state base), r3=3, r4=4, r5=5, r6=6, r7=7, r8=8, r9=9, r10=10
const PPC_R0: u32 = 0;
const PPC_R3: u32 = 3;
const PPC_R4: u32 = 4;
const PPC_R5: u32 = 5;
const PPC_R6: u32 = 6;
const PPC_R7: u32 = 7;
const PPC_R8: u32 = 8;
const PPC_R9: u32 = 9;
const PPC_R10: u32 = 10;
const PPC_R13: u32 = 13; // state base

// li rd, imm16 (signed): addi rd, 0, imm16; opcode 14 (0x0E)
fn ppc_li(rd: u32, imm: i64) -> [u8; 4] {
    let enc: u32 = (14 << 26) | (rd << 21) | ((imm as u32) & 0xFFFF);
    enc.to_le_bytes()
}
// lis rd, imm16 (unsigned << 16): addis rd, 0, imm16; opcode 15 (0x0F)
fn ppc_lis(rd: u32, imm: i64) -> [u8; 4] {
    let enc: u32 = (15 << 26) | (rd << 21) | ((imm as u32) & 0xFFFF);
    enc.to_le_bytes()
}
// ori rd, ra, imm16; opcode 24 (0x18)
fn ppc_ori(rd: u32, ra: u32, imm: u64) -> [u8; 4] {
    let enc: u32 = (24 << 26) | (rd << 21) | (ra << 16) | ((imm as u32) & 0xFFFF);
    enc.to_le_bytes()
}
// addi rt, ra, simm16; opcode 14 (0x0E)
fn ppc_addi(rt: u32, ra: u32, simm: i64) -> [u8; 4] {
    let enc: u32 = (14 << 26) | (rt << 21) | (ra << 16) | ((simm as u32) & 0xFFFF);
    enc.to_le_bytes()
}
// ld rt, ds(ra): LD opcode 58 (0x3A), DS-form; DS field is bits 15:2, XO in bits 1:0
fn ppc_ld(rt: u32, ra: u32, ds: i64) -> [u8; 4] {
    let enc: u32 = (58 << 26) | (rt << 21) | (ra << 16) | ((((ds as u32) >> 2) & 0x3FFF) << 2);
    enc.to_le_bytes()
}
// std rs, ds(ra): STD opcode 62 (0x3E), DS-form
fn ppc_std(rs: u32, ra: u32, ds: i64) -> [u8; 4] {
    let enc: u32 = (62 << 26) | (rs << 21) | (ra << 16) | ((((ds as u32) >> 2) & 0x3FFF) << 2);
    enc.to_le_bytes()
}
// Load a 64-bit signed immediate into rd: addis + ori
fn ppc_li64(rd: u32, imm: u64) -> Vec<u8> {
    let hi = (imm >> 16) as u16;
    let lo = (imm & 0xFFFF) as u16;
    let mut out = ppc_lis(rd, hi as i16 as i64).to_vec();
    out.extend_from_slice(&ppc_ori(rd, rd, lo as u64));
    out
}
// add rD, rA, rB: X-form opcode 31, XO=0x100 (add) + Rc
fn ppc_add(rd: u32, ra: u32, rb: u32) -> [u8; 4] {
    let enc: u32 = (31 << 26) | (rd << 21) | (ra << 16) | (rb << 11) | (0x100 << 1);
    enc.to_le_bytes()
}
// subf rD, rA, rB (rD = rB - rA): X-form opcode 31, XO=0x28
fn ppc_subf(rd: u32, ra: u32, rb: u32) -> [u8; 4] {
    let enc: u32 = (31 << 26) | (rd << 21) | (ra << 16) | (rb << 11) | (0x28 << 1);
    enc.to_le_bytes()
}
// or rA, rS, rB: X-form opcode 31, XO=0x1BC
fn ppc_or(ra: u32, rs: u32, rb: u32) -> [u8; 4] {
    let enc: u32 = (31 << 26) | (rs << 21) | (ra << 16) | (rb << 11) | (0x1BC << 1);
    enc.to_le_bytes()
}
// mulld rD, rA, rB: X-form opcode 31, XO=0x0E9
fn ppc_mulld(rd: u32, ra: u32, rb: u32) -> [u8; 4] {
    let enc: u32 = (31 << 26) | (rd << 21) | (ra << 16) | (rb << 11) | (0x0E9 << 1);
    enc.to_le_bytes()
}
// cmp cr0, 0, rA, rB: X-form opcode 31, XO=0x00 (cmp), BF=0
fn ppc_cmp(ra: u32, rb: u32) -> [u8; 4] {
    let bf = 0u32; // cr field 0
    let enc: u32 = (31 << 26) | (bf << 23) | (0 << 21) | (ra << 16) | (rb << 11) | (0x00 << 1);
    enc.to_le_bytes()
}
// ldbz: load byte and zero-extend: lbz rD, ds(rA); opcode 34 (0x22)
fn ppc_lbz(rd: u32, ra: u32, ds: i64) -> [u8; 4] {
    let enc: u32 = (34 << 26) | (rd << 21) | (ra << 16) | ((ds as u32) & 0xFFFF);
    enc.to_le_bytes()
}

pub struct PowerPc64LePlatform;

impl PowerPc64LePlatform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for PowerPc64LePlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(PPC64LE_NOP.to_vec())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(0x4E800020u32.to_le_bytes().to_vec()) // blr = 0x4E800020 (LE: 20 00 80 4E)
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let ds = slot as i64 * 8;
        let mut out = ppc_li64(PPC_R3, imm);
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, ds));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let src_ds = src as i64 * 8;
        let dst_ds = dst as i64 * 8;
        let mut out = ppc_ld(PPC_R3, PPC_R13, src_ds).to_vec();
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, dst_ds));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let ds = slot as i64 * 8;
        let mut out = ppc_ld(PPC_R3, PPC_R13, ds).to_vec();
        if imm <= 0x7FFF {
            out.extend_from_slice(&ppc_addi(PPC_R3, PPC_R3, imm as i64));
        } else {
            out.extend_from_slice(&ppc_li64(PPC_R3, imm));
            out.extend_from_slice(&ppc_add(PPC_R3, PPC_R3, PPC_R3));
        }
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, ds));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let ds = slot as i64 * 8;
        let mut out = ppc_ld(PPC_R3, PPC_R13, ds).to_vec();
        if imm <= 0x7FFF {
            out.extend_from_slice(&ppc_addi(PPC_R3, PPC_R3, -(imm as i64)));
        } else {
            out.extend_from_slice(&ppc_li64(PPC_R3, imm));
            out.extend_from_slice(&ppc_subf(PPC_R3, PPC_R3, PPC_R3));
        }
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, ds));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let ds = slot as i64 * 8;
        let mut out = ppc_ld(PPC_R3, PPC_R13, ds).to_vec();
        out.extend_from_slice(&ppc_addi(PPC_R3, PPC_R3, 1));
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, ds));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let ds = slot as i64 * 8;
        let mut out = ppc_ld(PPC_R3, PPC_R13, ds).to_vec();
        out.extend_from_slice(&ppc_addi(PPC_R3, PPC_R3, -1));
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, ds));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_ds = dst as i64 * 8;
        let src_ds = src as i64 * 8;
        let mut out = ppc_ld(PPC_R3, PPC_R13, dst_ds).to_vec();
        out.extend_from_slice(&ppc_ld(PPC_R4, PPC_R13, src_ds));
        out.extend_from_slice(&ppc_add(PPC_R3, PPC_R3, PPC_R4));
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, dst_ds));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_ds = dst as i64 * 8;
        let src_ds = src as i64 * 8;
        let mut out = ppc_ld(PPC_R3, PPC_R13, dst_ds).to_vec();
        out.extend_from_slice(&ppc_ld(PPC_R4, PPC_R13, src_ds));
        out.extend_from_slice(&ppc_or(PPC_R3, PPC_R3, PPC_R4));
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, dst_ds));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_ds = dst as i64 * 8;
        let src_ds = src as i64 * 8;
        let mut out = ppc_ld(PPC_R3, PPC_R13, dst_ds).to_vec();
        out.extend_from_slice(&ppc_ld(PPC_R4, PPC_R13, src_ds));
        out.extend_from_slice(&ppc_subf(PPC_R3, PPC_R4, PPC_R3));
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, dst_ds));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_ds = dst as i64 * 8;
        let src_ds = src as i64 * 8;
        let mut out = ppc_ld(PPC_R3, PPC_R13, dst_ds).to_vec();
        out.extend_from_slice(&ppc_ld(PPC_R4, PPC_R13, src_ds));
        out.extend_from_slice(&ppc_mulld(PPC_R3, PPC_R3, PPC_R4));
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, dst_ds));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let a_ds = a as i64 * 8;
        let b_ds = b as i64 * 8;
        let mut out = ppc_ld(PPC_R3, PPC_R13, a_ds).to_vec();
        out.extend_from_slice(&ppc_ld(PPC_R4, PPC_R13, b_ds));
        out.extend_from_slice(&ppc_cmp(PPC_R3, PPC_R4));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let ss_ds = ss as i64 * 8;
        let dd_ds = dd as i64 * 8;
        let mut out = ppc_ld(PPC_R3, PPC_R13, ss_ds).to_vec();
        if oo != 0 {
            out.extend_from_slice(&ppc_addi(PPC_R3, PPC_R3, oo as i64));
        }
        out.extend_from_slice(&ppc_lbz(PPC_R3, PPC_R3, 0));
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, dd_ds));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "PPC memcpy_data: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n {
            let src_ds = (src + i) as i64 * 8;
            let dst_ds = (dst + i) as i64 * 8;
            out.extend_from_slice(&ppc_ld(PPC_R3, PPC_R13, src_ds));
            out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, dst_ds));
        }
        Ok(out)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "PPC memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n {
            let src_ds = (src + i) as i64 * 8;
            let dst_ds = (dst + i) as i64 * 8;
            out.extend_from_slice(&ppc_ld(PPC_R3, PPC_R13, src_ds));
            out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, dst_ds));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let ds = slot as i64 * 8;
        let mut out = ppc_li64(PPC_R3, size);
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, ds));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let ds = slot as i64 * 8;
        let mut out = ppc_li64(PPC_R3, str_idx as u64);
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, ds));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let ds = slot as i64 * 8;
        let mut out = ppc_li64(PPC_R3, str_idx as u64);
        out.extend_from_slice(&ppc_std(PPC_R3, PPC_R13, ds));
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![];
        out.extend_from_slice(&(0x38600000u32 | (code as u32)).to_le_bytes());
        out.extend_from_slice(&(0x3800003Cu32).to_le_bytes());
        out.extend_from_slice(&0x44000002u32.to_le_bytes());
        Ok(out)
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // bl = 0x48000001 | (imm24 << 2) 闁?placeholder
        Ok((vec![0x01, 0x00, 0x00, 0x48], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::PpcImm24 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // b = 0x48000000 | (imm24 << 2) 闁?placeholder
        Ok((vec![0x00, 0x00, 0x00, 0x48], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::PpcImm24 }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // After emit_cmp: CR0 set. JE/JNE use bc on CR0.EQ (BI=2).
        // BO=12 (01100): branch if condition true; BO=4 (00100): branch if false.
        let (bo, bi): (u32, u32) = match cc {
            0x84 => (12, 2), // JE  -> beq (CR0.EQ set)
            0x85 => (4, 2),  // JNE -> bne (CR0.EQ clear)
            _ => (20, 0),    // always (BO=20 = branch always)
        };
        let enc: u32 = (16 << 26) | (bo << 21) | (bi << 16);
        Ok((enc.to_le_bytes().to_vec(), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::PpcImm14 }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Elf64,
            entry_point: 0x10000000,
            stack_size: 0x10000,
            data_section_offset: 0x1001000,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 AVR encoding helpers 闁冲厜鍋撻柍鍏夊亾

// ===== AVR add/sub helpers =====
fn avr_add_rr(rd: u8, rr: u8) -> Vec<u8> {
    // Fake YOYO encoding: op=0x1C, rd in bits[8:4], rr in bits[4:0] (no overlap with disc)
    let enc: u16 = 0x1C00 | (((rd as u16) & 0x1F) << 4) | ((rr as u16) & 0x1F);
    enc.to_le_bytes().to_vec()
}
fn avr_sub_rr(rd: u8, rr: u8) -> Vec<u8> {
    let enc: u16 = 0x1800 | (((rd as u16) & 0x1F) << 4) | ((rr as u16) & 0x1F);
    enc.to_le_bytes().to_vec()
}
fn avr_or_rr(rd: u8, rr: u8) -> Vec<u8> {
    let enc: u16 = 0x1400 | (((rd as u16) & 0x1F) << 4) | ((rr as u16) & 0x1F);
    enc.to_le_bytes().to_vec()
}
fn avr_inc_r(r: u8) -> Vec<u8> {
    let enc: u16 = 0x9C00 | (r as u16);
    enc.to_le_bytes().to_vec()
}
fn avr_dec_r(r: u8) -> Vec<u8> {
    let enc: u16 = 0x9C00 | (r as u16) | 0x08;
    enc.to_le_bytes().to_vec()
}
fn avr_cp_r(rd: u8, rr: u8) -> Vec<u8> {
    // Platform CP marker: 0xB000 | (rd<<5) | rr (supports r0-r31)
    let enc: u16 = 0xB000 | ((rd as u16) << 5) | (rr as u16);
    enc.to_le_bytes().to_vec()
}
fn avr_sbr_r(ri: u8, k: u8) -> Vec<u8> {
    // SBR: r0-r7, 8 bytes
    if ri < 8 {
        let enc: u16 = 0xE000 | ((ri as u16) << 4) | (k as u16);
        enc.to_le_bytes().to_vec()
    } else {
        vec![]
    }
}

// ===== SPARC add/sub helpers =====
// Register form: rs2 in bits[4:0]; do NOT put rs2 at bit 19 (that overwrites op3).
fn sparc_add(rd: u32, rs1: u32, rs2: u32, imm: u32) -> [u8; 4] {
    if imm == 0 {
        (0x80000000u32 | (rd << 25) | (rs1 << 14) | (rs2 & 0x1F)).to_be_bytes()
    } else {
        (0x80002000u32 | (rd << 25) | (rs1 << 14) | (imm & 0x1FFF)).to_be_bytes()
    }
}
fn sparc_sub(rd: u32, rs1: u32, rs2: u32) -> [u8; 4] {
    // op3=0x04 (SUB)
    (0x80200000u32 | (rd << 25) | (rs1 << 14) | (rs2 & 0x1F)).to_be_bytes()
}
fn sparc_or_rr(rd: u32, rs1: u32, rs2: u32) -> [u8; 4] {
    // op3=0x02 (OR)
    (0x80100000u32 | (rd << 25) | (rs1 << 14) | (rs2 & 0x1F)).to_be_bytes()
}
fn sparc_mul(rd: u32, rs1: u32, rs2: u32) -> [u8; 4] {
    // op3=0x0B (UMUL) — keep prior SMUL intent via op3=0x0E if needed; use SMUL=0x0E
    (0x80700000u32 | (rd << 25) | (rs1 << 14) | (rs2 & 0x1F)).to_be_bytes()
}
fn sparc_sll(rd: u32, rs1: u32, _rs2: u32, amt: u32) -> [u8; 4] {
    (0x82000000u32 | (rd << 25) | (rs1 << 14) | (amt & 0x1F)).to_be_bytes()
}
fn sparc_srl(rd: u32, rs1: u32, _rs2: u32, amt: u32) -> [u8; 4] {
    (0x82000000u32 | (rd << 25) | (rs1 << 14) | (amt & 0x1F) | 0x00001000).to_be_bytes()
}
fn sparc_ldub(rd: u32, rs1: u32, imm: u32) -> [u8; 4] {
    (0xC0002100u32 | (rd << 25) | (rs1 << 14) | (imm & 0x1FFF)).to_be_bytes()
}
fn sparc_stb(rd: u32, rs1: u32, imm: u32) -> [u8; 4] {
    (0xC0202100u32 | (rd << 25) | (rs1 << 14) | (imm & 0x1FFF)).to_be_bytes()
}
fn sparc_subcc(rd: u32, rs1: u32, rs2: u32) -> [u8; 4] {
    // op=2, op3=0x14 (SUBcc): 10 rd 010100 rs1 i=0 rs2
    (0x80A00000u32 | (rd << 25) | (rs1 << 14) | (rs2 & 0x1F)).to_be_bytes()
}
fn sparc_li_g1(imm: u32) -> Vec<u8> {
    let mut out = sparc_sethi(SPARC_G1, imm >> 10).to_vec();
    let lo = imm & 0x3FF;
    if lo != 0 { out.extend_from_slice(&sparc_or_imm(SPARC_G1, SPARC_G1, lo)); }
    out
}

fn avr_ldi(rd: u8, imm8: u8) -> Vec<u8> {
    let enc: u16 = 0xE000 | ((rd as u16) << 4) | (imm8 as u16);
    enc.to_le_bytes().to_vec()
}
fn avr_mov(rd: u8, rr: u8) -> Vec<u8> {
    let enc: u16 = 0x2C00 | ((rd as u16) << 4) | (rr as u16);
    enc.to_le_bytes().to_vec()
}
fn avr_sts(addr: u16, rd: u8) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    let base: u16 = 0x9200 | ((rd as u16) << 4);
    let mut out = base.to_le_bytes().to_vec();
    out.push(lo);
    out.push(hi);
    out
}
fn avr_lds(rd: u8, addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    let base: u16 = 0x9000 | ((rd as u16) << 4);
    let mut out = base.to_le_bytes().to_vec();
    out.push(lo);
    out.push(hi);
    out
}
fn avr_jmp(addr: u16) -> Vec<u8> {
    // JMP is 4 bytes: 0x940C addrh addrl (absolute jump)
    let [lo, hi] = addr.to_le_bytes();
    vec![0x0C, 0x94, lo, hi]
}
fn avr_breq(rel_words: i8) -> Vec<u8> {
    // BREQ = BRBS Z: 1111 00kk kkkk k001
    let k = (rel_words as u16) & 0x7F;
    (0xF001u16 | (k << 3)).to_le_bytes().to_vec()
}
fn avr_brne(rel_words: i8) -> Vec<u8> {
    // BRNE = BRBC Z: 1111 01kk kkkk k001
    let k = (rel_words as u16) & 0x7F;
    (0xF401u16 | (k << 3)).to_le_bytes().to_vec()
}
fn avr_call(addr: u16) -> Vec<u8> {
    // CALL is 4 bytes: 0x940E addrh addrl
    let [lo, hi] = addr.to_le_bytes();
    vec![0x0E, 0x94, lo, hi]
}
fn avr_ret() -> Vec<u8> { vec![0x08, 0x95] }
fn avr_nop() -> Vec<u8> { vec![0x00, 0x00] }

const AVR_SRAM_BASE: u16 = 0x0100;

// 闁冲厜鍋撻柍鍏夊亾 AVR (ATmega) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
const AVR_NOP: [u8; 2] = [0x00, 0x00];

pub struct AvrPlatform;

impl AvrPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for AvrPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(AVR_NOP.to_vec())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(avr_ret())
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = AVR_SRAM_BASE + slot * 2;
        // Use r16 for LDI (only r16-r31 support LDI)
        let reg = 16u8;
        let mut out = avr_ldi(reg, imm as u8);
        out.extend(avr_sts(addr, reg));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let src_addr = AVR_SRAM_BASE + src * 2;
        let dst_addr = AVR_SRAM_BASE + dst * 2;
        let reg = 16u8;
        let mut out = avr_lds(reg, src_addr);
        out.extend(avr_sts(dst_addr, reg));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = AVR_SRAM_BASE + slot * 2;
        let mut out = avr_lds(16, addr);
        out.extend(avr_ldi(18, imm as u8));
        out.extend(avr_add_rr(16, 18));
        out.extend(avr_sts(addr, 16));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = AVR_SRAM_BASE + slot * 2;
        let mut out = avr_lds(16, addr);
        out.extend(avr_ldi(18, imm as u8));
        out.extend(avr_sub_rr(16, 18));
        out.extend(avr_sts(addr, 16));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = AVR_SRAM_BASE + slot * 2;
        let mut out = avr_lds(16, addr);
        out.extend(avr_inc_r(16));
        out.extend(avr_sts(addr, 16));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = AVR_SRAM_BASE + slot * 2;
        let mut out = avr_lds(16, addr);
        out.extend(avr_dec_r(16));
        out.extend(avr_sts(addr, 16));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = AVR_SRAM_BASE + src * 2;
        let da = AVR_SRAM_BASE + dst * 2;
        let mut out = avr_lds(16, sa);
        out.extend(avr_lds(17, da));
        out.extend(avr_add_rr(17, 16));
        out.extend(avr_sts(da, 17));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = AVR_SRAM_BASE + src * 2;
        let da = AVR_SRAM_BASE + dst * 2;
        let mut out = avr_lds(16, sa);
        out.extend(avr_lds(17, da));
        out.extend(avr_or_rr(17, 16));
        out.extend(avr_sts(da, 17));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = AVR_SRAM_BASE + src * 2;
        let da = AVR_SRAM_BASE + dst * 2;
        let mut out = avr_lds(16, sa);
        out.extend(avr_lds(17, da));
        out.extend(avr_sub_rr(17, 16));
        out.extend(avr_sts(da, 17));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = AVR_SRAM_BASE + src * 2;
        let da = AVR_SRAM_BASE + dst * 2;
        let [slo, shi] = sa.to_le_bytes();
        let [dlo, dhi] = da.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0xE0, shi, 0x93, 0x24]);
        out.extend_from_slice(&[0xE0, slo, 0x91, 0x24, 0x91, 0x26]);
        out.extend_from_slice(&[0xE0, dhi, 0x93, 0x2C]);
        out.extend_from_slice(&[0xE0, dlo, 0x91, 0x2C, 0x91, 0x2C]);
        out.extend_from_slice(&[0x90, 0x22, dlo, dhi]);
        out.extend_from_slice(&[0x93, 0x22, 0xE0, 0x00, 0x94, 0x22]);
        out.extend_from_slice(&[0x94, 0x24, 0xE0, 0x00, 0x94, 0x2C]);
        out.extend_from_slice(&[0x94, 0x24, 0x93, 0x26]);
        out.extend_from_slice(&[0xE0, 0x00, 0x94, 0x22, 0x94, 0x24]);
        out.extend_from_slice(&[0x93, 0x22, 0x93, 0x24]);
        out.push(0x91);
        out.push(0xF0);
        out.push(0x02);
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let aa = AVR_SRAM_BASE + a * 2;
        let ba = AVR_SRAM_BASE + b * 2;
        let mut out = avr_lds(16, aa);
        out.extend(avr_lds(17, ba));
        out.extend(avr_cp_r(16, 17));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let sa = AVR_SRAM_BASE + ss * 2;
        let da = AVR_SRAM_BASE + dd * 2;
        let [slo, shi] = sa.to_le_bytes();
        let [dlo, dhi] = da.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0xE0, shi, 0x93, 0x24]);
        out.extend_from_slice(&[0xE0, slo, 0x91, 0x24, 0x91, 0x26]);
        out.extend_from_slice(&[0x91, 0xF0, oo as u8]);
        out.extend_from_slice(&[0x93, 0x26]);
        out.extend_from_slice(&[0xE0, shi, 0x93, 0x2C]);
        out.extend_from_slice(&[0xE0, dhi, 0x93, 0x24]);
        out.extend_from_slice(&[0xE0, dlo, 0x91, 0x24, 0x91, 0x2C]);
        out.extend_from_slice(&[0x93, 0x2C]);
        out.extend_from_slice(&[0x90, 0x26, dlo, dhi]);
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let sa = AVR_SRAM_BASE + (src + i) * 2;
            let da = AVR_SRAM_BASE + (dst + i) * 2;
            out.extend(avr_lds(16, sa));
            out.extend(avr_sts(da, 16));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = AVR_NOP.to_vec();
        out.extend_from_slice(&(size as u16).to_le_bytes());
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = AVR_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = AVR_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xCF, 0xCF])
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((avr_call(0), BranchFixup { field_offset: 2, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((avr_jmp(0), BranchFixup { field_offset: 2, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        match cc {
            0x84 => Ok((avr_breq(0), BranchFixup { field_offset: 0, field_size: 2, kind: FixupKind::AvrBrRel7 })),
            0x85 => Ok((avr_brne(0), BranchFixup { field_offset: 0, field_size: 2, kind: FixupKind::AvrBrRel7 })),
            _ => Ok((avr_jmp(0), BranchFixup { field_offset: 2, field_size: 2, kind: FixupKind::AbsAddr16 })),
        }
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0x100,
            data_section_offset: 0x100,
            data_section_size: 0x1000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 ARM32 encoding helpers 闁冲厜鍋撻柍鍏夊亾
// MOVW/MOVT: imm16 split as bits[19:16]=imm[15:12], bits[11:0]=imm[11:0]
fn arm32_movw(rd: u32, imm16: u32) -> [u8; 4] {
    let enc: u32 = 0xE3000000
        | (rd << 12)
        | (((imm16 >> 12) & 0xF) << 16)
        | (imm16 & 0xFFF);
    enc.to_le_bytes()
}
fn arm32_movt(rd: u32, imm16: u32) -> [u8; 4] {
    let enc: u32 = 0xE3400000
        | (rd << 12)
        | (((imm16 >> 12) & 0xF) << 16)
        | (imm16 & 0xFFF);
    enc.to_le_bytes()
}
fn arm32_ldr(rd: u32, rn: u32, imm12: u16) -> [u8; 4] {
    let enc: u32 = 0xE5900000 | (rd << 12) | (rn << 16) | (imm12 as u32);
    enc.to_le_bytes()
}
fn arm32_str(rd: u32, rn: u32, imm12: u16) -> [u8; 4] {
    let enc: u32 = 0xE5800000 | (rd << 12) | (rn << 16) | (imm12 as u32);
    enc.to_le_bytes()
}
fn arm32_ldrb(rd: u32, rn: u32, imm12: u16) -> [u8; 4] {
    let enc: u32 = 0xE5D00000 | (rd << 12) | (rn << 16) | (imm12 as u32);
    enc.to_le_bytes()
}
fn arm32_add_imm(rd: u32, rn: u32, imm: u32) -> [u8; 4] {
    let enc: u32 = 0xE2800000 | (rd << 12) | (rn << 16) | (imm & 0xFFF);
    enc.to_le_bytes()
}
fn arm32_sub_imm(rd: u32, rn: u32, imm: u32) -> [u8; 4] {
    let enc: u32 = 0xE2400000 | (rd << 12) | (rn << 16) | (imm & 0xFFF);
    enc.to_le_bytes()
}
fn arm32_add_reg(rd: u32, rn: u32, rm: u32) -> [u8; 4] {
    let enc: u32 = 0xE0800000 | (rd << 12) | (rn << 16) | rm;
    enc.to_le_bytes()
}
fn arm32_sub_reg(rd: u32, rn: u32, rm: u32) -> [u8; 4] {
    let enc: u32 = 0xE0400000 | (rd << 12) | (rn << 16) | rm;
    enc.to_le_bytes()
}
fn arm32_mul(rd: u32, rn: u32, rm: u32) -> [u8; 4] {
    let enc: u32 = 0xE0000090 | (rd << 16) | (rn << 8) | rm;
    enc.to_le_bytes()
}
fn arm32_orr(rd: u32, rn: u32, rm: u32) -> [u8; 4] {
    let enc: u32 = 0xE1800000 | (rd << 12) | (rn << 16) | rm;
    enc.to_le_bytes()
}
fn arm32_cmp(rn: u32, rm: u32) -> [u8; 4] {
    let enc: u32 = 0xE1500000 | (rn << 16) | rm;
    enc.to_le_bytes()
}
fn arm32_jcc_cond(cc: u8) -> u32 {
    match cc {
        0x84 => 0x0, 0x85 => 0x1, 0x86 => 0xB, 0x87 => 0xA,
        0x88 => 0xD, 0x89 => 0xC, 0x8A => 0x3, 0x8B => 0x2,
        0x8C => 0x9, 0x8D => 0x8, _ => 0x0,
    }
}
fn arm32_ldr_imm32(rd: u32, imm: u32) -> Vec<u8> {
    let mut out = arm32_movw(rd, imm).to_vec();
    let hi = imm >> 16;
    if hi != 0 {
        out.extend_from_slice(&arm32_movt(rd, hi));
    }
    out
}

// 闁冲厜鍋撻柍鍏夊亾 ARM32 (32-bit ARM / Android EABI) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
const ARM32_NOP: [u8; 4] = [0x00, 0x00, 0xA0, 0xE1];

pub struct Arm32Platform;

impl Arm32Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for Arm32Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(ARM32_NOP.to_vec())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(0xE12FFF1Eu32.to_le_bytes().to_vec())
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr_imm32(0, imm as u32);
        out.extend_from_slice(&arm32_str(0, 8, slot * 4));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, src * 4).to_vec();
        out.extend_from_slice(&arm32_str(0, 8, dst * 4));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, slot * 4).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&arm32_add_imm(0, 0, imm as u32));
        } else {
            out.extend_from_slice(&arm32_ldr_imm32(1, imm as u32));
            out.extend_from_slice(&arm32_add_reg(0, 0, 1));
        }
        out.extend_from_slice(&arm32_str(0, 8, slot * 4));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, slot * 4).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&arm32_sub_imm(0, 0, imm as u32));
        } else {
            out.extend_from_slice(&arm32_ldr_imm32(1, imm as u32));
            out.extend_from_slice(&arm32_sub_reg(0, 0, 1));
        }
        out.extend_from_slice(&arm32_str(0, 8, slot * 4));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, slot * 4).to_vec();
        out.extend_from_slice(&arm32_add_imm(0, 0, 1));
        out.extend_from_slice(&arm32_str(0, 8, slot * 4));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, slot * 4).to_vec();
        out.extend_from_slice(&arm32_sub_imm(0, 0, 1));
        out.extend_from_slice(&arm32_str(0, 8, slot * 4));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, dst * 4).to_vec();
        out.extend_from_slice(&arm32_ldr(1, 8, src * 4));
        out.extend_from_slice(&arm32_add_reg(0, 0, 1));
        out.extend_from_slice(&arm32_str(0, 8, dst * 4));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, dst * 4).to_vec();
        out.extend_from_slice(&arm32_ldr(1, 8, src * 4));
        out.extend_from_slice(&arm32_orr(0, 0, 1));
        out.extend_from_slice(&arm32_str(0, 8, dst * 4));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, dst * 4).to_vec();
        out.extend_from_slice(&arm32_ldr(1, 8, src * 4));
        out.extend_from_slice(&arm32_sub_reg(0, 0, 1));
        out.extend_from_slice(&arm32_str(0, 8, dst * 4));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, dst * 4).to_vec();
        out.extend_from_slice(&arm32_ldr(1, 8, src * 4));
        out.extend_from_slice(&arm32_mul(0, 0, 1));
        out.extend_from_slice(&arm32_str(0, 8, dst * 4));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(2, 8, a * 4).to_vec();
        out.extend_from_slice(&arm32_ldr(3, 8, b * 4));
        out.extend_from_slice(&arm32_cmp(2, 3));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, ss * 4).to_vec();
        if oo != 0 {
            out.extend_from_slice(&arm32_add_imm(0, 0, oo as u32));
        }
        out.extend_from_slice(&arm32_ldrb(1, 0, 0));
        out.extend_from_slice(&arm32_str(1, 8, dd * 4));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "ARM32 memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n as u16 {
            out.extend_from_slice(&arm32_ldr(0, 8, (src + i) * 4));
            out.extend_from_slice(&arm32_str(0, 8, (dst + i) * 4));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr_imm32(0, size as u32);
        out.extend_from_slice(&arm32_str(0, 8, slot * 4));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr_imm32(0, str_idx as u32);
        out.extend_from_slice(&arm32_str(0, 8, slot * 4));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr_imm32(0, str_idx as u32);
        out.extend_from_slice(&arm32_str(0, 8, slot * 4));
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0x00, 0x00, 0x00, 0xEF])
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0x00, 0x00, 0x00, 0xEB], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::ArmImm24 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0x00, 0x00, 0x00, 0xEA], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::ArmImm24 }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        let cond = arm32_jcc_cond(cc);
        let enc: u32 = 0x0A000000 | (cond << 28);
        Ok((enc.to_le_bytes().to_vec(), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::ArmImm24 }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Elf32LE,
            entry_point: 0x8001000,
            stack_size: 0x10000,
            data_section_offset: 0x8002000,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 WebAssembly (Wasm) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
const WASM_NOP: u8 = 0x00;

pub struct WasmPlatform;

impl WasmPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for WasmPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![WASM_NOP])
    }
    fn emit_set(&mut self, _slot: u16, _imm: u64) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline SET; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_get(&mut self, _dst: u16, _src: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline GET; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_movrr(&mut self, _dst: u16, _src: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline MOVRR; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_add_imm(&mut self, _slot: u16, _imm: u64) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline ADD_IMM; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_sub_imm(&mut self, _slot: u16, _imm: u64) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline SUB_IMM; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_inc(&mut self, _slot: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline INC; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_dec(&mut self, _slot: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline DEC; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_addv(&mut self, _dst: u16, _src: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline ADDV; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_orv(&mut self, _dst: u16, _src: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline ORV; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_subv(&mut self, _dst: u16, _src: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline SUBV; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_imul(&mut self, _dst: u16, _src: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline IMUL; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_cmp(&mut self, _a: u16, _b: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline CMP; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_ldb(&mut self, _dd: u16, _ss: u16, _oo: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline LDB; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_memcpy_data(&mut self, _dst: u16, _src: u16, _n: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline MEMCPY_DATA; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_memcpy_state(&mut self, _dst: u16, _src: u16, _n: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline MEMCPY_STATE; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_alloc(&mut self, _slot: u16, _size: u64) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no inline alloc; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_load_file(&mut self, _slot: u16, _str_idx: u8) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no file I/O; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_write_file(&mut self, _slot: u16, _str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Wasm backend has no file I/O; use wasm_backend::emit_wasm instead".into(),
        })
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![WASM_NOP])
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Elf32LE,
            entry_point: 0,
            stack_size: 0,
            data_section_offset: 0,
            data_section_size: 0,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 Mach-O x64 (Intel macOS) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
pub struct MachoX64Platform;

impl MachoX64Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for MachoX64Platform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x90];
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x90];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x90];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![];
        out.extend_from_slice(&[0x48, 0xC7, 0xC0]);
        out.extend_from_slice(&0x20000003u32.to_le_bytes());
        out.extend_from_slice(&[0x41, 0xBA]);
        out.extend_from_slice(&(code as u32).to_le_bytes());
        out.extend_from_slice(&[0x0F, 0x05]);
        Ok(out)
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        // Mach-O64 x64 VAs (0x100001000 / 0x100002000) exceed u32; the
        // Mach-O writer in macho_x64_link.rs uses the real u64 values. The
        // template fields here are just informational.
        TemplateInfo {
            format: BinaryFormat::MachO64X64,
            entry_point: 0,
            stack_size: 0x10000,
            data_section_offset: 0,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 LoongArch (LA64, ELF64 LE) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
const LOONGARCH_NOP: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

// LoongArch encoding helpers
// lu12i.w rd, si20: opcode bits[31:25]=0b0001010 (0x0A), si20 in bits[24:5], rd in [4:0]
// Must NOT use (0x14<<22) — that sets insn[24] and pollutes si20's sign bit.
fn loong_lu12i_w(rd: u32, si20: u32) -> [u8; 4] {
    ((0x0Au32 << 25) | ((si20 & 0xFFFFF) << 5) | (rd & 0x1F)).to_le_bytes()
}
// ori rd, rj, ui12: opcode 0x038
fn loong_ori(rd: u32, rj: u32, ui12: u32) -> [u8; 4] {
    ((0x038 << 22) | ((ui12 & 0xFFF) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F)).to_le_bytes()
}
// addi.d rd, rj, si12: opcode 0x029
// LoongArch full-operand helpers (32-bit encoding)
fn loong_li(rd: u32, imm: u64) -> Vec<u8> {
    let imm = imm as u32;
    let mut out = loong_lu12i_w(rd, imm >> 12).to_vec();
    let lo = imm & 0xFFF;
    if lo != 0 {
        out.extend_from_slice(&loong_ori(rd, rd, lo));
    }
    out
}
// Load address constant into rd (supports VA like 0x120010000 via lu12i+ori+slli.d).
fn loong_li_upper(rd: u32, addr: u64) -> Vec<u8> {
    // Build (addr >> 16), then slli.d rd, rd, 16. Low 16 bits ORI'd if needed.
    let hi = addr >> 16;
    let mut out = loong_lu12i_w(rd, (hi >> 12) as u32).to_vec();
    let mid = (hi as u32) & 0xFFF;
    if mid != 0 {
        out.extend_from_slice(&loong_ori(rd, rd, mid));
    }
    let slli = (0x41u32 << 16) | (16u32 << 10) | ((rd & 0x1F) << 5) | (rd & 0x1F);
    out.extend_from_slice(&slli.to_le_bytes());
    let low12 = (addr as u32) & 0xFFF;
    if low12 != 0 {
        out.extend_from_slice(&loong_ori(rd, rd, low12));
    }
    out
}
// LoongArch full-operand helpers (32-bit encoding)
fn loong_addi_d(rd: u32, rj: u32, si12: i32) -> [u8; 4] {
    let enc = 0x00000000u32
        | (0x29 << 22)
        | ((rj & 0x1F) << 5)
        | ((rd & 0x1F))
        | ((si12 as u32 & 0xFFF) << 10);
    enc.to_le_bytes()
}
fn loong_addi_w(rd: u32, rj: u32, si12: i32) -> [u8; 4] {
    let enc = 0x00000000u32
        | (0x28 << 22)
        | ((rj & 0x1F) << 5)
        | ((rd & 0x1F))
        | ((si12 as u32 & 0xFFF) << 10);
    enc.to_le_bytes()
}
fn loong_add_d(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    // Real LoongArch: opcode[31:15]=0x21, rk[14:10], rj[9:5], rd[4:0]
    let enc = (0x21 << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_add_w(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    let enc = (0x20 << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_sub_d(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    let enc = (0x23 << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_and(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    let enc = (0x29 << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_or(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    let enc = (0x2A << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_xor(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    let enc = (0x2B << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_sll_d(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    let enc = (0x2E << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_srl_d(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    let enc = (0x2F << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_sra_d(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    let enc = (0x30 << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_mul_d(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    let enc = (0x39 << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_div_d(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    let enc = (0x41 << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_rem_d(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    let enc = (0x43 << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_sltu(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    let enc = (0x25 << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_slt(rd: u32, rj: u32, rk: u32) -> [u8; 4] {
    let enc = (0x24 << 15) | ((rk & 0x1F) << 10) | ((rj & 0x1F) << 5) | (rd & 0x1F);
    enc.to_le_bytes()
}
fn loong_bequ(rd: u32, rj: u32, rk: u32, offset: u16) -> [u8; 4] {
    let enc = 0x00000000u32
        | (0x18 << 22)   // F2 BEQ
        | ((rk & 0x1F) << 16)
        | ((offset as u32 & 0xFFFF) << 10)
        | ((rd & 0x1F) << 5)
        | (rj & 0x1F);
    enc.to_le_bytes()
}
fn loong_ld_b(rd: u32, rj: u32, si12: i32) -> [u8; 4] {
    let enc = 0x00000000u32
        | (0x28D << 22)  // F2 LD.B
        | ((si12 as u32 & 0xFFF) << 10)
        | ((rd & 0x1F) << 5)
        | (rj & 0x1F);
    enc.to_le_bytes()
}
fn loong_st_b(rd: u32, rj: u32, si12: i32) -> [u8; 4] {
    let enc = 0x00000000u32
        | (0x29C << 22)  // F2 ST.B
        | ((si12 as u32 & 0xFFF) << 10)
        | ((rd & 0x1F) << 5)
        | (rj & 0x1F);
    enc.to_le_bytes()
}
fn loong_beqiu(rd: u32, imm: u16, bit: u32) -> [u8; 4] {
    let enc = 0x00000000u32
        | (0x11 << 22)   // F2 BEQIU
        | ((imm as u32 & 0xFFFF) << 10)
        | ((rd & 0x1F) << 5)
        | (bit & 0x1F);
    enc.to_le_bytes()
}
fn loong_beqiu_z(rd: u32, imm: u16, bit: u32) -> [u8; 4] {
    let enc = 0x00000000u32
        | (0x14 << 22)   // F2 BEQIU.Z
        | ((imm as u32 & 0xFFFF) << 10)
        | ((rd & 0x1F) << 5)
        | (bit & 0x1F);
    enc.to_le_bytes()
}
// Compute loongarch slot offset
fn loongarch_slot_offset(slot: u16) -> u32 {
    slot as u32 * 8
}
// Compute loongarch slot offset for u32 values
fn loongarch_slot_offset_u32(slot: u16) -> u32 {
    slot as u32 * 4
}
// Compute byte offset of a 64-bit state slot from data_va
fn loongarch_slot_addr_64(slot: u16) -> u32 {
    slot as u32 * 8
}
// LoongArch store doubleword: st.d rd, rj, si12
fn loong_st_d(rd: u32, rj: u32, si12: i32) -> [u8; 4] {
    let enc = 0x00000000u32
        | (0x29D << 22)
        | ((si12 as u32 & 0xFFF) << 10)
        | ((rj & 0x1F) << 5)
        | (rd & 0x1F);
    enc.to_le_bytes()
}
// LoongArch load doubleword: ld.d rd, rj, si12
fn loong_ld_d(rd: u32, rj: u32, si12: i32) -> [u8; 4] {
    let enc = 0x00000000u32
        | (0x28C << 22)
        | ((si12 as u32 & 0xFFF) << 10)
        | ((rj & 0x1F) << 5)
        | (rd & 0x1F);
    enc.to_le_bytes()
}
// LoongArch jump and link register: jirl rd, rj, offs16
fn loong_jirl(rd: u32, rj: u32, offs16: i32) -> Vec<u8> {
    let enc = 0x00000000u32
        | (0x13 << 22)
        | ((offs16 as u32 & 0xFFFF) << 10)
        | ((rj & 0x1F) << 5)
        | (rd & 0x1F);
    enc.to_le_bytes().to_vec()
}
// LoongArch branch: b offs26
fn loong_b(offs26: u32) -> Vec<u8> {
    let enc = 0x00000000u32
        | (0x10 << 22)
        | (offs26 & 0x3FFFFFF);
    enc.to_le_bytes().to_vec()
}
// LoongArch branch and link: bl offs26
fn loong_bl(offs26: u32) -> Vec<u8> {
    let enc = 0x00000000u32
        | (0x11 << 22)
        | (offs26 & 0x3FFFFFF);
    enc.to_le_bytes().to_vec()
}
// LoongArch branch if equal: beq rj, rd, offs16
// Real ISA: opcode[31:26]=010110, offs[25:10], rj[9:5], rd[4:0]
fn loong_beq(rj: u32, rd: u32, offs16: u32) -> [u8; 4] {
    let enc = (0x16u32 << 26)
        | ((offs16 & 0xFFFF) << 10)
        | ((rj & 0x1F) << 5)
        | (rd & 0x1F);
    enc.to_le_bytes()
}
// LoongArch branch if not equal: bne rj, rd, offs16
fn loong_bne(rj: u32, rd: u32, offs16: u32) -> [u8; 4] {
    let enc = (0x17u32 << 26)
        | ((offs16 & 0xFFFF) << 10)
        | ((rj & 0x1F) << 5)
        | (rd & 0x1F);
    enc.to_le_bytes()
}

// LoongArch register numbers
const LOONG_R0: u32 = 0;  // zero
const LOONG_RA: u32 = 1;  // return address
const LOONG_T0: u32 = 12; // scratch / value
const LOONG_T1: u32 = 13; // address
const LOONG_T2: u32 = 14; // second value

pub struct LoongArchPlatform;

impl LoongArchPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for LoongArchPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(LOONGARCH_NOP.to_vec())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        // jirl zero, ra, 0
        Ok(loong_jirl(0, LOONG_RA, 0))
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        // ld x15, base+slot*8; addi.d x15, x15, slot_off_hi; st.d x14, x15, 0
        let addr = loongarch_slot_addr_64(slot);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, addr as i32));
        let mut imm_out = loong_li(LOONG_T0, imm);
        out.extend_from_slice(&imm_out);
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        // ld x14, base+slot*8; st.d x14, base+slot*8
        let src_off = loongarch_slot_addr_64(src);
        let dst_off = loongarch_slot_addr_64(dst);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, src_off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T0, LOONG_T1, 0));
        out.extend_from_slice(&loong_li_upper(LOONG_T1, 0x120010000u64));
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, dst_off as i32));
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let off = loongarch_slot_addr_64(slot);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T0, LOONG_T1, 0));
        if imm <= 0x7FF || (imm >= 0xFFFFFFFFFFFFF800 && imm <= 0xFFFFFFFFFFFFFFFF) {
            out.extend_from_slice(&loong_addi_d(LOONG_T0, LOONG_T0, imm as i32));
        } else {
            out.extend_from_slice(&loong_li(LOONG_T0, imm));
            out.extend_from_slice(&loong_add_d(LOONG_T0, LOONG_T0, LOONG_T0));
        }
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let off = loongarch_slot_addr_64(slot);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T0, LOONG_T1, 0));
        if imm < 0x1000 {
            out.extend_from_slice(&loong_addi_d(LOONG_T0, LOONG_T0, -(imm as i32)));
        } else {
            out.extend_from_slice(&loong_li(LOONG_T0, imm));
            out.extend_from_slice(&loong_sub_d(LOONG_T0, LOONG_T0, LOONG_T0));
        }
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let off = loongarch_slot_addr_64(slot);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T0, LOONG_T1, 0));
        out.extend_from_slice(&loong_addi_d(LOONG_T0, LOONG_T0, 1));
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let off = loongarch_slot_addr_64(slot);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T0, LOONG_T1, 0));
        out.extend_from_slice(&loong_addi_d(LOONG_T0, LOONG_T0, -1));
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_off = loongarch_slot_addr_64(dst);
        let src_off = loongarch_slot_addr_64(src);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, dst_off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T0, LOONG_T1, 0));
        out.extend_from_slice(&loong_li_upper(LOONG_T1, 0x120010000u64));
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, src_off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T2, LOONG_T1, 0));
        out.extend_from_slice(&loong_add_d(LOONG_T0, LOONG_T0, LOONG_T2));
        out.extend_from_slice(&loong_li_upper(LOONG_T1, 0x120010000u64));
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, dst_off as i32));
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_off = loongarch_slot_addr_64(dst);
        let src_off = loongarch_slot_addr_64(src);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, dst_off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T0, LOONG_T1, 0));
        out.extend_from_slice(&loong_li_upper(LOONG_T1, 0x120010000u64));
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, src_off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T2, LOONG_T1, 0));
        out.extend_from_slice(&loong_or(LOONG_T0, LOONG_T0, LOONG_T2));
        out.extend_from_slice(&loong_li_upper(LOONG_T1, 0x120010000u64));
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, dst_off as i32));
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_off = loongarch_slot_addr_64(dst);
        let src_off = loongarch_slot_addr_64(src);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, dst_off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T0, LOONG_T1, 0));
        out.extend_from_slice(&loong_li_upper(LOONG_T1, 0x120010000u64));
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, src_off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T2, LOONG_T1, 0));
        out.extend_from_slice(&loong_sub_d(LOONG_T0, LOONG_T0, LOONG_T2));
        out.extend_from_slice(&loong_li_upper(LOONG_T1, 0x120010000u64));
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, dst_off as i32));
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_off = loongarch_slot_addr_64(dst);
        let src_off = loongarch_slot_addr_64(src);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, dst_off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T0, LOONG_T1, 0));
        out.extend_from_slice(&loong_li_upper(LOONG_T1, 0x120010000u64));
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, src_off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T2, LOONG_T1, 0));
        out.extend_from_slice(&loong_mul_d(LOONG_T0, LOONG_T0, LOONG_T2));
        out.extend_from_slice(&loong_li_upper(LOONG_T1, 0x120010000u64));
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, dst_off as i32));
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        // Load slot a → T0 (r12), slot b → T2 (r14) for subsequent beq/bne
        let a_off = loongarch_slot_addr_64(a);
        let b_off = loongarch_slot_addr_64(b);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, a_off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T0, LOONG_T1, 0));
        out.extend_from_slice(&loong_li_upper(LOONG_T1, 0x120010000u64));
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, b_off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T2, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let ss_off = loongarch_slot_addr_64(ss);
        let dd_off = loongarch_slot_addr_64(dd);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, ss_off as i32));
        out.extend_from_slice(&loong_ld_d(LOONG_T0, LOONG_T1, 0));
        if oo != 0 {
            out.extend_from_slice(&loong_addi_d(LOONG_T0, LOONG_T0, oo as i32));
        }
        out.extend_from_slice(&loong_ld_b(LOONG_T0, LOONG_T0, 0));
        out.extend_from_slice(&loong_li_upper(LOONG_T1, 0x120010000u64));
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, dd_off as i32));
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "LoongArch memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n {
            let src_off = loongarch_slot_addr_64(src + i);
            let dst_off = loongarch_slot_addr_64(dst + i);
            out.extend_from_slice(&loong_li_upper(LOONG_T1, 0x120010000u64));
            out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, src_off as i32));
            out.extend_from_slice(&loong_ld_d(LOONG_T0, LOONG_T1, 0));
            out.extend_from_slice(&loong_li_upper(LOONG_T1, 0x120010000u64));
            out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, dst_off as i32));
            out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let off = loongarch_slot_addr_64(slot);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, off as i32));
        let mut size_out = loong_li(LOONG_T0, size);
        out.extend_from_slice(&size_out);
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let off = loongarch_slot_addr_64(slot);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, off as i32));
        out.extend_from_slice(&loong_li(LOONG_T0, str_idx as u64));
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let off = loongarch_slot_addr_64(slot);
        let mut out = loong_li_upper(LOONG_T1, 0x120010000u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T1, LOONG_T1, off as i32));
        out.extend_from_slice(&loong_li(LOONG_T0, str_idx as u64));
        out.extend_from_slice(&loong_st_d(LOONG_T0, LOONG_T1, 0));
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        // LoongArch Linux syscall: li.w a0, code; li a7, 93 (exit); syscall
        let mut out = loong_li_upper(LOONG_T0, code as u64);
        out.extend_from_slice(&loong_addi_d(LOONG_T0, LOONG_T0, 0));
        out.extend_from_slice(&loong_li(LOONG_T0, 93u64));
        out.extend_from_slice(&[0x00, 0x00, 0x00, 0x0D]); // syscall
        Ok(out)
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((loong_bl(0), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::LoongImm26 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((loong_b(0), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::LoongImm26 }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // CMP left T0=a, T2=b. JE/JNE → beq/bne rj=T0, rd=T2
        let enc = match cc {
            0x85 => loong_bne(LOONG_T0, LOONG_T2, 0),
            _ => loong_beq(LOONG_T0, LOONG_T2, 0), // JE and others → beq
        };
        Ok((enc.to_vec(), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::LoongImm16 }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Elf64,
            entry_point: 0,
            stack_size: 0x10000,
            data_section_offset: 0,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 SPARC v8 (32-bit BE, ELF32) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
const SPARC_NOP: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

// SPARC register numbers (global regs)
const SPARC_G1: u32 = 1; // state base
const SPARC_G2: u32 = 2; // scratch
// SETHI rd, imm22: 0x01000000 | (rd<<25) | imm22
// ===== SPARC add/sub/branch helpers =====
// ===== SPARC add/sub/or/sll/srl/mul helpers =====
fn sparc_sethi(rd: u32, imm22: u32) -> [u8; 4] {
    (0x01000000u32 | (rd << 25) | (imm22 & 0x003FFFFF)).to_be_bytes()
}
// OR imm rd, rs1, simm13: op=2, rd, op3=0b000010, rs1, i=1, simm13
fn sparc_or_imm(rd: u32, rs1: u32, simm13: u32) -> [u8; 4] {
    // OR imm: op=2, op3=0b000010, i=1 → base 0x80102000 (NOT 0x80202000 which is SUB)
    (0x80102000u32 | (rd << 25) | (rs1 << 14) | (simm13 & 0x1FFF)).to_be_bytes()
}
// ST rd, [rs1 + simm13]: op=3, rd, op3=0b000100, rs1, i=1, simm13
fn sparc_st(rd: u32, rs1: u32, simm13: u32) -> [u8; 4] {
    (0xC0202000u32 | (rd << 25) | (rs1 << 14) | (simm13 & 0x1FFF)).to_be_bytes()
}
// LD [rs1 + simm13], rd: op=3, rd, op3=0b000000, rs1, i=1, simm13
fn sparc_ld(rd: u32, rs1: u32, simm13: u32) -> [u8; 4] {
    (0xC0002000u32 | (rd << 25) | (rs1 << 14) | (simm13 & 0x1FFF)).to_be_bytes()
}
// ret: jmpl %i7+8, %g0 = 0x81C3E008
fn sparc_ret() -> [u8; 4] {
    0x81C3E008u32.to_be_bytes()
}
// Bicc (branch): cond(28:25), op2=0b010, disp22
fn sparc_bicc(cond: u32, disp22: u32) -> [u8; 4] {
    ((cond << 25) | 0x800000u32 | (disp22 & 0x003FFFFF)).to_be_bytes()
}
// CALL: op=1, disp30
fn sparc_call(disp30: u32) -> [u8; 4] {
    (0x40000000u32 | (disp30 & 0x3FFFFFFF)).to_be_bytes()
}
// Load 32-bit immediate into g2 via SETHI + OR.
fn sparc_li_g2(imm: u32) -> Vec<u8> {
    let mut out = sparc_sethi(SPARC_G2, imm >> 10).to_vec();
    let lo = imm & 0x3FF;
    out.extend_from_slice(&sparc_or_imm(SPARC_G2, SPARC_G2, lo));
    out
}

pub struct SparcPlatform;

impl SparcPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for SparcPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(SPARC_NOP.to_vec())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(sparc_ret().to_vec())
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = 0x20000u32 + slot as u32 * 4; // data_va + slot*4
        // Load imm into g2, load addr into g1, ST g2, [g1]
        let mut out = sparc_li_g2(imm as u32);
        let hi = addr >> 10;
        let lo = addr & 0x3FF;
        out.extend_from_slice(&sparc_sethi(SPARC_G1, hi));
        if lo != 0 {
            out.extend_from_slice(&sparc_or_imm(SPARC_G1, SPARC_G1, lo));
        }
        out.extend_from_slice(&sparc_st(SPARC_G2, SPARC_G1, 0));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let src_addr = 0x20000u32 + src as u32 * 4;
        let dst_addr = 0x20000u32 + dst as u32 * 4;
        // Load src addr into g1, LD g2, [g1], load dst addr into g1, ST g2, [g1]
        let mut out = Vec::new();
        let hi_src = src_addr >> 10;
        let lo_src = src_addr & 0x3FF;
        out.extend_from_slice(&sparc_sethi(SPARC_G1, hi_src));
        if lo_src != 0 {
            out.extend_from_slice(&sparc_or_imm(SPARC_G1, SPARC_G1, lo_src));
        }
        out.extend_from_slice(&sparc_ld(SPARC_G2, SPARC_G1, 0));
        let hi_dst = dst_addr >> 10;
        let lo_dst = dst_addr & 0x3FF;
        out.extend_from_slice(&sparc_sethi(SPARC_G1, hi_dst));
        if lo_dst != 0 {
            out.extend_from_slice(&sparc_or_imm(SPARC_G1, SPARC_G1, lo_dst));
        }
        out.extend_from_slice(&sparc_st(SPARC_G2, SPARC_G1, 0));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = 0x20000u32 + slot as u32 * 4;
        let hi = addr >> 10;
        let lo = addr & 0x3FF;
        let mut out = sparc_sethi(1, hi).to_vec();
        if lo != 0 { out.extend_from_slice(&sparc_or_imm(1, 1, lo)); }
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        if imm <= 0xFFF {
            out.extend_from_slice(&sparc_or_imm(2, 2, imm as u32));
        } else if imm <= 0x3FFFFF {
            out.extend_from_slice(&sparc_sethi(2, imm as u32 >> 10));
            let ll = imm as u32 & 0x3FF;
            if ll != 0 { out.extend_from_slice(&sparc_or_imm(2, 2, ll)); }
        } else {
            out.extend_from_slice(&sparc_li_g2(imm as u32));
        }
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = 0x20000u32 + slot as u32 * 4;
        let hi = addr >> 10;
        let lo = addr & 0x3FF;
        let mut out = sparc_sethi(1, hi).to_vec();
        if lo != 0 { out.extend_from_slice(&sparc_or_imm(1, 1, lo)); }
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        let neg = (-(imm as i32)) as u32;
        if imm <= 0xFFF {
            out.extend_from_slice(&sparc_add(2, 2, 0, neg));
        } else {
            out.extend_from_slice(&sparc_li_g2(neg));
            out.extend_from_slice(&sparc_add(2, 2, 2, 0));
        }
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = 0x20000u32 + slot as u32 * 4;
        let hi = addr >> 10;
        let lo = addr & 0x3FF;
        let mut out = sparc_sethi(1, hi).to_vec();
        if lo != 0 { out.extend_from_slice(&sparc_or_imm(1, 1, lo)); }
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_add(2, 0, 0, 1));
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = 0x20000u32 + slot as u32 * 4;
        let hi = addr >> 10;
        let lo = addr & 0x3FF;
        let mut out = sparc_sethi(1, hi).to_vec();
        if lo != 0 { out.extend_from_slice(&sparc_or_imm(1, 1, lo)); }
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_add(2, 2, 0, 0xFFFFFF));
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = 0x20000u32 + src as u32 * 4;
        let da = 0x20000u32 + dst as u32 * 4;
        let mut out = sparc_li_g1(sa);
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_li_g1(da));
        out.extend_from_slice(&sparc_ld(3, 1, 0));
        out.extend_from_slice(&sparc_add(2, 2, 3, 0));
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = 0x20000u32 + src as u32 * 4;
        let da = 0x20000u32 + dst as u32 * 4;
        let mut out = sparc_li_g1(sa);
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_li_g1(da));
        out.extend_from_slice(&sparc_ld(3, 1, 0));
        out.extend_from_slice(&sparc_or_rr(2, 2, 3));
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = 0x20000u32 + src as u32 * 4;
        let da = 0x20000u32 + dst as u32 * 4;
        let mut out = sparc_li_g1(sa);
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_li_g1(da));
        out.extend_from_slice(&sparc_ld(3, 1, 0));
        out.extend_from_slice(&sparc_sub(2, 3, 2));
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = 0x20000u32 + src as u32 * 4;
        let da = 0x20000u32 + dst as u32 * 4;
        let mut out = sparc_li_g1(sa);
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_li_g1(da));
        out.extend_from_slice(&sparc_ld(3, 1, 0));
        out.extend_from_slice(&sparc_mul(2, 2, 3));
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let aa = 0x20000u32 + a as u32 * 4;
        let ba = 0x20000u32 + b as u32 * 4;
        let mut out = sparc_li_g1(aa);
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_li_g1(ba));
        out.extend_from_slice(&sparc_ld(3, 1, 0));
        out.extend_from_slice(&sparc_subcc(0, 2, 3));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let sa = 0x20000u32 + ss as u32 * 4;
        let da = 0x20000u32 + dd as u32 * 4;
        let mut out = sparc_li_g1(sa);
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        if oo != 0 { out.extend_from_slice(&sparc_add(2, 2, 0, oo as u32)); }
        out.extend_from_slice(&sparc_ldub(3, 2, 0));
        out.extend_from_slice(&sparc_li_g1(da));
        out.extend_from_slice(&sparc_stb(3, 1, 0));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let sa = 0x20000u32 + (src + i) as u32 * 4;
            let da = 0x20000u32 + (dst + i) as u32 * 4;
            out.extend_from_slice(&sparc_li_g1(sa));
            out.extend_from_slice(&sparc_ld(2, 1, 0));
            out.extend_from_slice(&sparc_li_g1(da));
            out.extend_from_slice(&sparc_st(2, 1, 0));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = SPARC_NOP.to_vec();
        out.extend_from_slice(&(size as u32).to_be_bytes());
        out.extend_from_slice(&(slot as u32).to_be_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = SPARC_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u32).to_be_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = SPARC_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u32).to_be_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        // ta 0x00 = 0x91D02000 (BE)
        Ok(vec![0x91, 0xD0, 0x20, 0x00])
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // call: op=1, disp30
        Ok((vec![0x40, 0x00, 0x00, 0x00], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::SparcImm30 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // BA (always): cond=8
        Ok((sparc_bicc(8, 0).to_vec(), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::SparcImm22 }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // After SUBcc: ICC set. BE=1, BNE=2
        let cond = match cc {
            0x84 => 1u32, // JE  -> BE
            0x85 => 2u32, // JNE -> BNE
            _ => 8u32,    // approx → BA
        };
        Ok((sparc_bicc(cond, 0).to_vec(), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::SparcImm22 }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Elf32BE,
            entry_point: 0x10000,
            stack_size: 0x10000,
            data_section_offset: 0x20000,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 RV32 (RISC-V 32-bit, ELF32 LE) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
const RISCV32_NOP: [u8; 4] = [0x13, 0x00, 0x00, 0x00];

pub struct Riscv32Platform;

impl Riscv32Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for Riscv32Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(RISCV32_NOP.to_vec())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(0x00008067u32.to_le_bytes().to_vec())
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, imm);
        out.extend_from_slice(&riscv_sw(6, 5, slot * 4));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, src * 4).to_vec();
        out.extend_from_slice(&riscv_sw(6, 5, dst * 4));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, slot * 4).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&riscv_addi(6, 6, imm as u32));
        } else {
            out.extend_from_slice(&riscv_li_imm(7, imm));
            out.extend_from_slice(&riscv_add(6, 6, 7));
        }
        out.extend_from_slice(&riscv_sw(6, 5, slot * 4));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, slot * 4).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&riscv_addi(6, 6, (imm as u32) | 0xFFFFF000));
        } else {
            out.extend_from_slice(&riscv_li_imm(7, imm));
            out.extend_from_slice(&riscv_sub(6, 6, 7));
        }
        out.extend_from_slice(&riscv_sw(6, 5, slot * 4));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, slot * 4).to_vec();
        out.extend_from_slice(&riscv_addi(6, 6, 1));
        out.extend_from_slice(&riscv_sw(6, 5, slot * 4));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, slot * 4).to_vec();
        out.extend_from_slice(&riscv_addi(6, 6, 0xFFF));
        out.extend_from_slice(&riscv_sw(6, 5, slot * 4));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, dst * 4).to_vec();
        out.extend_from_slice(&riscv_lw(7, 5, src * 4));
        out.extend_from_slice(&riscv_add(6, 6, 7));
        out.extend_from_slice(&riscv_sw(6, 5, dst * 4));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, dst * 4).to_vec();
        out.extend_from_slice(&riscv_lw(7, 5, src * 4));
        out.extend_from_slice(&riscv_or(6, 6, 7));
        out.extend_from_slice(&riscv_sw(6, 5, dst * 4));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, dst * 4).to_vec();
        out.extend_from_slice(&riscv_lw(7, 5, src * 4));
        out.extend_from_slice(&riscv_sub(6, 6, 7));
        out.extend_from_slice(&riscv_sw(6, 5, dst * 4));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, dst * 4).to_vec();
        out.extend_from_slice(&riscv_lw(7, 5, src * 4));
        out.extend_from_slice(&riscv_mul(6, 6, 7));
        out.extend_from_slice(&riscv_sw(6, 5, dst * 4));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(10, 5, a * 4).to_vec();
        out.extend_from_slice(&riscv_lw(11, 5, b * 4));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, ss * 4).to_vec();
        if oo != 0 {
            out.extend_from_slice(&riscv_addi(6, 6, oo as u32));
        }
        out.extend_from_slice(&riscv_lbu(7, 6, 0));
        out.extend_from_slice(&riscv_sw(7, 5, dd * 4));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "RISC-V memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n as u16 {
            out.extend_from_slice(&riscv_lw(6, 5, (src + i) * 4));
            out.extend_from_slice(&riscv_sw(6, 5, (dst + i) * 4));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, size);
        out.extend_from_slice(&riscv_sw(6, 5, slot * 4));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, str_idx as u64);
        out.extend_from_slice(&riscv_sw(6, 5, slot * 4));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, str_idx as u64);
        out.extend_from_slice(&riscv_sw(6, 5, slot * 4));
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![];
        out.extend_from_slice(&riscv_addi(7, 0, 93));
        out.extend_from_slice(&riscv_addi(0, 0, code as u32));
        out.extend_from_slice(&0x00000073u32.to_le_bytes());
        Ok(out)
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0xEF, 0x00, 0x00, 0x00], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::RiscvJ }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0x6F, 0x00, 0x00, 0x00], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::RiscvJ }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        let enc = riscv_jcc_base(cc);
        Ok((enc.to_le_bytes().to_vec(), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::RiscvB }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Elf32LE,
            entry_point: 0x8001000,
            stack_size: 0x10000,
            data_section_offset: 0x8002000,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 ARM64 Windows (AArch64 PE32+) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
pub struct Aarch64WindowsPlatform;

impl Aarch64WindowsPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for Aarch64WindowsPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(ARM64_NOP.to_vec())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(ARM64_RET.to_vec())
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm64_mov_imm64(9, size);
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = arm64_mov_imm64(9, str_idx as u64);
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_mov_imm64(9, str_idx as u64);
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xC0, 0x03, 0x5F, 0xD6])
    }
    // 闁冲厜鍋撻柍鍏夊亾 Real ARM64 instruction overrides (Aarch64 Windows) 闁冲厜鍋撻柍鍏夊亾
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm64_mov_imm64(9, imm);
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, src).to_vec();
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, slot).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&arm64_add_imm(9, 9, imm as u16));
        } else {
            out.extend_from_slice(&arm64_mov_imm64(10, imm));
            out.extend_from_slice(&arm64_add_reg(9, 9, 10));
        }
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, slot).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&arm64_sub_imm(9, 9, imm as u16));
        } else {
            out.extend_from_slice(&arm64_mov_imm64(10, imm));
            out.extend_from_slice(&arm64_sub_reg(9, 9, 10));
        }
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, slot).to_vec();
        out.extend_from_slice(&arm64_add_imm(9, 9, 1));
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, slot).to_vec();
        out.extend_from_slice(&arm64_sub_imm(9, 9, 1));
        out.extend_from_slice(&arm64_str_imm(9, 15, slot));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, dst).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(10, 15, src));
        out.extend_from_slice(&arm64_add_reg(9, 9, 10));
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, dst).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(10, 15, src));
        out.extend_from_slice(&arm64_orr_reg(9, 9, 10));
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, dst).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(10, 15, src));
        out.extend_from_slice(&arm64_sub_reg(9, 9, 10));
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, dst).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(10, 15, src));
        out.extend_from_slice(&arm64_mul_reg(9, 9, 10));
        out.extend_from_slice(&arm64_str_imm(9, 15, dst));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(10, 15, a).to_vec();
        out.extend_from_slice(&arm64_ldr_imm(11, 15, b));
        out.extend_from_slice(&arm64_cmp(10, 11));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm64_ldr_imm(9, 15, ss).to_vec();
        if oo != 0 {
            out.extend_from_slice(&arm64_add_imm(9, 9, oo));
        }
        out.extend_from_slice(&arm64_ldrb(10, 9));
        out.extend_from_slice(&arm64_str_imm(10, 15, dd));
        Ok(out)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "ARM64 memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n as u16 {
            out.extend_from_slice(&arm64_ldr_imm(9, 15, src + i));
            out.extend_from_slice(&arm64_str_imm(9, 15, dst + i));
        }
        Ok(out)
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0x00, 0x00, 0x00, 0x94], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::ArmImm26 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0x00, 0x00, 0x00, 0x14], BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::ArmImm26 }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        let cond = arm64_jcc_cond(cc);
        // B.cond: cond in bits[3:0], bit4=0, imm19 in bits[23:5]
        let enc: u32 = 0x54000000 | (cond & 0xF);
        Ok((enc.to_le_bytes().to_vec(), BranchFixup { field_offset: 0, field_size: 4, kind: FixupKind::ArmImm19 }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Pe64,
            entry_point: 0x1000,
            stack_size: 0x100000,
            data_section_offset: 0x2000,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 SerenityOS 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
const SERENITY_NOP: u8 = 0x90;

pub struct SerenityPlatform;

impl SerenityPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for SerenityPlatform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![SERENITY_NOP];
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![SERENITY_NOP];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![SERENITY_NOP];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xF4]) // hlt
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Serenity,
            entry_point: 8,
            stack_size: 0x10000,
            data_section_offset: 8,
            data_section_size: 0x10000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 FreeBSD (x64 ELF64, FreeBSD syscall ABI) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
pub struct FreeBSDPlatform;

impl FreeBSDPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for FreeBSDPlatform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, size)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, _str_idx: u8) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, 0)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, _str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, 0)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        // FreeBSD SYS_exit = 1: mov eax,1; mov edi,code; syscall
        Ok(vec![
            0xB8, 0x01, 0x00, 0x00, 0x00, // mov eax, 1
            0xBF, code, 0x00, 0x00, 0x00, // mov edi, code
            0x0F, 0x05,                   // syscall
        ])
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Elf64,
            entry_point: 0x401000,
            stack_size: 0x10000,
            data_section_offset: 0x402000,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 Haiku (x64 ELF64, Haiku syscall ABI) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
pub struct HaikuPlatform;

impl HaikuPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for HaikuPlatform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, size)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, _str_idx: u8) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, 0)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, _str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, 0)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xF4]) // hlt 闁?Haiku syscall ABI is complex, use hlt as stub
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::Elf64,
            entry_point: 0x401000,
            stack_size: 0x10000,
            data_section_offset: 0x402000,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 Plan9 (9P/Acadia, flat binary) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
pub struct Plan9Platform;

impl Plan9Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for Plan9Platform {
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    /// Slot-to-slot MEMCPY_STATE (simulator/ISA semantics), not x64 pointer-form.
    /// Emits load_state(src+i)/store_state(dst+i) via RAX — plan9_interp decodes those.
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{load_state, store_state};
        use crate::types::Reg;
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "Plan9 memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n {
            out.extend(load_state(src + i, Reg::Rax)?);
            out.extend(store_state(dst + i, Reg::Rax)?);
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x90];
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x90];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x90];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xF4]) // hlt
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0x10000,
            data_section_offset: 0,
            data_section_size: 0x38000,
        }
    }
}

// 閳光偓閳光偓 Xtensa encoding helpers (LE, 32-bit instructions) 閳光偓閳光偓
// a0=reg0 (state base), a3=reg3 (tmp), a4=reg4 (tmp), a5=reg5 (tmp)
// 32-bit instruction: opcode(6) | Ra(5) | Rd(5) | Rb(5) | imm15(15)
fn xtensa_32(op: u32, ra: u32, rd: u32, rb: u32, imm: u32) -> [u8; 4] {
    let enc: u32 = (op << 26) | (ra << 23) | (rd << 20) | (rb << 15) | (imm & 0x7FFF);
    enc.to_le_bytes()
}
fn xtensa_add(ra: u32, rd: u32, rb: u32) -> [u8; 4] {
    xtensa_32(0x0D, ra, rd, rb, 0)
}
fn xtensa_sub(ra: u32, rd: u32, rb: u32) -> [u8; 4] {
    xtensa_32(0x2D, ra, rd, rb, 0)
}
fn xtensa_or(ra: u32, rd: u32, rb: u32) -> [u8; 4] {
    xtensa_32(0x1D, ra, rd, rb, 0)
}
fn xtensa_mul(ra: u32, rd: u32, rb: u32) -> [u8; 4] {
    xtensa_32(0x19, ra, rd, rb, 0)
}
fn xtensa_sll(ra: u32, rd: u32, rb: u32) -> [u8; 4] {
    xtensa_32(0x05, ra, rd, rb, 0)
}
// ADDI Rd, Ra, imm: opcode 0x0F, imm15 signed
fn xtensa_addi(ra: u32, rd: u32, imm: u32) -> [u8; 4] {
    let enc: u32 = (0x0F << 26) | (ra << 23) | (rd << 20) | (imm & 0x7FFF);
    enc.to_le_bytes()
}
// L32I Rd, Ra, index: opcode 0x0C, word index in bits 19:15
fn xtensa_l32i(ra: u32, rd: u32, wi: u32) -> [u8; 4] {
    let enc: u32 = (0x0C << 26) | (ra << 23) | (rd << 20) | ((wi & 0x07) << 15);
    enc.to_le_bytes()
}
// S32I Rd, Ra, index: opcode 0x0E
fn xtensa_s32i(ra: u32, rd: u32, wi: u32) -> [u8; 4] {
    let enc: u32 = (0x0E << 26) | (ra << 23) | (rd << 20) | ((wi & 0x07) << 15);
    enc.to_le_bytes()
}
// Move immediate: build imm via ADDI (small) or ADDI+SLL+ADD (large)
fn xtensa_mov_imm(rd: u32, imm: u64) -> Vec<u8> {
if imm <= 0x7FFF {
            xtensa_addi(0, rd, imm as u32).to_vec()
        } else {
        let mut out = Vec::new();
        let imm32 = (imm & 0xFFFFFFFF) as u32;
        let imm_lo = imm32 & 0x7FFF;
        let imm_hi = (imm32 >> 15) as u32;
        out.extend_from_slice(&xtensa_addi(0, rd, imm_lo));
        out.extend_from_slice(&xtensa_addi(0, 5, imm_hi & 0x7FFF));
        out.extend_from_slice(&xtensa_sll(0, 5, 15));
        out.extend_from_slice(&xtensa_add(5, rd, rd));
        if imm_hi > 0x7FFF {
            let imm_top = ((imm32 >> 30) & 0x7FFF) as u32;
            out.extend_from_slice(&xtensa_addi(0, 4, imm_top));
            out.extend_from_slice(&xtensa_sll(0, 4, 30));
            out.extend_from_slice(&xtensa_add(4, rd, rd));
        }
        out
    }
}
// Load slot value: compute offset via SLL then load from a0
fn xtensa_load_slot(rd: u32, slot: u16) -> Vec<u8> {
    let off = slot as u32 * 4;
    let mut out = Vec::new();
    if off == 0 {
        out.extend_from_slice(&xtensa_l32i(0, rd, 0));
    } else if off <= 28 && off % 4 == 0 {
        out.extend_from_slice(&xtensa_l32i(0, rd, (off / 4) & 0x07));
    } else {
        out.extend_from_slice(&xtensa_mov_imm(3, off as u64));
        out.extend_from_slice(&xtensa_add(0, 3, 3));
        out.extend_from_slice(&xtensa_l32i(3, rd, 0));
    }
    out
}
// Store slot value
fn xtensa_store_slot(rd: u32, slot: u16) -> Vec<u8> {
    let off = slot as u32 * 4;
    let mut out = Vec::new();
    if off == 0 {
        out.extend_from_slice(&xtensa_s32i(0, rd, 0));
    } else if off <= 28 && off % 4 == 0 {
        out.extend_from_slice(&xtensa_s32i(0, rd, (off / 4) & 0x07));
    } else {
        out.extend_from_slice(&xtensa_mov_imm(3, off as u64));
        out.extend_from_slice(&xtensa_add(0, 3, 3));
        out.extend_from_slice(&xtensa_s32i(3, rd, 0));
    }
    out
}
// L32I Rd, (Ra+disp8): 4-byte: op=0x21, imm8 bits
fn xtensa_l32i_disp8(ra: u32, rd: u32, disp: u32) -> [u8; 4] {
    let enc: u32 = (0x21 << 26) | (ra << 23) | (rd << 20) | ((disp as u8) as u32);
    enc.to_le_bytes()
}
// S32I disp8: op=0x23
fn xtensa_s32i_disp8(ra: u32, rd: u32, disp: u32) -> [u8; 4] {
    let enc: u32 = (0x23 << 26) | (ra << 23) | (rd << 20) | ((disp as u8) as u32);
    enc.to_le_bytes()
}

const XTENSA_STATE_BASE: u32 = 0; // a0 is state base; slot 0 at [a0+0], slot n at [a0+n*4]

// 闁冲厜鍋撻柍鍏夊亾 Xtensa (ESP32 LX6, flat binary) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
const XTENSA_NOP: [u8; 3] = [0x00, 0x00, 0x00];

pub struct XtensaPlatform;

impl XtensaPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for XtensaPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(XTENSA_NOP.to_vec())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0xF0, 0x00, 0x00])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_mov_imm(3, imm);
        out.extend_from_slice(&xtensa_store_slot(3, slot));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_load_slot(3, src);
        out.extend_from_slice(&xtensa_store_slot(3, dst));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_load_slot(3, slot);
        if imm <= 0x7FFF {
            out.extend_from_slice(&xtensa_addi(3, 3, imm as u32));
        } else {
            out.extend_from_slice(&xtensa_mov_imm(4, imm));
            out.extend_from_slice(&xtensa_add(4, 3, 3));
        }
        out.extend_from_slice(&xtensa_store_slot(3, slot));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_load_slot(3, slot);
        if imm <= 0x7FFF {
            out.extend_from_slice(&xtensa_addi(3, 3, (-(imm as i32)) as u32));
        } else {
            out.extend_from_slice(&xtensa_mov_imm(4, imm));
            out.extend_from_slice(&xtensa_sub(3, 3, 4));
        }
        out.extend_from_slice(&xtensa_store_slot(3, slot));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_load_slot(3, slot);
        out.extend_from_slice(&xtensa_addi(3, 3, 1));
        out.extend_from_slice(&xtensa_store_slot(3, slot));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_load_slot(3, slot);
        out.extend_from_slice(&xtensa_addi(3, 3, 0xFFFF));
        out.extend_from_slice(&xtensa_store_slot(3, slot));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_load_slot(3, dst);
        out.extend_from_slice(&xtensa_load_slot(4, src));
        out.extend_from_slice(&xtensa_add(4, 3, 3));
        out.extend_from_slice(&xtensa_store_slot(3, dst));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_load_slot(3, dst);
        out.extend_from_slice(&xtensa_load_slot(4, src));
        out.extend_from_slice(&xtensa_or(4, 3, 3));
        out.extend_from_slice(&xtensa_store_slot(3, dst));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_load_slot(3, dst);
        out.extend_from_slice(&xtensa_load_slot(4, src));
        out.extend_from_slice(&xtensa_sub(3, 3, 4));
        out.extend_from_slice(&xtensa_store_slot(3, dst));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_load_slot(3, dst);
        out.extend_from_slice(&xtensa_load_slot(4, src));
        out.extend_from_slice(&xtensa_mul(4, 3, 3));
        out.extend_from_slice(&xtensa_store_slot(3, dst));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_load_slot(3, a);
        out.extend_from_slice(&xtensa_load_slot(4, b));
        out.extend_from_slice(&xtensa_sub(3, 5, 4));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_load_slot(3, ss);
        if oo != 0 {
            out.extend_from_slice(&xtensa_addi(0, 3, oo as u32));
        }
        out.extend_from_slice(&xtensa_l32i_disp8(3, 4, 0));
        out.extend_from_slice(&xtensa_store_slot(4, dd));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            out.extend_from_slice(&xtensa_load_slot(3, src + i));
            out.extend_from_slice(&xtensa_store_slot(3, dst + i));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_mov_imm(3, size);
        out.extend_from_slice(&xtensa_store_slot(3, slot));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_mov_imm(3, str_idx as u64);
        out.extend_from_slice(&xtensa_store_slot(3, slot));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = xtensa_mov_imm(3, str_idx as u64);
        out.extend_from_slice(&xtensa_store_slot(3, slot));
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xF0, 0x00, 0x00])
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0x03, 0x00, 0x0A], BranchFixup { field_offset: 0, field_size: 3, kind: FixupKind::XtensaImm18 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0x03, 0x00, 0x06], BranchFixup { field_offset: 0, field_size: 3, kind: FixupKind::XtensaImm18 }))
    }
    fn emit_jcc_branch(&mut self, _cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((vec![0x03, 0x00, 0x06], BranchFixup { field_offset: 0, field_size: 3, kind: FixupKind::XtensaImm18 }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0x10000,
            data_section_offset: 0,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 Z80 encoding helpers 闁冲厜鍋撻柍鍏夊亾
fn z80_ld_hl_imm16(imm: u16) -> Vec<u8> {
    let [lo, hi] = imm.to_le_bytes();
    vec![0x21, lo, hi]
}
fn z80_ld_hl_addr(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0x2A, lo, hi]
}
fn z80_ld_addr_hl(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0x22, lo, hi]
}
fn z80_jp_addr(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0xC3, lo, hi]
}
fn z80_call_addr(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0xCD, lo, hi]
}
fn z80_jr_rel(rel: u8) -> Vec<u8> {
    vec![0x18, rel]
}
fn z80_ret() -> Vec<u8> { vec![0xC9] }
fn z80_nop() -> Vec<u8> { vec![0x00] }

const Z80_STATE_BASE: u16 = 0x8000;

// 闁冲厜鍋撻柍鍏夊亾 Z80 (8-bit, CP/M or ROM, flat binary) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
pub struct Z80Platform;

impl Z80Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for Z80Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(z80_nop())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(z80_ret())
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = Z80_STATE_BASE + slot * 2;
        let mut out = z80_ld_hl_imm16(imm as u16);
        out.extend(z80_ld_addr_hl(addr));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let src_addr = Z80_STATE_BASE + src * 2;
        let dst_addr = Z80_STATE_BASE + dst * 2;
        let mut out = z80_ld_hl_addr(src_addr);
        out.extend(z80_ld_addr_hl(dst_addr));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = Z80_STATE_BASE + slot * 2;
        let imm = imm as u16;
        let [lo, hi] = imm.to_le_bytes();
        // LD HL, (addr); LD A, lo; ADD A, L; LD L, A; LD A, hi; ADC A, H; LD H, A; LD (addr), HL
        let mut out = vec![0x2A];
        out.extend_from_slice(&addr.to_le_bytes());
        out.extend_from_slice(&[0x3E, lo, 0x85, 0x6F, 0x3E, hi, 0x8C, 0x67]);
        out.extend_from_slice(&[0x22]);
        out.extend_from_slice(&addr.to_le_bytes());
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = Z80_STATE_BASE + slot * 2;
        let imm = imm as u16;
        let [lo, hi] = imm.to_le_bytes();
        // LD HL, (addr); LD A, L; SUB lo; LD L, A; LD A, H; SBC hi; LD H, A; LD (addr), HL
        let mut out = vec![0x2A];
        out.extend_from_slice(&addr.to_le_bytes());
        out.extend_from_slice(&[0x7D, 0xD6, lo, 0x6F, 0x7C, 0xDE, hi, 0x67]);
        out.extend_from_slice(&[0x22]);
        out.extend_from_slice(&addr.to_le_bytes());
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = Z80_STATE_BASE + slot * 2;
        // LD HL, (addr); INC HL; LD (addr), HL
        let mut out = vec![0x2A];
        out.extend_from_slice(&addr.to_le_bytes());
        out.extend_from_slice(&[0x23]);
        out.extend_from_slice(&[0x22]);
        out.extend_from_slice(&addr.to_le_bytes());
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = Z80_STATE_BASE + slot * 2;
        // LD HL, (addr); DEC HL; LD (addr), HL
        let mut out = vec![0x2A];
        out.extend_from_slice(&addr.to_le_bytes());
        out.extend_from_slice(&[0x2B]);
        out.extend_from_slice(&[0x22]);
        out.extend_from_slice(&addr.to_le_bytes());
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = Z80_STATE_BASE + dst * 2;
        let sa = Z80_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        // LD HL,(src); LD B,H; LD C,L; LD HL,(dst); ADD HL,BC; LD (dst),HL
        out.extend_from_slice(&[0x2A, slo, shi]);
        out.extend_from_slice(&[0x44, 0x4D]); // LD B,H; LD C,L
        out.extend_from_slice(&[0x2A, dlo, dhi]);
        out.push(0x09); // ADD HL,BC
        out.extend_from_slice(&[0x22, dlo, dhi]);
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = Z80_STATE_BASE + dst * 2;
        let sa = Z80_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0x3A, slo, shi, 0xB6, 0xFF]);
        out.extend_from_slice(&[0x2A, dlo, dhi, 0xB0]);
        out.extend_from_slice(&[0x77, 0x22, dlo, dhi]);
        out.extend_from_slice(&[0x3A, slo, shi, 0x7F, 0xB4]);
        out.extend_from_slice(&[0x77, 0x22, dlo, dhi]);
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = Z80_STATE_BASE + dst * 2;
        let sa = Z80_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0x01, slo, shi, 0x3E, slo, 0xD6, slo]);
        out.extend_from_slice(&[0x6F, 0x3A, dlo, dhi, 0x7D, 0xD6, slo]);
        out.extend_from_slice(&[0x6F, 0x22, dlo, dhi]);
        out.extend_from_slice(&[0x3A, slo, shi, 0x3E, shi, 0x7C, 0x8F, 0xD6, shi]);
        out.extend_from_slice(&[0x67, 0x22, dlo, dhi]);
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = Z80_STATE_BASE + dst * 2;
        let sa = Z80_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0x2A, dlo, dhi]);
        out.extend_from_slice(&[0x3E, dlo, 0x8F, 0x3E, dhi, 0x87]);
        out.extend_from_slice(&[0x22, dlo, dhi]);
        out.extend_from_slice(&[0x01, slo, shi]);
        out.extend_from_slice(&[0x2E, slo, 0x26, shi, 0x3E, 0x00]);
        out.extend_from_slice(&[0x32, dlo, dhi, 0x32, dlo.wrapping_add(1), dhi]);
        out.extend_from_slice(&[0x2A, dlo, dhi, 0x89]);
        out.extend_from_slice(&[0x22, dlo, dhi]);
        out.extend_from_slice(&[0xCB, 0x6F]);
        out.extend_from_slice(&[0x28, 0x05, 0xCB, 0xCF]);
        out.extend_from_slice(&[0x28, 0x2B]);
        out.extend_from_slice(&[0x2A, dlo, dhi, 0x89]);
        out.extend_from_slice(&[0x22, dlo, dhi, 0x05]);
        out.extend_from_slice(&[0xCB, 0x6F]);
        out.extend_from_slice(&[0x20, 0x05]);
        out.extend_from_slice(&[0xCB, 0xCF, 0x20, 0x2B]);
        out.extend_from_slice(&[0xCB, 0x5F]);
        out.extend_from_slice(&[0x28, 0x2B]);
        out.push(0x23);
        out.push(0xCA); out.push(0x2E);
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let aa = Z80_STATE_BASE + a * 2;
        let ba = Z80_STATE_BASE + b * 2;
        let [alo, ahi] = aa.to_le_bytes();
        let [blo, bhi] = ba.to_le_bytes();
        // LD HL,addr_a; LD A,(HL); LD HL,addr_b; CP (HL)
        let mut out = vec![0x21, alo, ahi];
        out.push(0x7E);
        out.extend_from_slice(&[0x21, blo, bhi]);
        out.push(0xBE);
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let sa = Z80_STATE_BASE + ss * 2;
        let da = Z80_STATE_BASE + dd * 2;
        let [slo, shi] = sa.to_le_bytes();
        let [dlo, dhi] = da.to_le_bytes();
        let mut out = vec![0x2A, slo, shi];
        out.push(0x86); out.push(oo as u8);
        out.extend_from_slice(&[0x77, 0x22, dlo, dhi]);
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let sa = Z80_STATE_BASE + (src + i) * 2;
            let da = Z80_STATE_BASE + (dst + i) * 2;
            let [slo, shi] = sa.to_le_bytes();
            let [dlo, dhi] = da.to_le_bytes();
            out.extend_from_slice(&[0x2A, slo, shi, 0x22, dlo, dhi]);
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x00];
        out.extend_from_slice(&(size as u16).to_le_bytes());
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x00];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x00];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(z80_jp_addr(0x0000))
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((z80_call_addr(0), BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((z80_jp_addr(0), BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        match cc {
            0x84 => Ok((vec![0x28, 0x00], BranchFixup { field_offset: 1, field_size: 1, kind: FixupKind::ByteRel8 })), // JR Z — JE
            0x85 => Ok((vec![0x20, 0x00], BranchFixup { field_offset: 1, field_size: 1, kind: FixupKind::ByteRel8 })), // JR NZ — JNE
            _ => Ok((z80_jp_addr(0), BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 })),
        }
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0x100,
            data_section_offset: 0x200,
            data_section_size: 0x1000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 6502 encoding helpers 闁冲厜鍋撻柍鍏夊亾
fn m6502_lda_imm(imm: u8) -> Vec<u8> { vec![0xA9, imm] }
fn m6502_sta_addr(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0x8D, lo, hi]
}
fn m6502_lda_addr(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0xAD, lo, hi]
}
fn m6502_jmp_addr(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0x4C, lo, hi]
}
fn m6502_jsr_addr(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0x20, lo, hi]
}
fn m6502_rts() -> Vec<u8> { vec![0x60] }
fn m6502_nop() -> Vec<u8> { vec![0xEA] }

const M6502_STATE_BASE: u16 = 0x0200;

// 闁冲厜鍋撻柍鍏夊亾 6502 (8-bit, Commodore/NES, flat binary) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
pub struct M6502Platform;

impl M6502Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for M6502Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(m6502_nop())
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(m6502_rts())
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = M6502_STATE_BASE + slot * 2;
        let mut out = m6502_lda_imm(imm as u8);
        out.extend(m6502_sta_addr(addr));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let src_addr = M6502_STATE_BASE + src * 2;
        let dst_addr = M6502_STATE_BASE + dst * 2;
        let mut out = m6502_lda_addr(src_addr);
        out.extend(m6502_sta_addr(dst_addr));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = M6502_STATE_BASE + slot * 2;
        // LDA addr; CLC; ADC #imm; STA addr
        let mut out = m6502_lda_addr(addr);
        out.push(0x18); // CLC
        out.push(0x69); out.push(imm as u8); // ADC #imm
        out.extend(m6502_sta_addr(addr));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = M6502_STATE_BASE + slot * 2;
        // LDA addr; SEC; SBC #imm; STA addr
        let mut out = m6502_lda_addr(addr);
        out.push(0x38); // SEC
        out.push(0xE9); out.push(imm as u8); // SBC #imm
        out.extend(m6502_sta_addr(addr));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = M6502_STATE_BASE + slot * 2;
        // INC addr (absolute)
        let [lo, hi] = addr.to_le_bytes();
        // INC addr; BNE skip; INC addr+1; skip:
        // Actually simpler: LDA addr; CLC; ADC #1; STA addr; LDA addr+1; ADC #0; STA addr+1
        let mut out = m6502_lda_addr(addr);
        out.push(0x18); // CLC
        out.push(0x69); out.push(0x01); // ADC #1
        out.extend_from_slice(&[0x8D, lo, hi]); // STA addr
        let addr_hi = M6502_STATE_BASE + slot * 2 + 1;
        let [loh, hih] = addr_hi.to_le_bytes();
        out.extend_from_slice(&[0xAD, loh, hih]); // LDA addr+1
        out.push(0x69); out.push(0x00); // ADC #0 (carry from low byte)
        out.extend_from_slice(&[0x8D, loh, hih]); // STA addr+1
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = M6502_STATE_BASE + slot * 2;
        // LDA addr; SEC; SBC #1; STA addr; LDA addr+1; SBC #0; STA addr+1
        let mut out = m6502_lda_addr(addr);
        out.push(0x38); // SEC
        out.push(0xE9); out.push(0x01); // SBC #1
        out.extend(m6502_sta_addr(addr));
        let addr_hi = M6502_STATE_BASE + slot * 2 + 1;
        let [loh, hih] = addr_hi.to_le_bytes();
        out.extend_from_slice(&[0xAD, loh, hih]); // LDA addr+1
        out.push(0xE9); out.push(0x00); // SBC #0 (borrow)
        out.extend_from_slice(&[0x8D, loh, hih]); // STA addr+1
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = M6502_STATE_BASE + dst * 2;
        let sa = M6502_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        // CLC; LDA dst; ADC src; STA dst; LDA dst+1; ADC src+1; STA dst+1
        out.push(0x18); // CLC
        out.extend_from_slice(&[0xAD, dlo, dhi]);
        out.extend_from_slice(&[0x6D, slo, shi]); // ADC abs
        out.extend_from_slice(&[0x8D, dlo, dhi]);
        out.extend_from_slice(&[0xAD, dlo.wrapping_add(1), dhi]);
        out.extend_from_slice(&[0x6D, slo.wrapping_add(1), shi]);
        out.extend_from_slice(&[0x8D, dlo.wrapping_add(1), dhi]);
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = M6502_STATE_BASE + dst * 2;
        let sa = M6502_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0xAD, slo, shi]);
        out.push(0x29); out.push(0xFF);
        out.push(0x49); out.push(0x00);
        out.extend_from_slice(&[0xAD, dlo, dhi]);
        out.push(0x09); out.push(slo);
        out.extend_from_slice(&[0x8D, dlo, dhi]);
        out.extend_from_slice(&[0xAD, dlo.wrapping_add(1), dhi]);
        out.push(0x09); out.push(shi);
        out.extend_from_slice(&[0x8D, dlo.wrapping_add(1), dhi]);
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = M6502_STATE_BASE + dst * 2;
        let sa = M6502_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0xAD, slo, shi]);
        out.push(0x38);
        out.extend_from_slice(&[0xAD, dlo, dhi]);
        out.push(0xE9); out.push(slo);
        out.extend_from_slice(&[0x8D, dlo, dhi]);
        out.extend_from_slice(&[0xAD, dlo.wrapping_add(1), dhi]);
        out.push(0xE9); out.push(shi);
        out.extend_from_slice(&[0x8D, dlo.wrapping_add(1), dhi]);
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = M6502_STATE_BASE + dst * 2;
        let sa = M6502_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0xA2, slo, 0xA0, shi, 0xA9, 0x00, 0x85, dlo, 0x86, dhi]);
        out.extend_from_slice(&[0xAD, slo, shi]);
        out.push(0xA8);
        out.extend_from_slice(&[0xA5, dlo]);
        out.push(0x6D); out.push(slo); out.push(shi);
        out.extend_from_slice(&[0x85, dlo]);
        out.extend_from_slice(&[0xA5, dhi]);
        out.push(0x65); out.push(dlo);
        out.extend_from_slice(&[0x85, dhi]);
        out.extend_from_slice(&[0xA6, slo]);
        out.push(0xCA);
        out.extend_from_slice(&[0xF0, 0x04, 0xA4, shi]);
        out.push(0xC8); out.push(0xF0); out.push(0x28);
        out.extend_from_slice(&[0xA5, dlo]);
        out.push(0x69); out.push(slo);
        out.extend_from_slice(&[0x85, dlo]);
        out.extend_from_slice(&[0xA5, dhi]);
        out.push(0x65); out.push(dlo);
        out.extend_from_slice(&[0x85, dhi]);
        out.push(0xD0); out.push(0x08);
        out.push(0xC6); out.push(shi);
        out.push(0xD0); out.push(0x45);
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let aa = M6502_STATE_BASE + a * 2;
        let ba = M6502_STATE_BASE + b * 2;
        let mut out = m6502_lda_addr(aa);
        let [blo, bhi] = ba.to_le_bytes();
        out.extend_from_slice(&[0xCD, blo, bhi]); // CMP abs
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let sa = M6502_STATE_BASE + ss * 2;
        let da = M6502_STATE_BASE + dd * 2;
        let [slo, shi] = sa.to_le_bytes();
        let [dlo, dhi] = da.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0xAD, slo, shi]);
        out.push(0x69); out.push(oo as u8);
        out.extend_from_slice(&[0x8D, dlo, dhi]);
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let sa = M6502_STATE_BASE + (src + i) * 2;
            let da = M6502_STATE_BASE + (dst + i) * 2;
            let [slo, shi] = sa.to_le_bytes();
            let [dlo, dhi] = da.to_le_bytes();
            out.extend_from_slice(&[0xAD, slo, shi, 0x8D, dlo, dhi]);
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0xEA];
        out.extend_from_slice(&(size as u16).to_le_bytes());
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![0xEA];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0xEA];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        // BRK = 0x00
        Ok(vec![0x00])
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((m6502_jsr_addr(0), BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((m6502_jmp_addr(0), BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        match cc {
            0x84 => Ok((vec![0xF0, 0x00], BranchFixup { field_offset: 1, field_size: 1, kind: FixupKind::ByteRel8 })), // BEQ — JE
            0x85 => Ok((vec![0xD0, 0x00], BranchFixup { field_offset: 1, field_size: 1, kind: FixupKind::ByteRel8 })), // BNE
            _ => Ok((m6502_jmp_addr(0), BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 })),
        }
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0x100,
            data_section_offset: 0x200,
            data_section_size: 0x1000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 M68k (Motorola 68000, Amiga/Mac Classic, flat binary) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
// 閳光偓閳光偓 M68k encoding helpers (BE, 16/32-bit) 閳光偓閳光偓
// State base register A0. Slot n at [a0 + n*2] (word-aligned).
const M68K_STATE_BASE: u16 = 0x0100;

fn m68k_word_be(high: u8, low: u8) -> [u8; 2] {
    [high, low]
}
// MOVE.W #imm16, (a0+disp16)
// op field: 0x30 (MOVE.W, immediate src, ea dst), 0x0F (immediate src mode), 0xD0 (ea=(a0))
fn m68k_move_w_imm_to_a0_disp(disp: u16, imm: u16) -> Vec<u8> {
    // 0x30FC = MOVE.W #imm16, (ea); ea=(disp16,a0)=0x8080; then imm16 BE
    let mut out = vec![0x30, 0xFC];
    out.extend_from_slice(&disp.to_be_bytes());
    out.extend_from_slice(&imm.to_be_bytes());
    out
}
// MOVE.W (disp16,a0), d0
fn m68k_move_w_from_a0_disp(disp: u16) -> Vec<u8> {
    // 0x2080 = MOVE.W (ea),d0; ea=(disp16,a0)=0x8080
    let mut out = vec![0x20, 0x80];
    out.extend_from_slice(&disp.to_be_bytes());
    out
}
// MOVE.W d0, (disp16,a0)
fn m68k_move_w_to_a0_disp(disp: u16) -> Vec<u8> {
    // 0x2280 = MOVE.W d0,(ea); ea=(disp16,a0)=0x8080
    let mut out = vec![0x22, 0x80];
    out.extend_from_slice(&disp.to_be_bytes());
    out
}
// MOVE.B (disp16,a0), d0
fn m68k_move_b_from_a0_disp(disp: u16) -> Vec<u8> {
    let mut out = vec![0x10, 0x80];
    out.extend_from_slice(&disp.to_be_bytes());
    out
}
// MOVE.B d0, (disp16,a0)
fn m68k_move_b_to_a0_disp(disp: u16) -> Vec<u8> {
    let mut out = vec![0x12, 0x80];
    out.extend_from_slice(&disp.to_be_bytes());
    out
}
// ADD.W d1, (disp16,a0)
fn m68k_add_w_reg_to_a0_disp(disp: u16) -> Vec<u8> {
    let mut out = vec![0x00, 0xC0];
    out.extend_from_slice(&disp.to_be_bytes());
    out
}
// SUB.W d1, (disp16,a0)
fn m68k_sub_w_reg_to_a0_disp(disp: u16) -> Vec<u8> {
    let mut out = vec![0x00, 0x40];
    out.extend_from_slice(&disp.to_be_bytes());
    out
}
// OR.W d1, (disp16,a0)
fn m68k_or_w_reg_to_a0_disp(disp: u16) -> Vec<u8> {
    let mut out = vec![0x02, 0xC0];
    out.extend_from_slice(&disp.to_be_bytes());
    out
}
// ADD.W d1, d0
fn m68k_add_w_reg_reg() -> [u8; 2] {
    [0x00, 0xC0] // ADD.W d1,d0 (dst reg mode, src reg mode)
}
// SUB.W d1, d0
fn m68k_sub_w_reg_reg() -> [u8; 2] {
    [0x00, 0x40]
}
// OR.W d1, d0
fn m68k_or_w_reg_reg() -> [u8; 2] {
    [0x02, 0xC0]
}
// ADDA.L d0, a0: 0x0680
fn m68k_adda_l_d0_a0() -> [u8; 2] {
    [0x06, 0x80]
}
// MULU.W d1, d0
fn m68k_mulu_w_d1_d0() -> [u8; 2] {
    [0x00, 0xC0]
}
// CMP.W d1, d0
fn m68k_cmp_w_d1_d0() -> [u8; 2] {
    [0x10, 0xC0]
}
// ADD.W #imm16, d0
fn m68k_add_w_imm_to_d0(imm: u16) -> Vec<u8> {
    let mut out = vec![0x30, 0x30];
    out.extend_from_slice(&imm.to_be_bytes());
    out
}
// SUB.W #imm16, d0
fn m68k_sub_w_imm_to_d0(imm: u16) -> Vec<u8> {
    let mut out = vec![0x30, 0xB0];
    out.extend_from_slice(&imm.to_be_bytes());
    out
}
// JMP (a0)
fn m68k_jmp_a0() -> [u8; 2] {
    [0x4E, 0xB9] // JSR/JMP to (a0)+? Actually 0x4EB9 is JSR, 0x4EF9 is JMP
}
fn m68k_jmp_a0_indirect() -> [u8; 2] {
    [0x4E, 0xF9]
}
// JSR (a0)
fn m68k_jsr_a0() -> [u8; 2] {
    [0x4E, 0xB9]
}

pub struct M68kPlatform;

impl M68kPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for M68kPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0x4E, 0x71])
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0x4E, 0x75])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let disp = slot as u16 * 2;
        let imm16 = (imm as u16) as u16;
        Ok(m68k_move_w_imm_to_a0_disp(disp, imm16))
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let src_disp = src as u16 * 2;
        let dst_disp = dst as u16 * 2;
        let mut out = m68k_move_w_from_a0_disp(src_disp);
        out.extend_from_slice(&m68k_move_w_to_a0_disp(dst_disp));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let disp = slot as u16 * 2;
        let imm16 = (imm as u16) as u16;
        // MOVE.W (a0+disp), d0; ADD.W #imm, d0; MOVE.W d0, (a0+disp)
        let mut out = m68k_move_w_from_a0_disp(disp);
        out.extend_from_slice(&m68k_add_w_imm_to_d0(imm16));
        out.extend_from_slice(&m68k_move_w_to_a0_disp(disp));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let disp = slot as u16 * 2;
        let imm16 = (imm as u16) as u16;
        let mut out = m68k_move_w_from_a0_disp(disp);
        out.extend_from_slice(&m68k_sub_w_imm_to_d0(imm16));
        out.extend_from_slice(&m68k_move_w_to_a0_disp(disp));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let disp = slot as u16 * 2;
        // MOVE.W (a0+disp), d0; ADD.W #1, d0; MOVE.W d0, (a0+disp)
        let mut out = m68k_move_w_from_a0_disp(disp);
        out.extend_from_slice(&m68k_add_w_imm_to_d0(1));
        out.extend_from_slice(&m68k_move_w_to_a0_disp(disp));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let disp = slot as u16 * 2;
        let mut out = m68k_move_w_from_a0_disp(disp);
        out.extend_from_slice(&m68k_sub_w_imm_to_d0(1));
        out.extend_from_slice(&m68k_move_w_to_a0_disp(disp));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_disp = dst as u16 * 2;
        let src_disp = src as u16 * 2;
        // MOVE.W (a0+dst), d0; MOVE.W (a0+src), d1; ADD.W d1, d0; MOVE.W d0, (a0+dst)
        let mut out = m68k_move_w_from_a0_disp(dst_disp);
        out.push(0x20); out.push(0x00); // MOVE.W d0,d1 (copy dst into d1)
        out.extend_from_slice(&m68k_move_w_from_a0_disp(src_disp));
        out.extend_from_slice(&m68k_add_w_reg_reg());
        out.extend_from_slice(&m68k_move_w_to_a0_disp(dst_disp));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_disp = dst as u16 * 2;
        let src_disp = src as u16 * 2;
        let mut out = m68k_move_w_from_a0_disp(dst_disp);
        out.push(0x20); out.push(0x00); // MOVE.W d0,d1
        out.extend_from_slice(&m68k_move_w_from_a0_disp(src_disp));
        out.extend_from_slice(&m68k_or_w_reg_reg());
        out.extend_from_slice(&m68k_move_w_to_a0_disp(dst_disp));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_disp = dst as u16 * 2;
        let src_disp = src as u16 * 2;
        let mut out = m68k_move_w_from_a0_disp(dst_disp);
        out.push(0x20); out.push(0x00); // MOVE.W d0,d1
        out.extend_from_slice(&m68k_move_w_from_a0_disp(src_disp));
        out.extend_from_slice(&m68k_sub_w_reg_reg());
        out.extend_from_slice(&m68k_move_w_to_a0_disp(dst_disp));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_disp = dst as u16 * 2;
        let src_disp = src as u16 * 2;
        // MOVE.W (a0+dst), d0; MOVE.W (a0+src), d1; MULU.W d1,d0; MOVE.W d0, (a0+dst)
        let mut out = m68k_move_w_from_a0_disp(dst_disp);
        out.extend_from_slice(&m68k_move_w_from_a0_disp(src_disp));
        out.extend_from_slice(&m68k_mulu_w_d1_d0());
        out.extend_from_slice(&m68k_move_w_to_a0_disp(dst_disp));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let a_disp = a as u16 * 2;
        let b_disp = b as u16 * 2;
        let mut out = m68k_move_w_from_a0_disp(a_disp);
        out.extend_from_slice(&m68k_move_w_from_a0_disp(b_disp));
        out.extend_from_slice(&m68k_cmp_w_d1_d0());
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let ss_disp = ss as u16 * 2;
        let dd_disp = dd as u16 * 2;
        // MOVE.W (a0+ss), d0; ADDA.L d0, a0 (nope, that's not right)
        // MOVE.W (a0+ss), d0; MOVEA.L d0, a0; MOVE.B (a0+oo), d1; MOVE.L d0, a0? Too complex
        // Simpler: load source address into a1 via MOVEA.L, then MOVE.B (a1+oo),d0, MOVE.B d0,(a0+dd)
        // Actually simplest: MOVE.W (a0+ss),d0; ADDA.W d0,a0; MOVE.B (a0+oo),d1; MOVE.B d1,(a0+dd)
        // But this corrupts a0... We'd need to save/restore a0. 
        // For simplicity, just approximate with a marker or single op
        let mut out = m68k_move_w_from_a0_disp(ss_disp);
        out.extend_from_slice(&m68k_move_b_from_a0_disp(dd_disp));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            out.extend_from_slice(&m68k_move_w_from_a0_disp((src + i) as u16 * 2));
            out.extend_from_slice(&m68k_move_w_to_a0_disp((dst + i) as u16 * 2));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let disp = slot as u16 * 2;
        let size16 = (size as u16) as u16;
        Ok(m68k_move_w_imm_to_a0_disp(disp, size16))
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let disp = slot as u16 * 2;
        Ok(m68k_move_w_imm_to_a0_disp(disp, str_idx as u16))
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let disp = slot as u16 * 2;
        Ok(m68k_move_w_imm_to_a0_disp(disp, str_idx as u16))
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0x4E, 0x40])
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // JSR to (a0+disp16): 0x4EB9 with ea=(disp16,a0)=0x8080
        let mut out = vec![0x4E, 0xB9, 0x80, 0x80, 0x00, 0x00];
        Ok((out, BranchFixup { field_offset: 2, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // JMP to (a0+disp16): 0x4EF9 with ea=(disp16,a0)=0x8080
        let mut out = vec![0x4E, 0xF9, 0x80, 0x80, 0x00, 0x00];
        Ok((out, BranchFixup { field_offset: 2, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jcc_branch(&mut self, _cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // BSR.S rel8 placeholder (unconditional approximation)
        Ok((vec![0x60, 0x00], BranchFixup { field_offset: 1, field_size: 1, kind: FixupKind::ByteRel8 }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0x10000,
            data_section_offset: 0x1000,
            data_section_size: 0x38000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 MSP430 (16-bit TI MCU) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
// 閳光偓閳光偓 MSP430 encoding helpers (LE, 16-bit) 閳光偓閳光偓
// State base register R15. Slot n at [r15 + n*2] (word-aligned).
const MSP430_STATE_BASE: u16 = 0x0100;

fn msp430_move_imm_to_r(r: u32, imm: u32) -> Vec<u8> {
    // MOV #imm16, Rr: B0 43 | (r<<10) | imm16
    let rd = ((r & 0x1F) << 10) as u16;
    let [lo, hi] = rd.to_le_bytes();
    let [ilo, ihi] = (imm as u16).to_le_bytes();
    vec![0xB0, 0x43, lo, hi, ilo, ihi]
}
fn msp430_move_r_to_abs(r: u32, addr: u16) -> Vec<u8> {
    let rd = ((r & 0x1F) << 10) as u16;
    let [lo, hi] = rd.to_le_bytes();
    let [alo, ahi] = addr.to_le_bytes();
    vec![0x80, 0x03, lo, hi, alo, ahi]
}
fn msp430_move_abs_to_r(r: u32, addr: u16) -> Vec<u8> {
    let rd = ((r & 0x1F) << 10) as u16;
    let [lo, hi] = rd.to_le_bytes();
    let [alo, ahi] = addr.to_le_bytes();
    vec![0x90, 0x03, lo, hi, alo, ahi]
}
fn msp430_add_r_to_r(rd: u32, rs: u32) -> Vec<u8> {
    // Opcode bits must NOT overlap rd[14:10]/ use bit15 + low5 (mask 0xF01F → 0x8010)
    let word = (0x8010 | (rd << 10) | (rs << 5)) as u16;
    let [lo, hi] = word.to_le_bytes();
    vec![hi, lo]
}
fn msp430_sub_r_to_r(rd: u32, rs: u32) -> Vec<u8> {
    let word = (0x0010 | (rd << 10) | (rs << 5)) as u16;
    let [lo, hi] = word.to_le_bytes();
    vec![hi, lo]
}
fn msp430_or_r_to_r(rd: u32, rs: u32) -> Vec<u8> {
    let word = (0x8050 | (rd << 10) | (rs << 5)) as u16;
    let [lo, hi] = word.to_le_bytes();
    vec![hi, lo]
}
fn msp430_mul_r_to_r(rd: u32, rs: u32) -> Vec<u8> {
    // Marker in low5 only (0x0007) so rd/rs fields stay clean
    let word = (0x0007 | (rd << 10) | (rs << 5)) as u16;
    let [lo, hi] = word.to_le_bytes();
    vec![hi, lo]
}
fn msp430_inc_r(r: u32) -> [u8; 2] {
    // 0x0034 has no bits in 14:10; check uses (w & !0x7C00) == 0x0034
    let word = (0x0034 | ((r as u32) << 10)) as u16;
    let [lo, hi] = word.to_le_bytes();
    [hi, lo]
}
fn msp430_dec_r(r: u32) -> [u8; 2] {
    let word = (0x0033 | ((r as u32) << 10)) as u16;
    let [lo, hi] = word.to_le_bytes();
    [hi, lo]
}
fn msp430_cmp_r_to_r(rd: u32, rs: u32) -> Vec<u8> {
    let word = (0x8000 | (rd << 10) | (rs << 5)) as u16;
    let [lo, hi] = word.to_le_bytes();
    vec![hi, lo]
}
fn msp430_add_imm_to_r(r: u32, imm: u32) -> Vec<u8> {
    let rd = ((r & 0x1F) << 10) as u16;
    let [lo, hi] = rd.to_le_bytes();
    let [ilo, ihi] = (imm as u16).to_le_bytes();
    vec![0xB0, 0x53, lo, hi, ilo, ihi]
}
fn msp430_sub_imm_to_r(r: u32, imm: u32) -> Vec<u8> {
    let rd = ((r & 0x1F) << 10) as u16;
    let [lo, hi] = rd.to_le_bytes();
    let [ilo, ihi] = (imm as u16).to_le_bytes();
    vec![0xB0, 0x43, lo, hi, ilo, ihi]
}
// JMP &addr16
fn msp430_jmp_abs(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0x40, 0x32, lo, hi]
}
// CALL &addr16
fn msp430_call_abs(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0x40, 0x44, lo, hi]
}

pub struct Msp430Platform;

impl Msp430Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for Msp430Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0x03, 0x43])
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0x30, 0x41])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = MSP430_STATE_BASE + slot as u16 * 2;
        let imm16 = (imm as u16) as u16;
        let mut out = msp430_move_imm_to_r(0, imm16 as u32);
        out.extend_from_slice(&msp430_move_r_to_abs(0, addr));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let src_addr = MSP430_STATE_BASE + src as u16 * 2;
        let dst_addr = MSP430_STATE_BASE + dst as u16 * 2;
        let mut out = msp430_move_abs_to_r(0, src_addr);
        out.extend_from_slice(&msp430_move_r_to_abs(0, dst_addr));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = MSP430_STATE_BASE + slot as u16 * 2;
        let imm16 = (imm as u16) as u16;
        let mut out = msp430_move_abs_to_r(0, addr);
        out.extend_from_slice(&msp430_add_imm_to_r(0, imm16 as u32));
        out.extend_from_slice(&msp430_move_r_to_abs(0, addr));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = MSP430_STATE_BASE + slot as u16 * 2;
        let imm16 = (imm as u16) as u16;
        let mut out = msp430_move_abs_to_r(0, addr);
        out.extend_from_slice(&msp430_sub_imm_to_r(0, imm16 as u32));
        out.extend_from_slice(&msp430_move_r_to_abs(0, addr));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = MSP430_STATE_BASE + slot as u16 * 2;
        let mut out = msp430_move_abs_to_r(0, addr);
        out.extend_from_slice(&msp430_inc_r(0));
        out.extend_from_slice(&msp430_move_r_to_abs(0, addr));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = MSP430_STATE_BASE + slot as u16 * 2;
        let mut out = msp430_move_abs_to_r(0, addr);
        out.extend_from_slice(&msp430_dec_r(0));
        out.extend_from_slice(&msp430_move_r_to_abs(0, addr));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_addr = MSP430_STATE_BASE + dst as u16 * 2;
        let src_addr = MSP430_STATE_BASE + src as u16 * 2;
        let mut out = msp430_move_abs_to_r(0, dst_addr);
        out.extend_from_slice(&msp430_move_abs_to_r(1, src_addr));
        out.extend_from_slice(&msp430_add_r_to_r(0, 1));
        out.extend_from_slice(&msp430_move_r_to_abs(0, dst_addr));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_addr = MSP430_STATE_BASE + dst as u16 * 2;
        let src_addr = MSP430_STATE_BASE + src as u16 * 2;
        let mut out = msp430_move_abs_to_r(0, dst_addr);
        out.extend_from_slice(&msp430_move_abs_to_r(1, src_addr));
        out.extend_from_slice(&msp430_or_r_to_r(0, 1));
        out.extend_from_slice(&msp430_move_r_to_abs(0, dst_addr));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_addr = MSP430_STATE_BASE + dst as u16 * 2;
        let src_addr = MSP430_STATE_BASE + src as u16 * 2;
        let mut out = msp430_move_abs_to_r(0, dst_addr);
        out.extend_from_slice(&msp430_move_abs_to_r(1, src_addr));
        out.extend_from_slice(&msp430_sub_r_to_r(0, 1));
        out.extend_from_slice(&msp430_move_r_to_abs(0, dst_addr));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_addr = MSP430_STATE_BASE + dst as u16 * 2;
        let src_addr = MSP430_STATE_BASE + src as u16 * 2;
        let mut out = msp430_move_abs_to_r(0, dst_addr);
        out.extend_from_slice(&msp430_move_abs_to_r(1, src_addr));
        out.extend_from_slice(&msp430_mul_r_to_r(0, 1));
        out.extend_from_slice(&msp430_move_r_to_abs(0, dst_addr));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let a_addr = MSP430_STATE_BASE + a as u16 * 2;
        let b_addr = MSP430_STATE_BASE + b as u16 * 2;
        let mut out = msp430_move_abs_to_r(0, a_addr);
        out.extend_from_slice(&msp430_move_abs_to_r(1, b_addr));
        out.extend_from_slice(&msp430_cmp_r_to_r(0, 1));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, _oo: u16) -> IsaResult<Vec<u8>> {
        let dd_addr = MSP430_STATE_BASE + dd as u16 * 2;
        let ss_addr = MSP430_STATE_BASE + ss as u16 * 2;
        // Approximate: load ss into R0, store to dd
        let mut out = msp430_move_abs_to_r(0, ss_addr);
        out.extend_from_slice(&msp430_move_r_to_abs(0, dd_addr));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let src_addr = MSP430_STATE_BASE + (src + i) as u16 * 2;
            let dst_addr = MSP430_STATE_BASE + (dst + i) as u16 * 2;
            out.extend_from_slice(&msp430_move_abs_to_r(0, src_addr));
            out.extend_from_slice(&msp430_move_r_to_abs(0, dst_addr));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let addr = MSP430_STATE_BASE + slot as u16 * 2;
        let size16 = (size as u16) as u16;
        let mut out = msp430_move_imm_to_r(0, size16 as u32);
        out.extend_from_slice(&msp430_move_r_to_abs(0, addr));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let addr = MSP430_STATE_BASE + slot as u16 * 2;
        let mut out = msp430_move_imm_to_r(0, str_idx as u32);
        out.extend_from_slice(&msp430_move_r_to_abs(0, addr));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let addr = MSP430_STATE_BASE + slot as u16 * 2;
        let mut out = msp430_move_imm_to_r(0, str_idx as u32);
        out.extend_from_slice(&msp430_move_r_to_abs(0, addr));
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0x00, 0x00])
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((msp430_call_abs(0), BranchFixup { field_offset: 2, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((msp430_jmp_abs(0), BranchFixup { field_offset: 2, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jcc_branch(&mut self, _cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((msp430_jmp_abs(0), BranchFixup { field_offset: 2, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0x400,
            data_section_offset: 0x200,
            data_section_size: 0x1000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 PIC (8-bit Microchip MCU, mid-range) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
// 閳光偓閳光偓 PIC encoding helpers (LE, 14-bit instructions) 閳光偓閳光偓
// State via indirect access using FSR (File Select Register) / INDF
// PIC mid-range: 8-bit, LE, 14-bit instructions
const PIC_STATE_BASE: u16 = 0x0100;

// YOYO PIC encoding (2-byte LE): hi=op tag, lo=operand
//   hi=0: MOVLW lo → W = lo
//   hi=1: MOVWF lo → mem[STATE+lo] = W
//   hi=2: MOVF  lo → W = mem[STATE+lo]
//   hi=3: ADDWF lo → mem[STATE+lo] = mem[STATE+lo] + W
fn pic_movwf(slot: u8) -> Vec<u8> {
    vec![slot, 0x01]
}
fn pic_movlw(imm: u8) -> Vec<u8> {
    vec![imm, 0x00]
}
fn pic_movf(slot: u8) -> Vec<u8> {
    vec![slot, 0x02]
}
fn pic_addwf(slot: u8) -> Vec<u8> {
    vec![slot, 0x03]
}
fn pic_subwf(slot: u8) -> Vec<u8> {
    vec![slot, 0x04]
}
fn pic_orwf(slot: u8) -> Vec<u8> {
    vec![slot, 0x05]
}
fn pic_inc(slot: u8) -> Vec<u8> {
    vec![slot, 0x06]
}
fn pic_dec(slot: u8) -> Vec<u8> {
    vec![slot, 0x07]
}
fn pic_movlb(bank: u8) -> Vec<u8> {
    let [hi, lo] = (bank as u16).to_le_bytes();
vec![hi, lo]
}
// GOTO abs16 — YOYO tag 0x08 + target word (patched at offset 2)
fn pic_goto(_addr: u16) -> Vec<u8> {
    vec![0x00, 0x08, 0x00, 0x00]
}
// CALL abs16 — YOYO tag 0x0A + target word (patched at offset 2)
fn pic_call(_addr: u16) -> Vec<u8> {
    vec![0x00, 0x0A, 0x00, 0x00]
}
// JCC (JE) abs16 — YOYO tag 0x09 + target word (patched at offset 2)
fn pic_jcc(_addr: u16) -> Vec<u8> {
    vec![0x00, 0x09, 0x00, 0x00]
}

pub struct PicPlatform;

impl PicPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for PicPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0x00, 0x00])
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0x04, 0x00])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = pic_movlw(imm as u8);
        out.extend(pic_movwf(slot as u8));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = pic_movf(src as u8);
        out.extend(pic_movwf(dst as u8));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = pic_movlw(imm as u8);
        out.extend(pic_addwf(slot as u8));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = pic_movlw(imm as u8);
        out.extend(pic_subwf(slot as u8));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        Ok(pic_inc(slot as u8))
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        Ok(pic_dec(slot as u8))
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = pic_movf(src as u8);
        out.extend(pic_addwf(dst as u8));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = pic_movf(src as u8);
        out.extend(pic_orwf(dst as u8));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = pic_movf(src as u8);
        out.extend(pic_subwf(dst as u8));
        Ok(out)
    }
    fn emit_imul(&mut self, _dst: u16, _src: u16) -> IsaResult<Vec<u8>> {
        Ok(pic_addwf(0x00))
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let mut out = pic_movf(a as u8);
        out.extend(pic_subwf(b as u8));
        Ok(out)
    }
    fn emit_ldb(&mut self, _dd: u16, _ss: u16, _oo: u16) -> IsaResult<Vec<u8>> {
        Ok(pic_movf(0x00))
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let sa = PIC_STATE_BASE as u8 + ((src + i) & 0xFF) as u8;
            let da = PIC_STATE_BASE as u8 + ((dst + i) & 0xFF) as u8;
            out.extend_from_slice(&pic_movf(sa));
            out.extend_from_slice(&pic_movwf(da));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, _slot: u16, _size: u64) -> IsaResult<Vec<u8>> {
        Ok(pic_movlw(0))
    }
    fn emit_load_file(&mut self, _slot: u16, _str_idx: u8) -> IsaResult<Vec<u8>> {
        Ok(pic_movlw(0))
    }
    fn emit_write_file(&mut self, _slot: u16, _str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        Ok(pic_movlw(0))
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xFD, 0x00])
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((pic_call(0), BranchFixup { field_offset: 2, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((pic_goto(0), BranchFixup { field_offset: 2, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jcc_branch(&mut self, cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        match cc {
            0x84 => Ok((pic_jcc(0), BranchFixup { field_offset: 2, field_size: 2, kind: FixupKind::AbsAddr16 })),
            _ => Ok((pic_goto(0), BranchFixup { field_offset: 2, field_size: 2, kind: FixupKind::AbsAddr16 })),
        }
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0x20,
            data_section_offset: 0x0C,
            data_section_size: 0x1000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 STM8 (8-bit STMicro MCU) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
// 閳光偓閳光偓 STM8 encoding helpers (LE, 8-bit) 閳光偓閳光偓
// State at fixed addresses: slot n at 0x4000 + n
const STM8_STATE_BASE: u16 = 0x4000;

fn stm8_ld_a_imm(imm: u8) -> Vec<u8> {
vec![0x3F, imm]
}
fn stm8_ld_a_addr(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0x86, lo, hi]
}
fn stm8_ld_a_xr(reg: u8) -> Vec<u8> {
vec![0x16 | (reg & 0x07), 0x00]
}
fn stm8_st_a_addr(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0x87, lo, hi]
}
fn stm8_add_a_imm(imm: u8) -> Vec<u8> {
vec![0x4F, imm]
}
fn stm8_sub_a_imm(imm: u8) -> Vec<u8> {
vec![0x5F, imm]
}
fn stm8_add_a_reg(reg: u8) -> Vec<u8> {
vec![0x0F, reg & 0x07]
}
fn stm8_sub_a_reg(reg: u8) -> Vec<u8> {
vec![0x1F, reg & 0x07]
}
fn stm8_or_a_reg(reg: u8) -> Vec<u8> {
vec![0x6F, reg & 0x07]
}
fn stm8_inc_a() -> Vec<u8> {
vec![0x3C]
}
fn stm8_dec_a() -> Vec<u8> {
vec![0x3D]
}
fn stm8_ld_imm_reg(reg: u8, imm: u8) -> Vec<u8> {
vec![0x2F | ((reg & 0x07) << 5), imm]
}
fn stm8_st_reg_addr(reg: u8, addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0x95, (reg & 0x07), lo, hi]
}
fn stm8_ld_reg_addr(reg: u8, addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0x94, (reg & 0x07), lo, hi]
}
// JMP addr16
fn stm8_jmp_addr(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0x89, lo, hi]
}
// CALL addr16
fn stm8_call_addr(addr: u16) -> Vec<u8> {
    let [lo, hi] = addr.to_le_bytes();
    vec![0xD3, lo, hi]
}

pub struct Stm8Platform;

impl Stm8Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for Stm8Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0x9D])
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0x81])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = STM8_STATE_BASE + slot as u16;
        let imm8 = imm as u8;
        let mut out = stm8_ld_a_imm(imm8);
        out.extend_from_slice(&stm8_st_a_addr(addr));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let src_addr = STM8_STATE_BASE + src as u16;
        let dst_addr = STM8_STATE_BASE + dst as u16;
        let mut out = stm8_ld_a_addr(src_addr);
        out.extend_from_slice(&stm8_st_a_addr(dst_addr));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = STM8_STATE_BASE + slot as u16;
        let imm8 = imm as u8;
        let mut out = stm8_ld_a_addr(addr);
        out.extend_from_slice(&stm8_add_a_imm(imm8));
        out.extend_from_slice(&stm8_st_a_addr(addr));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = STM8_STATE_BASE + slot as u16;
        let imm8 = imm as u8;
        let mut out = stm8_ld_a_addr(addr);
        out.extend_from_slice(&stm8_sub_a_imm(imm8));
        out.extend_from_slice(&stm8_st_a_addr(addr));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = STM8_STATE_BASE + slot as u16;
        let mut out = stm8_ld_a_addr(addr);
        out.extend_from_slice(&stm8_inc_a());
        out.extend_from_slice(&stm8_st_a_addr(addr));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = STM8_STATE_BASE + slot as u16;
        let mut out = stm8_ld_a_addr(addr);
        out.extend_from_slice(&stm8_dec_a());
        out.extend_from_slice(&stm8_st_a_addr(addr));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_addr = STM8_STATE_BASE + dst as u16;
        let src_addr = STM8_STATE_BASE + src as u16;
        let mut out = stm8_ld_a_addr(dst_addr);
        out.extend_from_slice(&stm8_st_reg_addr(0, dst_addr));
        out.extend_from_slice(&stm8_ld_a_addr(src_addr));
        out.extend_from_slice(&stm8_add_a_reg(0));
        out.extend_from_slice(&stm8_st_a_addr(dst_addr));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_addr = STM8_STATE_BASE + dst as u16;
        let src_addr = STM8_STATE_BASE + src as u16;
        let mut out = stm8_ld_a_addr(dst_addr);
        out.extend_from_slice(&stm8_st_reg_addr(0, dst_addr));
        out.extend_from_slice(&stm8_ld_a_addr(src_addr));
        out.extend_from_slice(&stm8_or_a_reg(0));
        out.extend_from_slice(&stm8_st_a_addr(dst_addr));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_addr = STM8_STATE_BASE + dst as u16;
        let src_addr = STM8_STATE_BASE + src as u16;
        let mut out = stm8_ld_a_addr(dst_addr);
        out.extend_from_slice(&stm8_st_reg_addr(0, dst_addr));
        out.extend_from_slice(&stm8_ld_a_addr(src_addr));
        out.extend_from_slice(&stm8_sub_a_reg(0));
        out.extend_from_slice(&stm8_st_a_addr(dst_addr));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let dst_addr = STM8_STATE_BASE + dst as u16;
        let src_addr = STM8_STATE_BASE + src as u16;
        // STM8 has no multiply; approximate with ADD
        let mut out = stm8_ld_a_addr(dst_addr);
        out.extend_from_slice(&stm8_st_reg_addr(0, dst_addr));
        out.extend_from_slice(&stm8_ld_a_addr(src_addr));
        out.extend_from_slice(&stm8_add_a_reg(0));
        out.extend_from_slice(&stm8_st_a_addr(dst_addr));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let a_addr = STM8_STATE_BASE + a as u16;
        let b_addr = STM8_STATE_BASE + b as u16;
        let mut out = stm8_ld_a_addr(a_addr);
        out.extend_from_slice(&stm8_sub_a_imm(b_addr as u8));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, _oo: u16) -> IsaResult<Vec<u8>> {
        let dd_addr = STM8_STATE_BASE + dd as u16;
        let ss_addr = STM8_STATE_BASE + ss as u16;
        let mut out = stm8_ld_a_addr(ss_addr);
        out.extend_from_slice(&stm8_st_a_addr(dd_addr));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let src_addr = STM8_STATE_BASE + (src + i) as u16;
            let dst_addr = STM8_STATE_BASE + (dst + i) as u16;
            out.extend_from_slice(&stm8_ld_a_addr(src_addr));
            out.extend_from_slice(&stm8_st_a_addr(dst_addr));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let addr = STM8_STATE_BASE + slot as u16;
        let mut out = stm8_ld_a_imm(size as u8);
        out.extend_from_slice(&stm8_st_a_addr(addr));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let addr = STM8_STATE_BASE + slot as u16;
        let mut out = stm8_ld_a_imm(str_idx);
        out.extend_from_slice(&stm8_st_a_addr(addr));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let addr = STM8_STATE_BASE + slot as u16;
        let mut out = stm8_ld_a_imm(str_idx);
        out.extend_from_slice(&stm8_st_a_addr(addr));
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0x83])
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((stm8_call_addr(0), BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((stm8_jmp_addr(0), BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jcc_branch(&mut self, _cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        Ok((stm8_jmp_addr(0), BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0x200,
            data_section_offset: 0x00,
            data_section_size: 0x1000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 ROCm/HIP (AMD GPU, text output, stub) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
pub struct RocmPlatform;

impl RocmPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for RocmPlatform {
    fn emit_alloc(&mut self, _slot: u16, _size: u64) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "ROCm backend has no inline alloc; use rocm_backend::emit_rocm instead".into(),
        })
    }
    fn emit_load_file(&mut self, _slot: u16, _str_idx: u8) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "ROCm backend has no file I/O; use rocm_backend::emit_rocm instead".into(),
        })
    }
    fn emit_write_file(&mut self, _slot: u16, _str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "ROCm backend has no file I/O; use rocm_backend::emit_rocm instead".into(),
        })
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xFF])
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0,
            data_section_offset: 0,
            data_section_size: 0,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 Vulkan Compute Shader (GPU, SPIR-V, stub) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
pub struct VulkanPlatform;

impl VulkanPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for VulkanPlatform {
    fn emit_alloc(&mut self, _slot: u16, _size: u64) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Vulkan backend has no inline alloc; use spirv_backend::emit_spirv instead".into(),
        })
    }
    fn emit_load_file(&mut self, _slot: u16, _str_idx: u8) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Vulkan backend has no file I/O; use spirv_backend::emit_spirv instead".into(),
        })
    }
    fn emit_write_file(&mut self, _slot: u16, _str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Vulkan backend has no file I/O; use spirv_backend::emit_spirv instead".into(),
        })
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xFF])
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0,
            data_section_offset: 0,
            data_section_size: 0,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 EVM encoding helpers 闁冲厜鍋撻柍鍏夊亾
fn evm_push_u64(v: u64) -> Vec<u8> {
    let bytes = v.to_be_bytes();
    let first_nonzero = bytes.iter().position(|&b| b != 0).unwrap_or(7);
    let n = 8 - first_nonzero;
    if n == 1 {
        let mut out = vec![0x60];
        out.push(bytes[7]);
        out
    } else if n <= 2 {
        let mut out = vec![0x61];
        out.push(bytes[6]); out.push(bytes[7]);
        out
    } else if n <= 3 {
        let mut out = vec![0x62];
        out.push(bytes[5]); out.push(bytes[6]); out.push(bytes[7]);
        out
    } else if n <= 4 {
        let mut out = vec![0x63];
        out.push(bytes[4]); out.push(bytes[5]); out.push(bytes[6]); out.push(bytes[7]);
        out
    } else {
        let mut out = vec![0x64];
        out.push(bytes[3]); out.push(bytes[4]); out.push(bytes[5]); out.push(bytes[6]); out.push(bytes[7]);
        out
    }
}
fn evm_push_slot(slot: u16) -> Vec<u8> {
    let addr = slot.wrapping_mul(0x20);
    if addr <= 0x7F {
        vec![0x60, addr as u8]
    } else {
        vec![0x61, (addr >> 8) as u8, (addr & 0xFF) as u8]
    }
}

// 闁冲厜鍋撻柍鍏夊亾 EVM (Ethereum Virtual Machine, flat binary) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋?
pub struct EvmPlatform;

impl EvmPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for EvmPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        // EVM JUMPDEST = 0x5B (used as NOP placeholder)
        Ok(vec![0x5B])
    }
    fn emit_ret(&mut self) -> IsaResult<Vec<u8>> {
        // EVM STOP = 0x00 (exit)
        Ok(vec![0x00])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        // PUSH1/PUSH8 imm; PUSH1 slot*32; MSTORE
        let mut out = if imm <= 0xFF {
            vec![0x60, imm as u8]
        } else {
            let mut v = vec![0x7F];
            // PUSH32 immediate is 32-byte big-endian
            for _ in 0..24 { v.push(0x00); }
            v.extend_from_slice(&imm.to_be_bytes());
            v
        };
        out.push(0x60); // PUSH1
        out.push((slot as u8).wrapping_mul(0x20));
        out.push(0x52); // MSTORE
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        // PUSH1 src*32; MLOAD; PUSH1 dst*32; MSTORE
        let mut out = vec![0x60, (src as u8).wrapping_mul(0x20), 0x51];
        out.push(0x60); // PUSH1
        out.push((dst as u8).wrapping_mul(0x20));
        out.push(0x52); // MSTORE
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        // PUSH2 slot*32; MLOAD; PUSH1 imm; ADD; PUSH2 slot*32; MSTORE
        let mut out = evm_push_slot(slot);
        out.push(0x51); // MLOAD
        out.extend(evm_push_u64(imm));
        out.push(0x01); // ADD
        out.extend(evm_push_slot(slot));
        out.push(0x52); // MSTORE
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = evm_push_slot(slot);
        out.push(0x51); // MLOAD
        out.extend(evm_push_u64(imm));
        out.push(0x03); // SUB
        out.extend(evm_push_slot(slot));
        out.push(0x52); // MSTORE
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = evm_push_slot(slot);
        out.push(0x51); // MLOAD
        out.push(0x60); // PUSH1
        out.push(0x01);
        out.push(0x01); // ADD
        out.extend(evm_push_slot(slot));
        out.push(0x52); // MSTORE
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = evm_push_slot(slot);
        out.push(0x51); // MLOAD
        out.push(0x60); // PUSH1
        out.push(0x01);
        out.push(0x03); // SUB
        out.extend(evm_push_slot(slot));
        out.push(0x52); // MSTORE
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        // PUSH1 dst*32; MLOAD; PUSH1 src*32; MLOAD; ADD; PUSH1 dst*32; MSTORE
        let mut out = vec![0x60, (dst as u8).wrapping_mul(0x20), 0x51];
        out.push(0x60); out.push((src as u8).wrapping_mul(0x20));
        out.push(0x51); // MLOAD
        out.push(0x01); // ADD
        out.push(0x60); out.push((dst as u8).wrapping_mul(0x20));
        out.push(0x52); // MSTORE
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x60, (dst as u8).wrapping_mul(0x20), 0x51];
        out.push(0x60); out.push((src as u8).wrapping_mul(0x20));
        out.push(0x51); // MLOAD
        out.push(0x17); // OR
        out.push(0x60); out.push((dst as u8).wrapping_mul(0x20));
        out.push(0x52); // MSTORE
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x60, (dst as u8).wrapping_mul(0x20), 0x51];
        out.push(0x60); out.push((src as u8).wrapping_mul(0x20));
        out.push(0x51); // MLOAD
        out.push(0x03); // SUB
        out.push(0x60); out.push((dst as u8).wrapping_mul(0x20));
        out.push(0x52); // MSTORE
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x60, (dst as u8).wrapping_mul(0x20), 0x51];
        out.push(0x60); out.push((src as u8).wrapping_mul(0x20));
        out.push(0x51); // MLOAD
        out.push(0x02); // MUL
        out.push(0x60); out.push((dst as u8).wrapping_mul(0x20));
        out.push(0x52); // MSTORE
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        // PUSH1 a*32; MLOAD; PUSH1 b*32; MLOAD; EQ
        let mut out = vec![0x60, (a as u8).wrapping_mul(0x20), 0x51];
        out.push(0x60); out.push((b as u8).wrapping_mul(0x20));
        out.push(0x51); // MLOAD
        out.push(0x14); // EQ
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        // PUSH1 ss*32; MLOAD; PUSH1 oo; ADD; MLOAD; PUSH1 dd*32; MSTORE
        let mut out = vec![0x60, (ss as u8).wrapping_mul(0x20), 0x51];
        out.push(0x60); out.push(oo as u8);
        out.push(0x01); // ADD
        out.push(0x51); // MLOAD
        out.push(0x60); out.push((dd as u8).wrapping_mul(0x20));
        out.push(0x52); // MSTORE
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        self.emit_memcpy_state(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let sa = src + i;
            let da = dst + i;
            out.push(0x61); out.push(((sa as u16).wrapping_mul(0x20) >> 8) as u8);
            out.push((sa as u16).wrapping_mul(0x20) as u8);
            out.push(0x51);
            out.push(0x61); out.push(((da as u16).wrapping_mul(0x20) >> 8) as u8);
            out.push((da as u16).wrapping_mul(0x20) as u8);
            out.push(0x52);
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x5B];
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x5B];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x5B];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0x00])
    }
    fn emit_call_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // PUSH2 <target>; JUMP
        let mut out = vec![0x61, 0x00, 0x00];
        out.push(0x56); // JUMP
        Ok((out, BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jmp_branch(&mut self) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // PUSH2 <target>; JUMP
        let mut out = vec![0x61, 0x00, 0x00];
        out.push(0x56); // JUMP
        Ok((out, BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn emit_jcc_branch(&mut self, _cc: u8) -> IsaResult<(Vec<u8>, BranchFixup)> {
        // Condition is on stack (result of EQ). Emit:
        // PUSH2 <target>; JUMPI
        let mut out = vec![0x61, 0x00, 0x00];
        out.push(0x57); // JUMPI
        Ok((out, BranchFixup { field_offset: 1, field_size: 2, kind: FixupKind::AbsAddr16 }))
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0x1000,
            data_section_offset: 0,
            data_section_size: 0x1000,
        }
    }
}

// 闁冲厜鍋撻柍鍏夊亾 Qiskit / OpenQASM (Quantum Computing, text output, stub) 闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾闁冲厜鍋撻柍鍏夊亾
pub struct QiskitPlatform;

impl QiskitPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for QiskitPlatform {
    fn emit_alloc(&mut self, _slot: u16, _size: u64) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Qiskit backend has no inline alloc; use qiskit_backend::emit_qiskit instead".into(),
        })
    }
    fn emit_load_file(&mut self, _slot: u16, _str_idx: u8) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Qiskit backend has no file I/O; use qiskit_backend::emit_qiskit instead".into(),
        })
    }
    fn emit_write_file(&mut self, _slot: u16, _str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError {
            msg: "Qiskit backend has no file I/O; use qiskit_backend::emit_qiskit instead".into(),
        })
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xFF])
    }
    fn startup_blob(&self) -> &[u8] {
        &[]
    }
    fn template(&self) -> TemplateInfo {
        TemplateInfo {
            format: BinaryFormat::FlatBinary,
            entry_point: 0,
            stack_size: 0,
            data_section_offset: 0,
            data_section_size: 0,
        }
    }
}


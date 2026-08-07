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
    // ── I/O & exit (existing) ─────────────────────────────────────
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>>;
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>>;
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, sz_slot: u16) -> IsaResult<Vec<u8>>;
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>>;
    fn startup_blob(&self) -> &[u8];
    fn template(&self) -> TemplateInfo;

    // ── Architecture-native emit (Phase 1: default x64; Phase 2: override per arch) ──

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
        // Default x64 LDB: load_state ss → rax; add imm8; movzx; store_state dd
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

    // ── Branch fixup abstraction ──
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
                let patched = (base & 0x1E00001F) | enc;
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
                // conditional branch (beq etc): imm14 = (target - branch_addr) & 0xFFFC
                let diff = target as i32 - branch_start as i32;
                let imm14 = (diff as u32) & 0xFFFC;
                let base = u32::from_le_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let patched = (base & 0xFFFC0003) | imm14;
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
                // beq/bne: 16-bit signed PC-relative, offset = (target - branch_addr) / 4
                let diff = target as i32 - branch_start as i32;
                let imm16 = (diff >> 2) & 0xFFFF;
                let base = u32::from_le_bytes(code[branch_start..branch_start + 4].try_into().unwrap());
                let patched = (base & 0xFFFF0000) | (imm16 as u32);
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
}

// ── Foreign-arch arithmetic helpers (marker format: NOP + type + operands) ──

fn mark_nop(op: u8, slot: u16, imm: u64) -> Vec<u8> {
    let mut v = vec![op];
    v.push((slot & 0xFF) as u8);
    v.push(((slot >> 8) & 0xFF) as u8);
    v.extend_from_slice(&imm.to_le_bytes());
    v
}

fn mark2_nop(op: u8, a: u16, b: u64) -> Vec<u8> {
    let mut v = vec![op];
    v.push((a & 0xFF) as u8);
    v.push(((a >> 8) & 0xFF) as u8);
    v.extend_from_slice(&(b as u64).to_le_bytes());
    v
}

fn mark1_nop(op: u8, slot: u16) -> Vec<u8> {
    let mut v = vec![op];
    v.push((slot & 0xFF) as u8);
    v.push(((slot >> 8) & 0xFF) as u8);
    v
}

// marker type bytes
const MK_SET: u8 = 0x80;
const MK_GET: u8 = 0x81;
const MK_MOV: u8 = 0x82;
const MK_ADDI: u8 = 0x83;
const MK_SUBI: u8 = 0x84;
const MK_INC: u8 = 0x85;
const MK_DEC: u8 = 0x86;
const MK_ADDV: u8 = 0x87;
const MK_ORV: u8 = 0x88;
const MK_SUBV: u8 = 0x89;
const MK_IMUL: u8 = 0x8A;
const MK_CMP: u8 = 0x8B;
const MK_LDB: u8 = 0x8C;
const MK_MCD: u8 = 0x8D;
const MK_MCS: u8 = 0x8E;

// foreign-arch stub body helpers (return just the body, not the NOP prefix)
fn foreign_set(slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
    Ok(mark_nop(MK_SET, slot, imm))
}
fn foreign_get(dst: u16, src: u16) -> IsaResult<Vec<u8>> {
    Ok(mark_nop(MK_GET, dst, src as u64))
}
fn foreign_movrr(dst: u16, src: u16) -> IsaResult<Vec<u8>> {
    Ok(mark_nop(MK_MOV, dst, src as u64))
}
fn foreign_add_imm(slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
    Ok(mark_nop(MK_ADDI, slot, imm))
}
fn foreign_sub_imm(slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
    Ok(mark_nop(MK_SUBI, slot, imm))
}
fn foreign_inc(slot: u16) -> IsaResult<Vec<u8>> {
    Ok(mark1_nop(MK_INC, slot))
}
fn foreign_dec(slot: u16) -> IsaResult<Vec<u8>> {
    Ok(mark1_nop(MK_DEC, slot))
}
fn foreign_addv(dst: u16, src: u16) -> IsaResult<Vec<u8>> {
    Ok(mark_nop(MK_ADDV, dst, src as u64))
}
fn foreign_orv(dst: u16, src: u16) -> IsaResult<Vec<u8>> {
    Ok(mark_nop(MK_ORV, dst, src as u64))
}
fn foreign_subv(dst: u16, src: u16) -> IsaResult<Vec<u8>> {
    Ok(mark_nop(MK_SUBV, dst, src as u64))
}
fn foreign_imul(dst: u16, src: u16) -> IsaResult<Vec<u8>> {
    Ok(mark_nop(MK_IMUL, dst, src as u64))
}
fn foreign_cmp(a: u16, b: u16) -> IsaResult<Vec<u8>> {
    Ok(mark_nop(MK_CMP, a, b as u64))
}
fn foreign_ldb(dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
    let mut v = mark_nop(MK_LDB, dd, ss as u64);
    v.push((oo & 0xFF) as u8);
    v.push(((oo >> 8) & 0xFF) as u8);
    Ok(v)
}
fn foreign_memcpy_data(dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
    let mut v = mark_nop(MK_MCD, dst, src as u64);
    v.push((n & 0xFF) as u8);
    v.push(((n >> 8) & 0xFF) as u8);
    Ok(v)
}
fn foreign_memcpy_state(dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
    let mut v = mark_nop(MK_MCS, dst, src as u64);
    v.push((n & 0xFF) as u8);
    v.push(((n >> 8) & 0xFF) as u8);
    Ok(v)
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

// ── Stub ──────────────────────────────────────────────────────────
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

// ── Win32 ─────────────────────────────────────────────────────────
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

// ── Linux ─────────────────────────────────────────────────────────
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

// ── CUDA ──────────────────────────────────────────────────────────
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

// ── Bare-metal ────────────────────────────────────────────────────
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

// ── Android — ARM64 (aarch64) + ELF64 ────────────────────────────
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

// ── ARM64 instruction encoding helpers ──
fn arm64_mov_imm64(rd: u32, imm: u64) -> Vec<u8> {
    let mut out = Vec::new();
    let mut written = false;
    for shift in (0..64).step_by(16) {
        let chunk = (imm >> shift) & 0xFFFF;
        if chunk != 0 || shift == 0 {
            if !written {
                let enc: u32 = 0x92800000 | rd | ((chunk as u32) << 5) | ((shift as u32 / 16) << 21);
                out.extend_from_slice(&enc.to_le_bytes());
                written = true;
            } else {
                let enc: u32 = 0x92800020 | rd | ((chunk as u32) << 5) | ((shift as u32 / 16) << 21);
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
    // ── Real ARM64 instruction overrides ──
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
        let enc: u32 = 0x54000000 | (cond << 4);
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

// ── Apple/iOS — ARM64 (aarch64) + Mach-O64 ──────────────────────
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
    // ── Real ARM64 instruction overrides (Apple ARM64) ──
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
        let enc: u32 = 0x54000000 | (cond << 4);
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

// ── 8051 ASM (Intel 8051 / ESP8266/ESP32 8051 core) ─────────────
pub struct Eight051Platform;

impl Eight051Platform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for Eight051Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0x00])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
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
        Ok(vec![0x80, 0xFE])
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

// ── x86 (32-bit Windows / PE32) ──────────────────────────────────
pub struct X86Platform {
    startup: Vec<u8>,
}

impl X86Platform {
    pub fn new() -> Self {
        Self { startup: vec![] }
    }
}

impl PlatformBackend for X86Platform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x90];
        out.extend_from_slice(&(size as u64).to_le_bytes());
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

// ── FreeDOS (DOS COM) ────────────────────────────────────────────
pub struct FreedosPlatform;

impl FreedosPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for FreedosPlatform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x90];
        out.extend_from_slice(&(size as u64).to_le_bytes());
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
            data_section_offset: 0,
            data_section_size: 0x10000,
        }
    }
}

// ── RISC-V encoding helpers ──
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
                    let enc: u32 = 0x00000013 | (lo12 << 20) | (rd << 15) | (rd << 7);
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
    match cc {
        0x84 => 0x00A50A63, // JE  -> beq x10, x11
        0x85 => 0x00A51A63, // JNE -> bne x10, x11
        0x86 => 0x40A50A63, // JL  -> blt x10, x11
        0x87 => 0x50A50A63, // JGE -> bge x10, x11
        0x88 => 0x50A5A663, // JLE -> bge x11, x10 (swapped)
        0x89 => 0x40A5A663, // JG  -> blt x11, x10 (swapped)
        0x8A => 0x60A50A63, // JB  -> bltu x10, x11
        0x8B => 0x70A50A63, // JAE -> bgeu x10, x11
        0x8C => 0x70A5A663, // JBE -> bgeu x11, x10 (swapped)
        0x8D => 0x60A5A663, // JA  -> bltu x11, x10 (swapped)
        _ => 0x00A50A63,
    }
}

// ── RISC-V RV64 (Linux ELF64) ─────────────────────────────────────
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
        out.extend_from_slice(&riscv_sd(6, 5, slot));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, src).to_vec();
        out.extend_from_slice(&riscv_sd(6, 5, dst));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, slot).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&riscv_addi(6, 6, imm as u32));
        } else {
            out.extend_from_slice(&riscv_li_imm(7, imm));
            out.extend_from_slice(&riscv_add(6, 6, 7));
        }
        out.extend_from_slice(&riscv_sd(6, 5, slot));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, slot).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&riscv_addi(6, 6, (imm as u32) | 0xFFFFF000)); // sub via addi negative
        } else {
            out.extend_from_slice(&riscv_li_imm(7, imm));
            out.extend_from_slice(&riscv_sub(6, 6, 7));
        }
        out.extend_from_slice(&riscv_sd(6, 5, slot));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, slot).to_vec();
        out.extend_from_slice(&riscv_addi(6, 6, 1));
        out.extend_from_slice(&riscv_sd(6, 5, slot));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, slot).to_vec();
        out.extend_from_slice(&riscv_addi(6, 6, 0xFFF)); // addi -1
        out.extend_from_slice(&riscv_sd(6, 5, slot));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, dst).to_vec();
        out.extend_from_slice(&riscv_ld(7, 5, src));
        out.extend_from_slice(&riscv_add(6, 6, 7));
        out.extend_from_slice(&riscv_sd(6, 5, dst));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, dst).to_vec();
        out.extend_from_slice(&riscv_ld(7, 5, src));
        out.extend_from_slice(&riscv_or(6, 6, 7));
        out.extend_from_slice(&riscv_sd(6, 5, dst));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, dst).to_vec();
        out.extend_from_slice(&riscv_ld(7, 5, src));
        out.extend_from_slice(&riscv_sub(6, 6, 7));
        out.extend_from_slice(&riscv_sd(6, 5, dst));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, dst).to_vec();
        out.extend_from_slice(&riscv_ld(7, 5, src));
        out.extend_from_slice(&riscv_mul(6, 6, 7));
        out.extend_from_slice(&riscv_sd(6, 5, dst));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(10, 5, a).to_vec(); // x10 = state[a]
        out.extend_from_slice(&riscv_ld(11, 5, b)); // x11 = state[b]
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_ld(6, 5, ss).to_vec(); // x6 = state[ss] (addr)
        if oo != 0 {
            out.extend_from_slice(&riscv_addi(6, 6, oo as u32));
        }
        out.extend_from_slice(&riscv_lbu(7, 6, 0)); // x7 = byte [x6]
        out.extend_from_slice(&riscv_sd(7, 5, dd));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, _dst: u16, _src: u16, _n: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError { msg: "RISC-V: memcpy_data not yet implemented".into() })
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "RISC-V memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n as u16 {
            out.extend_from_slice(&riscv_ld(6, 5, src + i));
            out.extend_from_slice(&riscv_sd(6, 5, dst + i));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, size);
        out.extend_from_slice(&riscv_sd(6, 5, slot));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, str_idx as u64);
        out.extend_from_slice(&riscv_sd(6, 5, slot));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, str_idx as u64);
        out.extend_from_slice(&riscv_sd(6, 5, slot));
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

// ── MIPS big-endian (ELF32 BE) ────────────────────────────────────
const MIPS_NOP: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

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
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = MIPS_NOP.to_vec();
        out.extend_from_slice(&(size as u32).to_be_bytes());
        out.extend_from_slice(&(slot as u32).to_be_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = MIPS_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u32).to_be_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = MIPS_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u32).to_be_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![];
        let imm = code as u32;
        out.extend_from_slice(&(0x34200000u32 | imm).to_be_bytes());
        out.extend_from_slice(&0x0000000Cu32.to_be_bytes());
        Ok(out)
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

// ── PowerPC64 LE (Linux ELF64) ────────────────────────────────────
const PPC64LE_NOP: [u8; 4] = [0x00, 0x00, 0x00, 0x60];

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
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = PPC64LE_NOP.to_vec();
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = PPC64LE_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = PPC64LE_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![];
        out.extend_from_slice(&(0x38600000u32 | (code as u32)).to_le_bytes());
        out.extend_from_slice(&(0x3800003Cu32).to_le_bytes());
        out.extend_from_slice(&0x44000002u32.to_le_bytes());
        Ok(out)
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

// ── AVR (ATmega) ─────────────────────────────────────────────────
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
        Ok(vec![0x95, 0x08]) // ret
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
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

// ── ARM32 encoding helpers ──
fn arm32_movw(rd: u32, imm16: u32) -> [u8; 4] {
    let enc: u32 = 0xE3000000 | (rd << 12) | (imm16 & 0xFFFF);
    enc.to_le_bytes()
}
fn arm32_movt(rd: u32, imm16: u32) -> [u8; 4] {
    let enc: u32 = 0xE3400000 | (rd << 12) | (imm16 & 0xFFFF);
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

// ── ARM32 (32-bit ARM / Android EABI) ────────────────────────────
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
        out.extend_from_slice(&arm32_str(0, 8, slot));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, src).to_vec();
        out.extend_from_slice(&arm32_str(0, 8, dst));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, slot).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&arm32_add_imm(0, 0, imm as u32));
        } else {
            out.extend_from_slice(&arm32_ldr_imm32(1, imm as u32));
            out.extend_from_slice(&arm32_add_reg(0, 0, 1));
        }
        out.extend_from_slice(&arm32_str(0, 8, slot));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, slot).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&arm32_sub_imm(0, 0, imm as u32));
        } else {
            out.extend_from_slice(&arm32_ldr_imm32(1, imm as u32));
            out.extend_from_slice(&arm32_sub_reg(0, 0, 1));
        }
        out.extend_from_slice(&arm32_str(0, 8, slot));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, slot).to_vec();
        out.extend_from_slice(&arm32_add_imm(0, 0, 1));
        out.extend_from_slice(&arm32_str(0, 8, slot));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, slot).to_vec();
        out.extend_from_slice(&arm32_sub_imm(0, 0, 1));
        out.extend_from_slice(&arm32_str(0, 8, slot));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, dst).to_vec();
        out.extend_from_slice(&arm32_ldr(1, 8, src));
        out.extend_from_slice(&arm32_add_reg(0, 0, 1));
        out.extend_from_slice(&arm32_str(0, 8, dst));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, dst).to_vec();
        out.extend_from_slice(&arm32_ldr(1, 8, src));
        out.extend_from_slice(&arm32_orr(0, 0, 1));
        out.extend_from_slice(&arm32_str(0, 8, dst));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, dst).to_vec();
        out.extend_from_slice(&arm32_ldr(1, 8, src));
        out.extend_from_slice(&arm32_sub_reg(0, 0, 1));
        out.extend_from_slice(&arm32_str(0, 8, dst));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, dst).to_vec();
        out.extend_from_slice(&arm32_ldr(1, 8, src));
        out.extend_from_slice(&arm32_mul(0, 0, 1));
        out.extend_from_slice(&arm32_str(0, 8, dst));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(2, 8, a).to_vec();
        out.extend_from_slice(&arm32_ldr(3, 8, b));
        out.extend_from_slice(&arm32_cmp(2, 3));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr(0, 8, ss).to_vec();
        if oo != 0 {
            out.extend_from_slice(&arm32_add_imm(0, 0, oo as u32));
        }
        out.extend_from_slice(&arm32_ldrb(1, 0, 0));
        out.extend_from_slice(&arm32_str(1, 8, dd));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, _dst: u16, _src: u16, _n: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError { msg: "ARM32: memcpy_data not yet implemented".into() })
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "ARM32 memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n as u16 {
            out.extend_from_slice(&arm32_ldr(0, 8, src + i));
            out.extend_from_slice(&arm32_str(0, 8, dst + i));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr_imm32(0, size as u32);
        out.extend_from_slice(&arm32_str(0, 8, slot));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr_imm32(0, str_idx as u32);
        out.extend_from_slice(&arm32_str(0, 8, slot));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = arm32_ldr_imm32(0, str_idx as u32);
        out.extend_from_slice(&arm32_str(0, 8, slot));
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

// ── WebAssembly (Wasm) ───────────────────────────────────────────
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

// ── Mach-O x64 (Intel macOS) ─────────────────────────────────────
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

// ── LoongArch (LA64, ELF64 LE) ──────────────────────────────────
const LOONGARCH_NOP: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

pub struct LoongArchPlatform;

impl LoongArchPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for LoongArchPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(LOONGARCH_NOP.to_vec())
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = LOONGARCH_NOP.to_vec();
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = LOONGARCH_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = LOONGARCH_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(LOONGARCH_NOP.to_vec())
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

// ── SPARC v8 (32-bit BE, ELF32) ─────────────────────────────────
const SPARC_NOP: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

pub struct SparcPlatform;

impl SparcPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for SparcPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(SPARC_NOP.to_vec())
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
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

// ── RV32 (RISC-V 32-bit, ELF32 LE) ──────────────────────────────
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
        out.extend_from_slice(&riscv_sw(6, 5, slot));
        Ok(out)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, src).to_vec();
        out.extend_from_slice(&riscv_sw(6, 5, dst));
        Ok(out)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        self.emit_get(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, slot).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&riscv_addi(6, 6, imm as u32));
        } else {
            out.extend_from_slice(&riscv_li_imm(7, imm));
            out.extend_from_slice(&riscv_add(6, 6, 7));
        }
        out.extend_from_slice(&riscv_sw(6, 5, slot));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, slot).to_vec();
        if imm < 0x1000 {
            out.extend_from_slice(&riscv_addi(6, 6, (imm as u32) | 0xFFFFF000));
        } else {
            out.extend_from_slice(&riscv_li_imm(7, imm));
            out.extend_from_slice(&riscv_sub(6, 6, 7));
        }
        out.extend_from_slice(&riscv_sw(6, 5, slot));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, slot).to_vec();
        out.extend_from_slice(&riscv_addi(6, 6, 1));
        out.extend_from_slice(&riscv_sw(6, 5, slot));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, slot).to_vec();
        out.extend_from_slice(&riscv_addi(6, 6, 0xFFF));
        out.extend_from_slice(&riscv_sw(6, 5, slot));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, dst).to_vec();
        out.extend_from_slice(&riscv_lw(7, 5, src));
        out.extend_from_slice(&riscv_add(6, 6, 7));
        out.extend_from_slice(&riscv_sw(6, 5, dst));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, dst).to_vec();
        out.extend_from_slice(&riscv_lw(7, 5, src));
        out.extend_from_slice(&riscv_or(6, 6, 7));
        out.extend_from_slice(&riscv_sw(6, 5, dst));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, dst).to_vec();
        out.extend_from_slice(&riscv_lw(7, 5, src));
        out.extend_from_slice(&riscv_sub(6, 6, 7));
        out.extend_from_slice(&riscv_sw(6, 5, dst));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, dst).to_vec();
        out.extend_from_slice(&riscv_lw(7, 5, src));
        out.extend_from_slice(&riscv_mul(6, 6, 7));
        out.extend_from_slice(&riscv_sw(6, 5, dst));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(10, 5, a).to_vec();
        out.extend_from_slice(&riscv_lw(11, 5, b));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_lw(6, 5, ss).to_vec();
        if oo != 0 {
            out.extend_from_slice(&riscv_addi(6, 6, oo as u32));
        }
        out.extend_from_slice(&riscv_lbu(7, 6, 0));
        out.extend_from_slice(&riscv_sw(7, 5, dd));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, _dst: u16, _src: u16, _n: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError { msg: "RISC-V: memcpy_data not yet implemented".into() })
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        if n > 64 {
            return Err(IsaError::PlatformError { msg: "RISC-V memcpy_state: n > 64".into() });
        }
        let mut out = Vec::new();
        for i in 0..n as u16 {
            out.extend_from_slice(&riscv_lw(6, 5, src + i));
            out.extend_from_slice(&riscv_sw(6, 5, dst + i));
        }
        Ok(out)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, size);
        out.extend_from_slice(&riscv_sw(6, 5, slot));
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, str_idx as u64);
        out.extend_from_slice(&riscv_sw(6, 5, slot));
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = riscv_li_imm(6, str_idx as u64);
        out.extend_from_slice(&riscv_sw(6, 5, slot));
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

// ── ARM64 Windows (AArch64 PE32+) ───────────────────────────────
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
    // ── Real ARM64 instruction overrides (Aarch64 Windows) ──
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
        let enc: u32 = 0x54000000 | (cond << 4);
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

// ── SerenityOS ───────────────────────────────────────────────────
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

// ── FreeBSD (x64 ELF64, FreeBSD syscall ABI) ─────────────────────
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

// ── Haiku (x64 ELF64, Haiku syscall ABI) ─────────────────────────
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
        Ok(vec![0xF4]) // hlt — Haiku syscall ABI is complex, use hlt as stub
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

// ── Plan9 (9P/Acadia, flat binary) ────────────────────────────────
pub struct Plan9Platform;

impl Plan9Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for Plan9Platform {
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

// ── Xtensa (ESP32 LX6, flat binary) ──────────────────────────────
const XTENSA_NOP: [u8; 3] = [0x00, 0x00, 0x00];

pub struct XtensaPlatform;

impl XtensaPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for XtensaPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(XTENSA_NOP.to_vec())
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = XTENSA_NOP.to_vec();
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = XTENSA_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = XTENSA_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        // Xtensa halt: 0x0000F0 (3 bytes LE: F0 00 00)
        Ok(vec![0xF0, 0x00, 0x00])
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

// ── Z80 (8-bit, CP/M or ROM, flat binary) ────────────────────────
pub struct Z80Platform;

impl Z80Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for Z80Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0x00])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
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
        // JP 0x0000 (warm boot) = 0xC3 0x00 0x00
        Ok(vec![0xC3, 0x00, 0x00])
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

// ── 6502 (8-bit, Commodore/NES, flat binary) ─────────────────────
pub struct M6502Platform;

impl M6502Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for M6502Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        Ok(vec![0xEA])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
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

// ── M68k (Motorola 68000, Amiga/Mac Classic, flat binary) ────────
pub struct M68kPlatform;

impl M68kPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for M68kPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        // M68k NOP = 0x4E71 (big-endian)
        Ok(vec![0x4E, 0x71])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x4E, 0x71];
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x4E, 0x71];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x4E, 0x71];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        // TRAP #0 = 0x4E40 (big-endian)
        Ok(vec![0x4E, 0x40])
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

// ── MSP430 (16-bit TI MCU) ────────────────────────────────────────
pub struct Msp430Platform;

impl Msp430Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for Msp430Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        // MSP430 NOP = 0x03 (1 byte)
        Ok(vec![0x03])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x03];
        out.extend_from_slice(&(size as u16).to_le_bytes());
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x03];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x03];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        // 0x00 0x00 = undefined opcode → trap on MSP430
        Ok(vec![0x00, 0x00])
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

// ── PIC (8-bit Microchip MCU, mid-range) ──────────────────────────
pub struct PicPlatform;

impl PicPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for PicPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        // PIC NOP = 0x0000 (2 bytes, LE)
        Ok(vec![0x00, 0x00])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x00, 0x00];
        out.extend_from_slice(&(size as u16).to_le_bytes());
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x00, 0x00];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x00, 0x00];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        // SLEEP = 0x00FD (LE: FD 00)
        Ok(vec![0xFD, 0x00])
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
            data_section_size: 0x100,
        }
    }
}

// ── STM8 (8-bit STMicro MCU) ──────────────────────────────────────
pub struct Stm8Platform;

impl Stm8Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for Stm8Platform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        // STM8 NOP = 0x9D (1 byte)
        Ok(vec![0x9D])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x9D];
        out.extend_from_slice(&(size as u16).to_le_bytes());
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x9D];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = vec![0x9D];
        out.push(str_idx);
        out.extend_from_slice(&(slot as u16).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        // 0x00 = RST (reset) — trap on STM8
        Ok(vec![0x00])
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

// ── ROCm/HIP (AMD GPU, text output, stub) ─────────────────────────
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

// ── Vulkan Compute Shader (GPU, SPIR-V, stub) ─────────────────────
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

// ── EVM (Ethereum Virtual Machine, flat binary) ───────────────────
pub struct EvmPlatform;

impl EvmPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for EvmPlatform {
    fn emit_nop(&mut self) -> IsaResult<Vec<u8>> {
        // EVM JUMPDEST = 0x5B (used as NOP placeholder)
        Ok(vec![0x5B])
    }
    fn emit_set(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_set(slot, imm)
    }
    fn emit_get(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_get(dst, src)
    }
    fn emit_movrr(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_movrr(dst, src)
    }
    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
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
        // EVM STOP = 0x00
        Ok(vec![0x00])
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

// ── Qiskit / OpenQASM (Quantum Computing, text output, stub) ──────
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


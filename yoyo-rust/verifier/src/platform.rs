//! Platform backends (PROMPT-v3 Part 7).

use crate::types::{IsaError, IsaResult};

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

    // ── Branch emit (placeholder bytes; rel32 patched in emit_internal) ──
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

impl PlatformBackend for AndroidPlatform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = ARM64_NOP.to_vec();
        out.extend_from_slice(&(size as u64).to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = ARM64_NOP.to_vec();
        out.extend_from_slice(&str_idx.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = ARM64_NOP.to_vec();
        out.extend_from_slice(&str_idx.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![];
        out.extend_from_slice(&arm64_movz_w_imm16(8, 93));
        out.extend_from_slice(&arm64_movz_w_imm16(0, code as u16));
        out.extend_from_slice(&[0x01, 0x00, 0x00, 0xD4]);
        Ok(out)
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
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = ARM64_NOP.to_vec();
        out.extend_from_slice(&(size as u64).to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = ARM64_NOP.to_vec();
        out.extend_from_slice(&str_idx.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = ARM64_NOP.to_vec();
        out.extend_from_slice(&str_idx.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
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

// ── RISC-V RV64 (Linux ELF64) ─────────────────────────────────────
const RISC64_NOP: [u8; 4] = [0x13, 0x00, 0x00, 0x00];

pub struct Riscv64Platform;

impl Riscv64Platform {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformBackend for Riscv64Platform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = RISC64_NOP.to_vec();
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = RISC64_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = RISC64_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![];
        out.extend_from_slice(&riscv_addi(7, 0, 93));
        out.extend_from_slice(&riscv_addi(0, 0, code as i32));
        out.extend_from_slice(&0x00000073u32.to_le_bytes());
        Ok(out)
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

fn riscv_addi(rd: u32, rs1: u32, imm12: i32) -> [u8; 4] {
    let imm12 = (imm12 & 0xFFF) as u32;
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

// ── ARM32 (32-bit ARM / Android EABI) ────────────────────────────
const ARM32_NOP: [u8; 4] = [0x00, 0x00, 0xA0, 0xE1];

pub struct Arm32Platform;

impl Arm32Platform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for Arm32Platform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = ARM32_NOP.to_vec();
        out.extend_from_slice(&(size as u32).to_le_bytes());
        out.extend_from_slice(&(slot as u32).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = ARM32_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u32).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = ARM32_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u32).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0x00, 0x00, 0x00, 0xEF]) // swi #0
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
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = RISCV32_NOP.to_vec();
        out.extend_from_slice(&(size as u32).to_le_bytes());
        out.extend_from_slice(&(slot as u32).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = RISCV32_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u32).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = RISCV32_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u32).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        let mut out = vec![];
        out.extend_from_slice(&riscv_addi(7, 0, 93));
        out.extend_from_slice(&riscv_addi(0, 0, code as i32));
        out.extend_from_slice(&0x00000073u32.to_le_bytes());
        Ok(out)
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
const AARCH64_WIN_NOP: [u8; 4] = [0x1F, 0x20, 0x03, 0xD5];

pub struct Aarch64WindowsPlatform;

impl Aarch64WindowsPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for Aarch64WindowsPlatform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        let mut out = AARCH64_WIN_NOP.to_vec();
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        let mut out = AARCH64_WIN_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        let mut out = AARCH64_WIN_NOP.to_vec();
        out.push(str_idx);
        out.extend_from_slice(&(slot as u64).to_le_bytes());
        Ok(out)
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        // ret = 0xD65F03C0 (LE: C0 03 5F D6)
        Ok(vec![0xC0, 0x03, 0x5F, 0xD6])
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

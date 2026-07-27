//! Platform backends (PROMPT-v3 Part 7).

use crate::types::{IsaError, IsaResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformKind {
    Win32,
    Linux,
    BareMetal,
    Stub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryFormat {
    Pe64,
    Elf64,
    FlatBinary,
    Multiboot,
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
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>>;
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>>;
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, sz_slot: u16) -> IsaResult<Vec<u8>>;
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>>;
    fn startup_blob(&self) -> &[u8];
    fn template(&self) -> TemplateInfo;
}

pub fn select_platform(target: PlatformKind) -> Box<dyn PlatformBackend> {
    match target {
        PlatformKind::Win32 => Box::new(Win32Platform::new()),
        PlatformKind::Linux => Box::new(LinuxPlatform::new()),
        PlatformKind::BareMetal => Box::new(BareMetalPlatform::new()),
        PlatformKind::Stub => Box::new(StubPlatform::new()),
    }
}

pub fn parse_platform(s: &str) -> IsaResult<PlatformKind> {
    match s.to_ascii_lowercase().as_str() {
        "win32" | "windows" | "pe" => Ok(PlatformKind::Win32),
        "linux" | "elf" => Ok(PlatformKind::Linux),
        "baremetal" | "bare" => Ok(PlatformKind::BareMetal),
        "stub" => Ok(PlatformKind::Stub),
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
        // Match JS: movabs rax, size; store_state S[slot]
        let mut out = movabs(Reg::Rax, size)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        // Match JS: movabs rax, str_idx; store_state S[slot]
        let mut out = movabs(Reg::Rax, str_idx as u64)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        // Match JS: movabs rax, str_idx; store_state S[slot]; ignore sz_slot
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
        // Startup is provided by pe_link.rs (lea r15, [rip+disp]; jmp user_code).
        Self { startup: vec![] }
    }
}

impl PlatformBackend for Win32Platform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        // DDC-matched emit (same as JS Stub): movabs rax, size; store_state S[slot]
        // Real VirtualAlloc + IAT wiring deferred to Phase 2.
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, size)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
        // DDC-matched emit (same as JS Stub): movabs rax, str_idx; store_state S[slot]
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, str_idx as u64)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        // DDC-matched emit (same as JS Stub): movabs rax, str_idx; store_state S[slot]
        use crate::assembler::{movabs, store_state};
        use crate::types::Reg;
        let mut out = movabs(Reg::Rax, str_idx as u64)?;
        out.extend(store_state(slot, Reg::Rax)?);
        Ok(out)
    }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        use crate::assembler::movabs;
        use crate::types::Reg;
        // mov ecx, code ; (ExitProcess via IAT — placeholder ret)
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
            // Phase 2 root cause fix: pre-allocate 0x38000
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
        // mmap syscall stub — store size as marker
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
        // mov eax, 60 ; mov edi, code ; syscall
        Ok(vec![
            0xB8, 60, 0, 0, 0, // mov eax, 60
            0xBF, code, 0, 0, 0, // mov edi, code
            0x0F, 0x05, // syscall
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
        // ATA PIO placeholder
        Ok(vec![0x90])
    }
    fn emit_write_file(&mut self, _slot: u16, _str_idx: u8, _sz: u16) -> IsaResult<Vec<u8>> {
        Ok(vec![0x90])
    }
    fn emit_exit(&mut self, _code: u8) -> IsaResult<Vec<u8>> {
        Ok(vec![0xF4]) // hlt
    }
    fn startup_blob(&self) -> &[u8] {
        // See startup.rs — referenced separately
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

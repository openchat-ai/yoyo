//! Platform readers (PE / ELF / disasm) — Rust portion of entity 6.

pub mod pe_read;
pub mod elf_read;
pub mod disasm;

pub use disasm::hex_dump;
pub use elf_read::is_elf;
pub use pe_read::is_pe;

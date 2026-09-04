//! verifier library — re-exports for selfhost and other dependents.

pub mod assembler;
pub mod ddc;
pub mod emit;
pub mod executor;
pub mod fixup;
pub mod elf_link;
pub mod pe_link;
pub mod h00_manual_map_wireup;
pub mod pe_manual_map;
pub mod pe_dll_link;
pub mod platform;
pub mod platform_io;
pub mod render;
pub mod selfhost;
pub mod self_test;
pub mod startup;
pub mod linux_selfhost;
pub mod tir;
pub mod ty_parser;
pub mod tyb_parser;
pub mod types;
pub mod variable;
pub mod win32_selfhost;
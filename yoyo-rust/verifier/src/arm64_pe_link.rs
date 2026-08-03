//! Minimal ARM64 Windows PE32+ linker (AArch64).
//!
//! Produces a valid ARM64 PE32+ executable wrapping emitted .text + .data.
//! Uses ARM64 NOP, startup stub (adrp+add+b), and ret for exit.
//! Data section size floor: 0x38000.

use crate::types::IsaResult;

const OUTPUT_DATA_NEED: u32 = 0x38000;

pub struct PeArm64Image {
    pub bytes: Vec<u8>,
}

/// Wrap raw ARM64 code (+ optional data) in a PE32+ image.
/// Machine = 0xAA64 (ARM64). Entry = 0x1000, data = 0x2000.
pub fn link_arm64_pe(code: &[u8], data: &[u8]) -> IsaResult<PeArm64Image> {
    let section_align: u32 = 0x1000;
    let file_align: u32 = 0x200;

    let code_raw = align_up(code.len() as u32, file_align);
    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, section_align));
    let data_raw = align_up(data_need, file_align);

    let headers_raw = 0x400u32;
    let text_rva = section_align; // 0x1000
    let startup_len = 16u32; // adrp + add + b + nop pad = 12B, pad to 16B
    let text_vs = align_up(code.len() as u32 + startup_len, section_align);
    let data_rva = text_rva + text_vs;
    let data_vs = data_need;

    let size_of_image = align_up(data_rva + data_vs, section_align);
    let size_of_headers = headers_raw;

    let mut img = vec![0u8; (headers_raw + code_raw + data_raw) as usize];

    // DOS header
    img[0] = 0x4D;
    img[1] = 0x5A; // MZ
    write_u32(&mut img, 0x3C, 0x80); // e_lfanew

    // PE signature at 0x80
    img[0x80] = b'P';
    img[0x81] = b'E';

    // COFF header
    write_u16(&mut img, 0x84, 0xAA64); // Machine ARM64
    write_u16(&mut img, 0x86, 2); // NumberOfSections
    write_u16(&mut img, 0x94, 0xF0); // SizeOfOptionalHeader
    write_u16(&mut img, 0x96, 0x22); // Characteristics: EXECUTABLE | LARGE_ADDRESS_AWARE

    // Optional header (PE32+)
    let opt = 0x98usize;
    write_u16(&mut img, opt, 0x20B); // PE32+
    img[opt + 2] = 1; // MajorLinkerVersion
    write_u32(&mut img, opt + 16, text_rva); // AddressOfEntryPoint
    write_u64(&mut img, opt + 24, 0x140000000); // ImageBase
    write_u32(&mut img, opt + 32, section_align);
    write_u32(&mut img, opt + 36, file_align);
    write_u16(&mut img, opt + 40, 6); // MajorOS
    write_u16(&mut img, opt + 44, 6); // MajorSubsystem
    write_u32(&mut img, opt + 56, size_of_image);
    write_u32(&mut img, opt + 60, size_of_headers);
    write_u16(&mut img, opt + 68, 3); // Subsystem = CONSOLE
    write_u16(&mut img, opt + 70, 0x8160); // DllCharacteristics
    write_u64(&mut img, opt + 72, 0x100000); // Stack Reserve
    write_u64(&mut img, opt + 80, 0x1000); // Stack Commit
    write_u64(&mut img, opt + 88, 0x100000); // Heap Reserve
    write_u64(&mut img, opt + 96, 0x1000); // Heap Commit
    write_u32(&mut img, opt + 108, 16); // NumberOfRvaAndSizes

    // Section .text
    let s1 = 0x98 + 0xF0; // 0x188
    write_name(&mut img, s1, b".text");
    write_u32(&mut img, s1 + 8, text_vs);
    write_u32(&mut img, s1 + 12, text_rva);
    write_u32(&mut img, s1 + 16, code_raw);
    write_u32(&mut img, s1 + 20, headers_raw);
    write_u32(&mut img, s1 + 36, 0x60000020); // CODE | EXECUTE | READ

    // Section .data
    let s2 = s1 + 40;
    write_name(&mut img, s2, b".data");
    write_u32(&mut img, s2 + 8, data_vs);
    write_u32(&mut img, s2 + 12, data_rva);
    write_u32(&mut img, s2 + 16, data_raw);
    write_u32(&mut img, s2 + 20, headers_raw + code_raw);
    write_u32(&mut img, s2 + 36, 0xC0000040); // INIT_DATA | READ | WRITE

    // SizeOfCode / SizeOfInitializedData
    write_u32(&mut img, opt + 4, code_raw);
    write_u32(&mut img, opt + 8, data_raw);
    write_u32(&mut img, opt + 20, text_rva); // BaseOfCode

    // ── ARM64 startup stub at start of .text ──
    //   adrp x15, data_va
    //   add  x15, x15, data_va & 0xFFF
    //   b    user_code
    //   (pad to 16 bytes)
    let text_file_off = headers_raw as usize;
    let user_code_va = text_rva + startup_len; // VA of user code

    // adrp x15, data_rva (page-aligned)
    img[text_file_off..text_file_off + 4].copy_from_slice(&arm64_adrp(15, data_rva as u64));

    // add x15, x15, lo12(data_rva)
    img[text_file_off + 4..text_file_off + 8]
        .copy_from_slice(&arm64_add_imm12(15, 15, data_rva as u64 & 0xFFF));

    // b imm26 — branch to user_code_va
    // b instruction is at VA text_rva + 8, target = user_code_va
    let b_imm = ((user_code_va as i32) - ((text_rva + 8) as i32)) >> 2;
    let b_enc = 0x14000000u32 | ((b_imm as u32) & 0x03FFFFFF);
    img[text_file_off + 8..text_file_off + 12].copy_from_slice(&b_enc.to_le_bytes());

    // NOP pad (4 bytes) to reach 16 bytes
    img[text_file_off + 12..text_file_off + 16].copy_from_slice(&[0x1F, 0x20, 0x03, 0xD5]);

    // Copy user code
    let code_dst = text_file_off + startup_len as usize;
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

    // Copy data
    let data_file_off = (headers_raw + code_raw) as usize;
    let copy_n = data.len().min(data_raw as usize);
    img[data_file_off..data_file_off + copy_n].copy_from_slice(&data[..copy_n]);

    // Entry point = text_rva (startup)
    write_u32(&mut img, opt + 16, text_rva);

    Ok(PeArm64Image { bytes: img })
}

/// `adrp rd, <addr>` — PC-relative page number load (PC assumed 0 for VA).
fn arm64_adrp(rd: u32, addr: u64) -> [u8; 4] {
    let imm = ((addr >> 12) & 0x7FFFF) as u32;
    let enc: u32 = 0x90000000
        | (imm & 0x3F)
        | (((imm >> 5) & 0x1F) << 5)
        | ((rd & 0x1F) << 10)
        | (((imm >> 6) & 0xFF) << 16);
    enc.to_le_bytes()
}

/// `add rd, rn, imm12` (signed 12-bit).
fn arm64_add_imm12(rd: u32, rn: u32, imm: u64) -> [u8; 4] {
    let imm12 = (imm & 0xFFF) as u32;
    let enc: u32 = 0x91000000
        | (imm12 << 10)
        | ((rn & 0x1F) << 5)
        | (rd & 0x1F);
    enc.to_le_bytes()
}

fn align_up(v: u32, a: u32) -> u32 {
    (v + a - 1) & !(a - 1)
}

fn write_u16(buf: &mut [u8], off: usize, v: u16) {
    buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
}

fn write_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

fn write_u64(buf: &mut [u8], off: usize, v: u64) {
    buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

fn write_name(buf: &mut [u8], off: usize, name: &[u8]) {
    let n = name.len().min(8);
    buf[off..off + n].copy_from_slice(&name[..n]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arm64_pe_has_mz_and_pe() {
        let pe = link_arm64_pe(&[0xC0, 0x03, 0x5F, 0xD6], &[]).unwrap();
        assert_eq!(&pe.bytes[0..2], b"MZ");
        let lfanew = u32::from_le_bytes(pe.bytes[0x3C..0x40].try_into().unwrap()) as usize;
        assert_eq!(&pe.bytes[lfanew..lfanew + 4], b"PE\0\0");
    }

    #[test]
    fn arm64_pe_machine() {
        let pe = link_arm64_pe(&[0xC0, 0x03, 0x5F, 0xD6], &[]).unwrap();
        let machine = u16::from_le_bytes(pe.bytes[0x84..0x86].try_into().unwrap());
        assert_eq!(machine, 0xAA64);
    }

    #[test]
    fn arm64_pe_data_floor() {
        let pe = link_arm64_pe(&[0xC0, 0x03, 0x5F, 0xD6], &[]).unwrap();
        assert!(pe.bytes.len() > 0x38000);
    }
}
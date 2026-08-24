//! Win32 selfhost startup x64 + import metadata for runtime DLL sidecar.

use crate::types::{IsaError, IsaResult};

/// Strings embedded in PE .data for the startup stub.
pub const RUNTIME_DLL_NAME: &str = "yoyo_runtime.dll";

/// Fixed size of `gen_selfhost_startup` body (bytes after pe_link 13-byte header).
pub const STARTUP_BODY_SIZE: usize = 46;

/// Metadata appended to `.data` for kernel32 imports + startup string RVAs.
pub struct SelfhostMeta {
    pub dll_name_rva: u32,
    pub export_name_rva: u32,
    pub iat_rva: u32,
    pub import_dir_rva: u32,
    pub import_dir_size: u32,
}

const KERNEL32_FUNCS: &[&str] = &["LoadLibraryA", "GetProcAddress", "ExitProcess"];

/// Load prebuilt `yoyo_runtime.dll` from release/debug target dirs (built after verifier).
pub fn runtime_dll_bytes() -> IsaResult<Vec<u8>> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let candidates = [
        root.join("target/release/yoyo_runtime.dll"),
        root.join("target/debug/yoyo_runtime.dll"),
        root.join(format!("target/{}/release/yoyo_runtime.dll", current_target())),
        root.join(format!("target/{}/debug/yoyo_runtime.dll", current_target())),
    ];
    for path in &candidates {
        if path.is_file() {
            return std::fs::read(path).map_err(|e| IsaError::IoError {
                msg: format!("read {}: {e}", path.display()),
            });
        }
    }
    Err(IsaError::IoError {
        msg: "yoyo_runtime.dll not found — run `cargo build --release -p yoyo-runtime` after verifier"
            .into(),
    })
}

fn current_target() -> String {
    std::env::var("TARGET").unwrap_or_else(|_| {
        if cfg!(target_env = "msvc") {
            "x86_64-pc-windows-msvc".into()
        } else {
            "x86_64-pc-windows-gnu".into()
        }
    })
}

/// Append import directory + strings to user data; RVAs use `data_rva` as section base.
pub fn build_selfhost_metadata(user_data: &[u8], data_rva: u32) -> IsaResult<(Vec<u8>, SelfhostMeta)> {
    let mut blob = user_data.to_vec();
    while blob.len() % 16 != 0 {
        blob.push(0);
    }
    let base = blob.len();

    let kernel32_name = b"kernel32.dll\0";
    let dll_name = b"yoyo_runtime.dll\0";
    let export_name = b"yoyo_runtime_selfhost_main\0";

    let mut hint_names: Vec<Vec<u8>> = Vec::new();
    for name in KERNEL32_FUNCS {
        let mut hn = Vec::new();
        hn.extend_from_slice(&0u16.to_le_bytes());
        hn.extend_from_slice(name.as_bytes());
        hn.push(0);
        while hn.len() % 2 != 0 {
            hn.push(0);
        }
        hint_names.push(hn);
    }

    let desc_size = 40;
    let kern_off = desc_size;
    let dll_off = kern_off + kernel32_name.len();
    let export_off = dll_off + dll_name.len();

    let mut hn_off = export_off + export_name.len();
    let mut hn_rvas: Vec<u32> = Vec::new();
    for hn in &hint_names {
        hn_rvas.push(data_rva + (base + hn_off) as u32);
        hn_off += hn.len();
    }

    let n = KERNEL32_FUNCS.len();
    let ilt_off = hn_off;
    let ilt_size = (n + 1) * 8;
    let iat_off = ilt_off + ilt_size;
    let iat_size = (n + 1) * 8;
    let total = iat_off + iat_size;

    blob.resize(base + total, 0);

    let rva = |off: usize| data_rva + (base + off) as u32;

    // Import descriptors (one DLL + terminator).
    write_u32(&mut blob, base, rva(ilt_off));
    write_u32(&mut blob, base + 12, rva(kern_off));
    write_u32(&mut blob, base + 16, rva(iat_off));

    blob[base + kern_off..base + kern_off + kernel32_name.len()].copy_from_slice(kernel32_name);
    blob[base + dll_off..base + dll_off + dll_name.len()].copy_from_slice(dll_name);
    blob[base + export_off..base + export_off + export_name.len()].copy_from_slice(export_name);

    let mut off = export_off + export_name.len();
    for hn in &hint_names {
        blob[base + off..base + off + hn.len()].copy_from_slice(hn);
        off += hn.len();
    }

    for (i, &hn_rva) in hn_rvas.iter().enumerate() {
        write_u64(&mut blob, base + ilt_off + i * 8, hn_rva as u64);
        write_u64(&mut blob, base + iat_off + i * 8, hn_rva as u64);
    }

    let meta = SelfhostMeta {
        dll_name_rva: rva(dll_off),
        export_name_rva: rva(export_off),
        iat_rva: rva(iat_off),
        import_dir_rva: rva(0),
        import_dir_size: desc_size as u32,
    };

    Ok((blob, meta))
}

/// Generate x64 startup: LoadLibraryA(dll_name) → GetProcAddress → call → ExitProcess.
pub fn gen_selfhost_startup(dll_name_rva: u32, iat_rva: u32, export_rva: u32) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::new();

    // sub rsp, 0x28
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]);

    // LoadLibraryA(dll_name)
    let lea1_at = c.len();
    c.extend_from_slice(&[0x48, 0x8D, 0x0D, 0, 0, 0, 0]);
    let call1_at = c.len();
    c.extend_from_slice(&[0xFF, 0x15, 0, 0, 0, 0]);

    // mov rbx, rax
    c.extend_from_slice(&[0x48, 0x89, 0xC3]);

    // mov rcx, rbx
    c.extend_from_slice(&[0x48, 0x89, 0xD9]);

    // lea rdx, [rip + disp32]
    let lea2_at = c.len();
    c.extend_from_slice(&[0x48, 0x8D, 0x15, 0, 0, 0, 0]);
    let call2_at = c.len();
    c.extend_from_slice(&[0xFF, 0x15, 0, 0, 0, 0]);

    // call rax
    c.extend_from_slice(&[0xFF, 0xD0]);

    // mov ecx, eax
    c.extend_from_slice(&[0x89, 0xC1]);
    let call3_at = c.len();
    c.extend_from_slice(&[0xFF, 0x15, 0, 0, 0, 0]);

    let text_rva = 0x1000u32;
    let startup_off = 13u32;
    let fix = |c: &mut Vec<u8>, insn_end: usize, target_rva: u32| {
        let insn_start = insn_end - 4;
        let next = (text_rva + startup_off + insn_end as u32) as i32;
        let disp = target_rva as i32 - next;
        c[insn_start..insn_start + 4].copy_from_slice(&disp.to_le_bytes());
    };

    fix(&mut c, lea1_at + 7, dll_name_rva);
    fix(&mut c, call1_at + 6, iat_rva);
    fix(&mut c, lea2_at + 7, export_rva);
    fix(&mut c, call2_at + 6, iat_rva + 8);
    fix(&mut c, call3_at + 6, iat_rva + 16);

    assert_eq!(c.len(), STARTUP_BODY_SIZE);
    c
}

fn write_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

fn write_u64(buf: &mut [u8], off: usize, v: u64) {
    buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_body_size_fixed() {
        let s = gen_selfhost_startup(0x20000, 0x21000, 0x22000);
        assert_eq!(s.len(), STARTUP_BODY_SIZE);
    }

    #[test]
    fn metadata_layout() {
        let (_, meta) = build_selfhost_metadata(&[1, 2, 3], 0x20000).unwrap();
        assert!(meta.dll_name_rva >= 0x20000);
        assert!(meta.iat_rva > meta.dll_name_rva);
        assert_eq!(meta.import_dir_size, 40);
    }
}

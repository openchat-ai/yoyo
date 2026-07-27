//! Scratch: one-shot yoyo.ty emit compare via executor
use std::fs;
fn main() {
    let root = std::env::current_dir().unwrap();
    let ty_path = root.join("yoyo/projects/yoyo.ty");
    let out_path = root.join("scripts/_probe/rs_yoyoty_h40.code.bin");
    let src = fs::read_to_string(&ty_path).unwrap();
    let compiled = yoyo_verifier::executor::compile_ty_source(&src, yoyo_verifier::platform::PlatformKind::Stub).unwrap();
    let hex: String = compiled.code.iter().map(|b| format!("{:02x}", b)).collect();
    fs::write(&out_path, &compiled.code).unwrap();
    println!("rs_yoyoty_h40:");
    println!("  len={}", compiled.code.len());
    println!("  full_hex={}", hex);
}

// Body-extend-001 Rust independent derivation.
// Use the public assembler API to compute bytes for `0x62 0x50 0x03`.
// = emit_add_imm(0x50, 3) + ret()
// = load_state(0x50, rax) + add_imm(rax, 3) + store_state(0x50, rax) + ret()
use yoyo_verifier::assembler::{emit_add_imm, load_state, store_state, add_imm, ret, Reg};

fn main() {
    let mut v = Vec::new();
    v.extend(load_state(0x50, Reg::Rax).expect("load_state"));
    v.extend(add_imm(Reg::Rax, 3).expect("add_imm"));
    v.extend(store_state(0x50, Reg::Rax).expect("store_state"));
    v.extend(ret());
    println!("Rust manual stream: {}", v.iter().map(|b| format!("{:02x}", b)).collect::<String>());
    println!("Rust manual len: {}", v.len());

    let mut v2 = emit_add_imm(0x50, 3).expect("emit_add_imm");
    v2.extend(ret());
    println!("Rust helper stream: {}", v2.iter().map(|b| format!("{:02x}", b)).collect::<String>());
    println!("Rust helper len: {}", v2.len());

    // Verify they match
    assert_eq!(v, v2, "manual vs helper must match");
    println!("Rust: manual ≡ helper ✓");
}

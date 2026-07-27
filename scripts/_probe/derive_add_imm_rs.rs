// Body-extend-001 Rust derivation: independent compute of expected bytes
// for `0x62 0x50 0x03` (ADD S[0x50] += 3) at H_2E.
// Compose via assembler::emit_add_imm(0x50, 3) + ret()
fn main() {
    use yoyo_verifier::assembler::{emit_add_imm, ret};
    use yoyo_verifier::types::Reg;
    let mut v = emit_add_imm(0x50, 3).expect("emit_add_imm");
    v.extend(ret());
    println!("Rust stream: {}", v.iter().map(|b| format!("{:02x}", b)).collect::<String>());
    println!("Rust len: {}", v.len());
}

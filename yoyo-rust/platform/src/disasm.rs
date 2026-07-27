//! Hex dump helper.

pub fn hex_dump(bytes: &[u8]) -> String {
    let mut out = String::new();
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let hex: Vec<String> = chunk.iter().map(|b| format!("{b:02X}")).collect();
        out.push_str(&format!("{:04X}: {}\n", i * 16, hex.join(" ")));
    }
    out
}

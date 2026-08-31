//! Pin Windows PE export order so H_00 stub can resolve AddressOfFunctions[0]
//! (`yoyo_runtime_selfhost_main` must be the first export).

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let def = out.join("yoyo_runtime_exports.def");
    std::fs::write(
        &def,
        "EXPORTS\nyoyo_runtime_selfhost_main\nyoyo_runtime_selfhost_paths\n",
    )
    .expect("write exports.def");
    println!("cargo:rustc-cdylib-link-arg=/DEF:{}", def.display());
    // Manual-map smoke calls export without DllMain — disable /GS stack cookies (else AV).
    println!("cargo:rustc-link-arg=/GS-");
}

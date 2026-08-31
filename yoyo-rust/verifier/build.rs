fn main() {
    // `option_env!("H00_BISECT_EXIT")` in h00_manual_map_wireup.rs — rebuild when CI bisect changes.
    println!("cargo:rerun-if-env-changed=H00_BISECT_EXIT");
}

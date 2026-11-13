use std::path::PathBuf;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path_to_ld = PathBuf::from(manifest_dir).join("linker.ld");
    println!("cargo::rustc-link-arg=-T{}", path_to_ld.display());
    println!("cargo::rerun-if-changed=linker.ld");
}

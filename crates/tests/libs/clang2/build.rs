fn main() {
    println!("cargo:rerun-if-env-changed=LIBCLANG_PATH");
    let path = std::env::var_os("LIBCLANG_PATH")
        .map_or_else(helpers::libclang_dir, std::path::PathBuf::from);
    println!("cargo:rustc-env=LIBCLANG_PATH={}", path.display());
}

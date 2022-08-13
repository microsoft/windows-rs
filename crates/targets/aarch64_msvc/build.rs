fn main() {
    let target = std::env::var("TARGET").unwrap();
    if !["aarch64-pc-windows-msvc", "aarch64-uwp-windows-msvc", "aarch64-pc-windows-gnullvm"].contains(&&*target) {
        return;
    }

    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}", std::path::Path::new(&dir).join("lib").display());
}

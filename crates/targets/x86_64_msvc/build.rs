fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target != "x86_64-pc-windows-msvc" && target != "x86_64-uwp-windows-msvc" {
        return;
    }

    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    println!(
        "cargo:rustc-link-search=native={}",
        std::path::Path::new(&dir).join("lib").display()
    );
}

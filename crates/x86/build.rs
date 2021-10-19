fn main() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    println!(
        "cargo:rustc-link-search=native={}",
        path.join("lib").display()
    );
}

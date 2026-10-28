fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let bindings = format!("{out_dir}/bindings.rs");

    windows_bindgen::builder()
        .output(&bindings)
        .filter("CONTEXT")
        .filter("M128A")
        .sys()
        .flat()
        .write();
}

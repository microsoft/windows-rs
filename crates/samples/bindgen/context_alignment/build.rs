fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let bindings = format!("{out_dir}/bindings.rs");

    windows_bindgen::builder()
        .input("default")
        .output(&bindings)
        .filter("CONTEXT")
        .sys()
        .flat()
        .write();
}

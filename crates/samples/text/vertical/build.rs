fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    windows_bindgen::builder()
        .output(std::path::Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("bindings.rs"))
        .filters([
            "RECT",
            "GetDpiForWindow",
            "SetWindowPos",
            "SWP_NOACTIVATE",
            "SWP_NOZORDER",
            "WM_DPICHANGED",
            "WM_MOUSEMOVE",
            "WM_LBUTTONDOWN",
            "WM_CHAR",
        ])
        .flat()
        .minimal()
        .write();
}

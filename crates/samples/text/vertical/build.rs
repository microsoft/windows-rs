fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    windows_bindgen::builder()
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
        .write();
}

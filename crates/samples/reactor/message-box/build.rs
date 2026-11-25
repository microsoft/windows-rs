fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows_bindgen::builder()
        .sys()
        .filters([
            "MessageBoxW",
            "MB_YESNO",
            "MB_ICONQUESTION",
            "IDYES",
            "IDNO",
        ])
        .write();
}

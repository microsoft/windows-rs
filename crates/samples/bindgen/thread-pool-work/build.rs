fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows_bindgen::builder()
        .sys()
        .filters([
            "CreateThreadpoolWork",
            "SubmitThreadpoolWork",
            "WaitForThreadpoolWorkCallbacks",
            "CloseThreadpoolWork",
        ])
        .write();
}

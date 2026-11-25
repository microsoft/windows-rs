fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows_bindgen::builder()
        .sys()
        .filters(["SERVICE_CONTROL_PAUSE", "SERVICE_CONTROL_CONTINUE"])
        .write();
}

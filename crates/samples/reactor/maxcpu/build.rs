fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows_reactor_setup::as_self_contained();

    windows_bindgen::builder()
        .output(std::path::Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("bindings.rs"))
        .filters(["SetThreadPriority", "THREAD_PRIORITY_LOWEST"])
        .minimal()
        .flat()
        .write();
}

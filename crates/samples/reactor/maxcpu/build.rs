fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows_reactor_setup::as_self_contained();

    windows_bindgen::builder()
        .filters([
            "SetThreadPriority",
            "THREAD_PRIORITY_LOWEST",
            "VirtualAlloc",
            "VirtualFree",
            "MEM_COMMIT",
            "MEM_RESERVE",
            "MEM_RELEASE",
            "PAGE_READWRITE",
        ])
        .write();
}

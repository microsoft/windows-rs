fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows_bindgen::builder()
        .filters([
            "MemoryBuffer::{Create,CreateReference}",
            "IMemoryBufferReference::get_Capacity",
            "IMemoryBufferByteAccess::GetBuffer",
        ])
        .write();
}

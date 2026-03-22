fn main() {
    let _ = windows_bindgen::bindgen([
        "--in",
        "default",
        "--out",
        "tests/bindings.rs",
        "--filter",
        "LoadLibraryExA",
        "LOAD_LIBRARY_AS_IMAGE_RESOURCE",
    ]);
}

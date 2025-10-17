fn main() {
    windows_bindgen::bindgen([
        "--out",
        "src/bindings.rs",
        "--filter",
        "WebAuthNAuthenticatorMakeCredential",
        "WebAuthNAuthenticatorGetAssertion",
        "LoadLibraryExA",
        "GetProcAddress",
        "FreeLibrary",
        "LOAD_LIBRARY_SEARCH_DEFAULT_DIRS",
        "--sys",
        "--sys-fn-ptrs",
        "--no-comment",
        "--no-deps",
    ])
    .unwrap();
}

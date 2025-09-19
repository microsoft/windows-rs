fn main() {
    windows_bindgen::bindgen([
        "--out",
        "src/bindings.rs",
        "--filter",
        "CoGetCallerTID", // windows_result::{HRESULT, Result}
        "IsCharLowerA",   // windows_result::BOOL
        "SysFreeString",  // windows_strings
        "--flat",
        "--no-comment",
        "--specific-deps",
    ])
    .unwrap();
}

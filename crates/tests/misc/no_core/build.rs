fn main() {
    windows_bindgen::bindgen([
        "--out",
        "src/bindings.rs",
        "--filter",
        "CoGetCallerTID",               // windows_result::{HRESULT, Result}
        "IsCharLowerA",                 // windows_result::BOOL
        "SysFreeString",                // windows_strings::BSTR
        "WindowsStringHasEmbeddedNull", // windows_strings::HSTRING
        "WindowsGetStringRawBuffer",    // windows_strings::PCWSTR
        "--flat",
        "--no-comment",
        "--specific-deps",
    ])
    .unwrap();
}

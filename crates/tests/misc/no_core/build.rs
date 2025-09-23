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
        "GetLastError",                 // windows_result::WIN32_ERROR
        "VhfStart",                     // windows_result::NTSTATUS
        "RpcMgmtEnableIdleCleanup",     // windows_result::RPC_STATUS
        "--flat",
        "--no-comment",
        "--specific-deps",
    ])
    .unwrap();
}

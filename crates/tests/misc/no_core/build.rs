fn main() {
    let mut filters = vec![
        "CoGetCallerTID",               // windows_result::{HRESULT, Result}
        "IsCharLowerA",                 // windows_result::BOOL
        "WindowsStringHasEmbeddedNull", // windows_strings::HSTRING
        "WindowsGetStringRawBuffer",    // windows_strings::PCWSTR
        "GetLastError",                 // windows_result::WIN32_ERROR
        "VhfStart",                     // windows_result::NTSTATUS
        "RpcMgmtEnableIdleCleanup",     // windows_result::RPC_STATUS
    ];
    if cfg!(windows) {
        filters.push("SysFreeString"); // windows_strings::BSTR
    }
    windows_bindgen::bindgen(
        ["--out", "src/bindings.rs", "--filter"]
            .iter()
            .chain(filters.iter())
            .chain(["--flat", "--no-comment", "--specific-deps"].iter()),
    )
    .unwrap();
}

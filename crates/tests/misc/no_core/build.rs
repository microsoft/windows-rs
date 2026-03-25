fn main() {
    windows_bindgen::bindgen([
        "--out",
        "src/bindings.rs",
        "--filter",
        "Windows.Win32.System.Com.CoGetCallerTID",               // windows_result::{HRESULT, Result}
        "Windows.Win32.UI.WindowsAndMessaging.IsCharLowerA",     // windows_result::BOOL
        "Windows.Win32.Foundation.SysFreeString",                // windows_strings::BSTR
        "Windows.Win32.System.WinRT.WindowsStringHasEmbeddedNull", // windows_strings::HSTRING
        "Windows.Win32.System.WinRT.WindowsGetStringRawBuffer",  // windows_strings::PCWSTR
        "Windows.Win32.Foundation.GetLastError",                 // windows_result::WIN32_ERROR
        "Windows.Wdk.Devices.HumanInterfaceDevice.VhfStart",    // windows_result::NTSTATUS
        "Windows.Win32.System.Rpc.RpcMgmtEnableIdleCleanup",    // windows_result::RPC_STATUS
        "--flat",
        "--no-comment",
        "--specific-deps",
    ])
    .unwrap();
}

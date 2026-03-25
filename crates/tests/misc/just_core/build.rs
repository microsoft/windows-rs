fn main() {
    windows_bindgen::bindgen([
        "--out",
        "src/bindings.rs",
        "--filter",
        "Windows.Win32.System.Com.CoGetCallerTID",
        "Windows.Win32.UI.WindowsAndMessaging.IsCharLowerA",
        "Windows.Win32.Foundation.SysFreeString",
        "Windows.Foundation.IStringable",
        "Windows.Win32.Foundation.GetLastError",
        "Windows.Wdk.Devices.HumanInterfaceDevice.VhfStart",
        "Windows.Win32.System.Rpc.RpcMgmtEnableIdleCleanup",
        "--flat",
        "--no-comment",
    ])
    .unwrap();
}

fn main() {
    windows_bindgen2::builder()
        .output("src/bindings.rs")
        .filters([
            "CoGetCallerTID",
            "IsCharLowerA",
            "SysFreeString",
            "IStringable",
            "GetLastError",
            "HidD_GetHidGuid",
            "RpcMgmtEnableIdleCleanup",
        ])
        .flat()
        .write()
        .unwrap();
}

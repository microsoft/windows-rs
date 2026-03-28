fn main() {
    let mut filters = vec![
        "CoGetCallerTID",
        "IsCharLowerA",
        "IStringable",
        "GetLastError",
        "VhfStart",
        "RpcMgmtEnableIdleCleanup",
    ];
    if cfg!(windows) {
        filters.push("SysFreeString");
    }
    windows_bindgen::bindgen(
        ["--out", "src/bindings.rs", "--filter"]
            .iter()
            .chain(filters.iter())
            .chain(["--flat", "--no-comment"].iter()),
    )
    .unwrap();
}

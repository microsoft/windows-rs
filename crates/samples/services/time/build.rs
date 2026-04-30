fn main() {
    windows_bindgen::builder()
        .output("src/bindings.rs")
        .flat()
        .sys()
        .no_deps()
        .filters([
            "SERVICE_ACCEPT_TIMECHANGE",
            "SERVICE_CONTROL_TIMECHANGE",
            "SERVICE_TIMECHANGE_INFO",
            "FileTimeToSystemTime",
            "FileTimeToLocalFileTime",
        ])
        .derives(["SYSTEMTIME=Debug", "SERVICE_TIMECHANGE_INFO=Debug"])
        .write()
        .unwrap();
}

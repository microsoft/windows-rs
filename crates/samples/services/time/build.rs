fn main() {
    let bindings = format!("{}/bindings.rs", std::env::var("OUT_DIR").unwrap());

    windows_bindgen::builder()
        .output(bindings)
        .flat()
        .sys()
        .filters([
            "SERVICE_ACCEPT_TIMECHANGE",
            "SERVICE_CONTROL_TIMECHANGE",
            "SERVICE_TIMECHANGE_INFO",
            "FileTimeToSystemTime",
            "FileTimeToLocalFileTime",
        ])
        .derives(["SYSTEMTIME=Debug", "SERVICE_TIMECHANGE_INFO=Debug"])
        .write();
}

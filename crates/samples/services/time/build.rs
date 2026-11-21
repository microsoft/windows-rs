fn main() {
    windows_bindgen::builder()
        .sys()
        .filters([
            "SERVICE_ACCEPT_TIMECHANGE",
            "SERVICE_CONTROL_TIMECHANGE",
            "SERVICE_TIMECHANGE_INFO",
            "FileTimeToSystemTime",
            "FileTimeToLocalFileTime",
        ])
        .derives(["SYSTEMTIME=Debug"])
        .write();
}

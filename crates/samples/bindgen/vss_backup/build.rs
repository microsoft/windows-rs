fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let bindings = format!("{out_dir}/bindings.rs");

    windows_bindgen::builder()
        .output(&bindings)
        .filters([
            "CreateVssBackupComponentsInternal",
            "CoIncrementMTAUsage",
            "VSS_BT_FULL",
        ])
        .flat()
        .write();
}

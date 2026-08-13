fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let bindings = format!("{out_dir}/bindings.rs");

    windows_bindgen2::builder()
        .output(&bindings)
        .filters([
            "CreateVssBackupComponentsInternal",
            "VSS_BT_FULL",
            "IVssBackupComponents",
        ])
        .flat()
        .write()
        .unwrap();
}

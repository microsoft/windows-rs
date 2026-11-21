fn main() {
    windows_bindgen::builder()
        .filters([
            "CreateVssBackupComponentsInternal",
            "VSS_BT_FULL",
            "IVssBackupComponents",
        ])
        .write();
}

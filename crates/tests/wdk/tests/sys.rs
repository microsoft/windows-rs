use windows_sys::Wdk::System::OfflineRegistry::*;

#[test]
fn offline_registry() {
    unsafe {
        let mut hive = 0;
        ORCreateHive(&mut hive);
        ORCloseHive(hive);
    }
}

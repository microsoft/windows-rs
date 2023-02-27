use windows_sys::Wdk::System::OfflineRegistry::*;

#[test]
fn test() {
    unsafe {
        let mut hive = 0;
        ORCreateHive(&mut hive);
        ORCloseHive(hive);
    }
}

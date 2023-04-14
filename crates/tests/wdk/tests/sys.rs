use windows_sys::{Wdk::System::OfflineRegistry::*, Wdk::System::SystemServices::*};

#[test]
fn offline_registry() {
    unsafe {
        let mut hive = 0;
        ORCreateHive(&mut hive);
        ORCloseHive(hive);
    }
}

// TODO: need the corresponding windows crate test once https://github.com/microsoft/wdkmetadata/issues/31 is fixed.
#[test]
fn ntoskrnl() {
    unsafe {
        assert_eq!(ExIsSoftBoot(), 0);
    }
}

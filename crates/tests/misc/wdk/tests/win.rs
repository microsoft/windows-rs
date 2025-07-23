use windows::{core::*, Wdk::System::OfflineRegistry::*};

#[test]
fn test() -> Result<(), HRESULT> {
    unsafe {
        let mut hive = ORHKEY::default();
        ORCreateHive(&mut hive).ok()?;
        ORCloseHive(hive).ok()?;
        Ok(())
    }
}

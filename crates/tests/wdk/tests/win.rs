use windows::{core::*, Wdk::System::OfflineRegistry::*};

#[test]
fn test() -> Result<()> {
    unsafe {
        let mut hive = ORHKEY::default();
        ORCreateHive(&mut hive)?;
        ORCloseHive(hive)?;
        Ok(())
    }
}

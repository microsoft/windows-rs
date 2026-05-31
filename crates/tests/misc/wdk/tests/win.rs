#![cfg(windows)]
use windows::{Wdk::System::OfflineRegistry::*, core::*};

#[test]
fn test() -> Result<()> {
    unsafe {
        let mut hive = ORHKEY::default();
        ORCreateHive(&mut hive).ok()?;
        ORCloseHive(hive).ok()?;
        Ok(())
    }
}

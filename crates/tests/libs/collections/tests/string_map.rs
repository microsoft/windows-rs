#![cfg(windows)]
// `StringMap` is a runtime-activated WinRT class; it requires the WinRT
// activation factory linker symbols and therefore only runs on Windows.

use windows::{core::*, Foundation::Collections::*};

#[test]
fn test() -> Result<()> {
    let map = StringMap::new()?;
    map.Insert(h!("key"), h!("value"))?;
    assert!(map.HasKey(h!("key"))?);
    assert!(map.HasKey(&HSTRING::from("key"))?);

    Ok(())
}

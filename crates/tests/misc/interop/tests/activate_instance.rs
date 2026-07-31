#![cfg(windows)]
use windows::{
    Foundation::Collections::StringMap,
    Win32::COINIT_MULTITHREADED,
    Win32::CoInitializeEx,
    Win32::RoActivateInstance,
    core::{HSTRING, Interface, Result},
};

#[test]
fn test() -> Result<()> {
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED as u32).ok()? };

    let instance: HSTRING = "Windows.Foundation.Collections.StringMap".into();
    let instance = unsafe { RoActivateInstance(&instance)? };

    let map = instance.cast::<StringMap>()?;
    let key: HSTRING = "hello".into();
    let value: HSTRING = "world".into();
    map.Insert(&key, &value)?;
    assert_eq!(map.Lookup(&key)?, "world");

    Ok(())
}

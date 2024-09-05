use windows::{
    core::{Interface, Result, HSTRING},
    Foundation::Collections::StringMap,
    Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED},
    Win32::System::WinRT::RoActivateInstance,
};

// Calling RoActivateInstance is a useful interop test because it is a function defined by Win32 metadata
// but refers to three types that are intrinsic to WinRT and thus directly mapped to type in the Windows
// crate. Calling RoActivateInstance in production code is discouraged. Instead, let the Windows crate
// activate WinRT types directly as it can do so far more efficiently.
#[test]
fn test() -> Result<()> {
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok()? };

    let instance: HSTRING = "Windows.Foundation.Collections.StringMap".into();
    let instance = unsafe { RoActivateInstance(&instance)? };

    let map = instance.cast::<StringMap>()?;
    let key: HSTRING = "hello".into();
    let value: HSTRING = "world".into();
    map.Insert(&key, &value)?;
    assert_eq!(map.Lookup(&key)?, "world");

    Ok(())
}

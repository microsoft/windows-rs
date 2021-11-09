use test_interop::{
    Windows::Foundation::Collections::StringMap,
    Windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED},
    Windows::Win32::System::WinRT::RoActivateInstance,
};

use windows::runtime::{Interface, Result};

// Calling RoActivateInstance is a useful interop test because it is a function defined by Win32 metadata
// but refers to three types that are intrinsic to WinRT and thus directly mapped to type in the Windows
// crate. Calling RoActivateInstance in production code is discouraged. Instead, let the Windows crate
// activate WinRT types directly as it can do so far more efficiently.
#[test]
fn test() -> Result<()> {
    unsafe { CoInitializeEx(core::ptr::null_mut(), COINIT_MULTITHREADED)? };

    let instance = unsafe { RoActivateInstance("Windows.Foundation.Collections.StringMap")? };

    let map = instance.cast::<StringMap>()?;
    map.Insert("hello", "world")?;
    assert_eq!(map.Lookup("hello")?, "world");

    Ok(())
}

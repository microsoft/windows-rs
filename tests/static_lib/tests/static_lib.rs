use test_static_lib::{
    Microsoft::Web::WebView2::Win32::CompareBrowserVersions,
    Windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED},
};

use windows::Result;

#[test]
fn test_less_than() -> Result<()> {
    unsafe { CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED)? };
    let mut result: i32 = 0;
    unsafe { CompareBrowserVersions("86.0.622.0", "86.0.623.0", &mut result)? };
    assert!(result < 0);
    Ok(())
}

#[test]
fn test_greater_than() -> Result<()> {
    unsafe { CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED)? };
    let mut result: i32 = 0;
    unsafe { CompareBrowserVersions("86.1.0.0", "86.0.622.0", &mut result)? };
    assert!(result > 0);
    Ok(())
}

#[test]
fn test_equal() -> Result<()> {
    unsafe { CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED)? };
    let mut result: i32 = 0x12345678;
    unsafe { CompareBrowserVersions("86.0.622.0", "86.0.622.0", &mut result)? };
    assert_eq!(result, 0);
    Ok(())
}

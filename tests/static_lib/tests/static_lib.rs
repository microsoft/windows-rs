use test_static_lib::Microsoft::Web::WebView2::Win32::CompareBrowserVersions;
use windows::{initialize_sta, Result};

#[test]
fn test_less_than() -> Result<()> {
    initialize_sta()?;
    let mut result: i32 = 0;
    unsafe { CompareBrowserVersions("86.0.622.0", "86.0.623.0", &mut result)? };
    assert!(result < 0);
    Ok(())
}

#[test]
fn test_greater_than() -> Result<()> {
    initialize_sta()?;
    let mut result: i32 = 0;
    unsafe { CompareBrowserVersions("86.1.0.0", "86.0.622.0", &mut result)? };
    assert!(result > 0);
    Ok(())
}

#[test]
fn test_equal() -> Result<()> {
    initialize_sta()?;
    let mut result: i32 = 0x12345678;
    unsafe { CompareBrowserVersions("86.0.622.0", "86.0.622.0", &mut result)? };
    assert_eq!(result, 0);
    Ok(())
}

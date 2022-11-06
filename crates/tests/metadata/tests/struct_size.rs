#[test]
fn test_windows() {
    use windows::Win32::Graphics::Gdi::*;

    assert_eq!(156, std::mem::size_of::<DEVMODEA>());
    assert_eq!(220, std::mem::size_of::<DEVMODEW>());
}

#[test]
fn test_sys() {
    use windows_sys::Win32::Graphics::Gdi::*;

    assert_eq!(156, std::mem::size_of::<DEVMODEA>());
    assert_eq!(220, std::mem::size_of::<DEVMODEW>());
}

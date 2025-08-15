#[test]
fn test_windows() {
    use windows::Win32::Graphics::Gdi::*;

    assert_eq!(156, size_of::<DEVMODEA>());
    assert_eq!(220, size_of::<DEVMODEW>());

    assert_eq!(4, align_of::<DEVMODEA>());
    assert_eq!(4, align_of::<DEVMODEW>());
}

#[test]
fn test_sys() {
    use windows_sys::Win32::Graphics::Gdi::*;

    assert_eq!(156, size_of::<DEVMODEA>());
    assert_eq!(220, size_of::<DEVMODEW>());

    assert_eq!(4, align_of::<DEVMODEA>());
    assert_eq!(4, align_of::<DEVMODEW>());
}

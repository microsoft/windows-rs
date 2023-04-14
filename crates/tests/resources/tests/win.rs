use windows::{
    core::*, Win32::Foundation::*, Win32::System::Com::*, Win32::UI::Controls::*,
    Win32::UI::WindowsAndMessaging::*,
};

/// These tests ensure `MAKEINTRESOURCEW` style constants an in particular negative constants like TD_ERROR_ICON
/// work as expected.
#[test]
fn win() -> Result<()> {
    unsafe {
        assert_eq!(IDI_APPLICATION.0 as u16, 32512);
        LoadIconW(None, IDI_APPLICATION)?;

        assert_eq!(TD_ERROR_ICON.0 as i16, -2);
        assert_eq!(
            LoadIconW(None, TD_ERROR_ICON).unwrap_err().code(),
            ERROR_RESOURCE_TYPE_NOT_FOUND.into()
        );

        assert_eq!(COLE_DEFAULT_PRINCIPAL.0 as usize, usize::MAX);

        Ok(())
    }
}

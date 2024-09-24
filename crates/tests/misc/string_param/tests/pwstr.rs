use windows::{core::*, Win32::UI::Shell::*};

#[test]
fn convert() {
    unsafe {
        let pcwstr: PCWSTR = w!("https://github.com/microsoft");
        let pwstr = PWSTR(pcwstr.0 as _);
        let pcstr: PCSTR = s!("https://github.com/microsoft");
        let pstr = PSTR(pcstr.0 as _);

        assert_eq!(0, UrlCompareW(pcwstr, pwstr, true));
        assert_eq!(0, UrlCompareA(pcstr, pstr, true));
    }
}

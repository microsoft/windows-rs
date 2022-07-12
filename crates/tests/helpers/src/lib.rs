use windows::core::HSTRING;
use windows::Win32::Globalization::{SetThreadPreferredUILanguages, MUI_LANGUAGE_NAME};

pub fn set_thread_ui_language(language_tag: &str) -> bool {
    unsafe {
        let mut _languages_set = 0;
        let tag: &HSTRING = &format!("{}\0\0", language_tag).into();
        SetThreadPreferredUILanguages(MUI_LANGUAGE_NAME, tag, _languages_set as *mut _).as_bool()
    }
}

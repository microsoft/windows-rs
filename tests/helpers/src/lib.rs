windows::include_bindings!();

use crate::Windows::Win32::Globalization::{SetThreadPreferredUILanguages, MUI_LANGUAGE_NAME};

pub fn set_thread_ui_language(language_tag: &str) -> bool {
    unsafe {
        let mut _languages_set = 0;
        SetThreadPreferredUILanguages(
            MUI_LANGUAGE_NAME,
            format!("{}\0\0", language_tag),
            _languages_set as *mut _,
        )
        .as_bool()
    }
}

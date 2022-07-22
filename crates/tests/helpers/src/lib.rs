use windows::core::*;
use windows::Win32::Globalization::*;

pub fn set_thread_ui_language() -> bool {
    unsafe { SetThreadPreferredUILanguages(MUI_LANGUAGE_NAME, w!("en-US"), None).as_bool() }
}

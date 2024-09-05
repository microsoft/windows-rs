// Enables testing without pulling in a dependency on the `windows` crate.
windows_targets::link!("kernel32.dll" "system" fn SetThreadPreferredUILanguages(flags : u32, language : *const u16, _ : *mut u32) -> i32);
pub const MUI_LANGUAGE_NAME: u32 = 8u32;

pub fn set_thread_ui_language() {
    let language: Vec<_> = "en-US".encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        assert_eq!(
            1,
            SetThreadPreferredUILanguages(
                MUI_LANGUAGE_NAME,
                language.as_ptr(),
                std::ptr::null_mut()
            )
        );
    }
}

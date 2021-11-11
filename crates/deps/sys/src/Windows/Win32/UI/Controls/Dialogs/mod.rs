#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ChooseColorA();
    fn ChooseColorW();
    fn ChooseFontA();
    fn ChooseFontW();
    fn CommDlgExtendedError();
    fn FindTextA();
    fn FindTextW();
    fn GetFileTitleA();
    fn GetFileTitleW();
    fn GetOpenFileNameA();
    fn GetOpenFileNameW();
    fn GetSaveFileNameA();
    fn GetSaveFileNameW();
    fn PageSetupDlgA();
    fn PageSetupDlgW();
    fn PrintDlgA();
    fn PrintDlgExA();
    fn PrintDlgExW();
    fn PrintDlgW();
    fn ReplaceTextA();
    fn ReplaceTextW();
}

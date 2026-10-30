fn main() -> windows_core::Result<()> {
    use windows::Win32::*;
    use windows_core::w;

    windows_core::init_mta()?;

    unsafe {
        let dialog: IFileSaveDialog = CoCreateInstance(&FileSaveDialog, None, CLSCTX_ALL as u32)?;

        let filters = [
            COMDLG_FILTERSPEC {
                pszName: w!("Text files"),
                pszSpec: w!("*.txt"),
            },
            COMDLG_FILTERSPEC {
                pszName: w!("All files"),
                pszSpec: w!("*.*"),
            },
        ];
        dialog.SetFileTypes(&filters).ok()?;

        if dialog.Show(None).is_ok() {
            let result = dialog.GetResult()?;
            let mut path = windows_core::PWSTR::null();
            result.GetDisplayName(SIGDN_FILESYSPATH, &mut path).ok()?;
            println!("user picked: {}", path.display());
            CoTaskMemFree(path.0 as _);
        } else {
            println!("user canceled");
        }

        Ok(())
    }
}

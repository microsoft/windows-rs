fn main() -> windows::core::Result<()> {
    use windows::{Win32::combaseapi::*, Win32::shobjidl_core::*, Win32::shtypes::*, core::*};

    unsafe {
        CoIncrementMTAUsage()?;

        let dialog: IFileSaveDialog = CoCreateInstance(&FileSaveDialog, None, CLSCTX_ALL)?;

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
        dialog
            .SetFileTypes(filters.len() as u32, filters.as_ptr())
            .ok()?;

        if dialog.Show(None).is_ok() {
            let result = dialog.GetResult()?;
            let path = result.GetDisplayName(SIGDN_FILESYSPATH)?;
            println!("user picked: {}", path.display());
            CoTaskMemFree(path.0 as _);
        } else {
            println!("user canceled");
        }

        Ok(())
    }
}

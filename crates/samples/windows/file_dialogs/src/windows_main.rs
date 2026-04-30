use windows::{core::*, Win32::System::Com::*, Win32::UI::Shell::Common::*, Win32::UI::Shell::*};

fn main() -> Result<()> {
    unsafe {
        CoIncrementMTAUsage()?;

        let dialog: IFileSaveDialog = CoCreateInstance(&FileSaveDialog, None, CLSCTX_ALL)?;

        dialog.SetFileTypes(&[
            COMDLG_FILTERSPEC {
                pszName: w!("Text files"),
                pszSpec: w!("*.txt"),
            },
            COMDLG_FILTERSPEC {
                pszName: w!("All files"),
                pszSpec: w!("*.*"),
            },
        ])?;

        if dialog.Show(None).is_ok() {
            let result = dialog.GetResult()?;
            let path = result.GetDisplayName(SIGDN_FILESYSPATH)?;
            println!("user picked: {}", path.display());
            CoTaskMemFree(Some(path.0 as _));
        } else {
            println!("user canceled");
        }

        Ok(())
    }
}

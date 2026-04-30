use windows::{core::*, Win32::Foundation::*, Win32::UI::Controls::*};

fn main() -> Result<()> {
    unsafe {
        let mut config = TASKDIALOGCONFIG {
            cbSize: size_of::<TASKDIALOGCONFIG>() as _,
            ..Default::default()
        };

        let buttons = [TASKDIALOG_BUTTON {
            nButtonID: 123,
            pszButtonText: w!("Let's do it"),
        }];

        config.pszWindowTitle = w!("Window title");
        config.pszMainInstruction = w!("Main instruction");
        config.pszContent = w!("Content");
        config.pButtons = buttons.as_ptr();
        config.cButtons = buttons.len() as _;
        config.pfCallback = Some(callback);

        config.dwFlags =
            TASKDIALOG_FLAGS(TDF_USE_COMMAND_LINKS.0 | TDF_ALLOW_DIALOG_CANCELLATION.0);

        let mut selection = 0;

        TaskDialogIndirect(&config, Some(&mut selection), None, None)?;

        if selection == buttons[0].nButtonID {
            println!("custom button");
        };

        Ok(())
    }
}

extern "system" fn callback(
    _: HWND,
    notification: TASKDIALOG_NOTIFICATIONS,
    _: WPARAM,
    _: LPARAM,
    _: isize,
) -> HRESULT {
    if notification == TDN_BUTTON_CLICKED {
        println!("button clicked");
    }

    HRESULT(0)
}

use windows_sys::{core::*, Win32::Foundation::*, Win32::UI::Controls::*};

fn main() {
    unsafe {
        let mut config = TASKDIALOGCONFIG {
            cbSize: size_of::<TASKDIALOGCONFIG>() as _,
            ..std::mem::zeroed()
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
        config.dwFlags = TDF_USE_COMMAND_LINKS | TDF_ALLOW_DIALOG_CANCELLATION;

        let mut selection = 0;

        TaskDialogIndirect(
            &config,
            &mut selection,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );

        if selection == buttons[0].nButtonID {
            println!("custom button");
        };
    }
}

extern "system" fn callback(_: HWND, notification: u32, _: WPARAM, _: LPARAM, _: isize) -> HRESULT {
    if notification == TDN_BUTTON_CLICKED as _ {
        println!("button clicked");
    }

    0
}

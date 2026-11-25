#[expect(nonstandard_style, clippy::upper_case_acronyms)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    use bindings::*;
    use windows_strings::w;

    extern "system" fn callback(_: HWND, notification: u32, _: WPARAM, _: LPARAM, _: isize) -> i32 {
        if notification == TDN_BUTTON_CLICKED as _ {
            println!("button clicked");
        }

        0
    }

    unsafe {
        let mut config = TASKDIALOGCONFIG {
            cbSize: size_of::<TASKDIALOGCONFIG>() as _,
            ..std::mem::zeroed()
        };

        let buttons = [TASKDIALOG_BUTTON {
            nButtonID: 123,
            pszButtonText: w!("Let's do it").as_ptr(),
        }];

        config.pszWindowTitle = w!("Window title").as_ptr();
        config.pszMainInstruction = w!("Main instruction").as_ptr();
        config.pszContent = w!("Content").as_ptr();
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

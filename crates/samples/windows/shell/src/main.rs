use windows::{
    core::*, Win32::System::Com::*, Win32::System::Variant::*, Win32::UI::Shell::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe { CoInitialize(None).ok()? };

    shell_execute_from_explorer(
        "https://github.com/microsoft/windows-rs",
        "",
        "",
        "",
        SW_SHOWNORMAL,
    )
}

// Ported from https://devblogs.microsoft.com/oldnewthing/20131118-00/?p=2643
fn shell_execute_from_explorer(
    file: &str,
    args: &str,
    directory: &str,
    operation: &str,
    show: SHOW_WINDOW_CMD,
) -> Result<()> {
    unsafe {
        let view: IShellView = find_desktop_folder_view()?;
        let background: IDispatch = view.GetItemObject(SVGIO_BACKGROUND)?;
        let folder: IShellFolderViewDual = background.cast()?;
        let shell: IShellDispatch2 = folder.Application()?.cast()?;

        shell.ShellExecute(
            &BSTR::from(file),
            &VARIANT::from(args),
            &VARIANT::from(directory),
            &VARIANT::from(operation),
            &VARIANT::from(show.0),
        )
    }
}

// Ported from https://devblogs.microsoft.com/oldnewthing/20130318-00/?p=4933
fn find_desktop_folder_view<T: Interface>() -> Result<T> {
    unsafe {
        let windows: IShellWindows = CoCreateInstance(&ShellWindows, None, CLSCTX_ALL)?;
        let mut handle = 0;

        let desktop = windows.FindWindowSW(
            &VARIANT::from(CSIDL_DESKTOP),
            &VARIANT::default(),
            SWC_DESKTOP,
            &mut handle,
            SWFO_NEEDDISPATCH,
        )?;

        let provider: IServiceProvider = desktop.cast()?;
        let browser: IShellBrowser = provider.QueryService(&SID_STopLevelBrowser)?;
        let view = browser.QueryActiveShellView()?;
        view.cast()
    }
}

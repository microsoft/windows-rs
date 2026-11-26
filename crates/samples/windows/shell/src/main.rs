fn main() -> windows::core::Result<()> {
    use windows::{
        Win32::System::Com::*, Win32::System::Variant::*, Win32::UI::Shell::*,
        Win32::UI::WindowsAndMessaging::*, core::*,
    };

    use std::mem::ManuallyDrop;

    fn variant_bstr(value: &str) -> VARIANT {
        VARIANT {
            Anonymous: VARIANT_0 {
                Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                    vt: VT_BSTR,
                    wReserved1: 0,
                    wReserved2: 0,
                    wReserved3: 0,
                    Anonymous: VARIANT_0_0_0 {
                        bstrVal: ManuallyDrop::new(BSTR::from(value)),
                    },
                }),
            },
        }
    }

    fn variant_i4(value: i32) -> VARIANT {
        VARIANT {
            Anonymous: VARIANT_0 {
                Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                    vt: VT_I4,
                    wReserved1: 0,
                    wReserved2: 0,
                    wReserved3: 0,
                    Anonymous: VARIANT_0_0_0 { lVal: value },
                }),
            },
        }
    }

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

            let mut args = variant_bstr(args);
            let mut directory = variant_bstr(directory);
            let mut operation = variant_bstr(operation);
            let mut show = variant_i4(show.0);

            shell.ShellExecute(&BSTR::from(file), &args, &directory, &operation, &show)?;

            VariantClear(&mut args)?;
            VariantClear(&mut directory)?;
            VariantClear(&mut operation)?;
            VariantClear(&mut show)?;
            Ok(())
        }
    }

    fn find_desktop_folder_view<T: Interface>() -> Result<T> {
        unsafe {
            let windows: IShellWindows = CoCreateInstance(&ShellWindows, None, CLSCTX_ALL)?;
            let mut handle = 0;

            let desktop = windows.FindWindowSW(
                &variant_i4(CSIDL_DESKTOP as i32),
                &VARIANT::default(),
                SWC_DESKTOP,
                &mut handle,
                SWFO_NEEDDISPATCH,
            )?;

            let provider: IServiceProvider = desktop.cast()?;
            let browser: IShellBrowser = provider.QueryService(&SID_STopLevelBrowser)?;
            browser.QueryActiveShellView()?.cast()
        }
    }

    unsafe { CoInitialize(None).ok()? };

    shell_execute_from_explorer(
        "https://github.com/microsoft/windows-rs",
        "",
        "",
        "",
        SW_SHOWNORMAL,
    )
}

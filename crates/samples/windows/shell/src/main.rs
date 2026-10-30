fn main() -> windows::core::Result<()> {
    use windows::{Win32::*, core::*};

    use std::mem::ManuallyDrop;

    fn variant_bstr(value: &str) -> VARIANT {
        VARIANT {
            Anonymous: tagVARIANT_0 {
                Anonymous: ManuallyDrop::new(tagVARIANT_0_0 {
                    vt: VT_BSTR as VARTYPE,
                    wReserved1: 0,
                    wReserved2: 0,
                    wReserved3: 0,
                    Anonymous: tagVARIANT_0_0_0 {
                        bstrVal: ManuallyDrop::new(BSTR::from(value)),
                    },
                }),
            },
        }
    }

    fn variant_i4(value: i32) -> VARIANT {
        VARIANT {
            Anonymous: tagVARIANT_0 {
                Anonymous: ManuallyDrop::new(tagVARIANT_0_0 {
                    vt: VT_I4 as VARTYPE,
                    wReserved1: 0,
                    wReserved2: 0,
                    wReserved3: 0,
                    Anonymous: tagVARIANT_0_0_0 { lVal: value },
                }),
            },
        }
    }

    fn shell_execute_from_explorer(
        file: &str,
        args: &str,
        directory: &str,
        operation: &str,
        show: u32,
    ) -> Result<()> {
        unsafe {
            let view: IShellView = find_desktop_folder_view()?;
            let background: IDispatch = view.GetItemObject(SVGIO_BACKGROUND as u32)?;
            let folder: IShellFolderViewDual = background.cast()?;
            let shell: IShellDispatch2 = folder.Application()?.cast()?;

            let mut args = variant_bstr(args);
            let mut directory = variant_bstr(directory);
            let mut operation = variant_bstr(operation);
            let mut show = variant_i4(show as i32);

            shell
                .ShellExecute(&BSTR::from(file), &args, &directory, &operation, &show)
                .ok()?;

            VariantClear(&mut args).ok()?;
            VariantClear(&mut directory).ok()?;
            VariantClear(&mut operation).ok()?;
            VariantClear(&mut show).ok()?;
            Ok(())
        }
    }

    fn find_desktop_folder_view<T: Interface>() -> Result<T> {
        unsafe {
            let windows: IShellWindows = CoCreateInstance(&ShellWindows, None, CLSCTX_ALL as u32)?;
            let mut handle = 0;

            let desktop = windows.FindWindowSW(
                &variant_i4(CSIDL_DESKTOP),
                &VARIANT::default(),
                SWC_DESKTOP,
                &mut handle,
                SWFO_NEEDDISPATCH,
            )?;

            let provider: IServiceProvider = desktop.cast()?;
            let mut browser = core::ptr::null_mut();
            provider
                .QueryService(&SID_STopLevelBrowser, &IShellBrowser::IID, &mut browser)
                .ok()?;
            let browser: IShellBrowser = imp::Type::from_abi(browser)?;
            let mut view = None;
            browser.QueryActiveShellView(&mut view).ok()?;
            view.unwrap().cast()
        }
    }

    unsafe { CoInitialize(None).ok()? };

    shell_execute_from_explorer(
        "https://github.com/microsoft/windows-rs",
        "",
        "",
        "",
        SW_SHOWNORMAL as u32,
    )
}

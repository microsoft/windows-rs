fn main() -> windows::core::Result<()> {
    use windows::{
        combaseapi::*, core::*, exdisp::*, oaidl::*, objbase::*, oleauto::*, servprov::*,
        shldisp::*, shlguid::*, shlobj_core::*, shobjidl_core::*, winuser::*, wtypes::*,
    };

    use std::mem::ManuallyDrop;

    fn variant_bstr(value: &str) -> VARIANT {
        VARIANT {
            Anonymous: VARIANT_0 {
                Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                    vt: VARTYPE(VT_BSTR as u16),
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
                    vt: VARTYPE(VT_I4 as u16),
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
        show: u32,
    ) -> Result<()> {
        unsafe {
            let view: IShellView = find_desktop_folder_view()?;
            let background: IDispatch = view.GetItemObject(SVGIO_BACKGROUND)?;
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

    // Ported from https://devblogs.microsoft.com/oldnewthing/20130318-00/?p=4933
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
            let view = browser.QueryActiveShellView()?;
            view.cast()
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

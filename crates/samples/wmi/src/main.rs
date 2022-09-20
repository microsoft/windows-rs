use windows::{core::*, Win32::Security::*, Win32::System::Com::*, Win32::System::Ole::*, Win32::System::Wmi::*};

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;

        CoInitializeSecurity(PSECURITY_DESCRIPTOR::default(), -1, None, None, RPC_C_AUTHN_LEVEL_DEFAULT, RPC_C_IMP_LEVEL_IMPERSONATE, None, EOAC_NONE, None)?;

        let locator: IWbemLocator = CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER)?;

        let server = locator.ConnectServer(&BSTR::from("root\\cimv2"), &BSTR::new(), &BSTR::new(), &BSTR::new(), 0, &BSTR::new(), None)?;

        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/1265
        let query = server.ExecQuery(&BSTR::from("WQL"), &BSTR::from("select Caption from Win32_LogicalDisk"), WBEM_FLAG_FORWARD_ONLY.0 | WBEM_FLAG_RETURN_IMMEDIATELY.0, None)?;

        loop {
            let mut row = [None; 1];
            let mut returned = 0;
            // TODO: workaround for https://github.com/microsoft/win32metadata/issues/1266
            query.Next(-1, &mut row, &mut returned).ok()?;

            if let Some(row) = &row[0] {
                let mut value = Default::default();
                row.Get(w!("Caption"), 0, &mut value, std::ptr::null_mut(), std::ptr::null_mut())?;
                println!("{}", VarFormat(&value, None, 0, 0, 0)?);

                // TODO: workaround for https://github.com/microsoft/windows-rs/issues/539
                VariantClear(&mut value)?;
            } else {
                break;
            }
        }

        Ok(())
    }
}

use windows::{
    core::*, Win32::Security::*, Win32::System::Com::*, Win32::System::Ole::*,
    Win32::System::Wmi::*,
};

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;

        CoInitializeSecurity(
            PSECURITY_DESCRIPTOR::default(),
            -1,
            None,
            None,
            RPC_C_AUTHN_LEVEL_DEFAULT,
            RPC_C_IMP_LEVEL_IMPERSONATE,
            None,
            EOAC_NONE,
            None,
        )?;

        let locator: IWbemLocator = CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER)?;

        let server = locator.ConnectServer(
            &BSTR::from("root\\cimv2"),
            &BSTR::new(),
            &BSTR::new(),
            &BSTR::new(),
            0,
            &BSTR::new(),
            None,
        )?;

        let query = server.ExecQuery(
            &BSTR::from("WQL"),
            &BSTR::from("select Caption from Win32_LogicalDisk"),
            WBEM_FLAG_FORWARD_ONLY | WBEM_FLAG_RETURN_IMMEDIATELY,
            None,
        )?;

        loop {
            let mut row = [None; 1];
            let mut returned = 0;
            query.Next(WBEM_INFINITE, &mut row, &mut returned).ok()?;

            if let Some(row) = &row[0] {
                let mut value = Default::default();
                row.Get(
                    w!("Caption"),
                    0,
                    &mut value,
                    None,
                    None,
                )?;
                println!(
                    "{}",
                    VarFormat(
                        &value,
                        None,
                        VARFORMAT_FIRST_DAY_SYSTEMDEFAULT,
                        VARFORMAT_FIRST_WEEK_SYSTEMDEFAULT,
                        0
                    )?
                );

                // TODO: workaround for https://github.com/microsoft/windows-rs/issues/539
                VariantClear(&mut value)?;
            } else {
                break;
            }
        }

        Ok(())
    }
}

fn main() -> windows::core::Result<()> {
    use windows::{Win32::*, core::*};

    unsafe fn variant_to_string(value: &VARIANT) -> String {
        let inner = unsafe { &value.Anonymous.Anonymous };

        if inner.vt == VARTYPE(VT_BSTR as u16) {
            unsafe { inner.Anonymous.bstrVal.display().to_string() }
        } else {
            unsafe { inner.Anonymous.lVal.to_string() }
        }
    }

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED as u32).ok()?;

        CoInitializeSecurity(
            None,
            -1,
            None,
            None,
            RPC_C_AUTHN_LEVEL_DEFAULT as u32,
            RPC_C_IMP_LEVEL_IMPERSONATE as u32,
            None,
            EOAC_NONE as u32,
            None,
        )
        .ok()?;

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
            query
                .Next(WBEM_INFINITE, 1, row.as_mut_ptr(), &mut returned)
                .ok()?;

            if let Some(row) = &row[0] {
                let mut value = VARIANT::default();
                row.Get(
                    w!("Caption"),
                    0,
                    &mut value,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
                .ok()?;
                let text = variant_to_string(&value);
                VariantClear(&mut value).ok()?;
                println!("{text}");
            } else {
                break;
            }
        }

        Ok(())
    }
}

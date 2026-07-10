fn main() -> windows::core::Result<()> {
    use windows::{
        combaseapi::*, core::*, oaidl::*, objbase::*, objidlbase::*, oleauto::*, rpc::*,
        wbemcli::*, wtypes::*, wtypesbase::*,
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
            None,
            None,
            RPC_C_AUTHN_LEVEL_DEFAULT,
            RPC_C_IMP_LEVEL_IMPERSONATE,
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

        //
        // ExecQuery example
        //

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

        //
        // ExecMethod example
        //

        let class_name = BSTR::from("Win32_Process");
        let method_name = BSTR::from("Create");

        let mut class = None;
        server
            .GetObject(
                &class_name,
                Default::default(),
                None,
                &mut class,
                std::ptr::null_mut(),
            )
            .ok()?;
        let class = class.unwrap();

        let mut input = None;
        class
            .GetMethod(&method_name, 0, &mut input, std::ptr::null_mut())
            .ok()?;
        let input = input.unwrap();

        let object = input.SpawnInstance(0)?;
        let mut command = variant_bstr("notepad.exe");
        object
            .Put(w!("CommandLine"), 0, &command, CIMTYPE(CIM_STRING))
            .ok()?;
        VariantClear(&mut command).ok()?;

        let mut output = None;
        server
            .ExecMethod(
                &class_name,
                &method_name,
                Default::default(),
                None,
                &object,
                &mut output,
                std::ptr::null_mut(),
            )
            .ok()?;
        let output = output.unwrap();

        let mut value = VARIANT::default();
        output
            .Get(
                w!("ReturnValue"),
                0,
                &mut value,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
            .ok()?;
        let text = variant_to_string(&value);
        VariantClear(&mut value).ok()?;
        println!("`Create` method return value: {text}");

        Ok(())
    }
}

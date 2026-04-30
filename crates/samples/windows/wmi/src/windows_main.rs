use windows::{core::*, Win32::System::Com::*, Win32::System::Variant::*, Win32::System::Wmi::*};

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        CoInitializeSecurity(
            None,
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
            query.Next(WBEM_INFINITE, &mut row, &mut returned).ok()?;

            if let Some(row) = &row[0] {
                let mut value = VARIANT::default();
                row.Get(w!("Caption"), 0, &mut value, None, None)?;
                println!("{value}",);
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
        server.GetObject(
            &class_name,
            Default::default(),
            None,
            Some(&mut class),
            None,
        )?;
        let class = class.unwrap();

        let mut input = None;
        class.GetMethod(&method_name, 0, &mut input, std::ptr::null_mut())?;
        let input = input.unwrap();

        let object = input.SpawnInstance(0)?;
        object.Put(w!("CommandLine"), 0, &VARIANT::from("notepad.exe"), 0)?;

        let mut output = None;
        server.ExecMethod(
            &class_name,
            &method_name,
            Default::default(),
            None,
            &object,
            Some(&mut output),
            None,
        )?;
        let output = output.unwrap();

        let mut value = VARIANT::default();
        output.Get(w!("ReturnValue"), 0, &mut value, None, None)?;
        println!("`Create` method return value: {value}");

        Ok(())
    }
}

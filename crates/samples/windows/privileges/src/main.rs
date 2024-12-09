use windows::{
    core::*, Win32::Foundation::*, Win32::Security::*, Win32::System::Memory::*,
    Win32::System::Threading::*,
};

fn main() -> Result<()> {
    unsafe {
        let mut token = HANDLE::default();
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token)?;

        let mut bytes_required = 0;
        _ = GetTokenInformation(token, TokenPrivileges, None, 0, &mut bytes_required);

        let buffer = Owned::new(LocalAlloc(LPTR, bytes_required as usize)?);

        GetTokenInformation(
            token,
            TokenPrivileges,
            Some(buffer.0 as *mut _),
            bytes_required,
            &mut bytes_required,
        )?;

        let header = &*(buffer.0 as *const TOKEN_PRIVILEGES);

        let privileges =
            std::slice::from_raw_parts(header.Privileges.as_ptr(), header.PrivilegeCount as usize);

        for privilege in privileges {
            let mut name_len = 0;
            _ = LookupPrivilegeNameW(None, &privilege.Luid, None, &mut name_len);

            let mut name = vec![0u16; (name_len + 1) as usize];
            let name = PWSTR(name.as_mut_ptr());
            LookupPrivilegeNameW(None, &privilege.Luid, Some(name), &mut name_len)?;

            println!("{}", name.display())
        }

        Ok(())
    }
}

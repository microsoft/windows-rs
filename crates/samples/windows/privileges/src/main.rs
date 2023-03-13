use windows::{
    core::*, Win32::Foundation::*, Win32::Security::*, Win32::System::Memory::*,
    Win32::System::Threading::*,
};

fn main() -> Result<()> {
    unsafe {
        let mut token = HANDLE::default();
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).ok()?;

        let mut bytes_required = 0;
        GetTokenInformation(token, TokenPrivileges, None, 0, &mut bytes_required);

        let buffer = LocalAlloc(LPTR, bytes_required as _)?;

        GetTokenInformation(
            token,
            TokenPrivileges,
            Some(buffer.0 as *mut _),
            bytes_required,
            &mut bytes_required,
        )
        .ok()?;

        let header = &*(buffer.0 as *const TOKEN_PRIVILEGES);

        let privileges =
            std::slice::from_raw_parts(header.Privileges.as_ptr(), header.PrivilegeCount as _);

        for privilege in privileges {
            let mut name_len = 0;
            LookupPrivilegeNameW(None, &privilege.Luid, PWSTR::null(), &mut name_len);

            let mut name = vec![0u16; (name_len + 1) as usize];
            let name = PWSTR(name.as_mut_ptr());
            LookupPrivilegeNameW(None, &privilege.Luid, name, &mut name_len).ok()?;

            println!("{}", name.display())
        }

        _ = LocalFree(buffer);
        Ok(())
    }
}

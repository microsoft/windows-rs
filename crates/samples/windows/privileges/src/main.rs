fn main() -> windows::core::Result<()> {
    use windows::{Win32::*, core::*};

    unsafe {
        let mut token = HANDLE::default();
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY as u32, &mut token).ok()?;

        let mut bytes_required = 0;
        let error = GetTokenInformation(token, TokenPrivileges, None, 0, &mut bytes_required)
            .ok()
            .unwrap_err();
        assert_eq!(
            error.code(),
            WIN32_ERROR(ERROR_INSUFFICIENT_BUFFER as u32).into()
        );

        let buffer = LocalAlloc(LPTR as u32, bytes_required as usize);
        if buffer.0.is_null() {
            return Err(Error::from_thread());
        }

        GetTokenInformation(
            token,
            TokenPrivileges,
            Some(buffer.0),
            bytes_required,
            &mut bytes_required,
        )
        .ok()?;

        let header = &*(buffer.0 as *const TOKEN_PRIVILEGES);
        let privileges =
            std::slice::from_raw_parts(header.Privileges.as_ptr(), header.PrivilegeCount as usize);

        for privilege in privileges {
            let mut name_len = 0;
            let error = LookupPrivilegeNameW(None, &privilege.Luid, None, &mut name_len)
                .ok()
                .unwrap_err();
            assert_eq!(
                error.code(),
                WIN32_ERROR(ERROR_INSUFFICIENT_BUFFER as u32).into()
            );

            let mut name = vec![0u16; (name_len + 1) as usize];
            let name = PWSTR(name.as_mut_ptr());
            LookupPrivilegeNameW(None, &privilege.Luid, Some(name), &mut name_len).ok()?;

            println!("{}", name.display());
        }

        _ = LocalFree(buffer);
        CloseHandle(token).ok()
    }
}

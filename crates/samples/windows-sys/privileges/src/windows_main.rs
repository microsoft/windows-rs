use windows_sys::{
    Win32::Foundation::*, Win32::Security::*, Win32::System::Memory::*, Win32::System::Threading::*,
};

fn main() {
    unsafe {
        let mut token = core::ptr::null_mut();
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token);
        let mut bytes_required = 0;

        GetTokenInformation(
            token,
            TokenPrivileges,
            std::ptr::null_mut(),
            0,
            &mut bytes_required,
        );

        let buffer = LocalAlloc(LPTR, bytes_required as usize);

        GetTokenInformation(
            token,
            TokenPrivileges,
            buffer as *mut _,
            bytes_required,
            &mut bytes_required,
        );

        let header = &*(buffer as *const TOKEN_PRIVILEGES);

        let privileges =
            std::slice::from_raw_parts(header.Privileges.as_ptr(), header.PrivilegeCount as usize);

        for privilege in privileges {
            let mut name_len = 0;

            LookupPrivilegeNameW(
                std::ptr::null(),
                &privilege.Luid,
                std::ptr::null_mut(),
                &mut name_len,
            );

            let mut name = vec![0u16; (name_len + 1) as usize];

            LookupPrivilegeNameW(
                std::ptr::null(),
                &privilege.Luid,
                name.as_mut_ptr(),
                &mut name_len,
            );

            println!("{}", String::from_utf16_lossy(&name));
        }

        LocalFree(buffer);
    }
}

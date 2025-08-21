use windows_sys::{
    core::{s, PCSTR, PSTR},
    Win32::Foundation::ERROR_BUSY,
    Win32::Networking::Ldap::LDAP_BUSY,
};

windows_link::link!("wldap32.dll" "C" fn LdapMapErrorToWin32(ldaperror: u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
windows_link::link!("user32.dll" "C" fn wsprintfA(param0: PSTR, param1: PCSTR, ...) -> i32);

#[test]
fn calling_convention() {
    unsafe {
        // This function requires cdecl on x86.
        assert_eq!(LdapMapErrorToWin32(LDAP_BUSY as u32), ERROR_BUSY);

        // This function requires stdcall on x86.
        GetTickCount();
    }
}

#[test]
fn variadic() {
    unsafe {
        let mut buffer = vec![0u8; 1024];
        let len = wsprintfA(buffer.as_mut_ptr(), s!("test-%d-%d!"), 123u32, 456u32);
        let result = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
            buffer.as_ptr(),
            len as usize,
        ));
        assert_eq!(result, "test-123-456!");
    }
}

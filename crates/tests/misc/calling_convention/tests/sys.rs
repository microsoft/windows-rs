use windows_sys::{
    core::*, Win32::Foundation::*, Win32::Networking::Ldap::*, Win32::System::SystemInformation::*,
    Win32::UI::WindowsAndMessaging::*,
};

#[test]
fn calling_convention() {
    unsafe {
        // This function requires cdecl on x86.
        assert_eq!(ERROR_BUSY, LdapMapErrorToWin32(LDAP_BUSY as u32));

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

#[test]
#[cfg(windows_raw_dylib)]
fn raw_dylib() {}

use windows::{
    Win32::Foundation::*, Win32::Networking::Ldap::*, Win32::System::SystemInformation::*,
};

#[test]
fn calling_convention() {
    unsafe {
        // This function requires cdecl on x86.
        assert_eq!(LdapMapErrorToWin32(LDAP_BUSY), ERROR_BUSY);

        // This function requires stdcall on x86.
        GetTickCount();
    }
}

#[test]
#[cfg(windows_raw_dylib)]
fn raw_dylib() {}

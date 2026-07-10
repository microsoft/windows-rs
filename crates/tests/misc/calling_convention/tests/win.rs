#![cfg(windows)]
use windows::{Win32::sysinfoapi::*, Win32::winerror::*, Win32::winldap::*};

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
#[cfg(windows_raw_dylib)]
fn raw_dylib() {}

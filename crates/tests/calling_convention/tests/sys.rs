use windows_sys::{Win32::Foundation::*, Win32::Networking::Ldap::*};

#[test]
fn test() {
    unsafe {
        assert_eq!(ERROR_BUSY, LdapMapErrorToWin32(LDAP_BUSY));
    }
}

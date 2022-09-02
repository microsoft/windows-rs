use windows::{Win32::Foundation::*, Win32::Networking::Ldap::*};

#[test]
fn test() {
    // TODO: workaround for https://github.com/microsoft/win32metadata/issues/1211
    unsafe {
        assert_eq!(ERROR_BUSY.0, LdapMapErrorToWin32(LDAP_BUSY.0 as _));
    }
}

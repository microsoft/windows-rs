#[test]
fn min() {
    windows_targets::link!("kernel32.dll" "system" fn SetLastError(code: u32));
    windows_targets::link!("kernel32.dll" "system" fn GetLastError() -> u32);

    unsafe {
        SetLastError(1234);
        assert_eq!(GetLastError(), 1234);
    }
}

#[test]
fn link_name() {
    windows_targets::link!("kernel32.dll" "system" "SetLastError" fn set_last_error(code: u32) -> ());
    windows_targets::link!("kernel32.dll" "system" "GetLastError" fn get_last_error() -> u32);

    unsafe {
        set_last_error(1234);
        assert_eq!(get_last_error(), 1234);
    }
}

#[test]
fn cdecl() {
    windows_targets::link!("wldap32.dll" "C" fn LdapMapErrorToWin32(code : i32) -> u32);
    const LDAP_BUSY: i32 = 51;
    const ERROR_BUSY: u32 = 170;

    unsafe {
        assert_eq!(LdapMapErrorToWin32(LDAP_BUSY), ERROR_BUSY);
    }
}

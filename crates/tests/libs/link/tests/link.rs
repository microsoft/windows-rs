#![cfg(windows)]

#[test]
fn min() {
    windows_link::link!("kernel32.dll" "system" fn SetLastError(code: u32));
    windows_link::link!("kernel32.dll" "system" fn GetLastError() -> u32);

    unsafe {
        SetLastError(1234);
        assert_eq!(GetLastError(), 1234);
    }
}

#[test]
fn link_name() {
    windows_link::link!("kernel32.dll" "system" "SetLastError" fn set_last_error(code: u32) -> ());
    windows_link::link!("kernel32.dll" "system" "GetLastError" fn get_last_error() -> u32);

    unsafe {
        set_last_error(1234);
        assert_eq!(get_last_error(), 1234);
    }
}

#[test]
fn cdecl() {
    windows_link::link!("wldap32.dll" "C" fn LdapMapErrorToWin32(code : i32) -> u32);
    const LDAP_BUSY: i32 = 51;
    const ERROR_BUSY: u32 = 170;

    unsafe {
        assert_eq!(LdapMapErrorToWin32(LDAP_BUSY), ERROR_BUSY);
    }
}

// Test for https://github.com/microsoft/windows-rs/pull/3669#issuecomment-3097317771
#[test]
fn fn_ptr() {
    windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> u32);

    type GetTickCountType = unsafe extern "system" fn() -> u32;

    static GET_TICK_COUNT: GetTickCountType = GetTickCount;

    unsafe {
        GET_TICK_COUNT();
    }
}

#[test]
fn variadic() {
    windows_link::link!("user32.dll" "C" fn wsprintfA(buffer: *mut u8, format: *const u8, ...) -> i32);

    unsafe {
        let mut buffer = [0u8; 1024];
        let len = wsprintfA(
            buffer.as_mut_ptr(),
            c"test-%d-%d!".as_ptr().cast(),
            123u32,
            456u32,
        );
        assert_eq!(&buffer[..len as usize], b"test-123-456!");
    }
}

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

#[test]
fn ordinal() {
    use core::ffi::c_void;

    // SHCreateMemStream is documented as shlwapi.dll ordinal 12.
    windows_link::link!("shlwapi.dll" "system" ordinal(12)
        fn create_stream(data: *const u8, len: u32) -> *mut c_void);

    #[repr(C)]
    struct Vtable {
        query_interface:
            unsafe extern "system" fn(*mut c_void, *const c_void, *mut *mut c_void) -> i32,
        add_ref: unsafe extern "system" fn(*mut c_void) -> u32,
        release: unsafe extern "system" fn(*mut c_void) -> u32,
        read: unsafe extern "system" fn(*mut c_void, *mut u8, u32, *mut u32) -> i32,
    }

    let function: create_stream = create_stream;
    let input = b"ordinal import";
    unsafe {
        let stream = function(input.as_ptr(), input.len() as u32);
        assert!(!stream.is_null());
        let vtable = &**stream.cast::<*const Vtable>();
        let mut output = [0; 14];
        let mut read = 0;
        let result = (vtable.read)(stream, output.as_mut_ptr(), output.len() as u32, &mut read);
        (vtable.release)(stream);
        assert_eq!(result, 0);
        assert_eq!(read as usize, input.len());
        assert_eq!(&output, input);
    }
}

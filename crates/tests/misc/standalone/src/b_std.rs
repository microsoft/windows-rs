#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_targets::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetLastError() -> WIN32_ERROR);
windows_targets::link!("advapi32.dll" "system" "SystemFunction036" fn RtlGenRandom(randombuffer : *mut core::ffi::c_void, randombufferlength : u32) -> BOOLEAN);
windows_targets::link!("ws2_32.dll" "system" fn socket(af : i32, r#type : WINSOCK_SOCKET_TYPE, protocol : i32) -> SOCKET);
windows_targets::link!("user32.dll" "cdecl" fn wsprintfA(param0 : PSTR, param1 : PCSTR, ...) -> i32);
pub type WIN32_ERROR = u32;
pub type WINSOCK_SOCKET_TYPE = i32;
pub type BCRYPT_ALG_HANDLE = *mut core::ffi::c_void;
pub type BOOL = i32;
pub type BOOLEAN = u8;
pub type HANDLE = *mut core::ffi::c_void;
pub type HMODULE = *mut core::ffi::c_void;
pub type SOCKET = usize;
pub type PSTR = *mut u8;
pub type PCSTR = *const u8;

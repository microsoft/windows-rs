#![allow(non_snake_case, non_camel_case_types, dead_code)]
#![cfg_attr(named_import_probe, no_std)]
#![cfg_attr(named_import_probe, no_main)]

include!("bindings.rs");

#[cfg(not(named_import_probe))]
fn main() {
    check_calls();
}

#[cfg(named_import_probe)]
windows_link::link!("kernel32.dll" "system" fn ExitProcess(code: u32) -> !);

#[cfg(named_import_probe)]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    unsafe { ExitProcess(1) }
}

#[cfg(named_import_probe)]
#[unsafe(no_mangle)]
extern "C" fn mainCRTStartup() -> ! {
    check_calls();
    unsafe { ExitProcess(0) }
}

fn check_calls() {
    let _: unsafe extern "system" fn(u16) -> u16 = htons;
    let _: unsafe extern "system" fn(u32) -> u32 = htonl;
    let _: unsafe extern "system" fn() -> i32 = WSAGetLastError;
    let _: unsafe extern "system" fn(
        i32,
        *mut fd_set,
        *mut fd_set,
        *mut fd_set,
        *const timeval,
    ) -> i32 = select;
    unsafe {
        let mut data = core::mem::MaybeUninit::<WSAData>::uninit();
        assert_eq!(WSAStartup(0x202, data.as_mut_ptr()), 0);
        assert_eq!(htons(0x1234), 0x3412);
        assert_eq!(ntohs(htons(0x1234)), 0x1234);
        assert_eq!(htonl(0x12345678), 0x78563412);
        assert_eq!(ntohl(htonl(0x12345678)), 0x12345678);
        WSASetLastError(1234);
        assert_eq!(WSAGetLastError(), 1234);
        let timeout = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        assert_eq!(
            select(
                0,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &timeout
            ),
            -1
        );
        assert_eq!(WSAGetLastError(), 10022);
        assert_eq!(WSACleanup(), 0);
    }
}

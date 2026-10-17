#![allow(non_snake_case, non_camel_case_types, dead_code)]
#![cfg_attr(named_import_probe, no_std)]
#![cfg_attr(named_import_probe, no_main)]

include!("bindings.rs");

windows_link::link!("ws2_32.dll" "system" fn WSACleanup() -> i32);

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
    let clone: ILClone = ILClone;
    let free: ILFree = ILFree;
    let startup: WSAStartup = WSAStartup;
    let _: unsafe extern "system" fn(LPCITEMIDLIST) -> LPITEMIDLIST = clone;
    let _: unsafe extern "system" fn(LPITEMIDLIST) = free;
    let _: unsafe extern "system" fn(WORD, LPWSADATA) -> i32 = startup;
    unsafe {
        let empty: ITEMIDLIST = core::mem::zeroed();
        let copy = clone(&empty);
        assert!(!copy.is_null());
        free(copy);
        let mut data = core::mem::MaybeUninit::<WSAData>::uninit();
        assert_eq!(startup(0x202, data.as_mut_ptr()), 0);
        assert_eq!((*data.as_ptr()).wVersion, 0x202);
        assert_eq!(WSACleanup(), 0);
    }
}

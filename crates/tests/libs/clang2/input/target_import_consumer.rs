#![allow(non_snake_case, non_camel_case_types, dead_code)]
#![cfg_attr(named_import_probe, no_std)]
#![cfg_attr(named_import_probe, no_main)]

include!("bindings.rs");

#[cfg(not(named_import_probe))]
fn main() {
    check_imports();
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
    check_imports();
    unsafe { ExitProcess(0) }
}

fn check_imports() {
    #[cfg(not(target_arch = "x86"))]
    {
        let size: unsafe extern "system" fn(*const u32, u32, *const BSTR) -> u32 = BSTR_UserSize64;
        core::hint::black_box(size);
        let name = b"example.com\0".map(u16::from);
        unsafe {
            assert_eq!(DnsValidateName_W(name.as_ptr(), DnsNameDomain), 0);
        }
    }
}

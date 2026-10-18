#![allow(non_snake_case, non_camel_case_types, dead_code)]
#![cfg_attr(named_import_probe, no_std)]
#![cfg_attr(named_import_probe, no_main)]

include!("bindings.rs");

#[cfg(target_arch = "x86")]
type ValidateName = unsafe extern "system" fn(PCWSTR, DNS_NAME_FORMAT) -> DNS_STATUS;
#[cfg(not(target_arch = "x86"))]
type ValidateName = unsafe extern "C" fn(PCWSTR, DNS_NAME_FORMAT) -> DNS_STATUS;
#[cfg(target_arch = "x86")]
type AddServer =
    unsafe extern "system" fn(DWORD, LPVOID, LPDHCP_SERVER_INFO, LPVOID, LPVOID) -> DWORD;
#[cfg(not(target_arch = "x86"))]
type AddServer = unsafe extern "C" fn(DWORD, LPVOID, LPDHCP_SERVER_INFO, LPVOID, LPVOID) -> DWORD;

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
    let validate: ValidateName = DnsValidateName_W;
    let add_server: AddServer = DhcpAddServer;
    // Resolve the management API without registering a DHCP server.
    core::hint::black_box(add_server);
    let name = b"example.com\0".map(u16::from);
    let invalid = b"bad label!\0".map(u16::from);
    unsafe {
        assert_eq!(validate(name.as_ptr(), DnsNameDomain), 0);
        assert_ne!(validate(invalid.as_ptr(), DnsNameHostnameLabel), 0);
    }
}

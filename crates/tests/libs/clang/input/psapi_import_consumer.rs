#![allow(non_snake_case, non_camel_case_types, dead_code)]
#![cfg_attr(named_import_probe, no_std)]
#![cfg_attr(named_import_probe, no_main)]

include!("bindings.rs");

windows_link::link!("kernel32.dll" "system" fn GetCurrentProcessId() -> u32);

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
    let calls: [unsafe extern "system" fn(*mut DWORD, DWORD, LPDWORD) -> BOOL; 2] =
        [EnumProcesses, K32EnumProcesses];
    for call in calls {
        let mut processes = [0u32; 4096];
        let mut bytes = 0;
        assert_ne!(
            unsafe {
                call(
                    processes.as_mut_ptr(),
                    core::mem::size_of_val(&processes) as u32,
                    &mut bytes,
                )
            },
            0
        );
        assert!(bytes > 0 && bytes as usize <= core::mem::size_of_val(&processes));
        assert_eq!(bytes % 4, 0);
        let count = bytes as usize / 4;
        if count < processes.len() {
            assert!(processes[..count].contains(&unsafe { GetCurrentProcessId() }));
        }
    }
}

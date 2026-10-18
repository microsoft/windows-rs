#![allow(non_snake_case, non_camel_case_types, dead_code)]
#![cfg_attr(named_import_probe, no_std)]
#![cfg_attr(named_import_probe, no_main)]

include!("bindings.rs");

const _: () = {
    #[cfg(target_arch = "x86")]
    {
        assert!(core::mem::size_of::<CONTEXT>() == 716);
        assert!(core::mem::align_of::<CONTEXT>() == 4);
        assert!(core::mem::offset_of!(CONTEXT, Eip) == 184);
    }
    #[cfg(target_arch = "x86_64")]
    {
        assert!(core::mem::size_of::<CONTEXT>() == 1232);
        assert!(core::mem::align_of::<CONTEXT>() == 16);
        assert!(core::mem::offset_of!(CONTEXT, Rip) == 248);
    }
    #[cfg(target_arch = "aarch64")]
    {
        assert!(core::mem::size_of::<CONTEXT>() == 912);
        assert!(core::mem::align_of::<CONTEXT>() == 16);
        assert!(core::mem::offset_of!(CONTEXT, Pc) == 264);
    }
};

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
    let _: unsafe extern "system" fn(HANDLE, LPCONTEXT) -> BOOL = GetThreadContext;
    let _: unsafe extern "system" fn(HANDLE, *const CONTEXT) -> BOOL = SetThreadContext;
    let capture: unsafe extern "system" fn(PCONTEXT) = RtlCaptureContext;
    let mut context: CONTEXT = unsafe { core::mem::zeroed() };
    unsafe { capture(&mut context) };
    assert_ne!(context.ContextFlags, 0);
    #[cfg(target_arch = "x86")]
    assert!(context.Eip != 0 && context.Esp != 0);
    #[cfg(target_arch = "x86_64")]
    assert!(context.Rip != 0 && context.Rsp != 0);
    #[cfg(target_arch = "aarch64")]
    assert!(context.Pc != 0 && context.Sp != 0);
}

#![allow(non_snake_case, non_camel_case_types, dead_code)]
#![cfg_attr(named_import_probe, no_std)]
#![cfg_attr(named_import_probe, no_main)]

include!("bindings.rs");

fn check_calls() {
    let pointer = core::mem::size_of::<usize>();
    assert_eq!(core::mem::size_of::<IAVIStream_Vtbl>(), 14 * pointer);
    assert_eq!(core::mem::size_of::<IAVIFile_Vtbl>(), 10 * pointer);
    assert_eq!(core::mem::size_of::<IAVIStreaming_Vtbl>(), 5 * pointer);
    assert_eq!(core::mem::offset_of!(IAVIStream_Vtbl, Info), 4 * pointer);
    assert_eq!(core::mem::offset_of!(IAVIFile_Vtbl, Info), 3 * pointer);
    assert_eq!(
        core::mem::offset_of!(IAVIStreaming_Vtbl, Begin),
        3 * pointer
    );
    unsafe {
        AVIFileInit();
        let mut stream = core::ptr::null_mut();
        assert_eq!(
            CreateEditableStream(&raw mut stream, core::ptr::null_mut()),
            0
        );
        assert!(!stream.is_null());
        let vtable = *stream.cast::<*const IAVIStream_Vtbl>();
        let mut info = core::mem::zeroed::<AVISTREAMINFOW>();
        assert_eq!(
            ((*vtable).Info)(
                stream,
                &raw mut info,
                core::mem::size_of::<AVISTREAMINFOW>() as i32
            ),
            0
        );
        assert_eq!(info.dwLength, 0);
        let mut clone = core::ptr::null_mut();
        assert_eq!(EditStreamClone(stream, &raw mut clone), 0);
        assert!(!clone.is_null());
        AVIStreamRelease(clone);
        AVIStreamRelease(stream);
        AVIFileExit();
    }
}

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

use windows_sys::{Win32::Graphics::Gdi::*, Win32::System::Diagnostics::Debug::*, Win32::System::Environment::*, Win32::UI::Controls::Dialogs::*};

#[test]
#[cfg(target_arch = "x86_64")]
fn test() {
    assert_eq!(1232, core::mem::size_of::<CONTEXT>());
    assert_eq!(16, core::mem::align_of::<CONTEXT>());

    assert_eq!(1280, core::mem::size_of::<MINIDUMP_THREAD_CALLBACK>());
    assert_eq!(16, core::mem::align_of::<MINIDUMP_THREAD_CALLBACK>());

    assert_eq!(72, core::mem::size_of::<CHOOSECOLORA>());
    assert_eq!(8, core::mem::align_of::<CHOOSECOLORA>());

    assert_eq!(14, core::mem::size_of::<BITMAPFILEHEADER>());
    assert_eq!(2, core::mem::align_of::<BITMAPFILEHEADER>());

    use windows_sys::Win32::System::Environment::VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64;

    extern "system" fn callback(_: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64) -> i32 {
        64
    }

    let callback: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Some(callback);
    assert_eq!(64, unsafe { callback.unwrap()(core::ptr::null()) });
}

#[test]
#[cfg(target_arch = "x86")]
fn test() {
    assert_eq!(716, core::mem::size_of::<CONTEXT>());
    assert_eq!(4, core::mem::align_of::<CONTEXT>());

    assert_eq!(744, core::mem::size_of::<MINIDUMP_THREAD_CALLBACK>());
    assert_eq!(4, core::mem::align_of::<MINIDUMP_THREAD_CALLBACK>());

    assert_eq!(36, core::mem::size_of::<CHOOSECOLORA>());
    assert_eq!(1, core::mem::align_of::<CHOOSECOLORA>());

    assert_eq!(14, core::mem::size_of::<BITMAPFILEHEADER>());
    assert_eq!(2, core::mem::align_of::<BITMAPFILEHEADER>());

    use windows_sys::Win32::System::Environment::VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32;

    extern "system" fn callback(_: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32) -> i32 {
        32
    }

    let callback: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = Some(callback);
    assert_eq!(32, unsafe { callback.unwrap()(core::ptr::null()) });
}

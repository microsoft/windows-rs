use bindings::Windows::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH, HINSTANCE,
};
use bindings::Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};
use std::ffi::c_void;

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern "stdcall" fn DllMain(module: HINSTANCE, reason: u32, reserved: *mut c_void) -> bool {
    match reason {
        DLL_PROCESS_ATTACH => unsafe {
            MessageBoxA(None, "Hello from the DLL!", "Hello World", MB_OK);
        },
        DLL_THREAD_ATTACH => (),
        DLL_THREAD_DETACH => (),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    true
}

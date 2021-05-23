use bindings::Windows::Win32::System::SystemServices::{
    BOOL, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH, HINSTANCE,
};
use std::ffi::c_void;

#[no_mangle]
#[allow(unused_variables)]
pub extern "stdcall" fn DllMain(module: HINSTANCE, reason: u32, reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => println!("Hello from the dll!"),
        DLL_THREAD_ATTACH => (),
        DLL_THREAD_DETACH => (),
        DLL_PROCESS_DETACH => println!("Goodbye from the dll!"),
        _ => (),
    }

    BOOL::from(true)
}

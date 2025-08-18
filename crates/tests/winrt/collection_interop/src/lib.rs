#![cfg(target_env = "msvc")]

mod bindings;
pub use bindings::*;
pub use windows_core::*;

pub fn make_cpp() -> Result<ITest> {
    unsafe extern "system" {
        fn make_cpp(test: *mut *mut std::ffi::c_void) -> HRESULT;
    }

    unsafe {
        let mut test = None;
        make_cpp(&mut test as *mut _ as *mut _).ok()?;
        Type::from_default(&test)
    }
}

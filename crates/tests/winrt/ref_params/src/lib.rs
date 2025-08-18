#![cfg(target_env = "msvc")]

mod bindings;
pub use bindings::*;
pub use windows_core::*;

pub fn consume(test: &ITest) -> Result<()> {
    unsafe extern "system" {
        fn consume(test: Ref<ITest>) -> HRESULT;
    }

    unsafe { consume(std::mem::transmute_copy(test)).ok() }
}

pub fn produce() -> Result<ITest> {
    unsafe extern "system" {
        fn produce(test: *mut *mut std::ffi::c_void) -> HRESULT;
    }

    unsafe {
        let mut test = None;
        produce(&mut test as *mut _ as *mut _).ok()?;
        Type::from_default(&test)
    }
}

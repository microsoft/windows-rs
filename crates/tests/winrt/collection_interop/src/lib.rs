#![cfg(target_env = "msvc")]

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]
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

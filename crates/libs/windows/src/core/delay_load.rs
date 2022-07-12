use super::*;
use bindings::*;

/// Load a function from a given library.
///
/// This is a small wrapper around `LoadLibrary` and `GetProcAddress`.
///
/// # Safety
///
/// * Both the library and function names must be valid PCSTR representations
pub unsafe fn delay_load(library: PCSTR, function: PCSTR) -> Result<*mut core::ffi::c_void> {
    let library = LoadLibraryA(library)?;

    if let Some(address) = GetProcAddress(library, function) {
        Ok(address as _)
    } else {
        FreeLibrary(library);
        Err(Error::from_win32())
    }
}

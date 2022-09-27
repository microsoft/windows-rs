use super::*;
use bindings::*;

/// Load a function from a given library.
///
/// This is a small wrapper around `LoadLibrary` and `GetProcAddress`.
///
/// # Safety
///
/// * Both the library and function names must be valid PCSTR representations
pub unsafe fn delay_load(library: PCSTR, function: PCSTR) -> Result<*const std::ffi::c_void> {
    let library = LoadLibraryA(library);

    if library == 0 {
        return Err(Error::from_win32());
    }

    let address = GetProcAddress(library, function);

    if !address.is_null() {
        return Ok(address);
    }

    let result = Err(Error::from_win32());
    FreeLibrary(library);
    result
}

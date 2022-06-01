use super::*;
use bindings::*;

/// Load a function from a given library.
///
/// This is a small wrapper around `LoadLibrary` and `GetProcAddress`.
///
/// # Safety
///
/// * Both the library and function names must be valid PCSTR representations
pub unsafe fn delay_load(library: &[u8], function: &[u8]) -> Result<RawPtr> {
    let library = LoadLibraryA(&PCSTR(library.as_ptr()))?;

    if let Some(address) = GetProcAddress(library, &PCSTR(function.as_ptr())) {
        Ok(address as _)
    } else {
        FreeLibrary(library);
        Err(Error::from_win32())
    }
}

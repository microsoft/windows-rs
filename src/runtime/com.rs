use crate::*;

use bindings::windows::win32::com::{CoCreateInstance, CoInitializeEx, COINIT};

/// Initializes COM for use by the calling thread for the multi-threaded apartment (MTA).
pub fn initialize_mta() -> Result<()> {
    // https://github.com/microsoft/win32metadata/issues/323
    // https://github.com/microsoft/win32metadata/issues/95
    unsafe { CoInitializeEx(std::ptr::null_mut(), COINIT::COINIT_MULTITHREADED.0 as u32).ok() }
}

/// Initializes COM for use by the calling thread for a single-threaded apartment (STA).
pub fn initialize_sta() -> Result<()> {
    unsafe {
        CoInitializeEx(
            std::ptr::null_mut(),
            COINIT::COINIT_APARTMENTTHREADED.0 as u32,
        )
        .ok()
    }
}

/// Creates a COM object with the given CLSID.
pub fn create_instance<T: Interface>(clsid: &Guid) -> Result<T> {
    let mut object = None;

    unsafe { CoCreateInstance(clsid, None, CLSCTX_ALL, &T::IID, object.set_abi()).and_some(object) }
}

// https://github.com/microsoft/win32metadata/issues/203
const CLSCTX_ALL: u32 = 23;

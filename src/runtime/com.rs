use crate::*;

use bindings::Windows::Win32::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED, COINIT_MULTITHREADED,
};

/// Initializes COM for use by the calling thread for the multi-threaded apartment (MTA).
pub fn initialize_mta() -> Result<()> {
    unsafe { CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED).ok() }
}

/// Initializes COM for use by the calling thread for a single-threaded apartment (STA).
pub fn initialize_sta() -> Result<()> {
    unsafe { CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED).ok() }
}

/// Creates a COM object with the given CLSID.
pub fn create_instance<T: Interface>(clsid: &Guid) -> Result<T> {
    let mut object = None;

    unsafe { CoCreateInstance(clsid, None, CLSCTX_ALL, &T::IID, object.set_abi()).and_some(object) }
}

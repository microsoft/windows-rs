use crate::*;

/// Initializes COM for use by the calling thread for the multi-threaded apartment (MTA).
pub fn initialize_mta() -> Result<()> {
    unsafe { CoInitializeEx(0, COINIT_MULTITHREADED).ok() }
}

/// Initializes COM for use by the calling thread for a single-threaded apartment (STA).
pub fn initialize_sta() -> Result<()> {
    unsafe { CoInitializeEx(0, COINIT_APARTMENTTHREADED).ok() }
}

/// Creates a COM object with the given CLSID.
pub fn create_instance<T: Interface>(clsid: &Guid) -> Result<T> {
    let mut object = None;

    unsafe { CoCreateInstance(clsid, None, CLSCTX_ALL, &T::IID, object.set_abi()).and_some(object) }
}

const COINIT_MULTITHREADED: u32 = 0;
const COINIT_APARTMENTTHREADED: u32 = 2;
const CLSCTX_ALL: u32 = 23;

#[link(name = "ole32")]
extern "system" {
    fn CoInitializeEx(reserved: isize, apartment: u32) -> ErrorCode;

    fn CoCreateInstance(
        clsid: &Guid,
        outer: Option<IUnknown>,
        clsctx: u32,
        iid: &Guid,
        object: *mut *mut std::ffi::c_void,
    ) -> ErrorCode;
}

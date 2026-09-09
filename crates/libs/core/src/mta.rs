use crate::{Result, imp};
use alloc::rc::Rc;
use core::marker::PhantomData;
use core::ptr::{null, null_mut};

/// Keeps the calling thread's COM single-threaded apartment initialized.
///
/// The apartment is uninitialized when this value is dropped. COM interface values created in the
/// apartment must be dropped first.
///
/// ```compile_fail
/// fn require_send<T: Send>() {}
/// require_send::<windows_core::StaApartment>();
/// ```
#[must_use = "dropping this value uninitializes the COM apartment"]
pub struct StaApartment(PhantomData<Rc<()>>);

impl Drop for StaApartment {
    fn drop(&mut self) {
        unsafe { imp::CoUninitialize() };
    }
}

/// Initializes the calling thread as a COM single-threaded apartment (STA).
pub fn init_sta() -> Result<StaApartment> {
    unsafe {
        imp::CoInitializeEx(null(), imp::COINIT_APARTMENTTHREADED as u32).ok()?;
    }
    Ok(StaApartment(PhantomData))
}

/// Initializes COM for apartment-agnostic code and keeps the multithreaded apartment alive for the
/// rest of the process.
pub fn init_mta() -> Result<()> {
    unsafe {
        let mut cookie = null_mut();
        imp::CoIncrementMTAUsage(&mut cookie).ok()
    }
}

use crate::{Result, imp};
use core::ptr::null_mut;

/// Initializes COM for apartment-agnostic code and keeps the multithreaded apartment alive for the
/// rest of the process.
///
/// This does not change the apartment of a thread that is already initialized, but places an
/// uninitialized calling thread into the multithreaded apartment.
///
/// This function must not be called during process shutdown or inside `DllMain`.
pub fn init_mta() -> Result<()> {
    unsafe {
        let mut cookie = null_mut();
        imp::CoIncrementMTAUsage(&mut cookie).ok()
    }
}

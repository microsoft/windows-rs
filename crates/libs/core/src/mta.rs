use crate::{Result, imp};
use core::ptr::null_mut;

/// Initializes COM for apartment-agnostic code and keeps the multithreaded apartment alive for the
/// rest of the process.
pub fn init_mta() -> Result<()> {
    unsafe {
        let mut cookie = null_mut();
        imp::CoIncrementMTAUsage(&mut cookie).ok()
    }
}

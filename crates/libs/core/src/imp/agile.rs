//! Platform abstraction for `AgileReference`'s inner storage.
//!
//! On Windows we wrap an `IAgileReference` obtained from `RoGetAgileReference`.
//! On non-Windows targets there is no apartment model, so any interface pointer
//! is already safe to use from any thread; we hold an `IUnknown` clone and
//! recover the requested interface on `resolve` via `QueryInterface`.

#[cfg(windows)]
use super::*;
use windows_core::*;

/// Inner storage for `AgileReference`. Wraps `IAgileReference` on Windows and
/// `IUnknown` elsewhere.
#[cfg(windows)]
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct AgileSlot(IAgileReference);

#[cfg(not(windows))]
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct AgileSlot(IUnknown);

impl AgileSlot {
    /// Creates a new agile slot for the given object.
    pub fn new<T: Interface>(object: &T) -> Result<Self> {
        #[cfg(windows)]
        unsafe {
            RoGetAgileReference(
                AGILEREFERENCE_DEFAULT,
                &T::IID,
                core::mem::transmute::<&T, &IUnknown>(object),
            )
            .map(Self)
        }
        #[cfg(not(windows))]
        {
            object.cast::<IUnknown>().map(Self)
        }
    }

    /// Resolves the slot back to the requested interface.
    pub fn resolve<T: Interface>(&self) -> Result<T> {
        #[cfg(windows)]
        unsafe {
            self.0.Resolve()
        }
        #[cfg(not(windows))]
        {
            self.0.cast()
        }
    }

    /// Returns the raw COM pointer.
    pub fn as_raw(&self) -> *mut core::ffi::c_void {
        self.0.as_raw()
    }
}

impl core::fmt::Debug for AgileSlot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

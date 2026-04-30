use super::*;
use core::marker::PhantomData;

/// A type representing an agile reference to a COM/WinRT object.
///
/// On Windows this wraps an `IAgileReference` obtained from `RoGetAgileReference`,
/// which marshals the object into the appropriate apartment on `resolve`.
/// On non-Windows targets there is no apartment model, so all interface pointers
/// are inherently agile; `AgileReference` simply holds an `IUnknown` reference and
/// `resolve` round-trips it back to the requested interface via `QueryInterface`.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct AgileReference<T>(
    #[cfg(windows)] imp::IAgileReference,
    #[cfg(not(windows))] IUnknown,
    PhantomData<T>,
);

impl<T: Interface> AgileReference<T> {
    /// Returns the raw COM pointer to the inner object.
    pub fn as_raw(&self) -> *mut core::ffi::c_void {
        self.0.as_raw()
    }

    /// Creates an agile reference to the object.
    pub fn new(object: &T) -> Result<Self> {
        const { assert!(T::UNKNOWN, "AgileReference requires a COM interface") };
        #[cfg(windows)]
        unsafe {
            imp::RoGetAgileReference(
                imp::AGILEREFERENCE_DEFAULT,
                &T::IID,
                core::mem::transmute::<&T, &IUnknown>(object),
            )
            .map(|reference| Self(reference, Default::default()))
        }
        #[cfg(not(windows))]
        {
            // On non-Windows there is no apartment model, so any interface
            // pointer is already safe to use from any thread. Hold an
            // `IUnknown` clone and recover `T` on `resolve` via QueryInterface.
            let unknown: IUnknown = object.cast()?;
            Ok(Self(unknown, PhantomData))
        }
    }

    /// Retrieves a proxy to the target of the `AgileReference` object that may safely be used within any thread context in which get is called.
    pub fn resolve(&self) -> Result<T> {
        #[cfg(windows)]
        unsafe {
            self.0.Resolve()
        }
        #[cfg(not(windows))]
        {
            self.0.cast()
        }
    }
}

unsafe impl<T: Interface> Send for AgileReference<T> {}
unsafe impl<T: Interface> Sync for AgileReference<T> {}

impl<T> core::fmt::Debug for AgileReference<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "AgileReference({:?})", &self.0)
    }
}

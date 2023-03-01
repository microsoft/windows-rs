use super::*;
use imp::*;
use std::marker::PhantomData;

/// A type representing an agile reference to a COM/WinRT object.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct AgileReference<T>(IAgileReference, PhantomData<T>);

impl<T: ComInterface> AgileReference<T> {
    /// Creates an agile reference to the object.
    pub fn new(object: &T) -> Result<Self> {
        let mut reference = std::ptr::null_mut();
        unsafe { RoGetAgileReference(AGILEREFERENCE_DEFAULT, &T::IID, object.as_raw(), &mut reference).from_abi(reference).map(|reference| Self(reference, Default::default())) }
    }

    /// Retrieves a proxy to the target of the `AgileReference` object that may safely be used within any thread context in which get is called.
    pub fn resolve(&self) -> Result<T> {
        unsafe { self.0.Resolve() }
    }
}

unsafe impl<T: ComInterface> Send for AgileReference<T> {}
unsafe impl<T: ComInterface> Sync for AgileReference<T> {}

impl<T> std::fmt::Debug for AgileReference<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AgileReference({:?})", &self.0 .0)
    }
}

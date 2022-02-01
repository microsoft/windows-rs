use super::*;
use bindings::*;
use core::marker::PhantomData;

/// A safe wrapper around WinRT interfaces.
///
/// Some interfaces are not marked as agile thus then don't implement [`Send`].
/// These interfaces can be made agile through this `AgileReference`.
pub struct AgileReference<T> {
    reference: IAgileReference,
    _marker: PhantomData<T>,
}

impl<T: Interface> AgileReference<T> {
    /// Creates a new wrapper around the reference
    pub fn new<'a>(from_ref: &'a T) -> Result<Self>
    where
        &'a T: IntoParam<'a, IUnknown>,
    {
        unsafe { RoGetAgileReference(AGILEREFERENCE_DEFAULT, &T::IID, from_ref).map(|reference| Self { reference, _marker: Default::default() }) }
    }

    /// Resolves the reference for the current thread.
    pub fn resolve(&self) -> Result<T> {
        unsafe { self.reference.Resolve() }
    }
}

unsafe impl<T: Interface> Send for AgileReference<T> {}
unsafe impl<T: Interface> Sync for AgileReference<T> {}

use crate::*;
use bindings::Windows::Win32::System::WinRT::{IWeakReference, IWeakReferenceSource};
use std::marker::PhantomData;

/// `Weak` holds a non-owning reference to an object.
#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct Weak<I: Interface>(Option<IWeakReference>, PhantomData<I>);

impl<I: Interface> Weak<I> {
    /// Creates a new `Weak` object without any backing object.
    pub fn new() -> Self {
        Self(None, PhantomData)
    }

    /// Attempts to upgrade the weak reference to a strong reference.
    pub fn upgrade(&self) -> Option<I> {
        self.0.as_ref().and_then(|inner| unsafe {
            // Calls IWeakReference's Resolve ABI directly as the metadata incorrectly types it as returning IInspectable.
            let mut result = None;
            let _ = (inner.vtable().3)(inner.abi(), &I::IID, &mut result as *mut _ as _);
            result
        })
    }

    pub(crate) fn downgrade(source: &IWeakReferenceSource) -> Result<Self> {
        let mut reference = None;
        unsafe {
            let _ = source.GetWeakReference(&mut reference);
        }
        Ok(Self(reference, PhantomData))
    }
}

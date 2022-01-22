use super::*;
use bindings::*;
use core::marker::PhantomData;

// TODO: does this need to exist now that the metadata correctly supports IUnknown?

/// `Weak` holds a non-owning reference to an object.
#[derive(Clone, PartialEq, Eq, Default)]
pub struct Weak<I: Interface>(Option<IWeakReference>, PhantomData<I>);

impl<I: Interface> Weak<I> {
    /// Creates a new `Weak` object without any backing object.
    pub fn new() -> Self {
        Self(None, PhantomData)
    }

    /// Attempts to upgrade the weak reference to a strong reference.
    pub fn upgrade(&self) -> Option<I> {
        self.0.as_ref().and_then(|inner| unsafe {
            // TODO: could just call Resolve directly
            let mut result = None;
            let _ = (inner.vtable().Resolve)(core::mem::transmute_copy(inner), &I::IID, &mut result as *mut _ as _);
            result
        })
    }

    pub(crate) fn downgrade(source: &IWeakReferenceSource) -> Result<Self> {
        // Why wrap?
        let reference = unsafe { source.GetWeakReference().ok() };
        Ok(Self(reference, PhantomData))
    }
}

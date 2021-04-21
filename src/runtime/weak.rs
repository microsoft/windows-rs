use crate::*;
use std::marker::PhantomData;

/// `Weak` holds a non-owning reference to an object.
#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct Weak<I: Interface>(Option<IWeakReference>, PhantomData<I>);

impl<I: Interface> Weak<I> {
    /// Creates a new `Weak` object without any backing object.
    pub fn new() -> Self {
        Self(None, PhantomData)
    }

    pub(crate) fn downgrade(source: &IWeakReferenceSource) -> Result<Self> {
        Ok(Self(Some(source.downgrade()?), PhantomData))
    }

    /// Attempts to upgradew the weak reference to a strong reference.
    pub fn upgrade(&self) -> Option<I> {
        self.0.as_ref().and_then(|inner| inner.upgrade::<I>())
    }
}

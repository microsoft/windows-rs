use super::*;
use bindings::*;
use core::marker::PhantomData;

pub struct AgileReference<T> {
    reference: IAgileReference,
    _marker: PhantomData<T>,
}

impl<T: Interface> AgileReference<T> {
    pub fn new<'a>(from_ref: &'a T) -> Result<Self>
    where
        &'a T: IntoParam<'a, IUnknown>,
    {
        unsafe { RoGetAgileReference(AGILEREFERENCE_DEFAULT, &T::IID, from_ref).map(|reference| Self { reference, _marker: Default::default() }) }
    }

    pub fn resolve(&self) -> Result<T> {
        unsafe { self.reference.Resolve() }
    }
}

unsafe impl<T: Interface> Send for AgileReference<T> {}
unsafe impl<T: Interface> Sync for AgileReference<T> {}

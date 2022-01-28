use super::*;
use bindings::*;
use core::marker::PhantomData;

pub struct AgileReference<T> {
    reference: IAgileReference,
    _marker: PhantomData<T>,
}

impl<T: Interface> AgileReference<T> {
    pub fn new<'a, R>(from_ref: R) -> Result<Self>
    where
        R: IntoParam<'a, T> + IntoParam<'a, IUnknown>,
    {
        let reference = unsafe { RoGetAgileReference(AGILEREFERENCE_DEFAULT, &T::IID, from_ref)? };
        Ok(Self { reference, _marker: Default::default() })
    }

    pub fn resolve(&self) -> Result<T> {
        unsafe {
            let mut ptr = core::ptr::null_mut();
            self.reference.Resolve(&T::IID, &mut ptr)?;

            let resolved = IUnknown(core::ptr::NonNull::new(ptr).ok_or_else(|| Error::from(E_NOINTERFACE))?);

            resolved.cast()
        }
    }
}

unsafe impl<T: Interface> Send for AgileReference<T> {}
unsafe impl<T: Interface> Sync for AgileReference<T> {}

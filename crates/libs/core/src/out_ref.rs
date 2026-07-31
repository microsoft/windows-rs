use super::*;

/// A borrowed ABI-compatible out parameter.
#[repr(transparent)]
pub struct OutRef<'a, T: Type<T>>(*mut T::Abi, core::marker::PhantomData<&'a T>);

impl<T: Type<T>> OutRef<'_, T> {
    /// Returns `true` if the argument is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Writes a value without reading or dropping the previous contents.
    pub fn write(self, value: T::Default) -> Result<()> {
        if self.0.is_null() {
            Err(Error::from_hresult(imp::E_POINTER))
        } else {
            unsafe { *self.0 = core::mem::transmute_copy(&value) }
            core::mem::forget(value);
            Ok(())
        }
    }
}

impl<'a, T: Type<T>> From<&'a mut T::Default> for OutRef<'a, T> {
    fn from(from: &'a mut T::Default) -> Self {
        unsafe { core::mem::transmute(from) }
    }
}

impl<T: Type<T>> Default for OutRef<'_, T> {
    fn default() -> Self {
        OutRef(core::ptr::null_mut(), core::marker::PhantomData)
    }
}

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

impl<'a, T: Type<T>> From<&'a mut T::Default> for OutRef<'a, T>
where
    &'a mut T::Default: OutParam<T>,
{
    /// Releases the previous owned value before borrowing the output storage.
    fn from(from: &'a mut T::Default) -> Self {
        // The pointer borrows the caller's storage, not the local reference.
        Self(
            unsafe { OutParam::<T>::borrow_mut(&from) }.0,
            core::marker::PhantomData,
        )
    }
}

impl<T: Type<T>> Default for OutRef<'_, T> {
    fn default() -> Self {
        OutRef(core::ptr::null_mut(), core::marker::PhantomData)
    }
}

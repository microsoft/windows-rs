use super::*;

/// A borrowed type with the same memory layout as the type itself that can be used to construct ABI-compatible function signatures.
#[repr(transparent)]
pub struct Ref<'a, T: Type<T>>(T::Abi, std::marker::PhantomData<&'a T>);

impl<'a, T: Type<T>> Ref<'a, T> {
    /// Reads the borrowed value.
    pub fn read(&self) -> &T::Default {
        unsafe { std::mem::transmute(&self.0) }
    }

    /// Clones the borrowed value.
    pub fn ok(&self) -> Result<T> {
        T::from_default(self.read())
    }
}

use super::*;

/// A borrowed type with the same memory layout as the type itself that can be used to construct ABI-compatible function signatures.
#[repr(transparent)]
pub struct Ref<'a, T: Type<T>>(T::Abi, std::marker::PhantomData<&'a T>);

impl<'a, T: Type<T>> std::ops::Deref for Ref<'a, T> {
    type Target = T::Default;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.0) }
    }
}

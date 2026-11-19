use super::*;
use core::mem::transmute;

/// A borrowed ABI-compatible input parameter.
#[repr(transparent)]
pub struct InRef<'a, T: Type>(T::Abi, core::marker::PhantomData<&'a T>);

impl<T: Type> InRef<'_, T> {
    /// Returns `true` if the argument is null.
    pub fn is_null(&self) -> bool {
        self.as_ref().is_none()
    }

    /// Returns the referenced value or `E_POINTER` if null.
    pub fn ok(&self) -> Result<&T> {
        self.as_ref()
            .ok_or_else(|| Error::from_hresult(imp::E_POINTER))
    }

    /// Returns the referenced value if non-null.
    pub fn as_ref(&self) -> Option<&T> {
        T::as_ref(self)
    }

    /// Returns the referenced value.
    ///
    /// # Panics
    ///
    /// Panics if the argument is null.
    #[track_caller]
    pub fn unwrap(&self) -> &T {
        self.as_ref()
            .expect("called `InRef::unwrap` on a null value")
    }

    /// Converts the argument to an [`Option<T>`] by cloning the reference.
    pub fn cloned(&self) -> Option<T> {
        self.as_ref().cloned()
    }
}

impl<T: Type> Default for InRef<'_, T> {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

impl<T: Type> core::ops::Deref for InRef<'_, T> {
    type Target = T::Default;
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.0) }
    }
}

impl<'a, T: Type<InterfaceType>> From<&'a Option<T>> for InRef<'a, T>
where
    T: TypeKind<TypeKind = InterfaceType>,
{
    fn from(from: &'a Option<T>) -> Self {
        unsafe { core::mem::transmute_copy(from) }
    }
}

impl<'a, T: Type<InterfaceType>> From<Option<&'a T>> for InRef<'a, T>
where
    T: TypeKind<TypeKind = InterfaceType>,
{
    fn from(from: Option<&'a T>) -> Self {
        if let Some(from) = from {
            unsafe { core::mem::transmute_copy(from) }
        } else {
            unsafe { core::mem::zeroed() }
        }
    }
}

impl<'a, T: Type> From<&'a T> for InRef<'a, T> {
    fn from(from: &'a T) -> Self {
        unsafe { core::mem::transmute_copy(from) }
    }
}

use super::*;

#[doc(hidden)]
pub enum Param<T: Type<T>> {
    Owned(T),
    Borrowed(T::Abi),
}

impl<T: Type<T>> Param<T> {
    pub fn abi(&self) -> T::Abi {
        unsafe {
            match self {
                Self::Owned(item) => std::mem::transmute_copy(item),
                Self::Borrowed(borrowed) => std::mem::transmute_copy(borrowed),
            }
        }
    }
}

#[doc(hidden)]
pub trait CanInto<T>: Sized {
    const QUERY: bool = false;
}

impl<T> CanInto<T> for T where T: Clone {}

#[doc(hidden)]
pub trait IntoParam<T: TypeKind, C = <T as TypeKind>::TypeKind>: Sized
where
    T: Type<T>,
{
    unsafe fn into_param(self) -> Param<T>;
}

impl<T> IntoParam<T> for Option<&T>
where
    T: Type<T>,
{
    unsafe fn into_param(self) -> Param<T> {
        Param::Borrowed(match self {
            Some(item) => std::mem::transmute_copy(item),
            None => std::mem::zeroed(),
        })
    }
}

impl<T, U> IntoParam<T, ReferenceType> for &U
where
    T: TypeKind<TypeKind = ReferenceType> + Clone,
    T: Interface,
    U: Interface,
    U: CanInto<T>,
{
    unsafe fn into_param(self) -> Param<T> {
        if U::QUERY {
            self.cast().map_or(Param::Borrowed(std::mem::zeroed()), |ok| Param::Owned(ok))
        } else {
            Param::Borrowed(std::mem::transmute_copy(self))
        }
    }
}

impl<T> IntoParam<T, ValueType> for &T
where
    T: TypeKind<TypeKind = ValueType> + Clone,
{
    unsafe fn into_param(self) -> Param<T> {
        Param::Borrowed(std::mem::transmute_copy(self))
    }
}

impl<T, U> IntoParam<T, CopyType> for U
where
    T: TypeKind<TypeKind = CopyType> + Clone,
    U: TypeKind<TypeKind = CopyType> + Clone,
    U: CanInto<T>,
{
    unsafe fn into_param(self) -> Param<T> {
        Param::Owned(std::mem::transmute_copy(&self))
    }
}

impl IntoParam<PCWSTR> for &BSTR {
    unsafe fn into_param(self) -> Param<PCWSTR> {
        Param::Owned(PCWSTR(self.as_ptr()))
    }
}

impl IntoParam<PCWSTR> for &HSTRING {
    unsafe fn into_param(self) -> Param<PCWSTR> {
        Param::Owned(PCWSTR(self.as_ptr()))
    }
}

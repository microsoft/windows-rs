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

pub trait CanInto<T>: Sized {
    const QUERY: bool = false;
}

impl<T> CanInto<T> for T where T: Clone {}

#[doc(hidden)]
pub trait IntoParam<T: TypeKind, C = <T as TypeKind>::TypeKind>: Sized
where
    T: Type<T>,
{
    fn into_param(self) -> Param<T>;
}

impl<T> IntoParam<T> for Option<&T>
where
    T: Type<T>,
{
    fn into_param(self) -> Param<T> {
        Param::Borrowed(match self {
            Some(item) => item.abi(),
            None => unsafe { std::mem::zeroed() },
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
    fn into_param(self) -> Param<T> {
        unsafe {
            if U::QUERY {
                self.cast().map_or(Param::Borrowed(std::mem::zeroed()), |ok| Param::Owned(ok))
            } else {
                Param::Borrowed(std::mem::transmute_copy(self))
            }
        }
    }
}

impl<T> IntoParam<T, ValueType> for &T
where
    T: TypeKind<TypeKind = ValueType> + Clone,
{
    fn into_param(self) -> Param<T> {
        Param::Borrowed(self.abi())
    }
}

impl<T, U> IntoParam<T, CopyType> for U
where
    T: TypeKind<TypeKind = CopyType> + Clone,
    U: TypeKind<TypeKind = CopyType> + Clone,
    U: CanInto<T>,
{
    fn into_param(self) -> Param<T> {
        Param::Owned(unsafe { std::mem::transmute_copy(&self) })
    }
}

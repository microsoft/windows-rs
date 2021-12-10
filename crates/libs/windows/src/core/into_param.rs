use super::*;

#[doc(hidden)]
pub trait IntoParam<'a, T: Abi> {
    fn into_param(self) -> Param<'a, T>;
}

impl<'a, T: Abi> IntoParam<'a, T> for T {
    fn into_param(self) -> Param<'a, T> {
        Param::Owned(self)
    }
}

impl<'a, T: Abi> IntoParam<'a, T> for &'a T {
    fn into_param(self) -> Param<'a, T> {
        Param::Borrowed(self)
    }
}

impl<'a, T: Abi> IntoParam<'a, T> for Option<T> {
    fn into_param(self) -> Param<'a, T> {
        match self {
            Some(value) => Param::Owned(value),
            None => Param::None,
        }
    }
}

impl<'a, T: Abi> IntoParam<'a, T> for &'a Option<T> {
    fn into_param(self) -> Param<'a, T> {
        match self {
            Some(value) => Param::Borrowed(value),
            None => Param::None,
        }
    }
}

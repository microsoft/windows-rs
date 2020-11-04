use crate::*;

/// A WinRT method parameter used to accept either a reference or value.
pub enum Param<'a, T: GetAbi> {
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T: GetAbi> Param<'a, T> {
    pub unsafe fn get_abi(&mut self) -> T::Abi {
        match self {
            Param::Borrowed(value) => value.get_abi(),
            Param::Owned(value) => value.get_abi(),
        }
    }
}

impl<'a, T: GetAbi> From<T> for Param<'a, T> {
    fn from(value: T) -> Self {
        Param::Owned(value)
    }
}

impl<'a, T: GetAbi> From<&'a T> for Param<'a, T> {
    fn from(value: &'a T) -> Self {
        Param::Borrowed(value)
    }
}

impl<'a> From<&'a str> for Param<'a, HString> {
    fn from(value: &'a str) -> Self {
        Param::Owned(value.into())
    }
}

impl<'a> From<String> for Param<'a, HString> {
    fn from(value: String) -> Self {
        Param::Owned(value.into())
    }
}

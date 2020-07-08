use crate::{AbiTransferable, HString};

/// A WinRT method parameter used to accept either a reference or value.
pub enum Param<'a, T: AbiTransferable> {
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T: AbiTransferable> Param<'a, T> {
    pub fn get_abi(&mut self) -> T::Abi {
        match self {
            Param::Borrowed(value) => value.get_abi(),
            Param::Owned(value) => value.get_abi(),
        }
    }
}

impl<'a, T: AbiTransferable> From<T> for Param<'a, T> {
    fn from(value: T) -> Self {
        Param::Owned(value)
    }
}

impl<'a, T: AbiTransferable> From<&'a T> for Param<'a, T> {
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

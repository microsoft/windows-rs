use crate::*;

/// A WinRT method parameter used to accept either a reference or value.
pub enum Param<'a, T: Abi> {
    Borrowed(&'a T),
    Owned(T),
    None,
}

impl<'a, T: Abi> Param<'a, T> {
    pub unsafe fn get_abi(&mut self) -> T::Abi {
        match self {
            Param::Borrowed(value) => value.get_abi(),
            Param::Owned(value) => value.get_abi(),
            Param::None => std::mem::zeroed(),
        }
    }
}

impl<'a, T: Abi> From<T> for Param<'a, T> {
    fn from(value: T) -> Self {
        Param::Owned(value)
    }
}

impl<'a, T: Abi> From<&'a T> for Param<'a, T> {
    fn from(value: &'a T) -> Self {
        Param::Borrowed(value)
    }
}

impl<'a, T: Interface> From<Option<T>> for Param<'a, T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Param::Owned(value),
            None => Param::None,
        }
    }
}

impl<'a, T: Interface> From<&'a Option<T>> for Param<'a, T> {
    fn from(value: &'a Option<T>) -> Self {
        match value {
            Some(value) => Param::Borrowed(value),
            None => Param::None,
        }
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

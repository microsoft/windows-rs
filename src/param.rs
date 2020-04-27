use crate::*;

pub enum Param<'a, T: RuntimeType> {
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T: RuntimeType> Param<'a, T> {
    pub fn abi(&mut self) -> T::Abi {
        match self {
            Param::Borrowed(value) => value.abi(),
            Param::Owned(value) => value.abi(),
        }
    }
}

impl<'a, T: RuntimeType> From<T> for Param<'a, T> {
    fn from(value: T) -> Param<'a, T> {
        Param::Owned(value)
    }
}

impl<'a, T: RuntimeType> From<&'a T> for Param<'a, T> {
    fn from(value: &'a T) -> Param<'a, T> {
        Param::Borrowed(value)
    }
}

impl<'a> From<&'a str> for Param<'a, HString> {
    fn from(value: &'a str) -> Param<'a, HString> {
        Param::Owned(value.into())
    }
}

impl<'a> From<String> for Param<'a, HString> {
    fn from(value: String) -> Param<'a, HString> {
        Param::Owned(value.into())
    }
}

use crate::*;

pub enum Param<'a, T> {
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

pub enum StringParam<'a> {
    Borrowed(&'a str),
    Owned(String),
    RuntimeBorrowed(&'a super::HString),
    RuntimeOwned(super::HString),
}

impl<'a> StringParam<'a> {
    pub fn abi(&mut self) -> RawPtr {
        match self {
            StringParam::Borrowed(value) => {
                *self = StringParam::RuntimeOwned((*value).into());
                self.abi()
            }
            StringParam::Owned(value) => {
                *self = StringParam::RuntimeOwned(value.as_str().into());
                self.abi()
            }
            StringParam::RuntimeOwned(value) => value.abi(),
            StringParam::RuntimeBorrowed(value) => value.abi(),
        }
    }
}

impl<'a> Into<StringParam<'a>> for &'a str {
    fn into(self) -> StringParam<'a> {
        StringParam::Borrowed(self)
    }
}

impl<'a> Into<StringParam<'a>> for String {
    fn into(self) -> StringParam<'a> {
        StringParam::Owned(self)
    }
}

impl<'a> Into<StringParam<'a>> for super::HString {
    fn into(self) -> StringParam<'a> {
        StringParam::RuntimeOwned(self)
    }
}

impl<'a> Into<StringParam<'a>> for &'a super::HString {
    fn into(self) -> StringParam<'a> {
        StringParam::RuntimeBorrowed(self)
    }
}

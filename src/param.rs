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

impl<'a> From<&'a str> for StringParam<'a> {
    fn from(value: &'a str) -> StringParam<'a> {
        StringParam::Borrowed(value)
    }
}

impl<'a> From<String> for StringParam<'a> {
    fn from(value: String) -> StringParam<'a> {
        StringParam::Owned(value)
    }
}

impl<'a> From<HString> for StringParam<'a> {
    fn from(value: HString) -> StringParam<'a> {
        StringParam::RuntimeOwned(value)
    }
}

impl<'a> From<&'a HString> for StringParam<'a> {
    fn from(value: &'a HString) -> StringParam<'a> {
        StringParam::RuntimeBorrowed(value)
    }
}

use crate::*;

pub enum Param<'a, T> {
    Ref(&'a T),
    Value(T),
}

impl<'a, T: RuntimeType> Param<'a, T> {
    pub fn abi(&mut self) -> T::Abi {
        match self {
            Param::Ref(value) => value.abi(),
            Param::Value(value) => value.abi(),
        }
    }
}

pub enum StringParam<'a> {
    Ref(&'a str),
    String(std::string::String),
    Winrt(super::String),
    WinrtRef(&'a super::String),
}

impl<'a> StringParam<'a> {
    pub fn abi(&mut self) -> RawPtr {
        match self {
            StringParam::Ref(value) => {
                *self = StringParam::Winrt((*value).into());
                self.abi()
            }
            StringParam::String(value) => {
                *self = StringParam::Winrt(value.as_str().into());
                self.abi()
            }
            StringParam::Winrt(value) => value.abi(),
            StringParam::WinrtRef(value) => value.abi(),
        }
    }
}

impl<'a> Into<StringParam<'a>> for &'a str {
    fn into(self) -> StringParam<'a> {
        StringParam::Ref(self)
    }
}

impl<'a> Into<StringParam<'a>> for std::string::String {
    fn into(self) -> StringParam<'a> {
        StringParam::String(self)
    }
}

impl<'a> Into<StringParam<'a>> for super::String {
    fn into(self) -> StringParam<'a> {
        StringParam::Winrt(self)
    }
}

impl<'a> Into<StringParam<'a>> for &'a super::String {
    fn into(self) -> StringParam<'a> {
        StringParam::WinrtRef(self)
    }
}

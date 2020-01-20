use crate::*;

pub enum String<'a> {
    Ref(&'a str),
    String(std::string::String),
    Winrt(super::String),
    WinrtRef(&'a super::String),
}

impl<'a> InParamType for String<'a> {
    type Abi = RawPtr;

    fn as_abi(&mut self) -> Self::Abi {
        match self {
            String::Ref(value) => {
                *self = String::Winrt((*value).into());
                self.as_abi()
            }
            String::String(value) => {
                *self = String::Winrt(value.as_str().into());
                self.as_abi()
            }
            String::Winrt(value) => value.as_abi(),
            String::WinrtRef(value) => value.as_abi(),
        }
    }
}

impl<'a> Into<String<'a>> for &'a str {
    fn into(self) -> String<'a> {
        String::Ref(self)
    }
}

impl<'a> Into<String<'a>> for std::string::String {
    fn into(self) -> String<'a> {
        String::String(self)
    }
}

impl<'a> Into<String<'a>> for super::String {
    fn into(self) -> String<'a> {
        String::Winrt(self)
    }
}

impl<'a> Into<String<'a>> for &'a super::String {
    fn into(self) -> String<'a> {
        String::WinrtRef(self)
    }
}

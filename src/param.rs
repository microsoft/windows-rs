use crate::*;

pub enum String<'a> {
    Ref(&'a str),
    String(std::string::String),
    Winrt(super::String),
    WinrtRef(&'a super::String),
}

impl<'a> String<'a> {
    pub fn as_abi_in(&mut self) -> *const std::ffi::c_void {
        match self {
            String::Ref(value) => {
                *self = String::Winrt((*value).into());
                self.as_abi_in()
            },
            String::String(value) => {
                *self = String::Winrt(value.as_str().into());
                self.as_abi_in()
            },
            String::Winrt(value) => value.as_abi_in(),
            String::WinrtRef(value) => value.as_abi_in(),
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

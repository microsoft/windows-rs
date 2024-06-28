use super::*;

pub trait ToUtf8String {
    fn to_utf8_string(&self) -> PCSTR;
}

pub trait ToUtf16String {
    fn to_utf16_string(&self) -> PCWSTR;
}

impl ToUtf8String for String {
    fn to_utf8_string(&self) -> PCSTR {
        s!(self.as_str())
    }
}

impl ToUtf16String for String {
    fn to_utf16_string(&self) -> PCWSTR {
        w!(self.as_str())
    }
}

impl From<String> for PCSTR {
    fn from(s: String) -> PCSTR {
        s.to_utf8_string()
    }
}

impl From<String> for PCWSTR {
    fn from(s: String) -> PCWSTR {
        s.to_utf16_string()
    }
}

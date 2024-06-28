use super::*;

pub trait ToWideString {
    fn to_wide_string(&self) -> PCWSTR;
}

impl ToWideString for String {
    fn to_wide_string(&self) -> PCWSTR {
        w!(self)
    }
}

impl From<String> for PCWSTR {
    fn from(s: String) -> PCWSTR {
        s.to_wide_string()
    }
}

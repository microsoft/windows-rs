pub enum String<'a> {
    Ref(&'a str),
    String(std::string::String),
    WinrtString(super::String),
    WinrtStringRef(&'a super::String),
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
        String::WinrtString(self)
    }
}


impl<'a> Into<String<'a>> for &'a super::String {
    fn into(self) -> String<'a> {
        String::WinrtStringRef(self)
    }
}

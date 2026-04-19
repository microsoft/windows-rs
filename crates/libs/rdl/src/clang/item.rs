use super::*;

#[derive(Debug)]
pub enum Item {
    Callback(Callback),
    Const(Const),
    Enum(Enum),
    Interface(Interface),
    Struct(Struct),
    Typedef(Typedef),
    Fn(Fn),
}

impl Item {
    pub fn write(&self) -> Result<TokenStream, Error> {
        match self {
            Self::Callback(item) => item.write(),
            Self::Const(item) => item.write(),
            Self::Enum(item) => item.write(),
            Self::Interface(item) => item.write(),
            Self::Struct(item) => item.write(),
            Self::Typedef(item) => item.write(),
            Self::Fn(item) => item.write(),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Callback(item) => item.name.fmt(f),
            Self::Const(item) => item.name.fmt(f),
            Self::Enum(item) => item.name.fmt(f),
            Self::Interface(item) => item.name.fmt(f),
            Self::Struct(item) => item.name.fmt(f),
            Self::Typedef(item) => item.name.fmt(f),
            Self::Fn(item) => item.name.fmt(f),
        }
    }
}

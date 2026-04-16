use super::*;

#[derive(Debug)]
pub enum Item {
    Enum(Enum),
    Struct(Struct),
    Typedef(Typedef),
}

impl Item {
    pub fn write(&self) -> Result<TokenStream, Error> {
        match self {
            Self::Enum(item) => item.write(),
            Self::Struct(item) => item.write(),
            Self::Typedef(item) => item.write(),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Enum(item) => item.name.fmt(f),
            Self::Struct(item) => item.name.fmt(f),
            Self::Typedef(item) => item.name.fmt(f),
        }
    }
}

use crate::flags::*;
use crate::tables::*;
use crate::types::*;
use crate::TypeReader;

use squote::TokenStream;

#[derive(Debug)]
pub enum Type {
    Class(Class),
    Interface(Interface),
    Enum(Enum),
    Struct(Struct),
    Delegate(Delegate),
}

impl Type {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def, "");
        match def.category(reader) {
            TypeCategory::Interface => Self::Interface(Interface::from_type_name(reader, name)),
            TypeCategory::Class => Self::Class(Class::from_type_name(reader, name)),
            TypeCategory::Enum => Self::Enum(Enum::from_type_name(reader, name)),
            TypeCategory::Struct => Self::Struct(Struct::from_type_name(reader, name)),
            TypeCategory::Delegate => Self::Delegate(Delegate::from_type_name(reader, name)),
        }
    }

    pub fn to_tokens(&self) -> TokenStream {
        match self {
            Type::Class(t) => t.to_tokens(),
            Type::Interface(t) => t.to_tokens(),
            Type::Enum(t) => t.to_tokens(),
            Type::Struct(t) => t.to_tokens(),
            Type::Delegate(t) => t.to_tokens(),
        }
    }

    pub fn name(&self) -> &TypeName {
        match self {
            Type::Class(t) => &t.name,
            Type::Interface(t) => &t.name,
            Type::Enum(t) => &t.name,
            Type::Struct(t) => &t.name,
            Type::Delegate(t) => &t.name,
        }
    }

    // TODO: ideally this would return an iterator to avoid repeated allocations
    pub fn dependencies(&self) -> Vec<TypeDef> {
        match self {
            Type::Class(t) => t.dependencies(),
            Type::Interface(t) => t.dependencies(),
            Type::Enum(_t) => Vec::new(),
            Type::Struct(t) => t.dependencies(),
            Type::Delegate(t) => t.dependencies(),
        }
    }
}

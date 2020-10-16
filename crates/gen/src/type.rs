use crate::*;

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
    pub fn from_type_def(reader: &winmd::TypeReader, def: winmd::TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def, "");
        match def.category(reader) {
            winmd::TypeCategory::Interface => {
                Self::Interface(Interface::from_type_name(reader, name))
            }
            winmd::TypeCategory::Class => Self::Class(Class::from_type_name(reader, name)),
            winmd::TypeCategory::Enum => Self::Enum(Enum::from_type_name(reader, name)),
            winmd::TypeCategory::Struct => Self::Struct(Struct::from_type_name(reader, name)),
            winmd::TypeCategory::Delegate => Self::Delegate(Delegate::from_type_name(reader, name)),
        }
    }

    pub fn gen(&self) -> TokenStream {
        match self {
            Type::Class(t) => t.gen(),
            Type::Interface(t) => t.gen(),
            Type::Enum(t) => t.gen(),
            Type::Struct(t) => t.gen(),
            Type::Delegate(t) => t.gen(),
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

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        match self {
            Type::Class(t) => t.dependencies(),
            Type::Interface(t) => t.dependencies(),
            Type::Enum(_t) => Vec::new(),
            Type::Struct(t) => t.dependencies(),
            Type::Delegate(t) => t.dependencies(),
        }
    }
}

use crate::*;

use squote::TokenStream;

#[derive(Debug)]
pub enum Type {
    Class(gen::Class),
    Interface(gen::Interface),
    Enum(gen::Enum),
    Struct(gen::Struct),
    Delegate(gen::Delegate),
}

impl Type {
    pub fn from_type_def(reader: &winmd::TypeReader, def: winmd::TypeDef) -> Self {
        let name = gen::TypeName::from_type_def(reader, def, "");
        match def.category(reader) {
            winmd::TypeCategory::Interface => {
                Self::Interface(gen::Interface::from_type_name(reader, name))
            }
            winmd::TypeCategory::Class => Self::Class(gen::Class::from_type_name(reader, name)),
            winmd::TypeCategory::Enum => Self::Enum(gen::Enum::from_type_name(reader, name)),
            winmd::TypeCategory::Struct => Self::Struct(gen::Struct::from_type_name(reader, name)),
            winmd::TypeCategory::Delegate => {
                Self::Delegate(gen::Delegate::from_type_name(reader, name))
            }
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

    pub fn name(&self) -> &gen::TypeName {
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

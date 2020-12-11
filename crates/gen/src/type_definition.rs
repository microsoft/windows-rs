use crate::*;

use squote::TokenStream;

#[derive(Debug)]
pub enum TypeDefinition {
    Class(Class),
    Interface(Interface),
    Enum(Enum),
    Struct(Struct),
    Delegate(Delegate),
    Class32(Class32),
    Interface32(Interface32),
    Delegate32(Delegate32),
}

impl TypeDefinition {
    pub fn from_type_def(def: &winmd::TypeDef) -> Self {
        let name = TypeName::from_type_def(def, def.name().0);

        match def.category() {
            winmd::TypeCategory::Interface => {
                if def.is_winrt() {
                    Self::Interface(Interface::from_type_name(name))
                } else {
                    Self::Interface32(Interface32::from_type_name(name))
                }
            }
            winmd::TypeCategory::Class => {
                if def.is_winrt() {
                    Self::Class(Class::from_type_name(name))
                } else {
                    Self::Class32(Class32::from_type_name(name))
                }
            }
            winmd::TypeCategory::Enum => Self::Enum(Enum::from_type_name(name)),
            winmd::TypeCategory::Struct => Self::Struct(Struct::from_type_name(name)),
            winmd::TypeCategory::Delegate => {
                if def.is_winrt() {
                    Self::Delegate(Delegate::from_type_name(name))
                } else {
                    Self::Delegate32(Delegate32::from_type_name(name))
                }
            }
            _ => panic!("TypeDefinition.from_type_def {:?}", def.name()),
        }
    }

    pub fn gen(&self) -> TokenStream {
        match self {
            Self::Class(t) => t.gen(),
            Self::Interface(t) => t.gen(),
            Self::Enum(t) => t.gen(),
            Self::Struct(t) => t.gen(),
            Self::Delegate(t) => t.gen(),
            Self::Class32(t) => t.gen(),
            Self::Interface32(t) => t.gen(),
            Self::Delegate32(t) => t.gen(),
        }
    }

    pub fn name(&self) -> &TypeName {
        match self {
            Self::Class(t) => &t.name,
            Self::Interface(t) => &t.name,
            Self::Enum(t) => &t.name,
            Self::Struct(t) => &t.name,
            Self::Delegate(t) => &t.name,
            Self::Class32(t) => &t.name,
            Self::Interface32(t) => &t.name,
            Self::Delegate32(t) => &t.name,
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        match self {
            Self::Class(t) => t.dependencies(),
            Self::Interface(t) => t.dependencies(),
            Self::Enum(_) => Vec::new(),
            Self::Struct(t) => t.dependencies(),
            Self::Delegate(t) => t.dependencies(),
            Self::Class32(t) => t.dependencies(),
            Self::Interface32(t) => t.dependencies(),
            Self::Delegate32(t) => t.dependencies(),
        }
    }
}

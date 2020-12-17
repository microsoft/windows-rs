use crate::*;

use squote::TokenStream;

#[derive(Debug)]
pub enum TypeDefinition {
    Class(Class),
    Interface(Interface),
    Enum(Enum),
    Struct(Struct),
    Delegate(Delegate),
    ComClass(ComClass),
    ComInterface(ComInterface),
    Callback(Callback),
    Constant(Constant),
    Function(Function),
}

impl TypeDefinition {
    pub fn from_type_def(def: &winmd::TypeDef) -> Self {
        let name = TypeName::from_type_def(def, def.name().0);

        match def.category() {
            winmd::TypeCategory::Interface => {
                if def.is_winrt() {
                    Self::Interface(Interface::from_type_name(name))
                } else {
                    Self::ComInterface(ComInterface::from_type_name(name))
                }
            }
            winmd::TypeCategory::Class => {
                if def.is_winrt() {
                    Self::Class(Class::from_type_name(name))
                } else {
                    Self::ComClass(ComClass::from_type_name(name))
                }
            }
            winmd::TypeCategory::Enum => Self::Enum(Enum::from_type_name(name)),
            winmd::TypeCategory::Struct => Self::Struct(Struct::from_type_name(name)),
            winmd::TypeCategory::Delegate => {
                if def.is_winrt() {
                    Self::Delegate(Delegate::from_type_name(name))
                } else {
                    Self::Callback(Callback::from_type_name(name))
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
            Self::ComClass(t) => t.gen(),
            Self::ComInterface(t) => t.gen(),
            Self::Callback(t) => t.gen(),
            Self::Constant(t) => t.gen(),
            Self::Function(t) => t.gen(),
        }
    }

    // TODO: get rid of this 
    pub fn name(&self) -> &TypeName {
        match self {
            Self::Class(t) => &t.name,
            Self::Interface(t) => &t.name,
            Self::Enum(t) => &t.name,
            Self::Struct(t) => &t.name,
            Self::Delegate(t) => &t.name,
            Self::ComClass(t) => &t.name,
            Self::ComInterface(t) => &t.name,
            Self::Callback(t) => &t.name,
            Self::Constant(t) => &t.name,
            Self::Function(t) => &t.name,
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        match self {
            Self::Class(t) => t.dependencies(),
            Self::Interface(t) => t.dependencies(),
            Self::Struct(t) => t.dependencies(),
            Self::Delegate(t) => t.dependencies(),
            Self::ComInterface(t) => t.dependencies(),
            Self::Callback(t) => t.dependencies(),
            Self::Function(t) => t.dependencies(),
            _ => Vec::new(),
        }
    }
}

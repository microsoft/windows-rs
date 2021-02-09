use crate::*;

use squote::TokenStream;

#[derive(Debug)]
pub enum TypeDefinition {
    Class(Class),
    Interface(Interface),
    Enum(Enum),
    Struct(Struct),
    Delegate(Delegate),
    ComInterface(ComInterface),
    Callback(Callback),
    Constant(Constant),
    Function(Function),
}

impl TypeDefinition {
    pub fn from_type_row(row: &winmd::Type) -> Self {
        match row {
            winmd::Type::TypeDef(def) => Self::from_type_def(def),
            winmd::Type::MethodDef((def, method)) => Self::Function(Function::new(
                TypeName::from_type_def(def, def.namespace()),
                method,
            )),
            winmd::Type::Field(field) => Self::Constant(Constant::new(
                field,
            )),
        }
    }

    pub fn from_method_def(def: &winmd::TypeDef, method: &winmd::MethodDef) -> Self {
        let name = TypeName::from_type_def(def, def.namespace());
        TypeDefinition::Function(Function::new(name, method))
    }

    pub fn from_field(field: &winmd::Field) -> Self {
        TypeDefinition::Constant(Constant::new(field))
    }

    pub fn from_type_def(def: &winmd::TypeDef) -> Self {
        let name = TypeName::from_type_def(def, def.namespace());

        match def.category() {
            winmd::TypeCategory::Interface => {
                if def.is_winrt() {
                    Self::Interface(Interface::from_type_name(name))
                } else {
                    Self::ComInterface(ComInterface::from_type_name(name))
                }
            }
            winmd::TypeCategory::Class => Self::Class(Class::from_type_name(name)),
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
            Self::ComInterface(t) => t.gen(),
            Self::Callback(t) => t.gen(),
            Self::Constant(t) => t.gen(),
            Self::Function(t) => t.gen(),
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

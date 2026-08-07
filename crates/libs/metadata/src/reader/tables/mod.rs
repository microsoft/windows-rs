use super::*;

mod assembly_ref;
mod attribute;
mod class_layout;
mod constant;
mod event;
mod event_map;
mod field;
mod field_layout;
mod generic_param;
mod impl_map;
mod interface_impl;
mod member_ref;
mod method_def;
mod method_param;
mod method_semantics;
mod module;
mod module_ref;
mod nested_class;
mod property;
mod property_map;
mod type_def;
mod type_ref;
mod type_spec;

pub use attribute::{AttributeArg, AttributeArgKind, AttributeValueError};
pub use method_def::{MethodParamMap, MethodParamSequenceError};
pub use method_param::{BufferRelationship, ParamDirection};

macro_rules! table_id {
    (MethodParam) => {
        TableId::Param
    };
    ($name:ident) => {
        TableId::$name
    };
}

macro_rules! tables {
    ($(($name:ident, $table:literal))+) => {
        $(
        #[derive(Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
        pub struct $name<'a>(pub(crate) Row<'a>);
        impl<'a> AsRow<'a> for $name<'a> {
            const TABLE: usize = $table;
            const TABLE_ID: TableId = table_id!($name);
            fn to_row(&self) -> Row<'a> {
                self.0
            }
            fn from_row(row: Row<'a>) -> Self {
                $name(row)
            }
        }
    )*
    };
}

tables! {
    (Assembly, 0)
    (AssemblyRef, 1)
    (Attribute, 2)
    (ClassLayout, 3)
    (Constant, 4)
    (Event, 5)
    (EventMap, 6)
    (Field, 7)
    (GenericParam, 8)
    (ImplMap, 9)
    (InterfaceImpl, 10)
    (MemberRef, 11)
    (MethodDef, 12)
    (MethodParam, 13)
    (MethodSemantics, 14)
    (Module, 15)
    (ModuleRef, 16)
    (NestedClass, 17)
    (Property, 18)
    (PropertyMap, 19)
    (TypeDef, 20)
    (TypeRef, 21)
    (TypeSpec, 22)
    (FieldLayout, 23)
}

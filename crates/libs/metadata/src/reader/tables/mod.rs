use super::*;

mod assembly_ref;
mod attribute;
mod class_layout;
mod constant;
mod field;
mod generic_param;
mod impl_map;
mod interface_impl;
mod member_ref;
mod method_def;
mod method_param;
mod module;
mod module_ref;
mod nested_class;
mod type_def;
mod type_ref;
mod type_spec;

macro_rules! tables {
    ($(($name:ident, $table:literal))+) => {
        $(
        #[derive(Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
        pub struct $name<'a>(pub(crate) Row<'a>);
        impl<'a> AsRow<'a> for $name<'a> {
            const TABLE: usize = $table;
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
    (Attribute, 1)
    (ClassLayout, 16)
    (Constant, 0)
    (Field, 2)
    (GenericParam, 3)
    (ImplMap, 11)
    (InterfaceImpl, 4)
    (MemberRef, 5)
    (MethodDef, 6)
    (ModuleRef, 12)
    (NestedClass, 13)
    (MethodParam, 7)
    (TypeDef, 8)
    (TypeRef, 9)
    (TypeSpec, 10)
    (Module, 0)
    (AssemblyRef, 0x23)
}

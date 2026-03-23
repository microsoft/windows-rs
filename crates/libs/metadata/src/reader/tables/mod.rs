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
    (Assembly, 0)
    (AssemblyRef, 1)
    (Attribute, 2)
    (ClassLayout, 3)
    (Constant, 4)
    (Field, 5)
    (GenericParam, 6)
    (ImplMap, 7)
    (InterfaceImpl, 8)
    (MemberRef, 9)
    (MethodDef, 10)
    (MethodParam, 11)
    (Module, 12)
    (ModuleRef, 13)
    (NestedClass, 14)
    (TypeDef, 15)
    (TypeRef, 16)
    (TypeSpec, 17)
}

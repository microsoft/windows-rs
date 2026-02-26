use super::*;

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
mod module_ref;
mod nested_class;
mod type_def;
mod type_ref;
mod type_spec;

pub use attribute::*;
pub use constant::*;
pub use field::*;
pub use impl_map::*;
pub use interface_impl::*;
pub use method_def::*;
pub use type_def::*;
pub use type_ref::*;

pub type Attribute = windows_metadata::reader::Attribute<'static>;
#[allow(dead_code)]
pub type ClassLayout = windows_metadata::reader::ClassLayout<'static>;
pub type Constant = windows_metadata::reader::Constant<'static>;
pub type Field = windows_metadata::reader::Field<'static>;
pub type GenericParam = windows_metadata::reader::GenericParam<'static>;
pub type ImplMap = windows_metadata::reader::ImplMap<'static>;
pub type InterfaceImpl = windows_metadata::reader::InterfaceImpl<'static>;
#[allow(dead_code)]
pub type MemberRef = windows_metadata::reader::MemberRef<'static>;
pub type MethodDef = windows_metadata::reader::MethodDef<'static>;
pub type MethodParam = windows_metadata::reader::MethodParam<'static>;
pub type ModuleRef = windows_metadata::reader::ModuleRef<'static>;
#[allow(dead_code)]
pub type NestedClass = windows_metadata::reader::NestedClass<'static>;
pub type TypeDef = windows_metadata::reader::TypeDef<'static>;
pub type TypeRef = windows_metadata::reader::TypeRef<'static>;
#[allow(dead_code)]
pub type TypeSpec = windows_metadata::reader::TypeSpec<'static>;


use super::*;

mod reader;
pub use reader::*;

// Type aliases using 'static lifetime.
// Safety: the TypeIndex is leaked in Reader::new (Box::leak), so TypeDef<'static>, etc.
// truly live forever.
pub type TypeDef = windows_metadata::reader::TypeDef<'static>;
pub type Field = windows_metadata::reader::Field<'static>;
pub type MethodDef = windows_metadata::reader::MethodDef<'static>;
pub type MethodParam = windows_metadata::reader::MethodParam<'static>;
pub type GenericParam = windows_metadata::reader::GenericParam<'static>;
pub type InterfaceImpl = windows_metadata::reader::InterfaceImpl<'static>;
pub type Constant = windows_metadata::reader::Constant<'static>;

// Coded index type aliases
pub type TypeDefOrRef = windows_metadata::reader::TypeDefOrRef<'static>;
pub type MemberRefParent = windows_metadata::reader::MemberRefParent<'static>;
pub type File = windows_metadata::reader::File;

pub use windows_metadata::reader::AsRow;
pub use windows_metadata::HasAttributes;
pub use windows_metadata::{
    FieldAttributes, MethodAttributes, PInvokeAttributes, ParamAttributes, TypeAttributes,
};

pub use windows_metadata::MethodCallAttributes;

// Extension trait for TypeDefOrRef: adds bindgen-specific helpers.
pub trait TypeDefOrRefExt {
    fn type_name(&self) -> TypeName;
}

impl TypeDefOrRefExt for TypeDefOrRef {
    fn type_name(&self) -> TypeName {
        TypeName(self.namespace(), windows_metadata::trim_tick(self.name()))
    }
}

// Extension trait for MemberRefParent: adds type_name helper.
pub trait MemberRefParentExt {
    fn type_name(&self) -> TypeName;
}

impl MemberRefParentExt for MemberRefParent {
    fn type_name(&self) -> TypeName {
        TypeName(self.namespace(), windows_metadata::trim_tick(self.name()))
    }
}

// Extension trait for guid_attribute(), which uses our Value type rather than metadata's.
pub trait GuidAttributeExt {
    fn guid_attribute(&self) -> Option<GUID>;
}

impl<T: windows_metadata::HasAttributes<'static>> GuidAttributeExt for T {
    fn guid_attribute(&self) -> Option<GUID> {
        self.find_attribute("GuidAttribute").map(|attribute| {
            fn unwrap_u32(value: &Value) -> u32 {
                match value {
                    Value::U32(value) => *value,
                    _ => panic!(),
                }
            }
            fn unwrap_u16(value: &Value) -> u16 {
                match value {
                    Value::U16(value) => *value,
                    rest => panic!("{rest:?}"),
                }
            }
            fn unwrap_u8(value: &Value) -> u8 {
                match value {
                    Value::U8(value) => *value,
                    rest => panic!("{rest:?}"),
                }
            }

            let args = attribute.value();

            GUID(
                unwrap_u32(&args[0].1),
                unwrap_u16(&args[1].1),
                unwrap_u16(&args[2].1),
                unwrap_u8(&args[3].1),
                unwrap_u8(&args[4].1),
                unwrap_u8(&args[5].1),
                unwrap_u8(&args[6].1),
                unwrap_u8(&args[7].1),
                unwrap_u8(&args[8].1),
                unwrap_u8(&args[9].1),
                unwrap_u8(&args[10].1),
            )
        })
    }
}

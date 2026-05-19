use super::*;

mod reader;
pub use reader::*;

// Row type re-exports. These are now lifetime-free owned handles backed by an
// Arc<TypeIndexData> inside each Row, so no 'static aliases are needed.
pub use windows_metadata::reader::TypeDef;
pub use windows_metadata::reader::Field;
pub use windows_metadata::reader::MethodDef;
pub use windows_metadata::reader::MethodParam;
pub use windows_metadata::reader::GenericParam;
pub use windows_metadata::reader::InterfaceImpl;
pub use windows_metadata::reader::Constant;

// Coded index type re-exports (also lifetime-free)
pub use windows_metadata::reader::TypeDefOrRef;
pub use windows_metadata::reader::MemberRefParent;
pub use windows_metadata::reader::File;

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
        TypeName::new(self.namespace(), windows_metadata::trim_tick(self.name()))
    }
}

// Extension trait for MemberRefParent: adds type_name helper.
pub trait MemberRefParentExt {
    fn type_name(&self) -> TypeName;
}

impl MemberRefParentExt for MemberRefParent {
    fn type_name(&self) -> TypeName {
        TypeName::new(self.namespace(), windows_metadata::trim_tick(self.name()))
    }
}

// Extension trait for guid_attribute(), which uses our Value type rather than metadata's.
pub trait GuidAttributeExt {
    fn guid_attribute(&self) -> Option<GUID>;
}

impl<T: windows_metadata::HasAttributes> GuidAttributeExt for T {
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

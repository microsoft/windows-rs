use super::*;

mod bindings;
mod reader;
pub use bindings::*;
pub use reader::*;

// Type aliases using 'static lifetime.
// Safety: the TypeIndex backing these types is heap-allocated (via Box::into_raw in Reader::new),
// never moved, and lives as long as the Reader, which is guaranteed to outlive all uses.
pub type TypeDef = windows_metadata::reader::TypeDef<'static>;
pub type Field = windows_metadata::reader::Field<'static>;
pub type MethodDef = windows_metadata::reader::MethodDef<'static>;
pub type MethodParam = windows_metadata::reader::MethodParam<'static>;
pub type Attribute = windows_metadata::reader::Attribute<'static>;
pub type GenericParam = windows_metadata::reader::GenericParam<'static>;
pub type InterfaceImpl = windows_metadata::reader::InterfaceImpl<'static>;
pub type ClassLayout = windows_metadata::reader::ClassLayout<'static>;
pub type Constant = windows_metadata::reader::Constant<'static>;
pub type ImplMap = windows_metadata::reader::ImplMap<'static>;
pub type MemberRef = windows_metadata::reader::MemberRef<'static>;
pub type ModuleRef = windows_metadata::reader::ModuleRef<'static>;
pub type NestedClass = windows_metadata::reader::NestedClass<'static>;
pub type TypeRef = windows_metadata::reader::TypeRef<'static>;
pub type TypeSpec = windows_metadata::reader::TypeSpec<'static>;

// Coded index type aliases
pub type TypeDefOrRef = windows_metadata::reader::TypeDefOrRef<'static>;
pub type TypeOrMethodDef = windows_metadata::reader::TypeOrMethodDef<'static>;
pub type HasAttribute = windows_metadata::reader::HasAttribute<'static>;
pub type AttributeType = windows_metadata::reader::AttributeType<'static>;
pub type MemberRefParent = windows_metadata::reader::MemberRefParent<'static>;
pub type HasConstant = windows_metadata::reader::HasConstant<'static>;
pub type MemberForwarded = windows_metadata::reader::MemberForwarded<'static>;
pub type RowIterator<R> = windows_metadata::reader::RowIterator<'static, R>;
pub type Blob = windows_metadata::reader::Blob<'static>;
pub type File = windows_metadata::reader::File;

pub use windows_metadata::reader::AsRow;
pub use windows_metadata::HasAttributes;
pub use windows_metadata::{
    FieldAttributes, MethodAttributes, PInvokeAttributes, ParamAttributes, TypeAttributes,
};

pub use windows_metadata::MethodCallAttributes;
// Extension trait for Blob: adds bindgen-specific read helpers.
pub trait BlobExt {
    /// Read a compressed integer (same as read_compressed but named for compat).
    fn read_usize(&mut self) -> usize;
    /// Read a UTF-8 string as &'static str (safe because backing TypeIndex is 'static).
    fn read_str(&mut self) -> &'static str;
    /// Read an integer value using a bindgen Type to determine the width.
    fn read_integer(&mut self, ty: Type) -> Value;
    /// Access the current Reader from the thread-local.
    fn reader(&self) -> &'static Reader;
}

impl BlobExt for Blob {
    fn read_usize(&mut self) -> usize {
        self.read_compressed()
    }

    fn read_str(&mut self) -> &'static str {
        let len = self.read_compressed();
        // Safety: Blob<'static> Deref gives &'static [u8] backed by the heap-pinned TypeIndex.
        let ptr = self.as_ptr();
        let s: &'static str =
            unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len)) };
        // Advance past the string bytes by consuming them one by one.
        for _ in 0..len {
            self.read_u8();
        }
        s
    }

    fn read_integer(&mut self, ty: Type) -> Value {
        match ty {
            Type::I8 => Value::I8(self.read_i8()),
            Type::U8 => Value::U8(self.read_u8()),
            Type::I16 => Value::I16(self.read_i16()),
            Type::U16 => Value::U16(self.read_u16()),
            Type::I32 => Value::I32(self.read_i32()),
            Type::U32 => Value::U32(self.read_u32()),
            Type::I64 => Value::I64(self.read_i64()),
            Type::U64 => Value::U64(self.read_u64()),
            _ => panic!("unexpected integer type: {ty:?}"),
        }
    }

    fn reader(&self) -> &'static Reader {
        current_reader()
    }
}

// Extension trait for TypeDefOrRef: adds bindgen-specific helpers.
pub trait TypeDefOrRefExt {
    fn type_name(&self) -> TypeName;
}

impl TypeDefOrRefExt for TypeDefOrRef {
    fn type_name(&self) -> TypeName {
        // Safety: TypeDefOrRef<'static> references data in a 'static TypeIndex.
        // The implicit lifetime elision in namespace()/name() ties to &self, but
        // the actual string data lives as long as the TypeIndex (which is 'static).
        let ns: &'static str = unsafe { std::mem::transmute(self.namespace()) };
        let name: &'static str = unsafe { std::mem::transmute(self.name()) };
        TypeName(ns, windows_metadata::trim_tick(name))
    }
}

// Extension trait for MemberRefParent: adds type_name helper.
pub trait MemberRefParentExt {
    fn type_name(&self) -> TypeName;
}

impl MemberRefParentExt for MemberRefParent {
    fn type_name(&self) -> TypeName {
        // Safety: same reasoning as TypeDefOrRefExt::type_name.
        let ns: &'static str = unsafe { std::mem::transmute(self.namespace()) };
        let name: &'static str = unsafe { std::mem::transmute(self.name()) };
        TypeName(ns, windows_metadata::trim_tick(name))
    }
}

/// Extension trait providing `reader()` access on all table row types.
///
/// # Panics
/// Panics if called before `Reader::new()` has completed or after the `Reader` is dropped,
/// since this accesses the thread-local `CURRENT_READER` pointer.
pub trait HasReader {
    fn reader(&self) -> &'static Reader {
        current_reader()
    }
}

impl HasReader for TypeDef {}
impl HasReader for Field {}
impl HasReader for MethodDef {}
impl HasReader for MethodParam {}
impl HasReader for Attribute {}
impl HasReader for GenericParam {}
impl HasReader for InterfaceImpl {}
impl HasReader for ClassLayout {}
impl HasReader for Constant {}
impl HasReader for ImplMap {}
impl HasReader for MemberRef {}
impl HasReader for ModuleRef {}
impl HasReader for NestedClass {}
impl HasReader for TypeRef {}
impl HasReader for TypeSpec {}
impl HasReader for TypeDefOrRef {}

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

            let args = attribute.args();

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

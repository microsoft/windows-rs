pub use super::*;

mod class_layout;
mod constant;
mod custom_attribute;
mod field;
mod generic_param;
mod impl_map;
mod interface_impl;
mod member_ref;
mod method_def;
mod module;
mod module_ref;
mod nested_class;
mod param;
mod type_def;
mod type_ref;
mod type_spec;

pub use class_layout::*;
pub use constant::*;
pub use custom_attribute::*;
pub use field::*;
pub use generic_param::*;
pub use impl_map::*;
pub use interface_impl::*;
pub use member_ref::*;
pub use method_def::*;
pub use module::*;
pub use module_ref::*;
pub use nested_class::*;
pub use param::*;
pub use type_def::*;
pub use type_ref::*;
pub use type_spec::*;

#[derive(Default)]
pub struct Tables {
    pub module: Vec<Module>,
    pub type_ref: Vec<TypeRef>,
    pub type_def: Vec<TypeDef>,
    pub field: Vec<Field>,
    pub method_def: Vec<MethodDef>,
    pub param: Vec<Param>,
    pub interface_impl: Vec<InterfaceImpl>,
    pub member_ref: Vec<MemberRef>,
    pub constant: Vec<Constant>,
    pub custom_attribute: Vec<CustomAttribute>,
    pub class_layout: Vec<ClassLayout>,
    pub module_ref: Vec<ModuleRef>,
    pub type_spec: Vec<TypeSpec>,
    pub impl_map: Vec<ImplMap>,
    pub nested_class:Vec<NestedClass>,
    pub generic_param:Vec<GenericParam>,
}

impl Tables {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_stream(&self, strings: &mut Strings) -> Vec<u8> {
        let mut buffer = Vec::new();
        let header = Header::new();
        buffer.write(&header);

        // Row sizes (ordered by table ID)
        buffer.write(&(self.module.len() as u32));
        buffer.write(&(self.type_ref.len() as u32));
        buffer.write(&(self.type_def.len() as u32));
        buffer.write(&(self.field.len() as u32));
        buffer.write(&(self.method_def.len() as u32));
        buffer.write(&(self.param.len() as u32));
        buffer.write(&(self.interface_impl.len() as u32));
        buffer.write(&(self.member_ref.len() as u32));
        buffer.write(&(self.constant.len() as u32));
        buffer.write(&(self.custom_attribute.len() as u32));
        buffer.write(&(self.class_layout.len() as u32));
        buffer.write(&(self.module_ref.len() as u32));
        buffer.write(&(self.type_spec.len() as u32));
        buffer.write(&(self.impl_map.len() as u32));
        buffer.write(&(self.nested_class.len() as u32));
        buffer.write(&(self.generic_param.len() as u32));

        for module in &self.module {
            buffer.write(&0u16); // Generation (reserved)
            buffer.write(&strings.insert(&module.name));
            buffer.write(&1u32); // Mvid (zero guid)
            buffer.write(&0u32); // EncId (reserved)
            buffer.write(&0u32); // EncBaseId (reserved)
        }

        // ILDASM needs <Module> to be present and the first TypeDef row.
        buffer.write(&0u32);
        buffer.write(&strings.insert("<Module>"));
        buffer.write(&0u32);
        buffer.write(&0u16);
        buffer.write(&1u16);
        buffer.write(&1u16);

        for type_def in &self.type_def {
            buffer.write(&0x40A1u32); // Flags
            buffer.write(&strings.insert(&type_def.name));
            buffer.write(&strings.insert(&type_def.namespace));
            buffer.write(&0u16); // Extends // TODO: use composite_index_size to work out size of TypeDefOrRef
            buffer.write(&1u16); // FieldList // TODO: use Field->TableData::index_size to work this out
            buffer.write(&1u16); // MethodList // TODO: use MethodDef->TableData::index_size to work this out
        }

        buffer.resize(round(buffer.len(), 4), 0);
        buffer
    }
}

#[repr(C)]
#[derive(Default)]
struct Header {
    reserved1: u32,
    major_version: u8,
    minor_version: u8,
    heap_sizes: u8,
    reserved2: u8,
    valid: u64,
    sorted: u64,
}

impl Header {
    fn new() -> Self {
        Self {
            major_version: 2,
            reserved2: 1,
            heap_sizes: 0b111, // 4 byte indexes
            valid: 1 << 0 |    // Module 
                   1 << 0x01 | // TypeRef
                   1 << 0x02 | // TypeDef
                   1 << 0x04 | // Field
                   1 << 0x06 | // MethodDef
                   1 << 0x08 | // Param
                   1 << 0x09 | // InterfaceImpl
                   1 << 0x0A | // MemberRef
                   1 << 0x0B | // Constant
                   1 << 0x0C | // CustomAttribute
                   1 << 0x0F | // ClassLayout
                   1 << 0x1A | // ModuleRef
                   1 << 0x1B | // TypeSpec,
                   1 << 0x1C | // ImplMap
                   1 << 0x29 | // NestedClass
                   1 << 0x2A, // GenericParam
            // TODO: mark sorted tables?
            ..Default::default()
        }
    }
}

fn index_size(len: usize) -> u32 {
    if len < (1 << 16) {
        2
    } else {
        4
    }
}

enum Index {
    U16(u16),
    U32(u32),
}

impl Index {
    fn index(index: usize, len: usize) -> Self {
        if len < (1 << 16) {
            Self::U16(index as _)
        } else {
            Self::U32(index as _)
        }
    }

    fn write(&self, buffer: &mut Vec<u8>) {
        match self {
            Self::U16(index) => buffer.write(&index),
            Self::U32(index) => buffer.write(&index),
        }
    }
}

// TODO: use CodedIndex enum?
fn coded_index_size(tables: &[usize]) -> u32 {
    fn small(row_count: usize, bits: u8) -> bool {
        (row_count as u64) < (1u64 << (16 - bits))
    }

    fn bits_needed(value: usize) -> u8 {
        let mut value = value - 1;
        let mut bits: u8 = 1;
        loop {
            value >>= 1;
            if value == 0 {
                break;
            }
            bits += 1;
        }
        bits
    }

    let bits_needed = bits_needed(tables.len());

    if tables.iter().all(|len| small(*len, bits_needed)) {
        2
    } else {
        4
    }
}

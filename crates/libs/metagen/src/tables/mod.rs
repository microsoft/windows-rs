pub use super::*;

mod custom_attribute;
mod class_layout;
mod constant;
mod field;
mod generic_param;
mod impl_map;
mod interface_impl;
mod member_ref;
mod method_def;
mod module_ref;
mod nested_class;
mod param;
mod type_def;
mod type_ref;
mod type_spec;
mod module;

pub use module::*;
pub use custom_attribute::*;
pub use class_layout::*;
pub use constant::*;
pub use field::*;
pub use generic_param::*;
pub use impl_map::*;
pub use interface_impl::*;
pub use member_ref::*;
pub use method_def::*;
pub use module_ref::*;
pub use nested_class::*;
pub use param::*;
pub use type_def::*;
pub use type_ref::*;
pub use type_spec::*;

#[derive(Default)]
pub(crate) struct Tables {
    type_ref: Vec<TypeRef>,
    type_def: Vec<TypeDef>,
    type_spec: Vec<TypeSpec>,
}

impl Tables {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_type_def(&mut self, type_def: TypeDef) {
        self.type_def.push(type_def);
    }

    pub fn into_stream(&self, strings: &mut Strings) -> Vec<u8> {
        let mut buffer = Vec::new();
        let header = Header::new();
        buffer.write(&header);

        // Row sizes (ordered by table ID)
        buffer.write(&1u32); // Module
        buffer.write(&(self.type_ref.len() as u32));
        buffer.write(&(self.type_def.len() as u32));
        buffer.write(&(self.type_spec.len() as u32));

        Module::write(&mut buffer);
        self.type_ref.iter().for_each(|row|row.write(&mut buffer, strings));
        for type_def in &self.type_def {
            buffer.write(&0u32); // Flags
            buffer.write(&strings.insert(&type_def.name));
            buffer.write(&strings.insert(&type_def.namespace));
            buffer.write(&0u16); // Extends // TODO: use composite_index_size to work out size of TypeDefOrRef
            buffer.write(&1u16); // FieldList // TODO: use Field->TableData::index_size to work this out
            buffer.write(&1u16); // MethodList // TODO: use MethodDef->TableData::index_size to work this out
        }
        self.type_spec.iter().for_each(|row|row.write(&mut buffer, strings));

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
            valid: Module::ID | TypeRef::ID | TypeDef::ID | TypeSpec::ID,
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
    U32(u32)
}

impl Index{
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

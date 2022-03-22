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

pub(crate) struct Tables {
    // TODO: probably need a map or tree
    TypeDef: Vec<TypeDef>,
}

impl Tables {
    pub fn new() -> Self {
        Self {
            TypeDef: vec![TypeDef::new("IStringable", "Windows.Foundation")]
        }
    }

    pub fn into_stream(&self, _strings: &mut Strings) -> Vec<u8> {
        let mut buffer = Vec::new();
        let header = Header::new();
        buffer.write(&header);

        // Row sizes (ordered by table ID)
        buffer.write(&1u32); // Module
        buffer.write(&(self.TypeDef.len() as u32));

        Module::write(&mut buffer);
        // for row in &self.TypeDef {
        //     row.write(&mut buffer, strings);
        // }

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
            valid: Module::ID,
            // TODO: mark sorted tables?
            ..Default::default()
        }
    }
}


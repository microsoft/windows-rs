use super::*;

#[derive(Default)]
pub struct TypeDef {
    pub name: String,
    pub namespace: String,
}

impl TypeDef {
    pub const ID: u64 = 1 << 0x02;

    pub fn new(name: &str, namespace: &str) -> Self {
        Self {
            name: name.to_string(),
            namespace: namespace.to_string(),
        }
    }

    pub fn write(&self, buffer: &mut Vec<u8>, strings: &mut Strings) {
        buffer.write(&0u32); // Flags
        buffer.write(&strings.insert(&self.name));
        buffer.write(&strings.insert(&self.namespace));
        buffer.write(&0u32); // Extends // TODO: use composite_index_size to work out size of TypeDefOrRef
        buffer.write(&1u32); // FieldList // TODO: use Field->TableData::index_size to work this out
        buffer.write(&1u32); // MethodList // TODO: use MethodDef->TableData::index_size to work this out
    }
}

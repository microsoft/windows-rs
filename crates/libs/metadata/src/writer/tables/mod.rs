pub use super::*;

mod assembly_ref;
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
mod property;
mod type_def;
mod type_ref;
mod type_spec;

pub use assembly_ref::*;
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
pub use property::*;
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
    pub property: Vec<Property>,
    pub module_ref: Vec<ModuleRef>,
    pub type_spec: Vec<TypeSpec>,
    pub impl_map: Vec<ImplMap>,
    pub assembly_ref: Vec<AssemblyRef>,
    pub nested_class: Vec<NestedClass>,
    pub generic_param: Vec<GenericParam>,
}

impl Tables {
    pub fn new(module: &str) -> Self {
        let mut new = Self::default();
        new.module.push(Module::new(module));
        new.type_def.push(TypeDef::module());
        new
    }

    pub(crate) fn into_stream(mut self, strings: &mut Strings, blobs: &mut Blobs) -> Vec<u8> {
        self.normalize();

        let resolution_scope = coded_index_size(&[self.module.len(), self.module_ref.len(), self.assembly_ref.len(), self.type_ref.len()]);
        let type_def_or_ref = coded_index_size(&[self.type_def.len(), self.type_ref.len(), self.type_spec.len()]);
        let has_constant = coded_index_size(&[self.field.len(), self.param.len(), self.property.len()]);

        let mut buffer = Vec::new();
        let header = Header::new();
        buffer.write(&header);

        write_table_sizes(&mut buffer, &[self.module.len(), self.type_ref.len(), self.type_def.len(), self.field.len(), self.method_def.len(), self.param.len(), self.interface_impl.len(), self.member_ref.len(), self.constant.len(), self.custom_attribute.len(), self.class_layout.len(), self.property.len(), self.module_ref.len(), self.type_spec.len(), self.impl_map.len(), self.assembly_ref.len(), self.nested_class.len(), self.generic_param.len()]);

        for module in &self.module {
            buffer.write(&0u16); // Generation (reserved)
            buffer.write(&strings.insert(&module.name));
            buffer.write(&1u32); // Mvid (zero guid)
            buffer.write(&0u32); // EncId (reserved)
            buffer.write(&0u32); // EncBaseId (reserved)
        }

        for type_ref in &self.type_ref {
            write_coded_index(&mut buffer, type_ref.assembly_index.encode(), resolution_scope);
            buffer.write(&strings.insert(&type_ref.type_name.name));
            buffer.write(&strings.insert(&type_ref.type_name.namespace));
        }

        for type_def in &self.type_def {
            buffer.write(&(type_def.flags.0 as u32));
            buffer.write(&strings.insert(&type_def.type_name.name));
            buffer.write(&strings.insert(&type_def.type_name.namespace));
            write_coded_index(&mut buffer, type_def.extends_index.encode(), type_def_or_ref);
            write_index(&mut buffer, type_def.field_index, self.field.len());
            write_index(&mut buffer, type_def.method_index, self.method_def.len());
        }

        for field in &self.field {
            buffer.write(&(field.flags.0 as u16));
            buffer.write(&strings.insert(&field.name));
            buffer.write(&blobs.insert(&field.ty.to_field_sig()));
        }

        for method_def in &self.method_def {
            buffer.write(&0u32); // RVA
            buffer.write(&0u16); // ImplFlags
            buffer.write(&0u16); // Flags
            buffer.write(&strings.insert(&method_def.name));
            buffer.write(&blobs.insert(&method_def.signature));
            buffer.write(&1u16); // ParamList
        }

        for param in &self.param {
            buffer.write(&(param.flags.0 as u16));
            buffer.write(&param.sequence);
            buffer.write(&strings.insert(&param.name));
        }

        for constant in &self.constant {
            buffer.write(&(constant.value.ty().to_code().expect("Unexpected constant type") as u16));
            write_coded_index(&mut buffer, constant.parent_index.encode(), has_constant);
            buffer.write(&blobs.insert(&constant.value.to_blob()));
        }

        for assembly_ref in &self.assembly_ref {
            buffer.write(&assembly_ref.major_version);
            buffer.write(&assembly_ref.minor_version);
            buffer.write(&assembly_ref.build_number);
            buffer.write(&assembly_ref.revision_number);
            buffer.write(&0u32); // Flags
            buffer.write(&0u32); // PublicKeyOrToken
            buffer.write(&strings.insert(&assembly_ref.name));
            buffer.write(&0u32); // Culture
            buffer.write(&0u32); // HashValue
        }

        buffer.resize(round(buffer.len(), 4), 0);
        buffer
    }

    // Once all of the type information has been added, normalization is the process of packing
    // the various relational records into their respective tables and leaving only indexes behind.
    fn normalize(&mut self) {
        for type_def in &mut self.type_def {
            type_def.field_index = self.field.len();
            type_def.method_index = self.method_def.len();
            self.field.append(&mut type_def.field_list);
            self.method_def.append(&mut type_def.method_list);

            if let Some(extends) = type_def.extends.take() {
                let index = if let Some(index) = self.type_ref.iter().position(|row| row.type_name == extends.type_name) {
                    index
                } else {
                    self.type_ref.push(extends);
                    self.type_ref.len() - 1
                };
                type_def.extends_index = TypeDefOrRef::TypeRef(index);
            }
        }

        for method_def in &mut self.method_def {
            method_def.param_index = self.param.len();
            self.param.append(&mut method_def.param_list);
        }

        for type_ref in &mut self.type_ref {
            let index = if let Some(index) = self.assembly_ref.iter().position(|row| row.name == type_ref.assembly_ref.name) {
                index
            } else {
                self.assembly_ref.push(type_ref.assembly_ref.clone());
                self.assembly_ref.len() - 1
            };
            type_ref.assembly_index = ResolutionScope::AssemblyRef(index);
        }

        for (field_index, field) in self.field.iter_mut().enumerate() {
            if let Some(value) = field.constant.take() {
                self.constant.push(Constant { value, parent_index: HasConstant::Field(field_index) })
            }
        }
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
                   1 << 0x17 | // Property
                   1 << 0x1A | // ModuleRef
                   1 << 0x1B | // TypeSpec
                   1 << 0x1C | // ImplMap
                   1 << 0x23 | // AssemblyRef
                   1 << 0x29 | // NestedClass
                   1 << 0x2A, // GenericParam
            // TODO: mark sorted tables?
            ..Default::default()
        }
    }
}

fn write_table_sizes(buffer: &mut Vec<u8>, tables: &[usize]) {
    for table in tables {
        buffer.write(&(*table as u32));
    }
}

fn write_index(buffer: &mut Vec<u8>, index: usize, len: usize) {
    if len < (1 << 16) {
        buffer.write(&(index as u16 + 1))
    } else {
        buffer.write(&(index as u32 + 1))
    }
}

fn write_coded_index(buffer: &mut Vec<u8>, value: usize, size: usize) {
    if size == 2 {
        buffer.write(&(value as u16))
    } else {
        buffer.write(&(value as u32))
    }
}

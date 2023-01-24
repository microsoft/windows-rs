mod blobs;
mod codes;
mod definitions;
mod file;
mod references;
mod strings;
mod tables;

use super::*;
use blobs::*;
use codes::*;
use definitions::*;
use references::*;
use strings::*;
use tables::Tables;

use std::collections::BTreeMap;

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

pub fn write(name: &str, winrt: bool, definitions: &[Item], assemblies: &[&str]) -> Vec<u8> {
    // Index assemblies used to resolve references to existing winmd files.
    let assemblies = reader::File::with_default(assemblies).expect("Assemblies could not be loaded");
    let assemblies = &reader::Reader::new(&assemblies);

    // Build sorted list of definitions.
    let definitions = &{
        let mut index = Definitions::default();
        definitions.iter().for_each(|item| index.insert(item));
        index.stage()
    };

    // Build sorted list of references.
    let references = &{
        let mut index = References::default();
        for item in definitions.items() {
            match item {
                Item::Struct(ty) => ty.fields.iter().for_each(|field| type_reference(&field.ty, definitions, assemblies, &mut index)),
                Item::Interface(_ty) => {}
                _ => {}
            }
        }
        index.stage()
    };

    // Now that we have stable type indexes, build blobs and index strings.
    let (blobs, strings) = {
        let mut blobs = Blobs::default();
        let mut strings = Strings::default();
        strings.insert(name);
        strings.insert("<Module>");
        strings.insert("mscorlib");
        strings.insert("System");
        strings.insert("ValueType");
        strings.insert("Enum");

        for item in definitions.items() {
            match item {
                Item::Struct(ty) => {
                    strings.insert(&ty.namespace);
                    strings.insert(&ty.name);
                    ty.fields.iter().for_each(|field| {
                        strings.insert(&field.name);
                        blobs.insert(field_blob(field, definitions, references));
                    });
                }
                Item::Enum(ty) => {
                    strings.insert(&ty.namespace);
                    strings.insert(&ty.name);
                }
                Item::Interface(ty) => {
                    strings.insert(&ty.namespace);
                    strings.insert(&ty.name);
                }
            }
        }

        (blobs.stage(), strings.stage())
    };

    // Now that everything is indexed in various heaps, write out the table records.
    let tables = {
        let mut tables = Tables::default();
        tables.Module.push(tables::Module { Name: strings.index(name), Mvid: 1, ..Default::default() });
        tables.TypeDef.push(tables::TypeDef { TypeName: strings.index("<Module>"), ..Default::default() });
        let mscorlib = tables.AssemblyRef.push2(tables::AssemblyRef { MajorVersion: 4, Name: strings.index("mscorlib"), ..Default::default() });
        let value_type = tables.TypeRef.push2(tables::TypeRef { TypeName: strings.index("ValueType"), TypeNamespace: strings.index("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib).encode() });
        let _enum_type = tables.TypeRef.push2(tables::TypeRef { TypeName: strings.index("Enum"), TypeNamespace: strings.index("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib).encode() });

        for (_index, item) in definitions.iter() {
            match item {
                Item::Struct(ty) => {
                    let mut flags = TypeAttributes(0);
                    flags.set_public();
                    if winrt {
                        flags.set_winrt();
                    }
                    tables.TypeDef.push(tables::TypeDef {
                        Flags: flags.0,
                        TypeName: strings.index(&ty.name),
                        TypeNamespace: strings.index(&ty.namespace),
                        Extends: TypeDefOrRef::TypeRef(value_type).encode(),
                        FieldList: tables.Field.len() as _,
                        MethodList: 0,
                    });
                    for field in &ty.fields {
                        let mut flags = FieldAttributes(0);
                        flags.set_public();
                        tables.Field.push(tables::Field { Flags: flags.0, Name: strings.index(&field.name), Signature: blobs.index(&field_blob(field, definitions, references)) })
                    }
                }
                Item::Enum(_ty) => {}
                Item::Interface(_ty) => {}
            }
        }

        tables
    };

    // With all of the streams prepared, write out ECMA-335 file format.
    file::write(tables, strings, blobs)
}

fn type_reference<'a>(ty: &'a Type, definitions: &StagedDefinitions, assemblies: &reader::Reader, references: &mut References<'a>) {
    match ty {
        Type::Named((namespace, name)) => {
            if definitions.get(namespace, name).is_none() {
                references.insert(namespace, name, assemblies);
            }
        }
        _ => {}
    }
}

fn item_type_name(item: &Item) -> (&str, &str) {
    match item {
        Item::Struct(ty) => (ty.namespace.as_str(), ty.name.as_str()),
        Item::Enum(ty) => (ty.namespace.as_str(), ty.name.as_str()),
        Item::Interface(ty) => (ty.namespace.as_str(), ty.name.as_str()),
    }
}

fn item_value_type(item: &Item) -> bool {
    match item {
        Item::Struct(_) | Item::Enum(_) => true,
        Item::Interface(_) => false,
    }
}

fn field_blob(field: &Field, definitions: &StagedDefinitions, references: &StagedReferences) -> Vec<u8> {
    let mut blob = vec![0x6];
    type_blob(&field.ty, &mut blob, definitions, references);
    blob
}

fn type_blob(ty: &Type, blob: &mut Vec<u8>, definitions: &StagedDefinitions, references: &StagedReferences) {
    match ty {
        Type::Void => blob.push(0x01),
        Type::Bool => blob.push(0x02),
        Type::Char => blob.push(0x03),
        Type::I8 => blob.push(0x04),
        Type::U8 => blob.push(0x05),
        Type::I16 => blob.push(0x06),
        Type::U16 => blob.push(0x07),
        Type::I32 => blob.push(0x08),
        Type::U32 => blob.push(0x09),
        Type::I64 => blob.push(0x0a),
        Type::U64 => blob.push(0x0b),
        Type::F32 => blob.push(0x0c),
        Type::F64 => blob.push(0x0d),
        Type::ISize => blob.push(0x18),
        Type::USize => blob.push(0x19),
        Type::String => blob.push(0x0e),
        //Type::IInspectable => blob.push(0x1c),
        Type::Named((namespace, name)) => {
            let (value_type, code) = type_name_encode(namespace, name, definitions, references);
            value_type_blob(value_type, blob);
            u32_blob(code, blob);
        }
    }
}

fn value_type_blob(value_type: bool, blob: &mut Vec<u8>) {
    if value_type {
        blob.push(0x11);
    } else {
        blob.push(0x12);
    }
}

fn u32_blob(value: u32, blob: &mut Vec<u8>) {
    if value < 0x80 {
        blob.push(value as _);
    } else if value < 0x4000 {
        blob.push((0x40 | (value & 0xFF00)) as _);
        blob.push((value | 0xFF) as _);
    } else {
        blob.push((0x60 | (value & 0xFF000000)) as _);
        blob.push((value | 0xFF0000) as _);
        blob.push((value | 0xFF00) as _);
        blob.push((value | 0xFF) as _);
    }
}

/// Returns the TypeDefOrRef-encoded value for the type name as well as whether the type is a value type, needed
/// in some cases like when a TypeDefOrRef appears in a signature.
fn type_name_encode(namespace: &str, name: &str, definitions: &StagedDefinitions, references: &StagedReferences) -> (bool, u32) {
    if let Some(definition) = definitions.get(namespace, name) {
        return (definition.value_type, TypeDefOrRef::TypeDef(definition.index + 1).encode());
    }
    let reference = references.get(namespace, name).expect("Type not found");
    (reference.value_type, TypeDefOrRef::TypeRef(reference.index + 1).encode())
}

pub trait Write {
    unsafe fn write_header<T: Sized>(&mut self, value: &T);
    fn write_u8(&mut self, value: u8);
    fn write_u16(&mut self, value: u16);
    fn write_u32(&mut self, value: u32);
    fn write_u64(&mut self, value: u64);
    fn write_code(&mut self, value: u32, size: usize);
    fn write_index(&mut self, index: u32, len: usize);
}

impl Write for Vec<u8> {
    unsafe fn write_header<T: Sized>(&mut self, value: &T) {
        self.extend_from_slice(std::slice::from_raw_parts(value as *const _ as _, std::mem::size_of::<T>()));
    }

    fn write_u8(&mut self, value: u8) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u16(&mut self, value: u16) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u32(&mut self, value: u32) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u64(&mut self, value: u64) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_code(&mut self, value: u32, size: usize) {
        if size == 2 {
            self.write_u16(value as _);
        } else {
            self.write_u32(value);
        }
    }

    fn write_index(&mut self, index: u32, len: usize) {
        if len < (1 << 16) {
            self.write_u16(index as u16 + 1);
        } else {
            self.write_u32(index + 1);
        }
    }
}

trait Push2<T> {
    fn push2(&mut self, value: T) -> u32;
}

impl<T> Push2<T> for Vec<T> {
    fn push2(&mut self, value: T) -> u32 {
        self.push(value);
        (self.len() - 1) as _
    }
}

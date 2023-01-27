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
        strings.insert("value__");

        for item in definitions.items() {
            match item {
                Item::Struct(ty) => {
                    strings.insert(&ty.namespace);
                    strings.insert(&ty.name);
                    ty.fields.iter().for_each(|field| {
                        strings.insert(&field.name);
                        blobs.insert(field_blob(&field.ty, definitions, references));
                    });
                }
                Item::Enum(ty) => {
                    strings.insert(&ty.namespace);
                    strings.insert(&ty.name);
                    let enum_type = Type::named(&ty.namespace, &ty.name);
                    blobs.insert(field_blob(&enum_type, definitions, references));
                    blobs.insert(field_blob(&value_to_type(&ty.constants[0].value), definitions, references));
                    ty.constants.iter().for_each(|constant| {
                        strings.insert(&constant.name);
                        blobs.insert(value_blob(&constant.value));
                    });
                }
                Item::Interface(ty) => {
                    strings.insert(&ty.namespace);
                    strings.insert(&ty.name);
                    ty.methods.iter().for_each(|method| {
                        strings.insert(&method.name);
                        blobs.insert(method_blob(method, definitions, references));
                        method.params.iter().for_each(|param| {
                            strings.insert(&param.name);
                    });
                    });
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
        let enum_type = tables.TypeRef.push2(tables::TypeRef { TypeName: strings.index("Enum"), TypeNamespace: strings.index("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib).encode() });

        for (_index, item) in definitions.iter() {
            match item {
                Item::Struct(ty) => {
                    let mut flags = TypeAttributes::PUBLIC | TypeAttributes::SEQUENTIAL_LAYOUT  |TypeAttributes::SEALED;
                    if winrt {
                        flags |= TypeAttributes::WINRT;
                    }
                    tables.TypeDef.push(tables::TypeDef {
                        Flags: flags.0,
                        TypeName: strings.index(&ty.name),
                        TypeNamespace: strings.index(&ty.namespace),
                        Extends: TypeDefOrRef::TypeRef(value_type).encode(),
                        FieldList: tables.Field.len() as _,
                        MethodList: tables.MethodDef.len() as _,
                    });
                    for field in &ty.fields {
                        let flags = FieldAttributes::PUBLIC;
                        tables.Field.push(tables::Field { Flags: flags.0, Name: strings.index(&field.name), Signature: blobs.index(&field_blob(&field.ty, definitions, references)) });
                    }
                }
                Item::Enum(ty) => {
                    let mut flags = TypeAttributes::PUBLIC | TypeAttributes::SEALED;
                    if winrt {
                        flags |= TypeAttributes::WINRT;
                    }
                    tables.TypeDef.push(tables::TypeDef {
                        Flags: flags.0,
                        TypeName: strings.index(&ty.name),
                        TypeNamespace: strings.index(&ty.namespace),
                        Extends: TypeDefOrRef::TypeRef(enum_type).encode(),
                        FieldList: tables.Field.len() as _,
                        MethodList: tables.MethodDef.len() as _,
                    });
                    let enum_type = Type::named(&ty.namespace, &ty.name);
                    let flags = FieldAttributes::PRIVATE | FieldAttributes::SPECIAL | FieldAttributes::RUNTIME_SPECIAL;
                    tables.Field.push2(tables::Field {
                        Flags: flags.0,
                        Name: strings.index("value__"),
                        Signature: blobs.index(&field_blob(&value_to_type(&ty.constants[0].value), definitions, references)),
                    });
                    for constant in &ty.constants {
                        let flags = FieldAttributes::PUBLIC | FieldAttributes::STATIC | FieldAttributes::LITERAL | FieldAttributes::HAS_DEFAULT;
                        let field = tables.Field.push2(tables::Field { Flags: flags.0, Name: strings.index(&constant.name), Signature: blobs.index(&field_blob(&enum_type, definitions, references)) });
                        tables.Constant.push(tables::Constant { Type: value_type_code(&constant.value), Parent: HasConstant::Field(field).encode(), Value: blobs.index(&value_blob(&constant.value)) });
                    }
                }
                Item::Interface(ty) => {
                    let mut flags = TypeAttributes::PUBLIC | TypeAttributes::INTERFACE | TypeAttributes::ABSTRACT;
                    if winrt {
                        flags |= TypeAttributes::WINRT;
                    }
                    tables.TypeDef.push(tables::TypeDef { 
                        Flags: flags.0, 
                        TypeName: strings.index(&ty.name), 
                        TypeNamespace: strings.index(&ty.namespace), 
                        Extends: 0, 
                        FieldList: tables.Field.len() as _,
                        MethodList: tables.MethodDef.len() as _,
                    });
                    for method in &ty.methods {
                        let flags = MethodAttributes::ABSTRACT | MethodAttributes::HIDE_BY_SIG | MethodAttributes::NEW_SLOT | MethodAttributes::PUBLIC | MethodAttributes::VIRTUAL;
                        tables.MethodDef.push(tables::MethodDef { 
                            RVA: 0,
                            ImplFlags: 0,
                            Flags: flags.0,
                            Name: strings.index(&method.name), 
                            Signature: blobs.index(&method_blob(method, definitions, references)),
                            ParamList: tables.Param.len() as _,
                        });
                        for (sequence, param) in method.params.iter().enumerate() {
                            tables.Param.push(tables::Param {
                                 Flags: param_flags_to_attributes(param.flags).0,
                                 Sequence: (sequence + 1) as _,
                                 Name: strings.index(&param.name), 
                            });
                        }
                    }
                }
            }
        }

        tables
    };

    // With all of the streams prepared, write out ECMA-335 file format.
    file::write(tables, strings, blobs)
}

fn type_reference<'a>(ty: &'a Type, definitions: &StagedDefinitions, assemblies: &reader::Reader, references: &mut References<'a>) {
    // TODO: More matches to come...
    #[allow(clippy::single_match)]
    match ty {
        Type::Named((namespace, name)) => {
            if definitions.get(namespace, name).is_none() {
                references.insert(namespace, name, assemblies);
            }
        }
        _ => {}
    }
}

fn param_flags_to_attributes(flags: ParamFlags) -> ParamAttributes {
    let mut attributes = ParamAttributes(0);
    if flags.contains(ParamFlags::INPUT) { attributes |= ParamAttributes::INPUT; }
    if flags.contains(ParamFlags::OUTPUT) { attributes |= ParamAttributes::OUTPUT; }
    if flags.contains(ParamFlags::OPTIONAL) { attributes |= ParamAttributes::OPTIONAL; }
    attributes
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

fn method_blob(method: &Method, definitions: &StagedDefinitions, references: &StagedReferences) -> Vec<u8> {
    let mut blob = vec![0x20]; // HASTHIS
    u32_blob(method.params.len() as _, &mut blob);
    for param in &method.params {
        type_blob(&param.ty, &mut blob, definitions, references);    
    }
    type_blob(&method.return_type, &mut blob, definitions, references);
    blob
}

fn field_blob(ty: &Type, definitions: &StagedDefinitions, references: &StagedReferences) -> Vec<u8> {
    let mut blob = vec![0x6];
    type_blob(ty, &mut blob, definitions, references);
    blob
}

fn value_blob(value: &Value) -> Vec<u8> {
    match value {
        Value::I8(value) => value.to_le_bytes().to_vec(),
        Value::U8(value) => value.to_le_bytes().to_vec(),
        Value::I16(value) => value.to_le_bytes().to_vec(),
        Value::U16(value) => value.to_le_bytes().to_vec(),
        Value::I32(value) => value.to_le_bytes().to_vec(),
        Value::U32(value) => value.to_le_bytes().to_vec(),
        Value::I64(value) => value.to_le_bytes().to_vec(),
        Value::U64(value) => value.to_le_bytes().to_vec(),
        _ => panic!("Unsupported value type"),
    }
}

fn value_to_type(value: &Value) -> Type {
    match value {
        Value::I8(_) => Type::I8,
        Value::U8(_) => Type::U8,
        Value::I16(_) => Type::I16,
        Value::U16(_) => Type::U16,
        Value::I32(_) => Type::I32,
        Value::U32(_) => Type::U32,
        Value::I64(_) => Type::I64,
        Value::U64(_) => Type::U64,
        _ => panic!("Unsupported value type"),
    }
}

fn value_type_code(value: &Value) -> u16 {
    match value {
        Value::I8(_) => 0x04,
        Value::U8(_) => 0x05,
        Value::I16(_) => 0x06,
        Value::U16(_) => 0x07,
        Value::I32(_) => 0x08,
        Value::U32(_) => 0x09,
        Value::I64(_) => 0x0a,
        Value::U64(_) => 0x0b,
        _ => panic!("Unsupported value type"),
    }
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

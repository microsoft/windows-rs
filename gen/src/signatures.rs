use std::convert::TryInto;

use crate::codes::*;
use crate::read::*;

pub struct GenericSig {
    definition: TypeDefOrRef,
    args: Vec<TypeSig>,
}

// TODO: Why not collapse this?
struct ModifierSig {
    definition: TypeDefOrRef,
}

pub struct MethodSig {
    return_type: Option<ParamSig>,
    params: Vec<ParamSig>,
}

pub struct ParamSig {
    name: String,
    definition: TypeSig,
    input: bool,
    by_ref: bool,
}

pub struct TypeSig {
    array: bool,
    definition: TypeSigType,
}

#[derive(PartialEq)]
pub enum ArgumentSig {
    Bool(bool),
    Char(char),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    String(String),
    TypeDef(TypeDef),
}

pub enum TypeSigType {
    ElementType(ElementType),
    TypeDefOrRef(TypeDefOrRef),
    GenericSig(GenericSig),
    GenericTypeIndex(u32),
}

#[derive(PartialEq)]
pub enum ElementType {
    Bool,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    String,
    Object,
}

#[derive(PartialEq)]
pub enum ParamCategory {
    Array,
    Enum,
    Generic,
    Object,
    Primitive,
    String,
    Struct,
}

impl GenericSig {
    pub(crate) fn new(file: u16, bytes: &mut &[u8]) -> GenericSig {
        read_unsigned(bytes);
        let definition = TypeDefOrRef::decode(read_unsigned(bytes), file);
        let arg_count = read_unsigned(bytes);
        let mut args = Vec::with_capacity(arg_count as usize);

        for _ in 0..arg_count {
            args.push(TypeSig::new(file, bytes));
        }

        GenericSig { definition, args }
    }

    pub fn definition(&self) -> &TypeDefOrRef {
        &self.definition
    }

    pub fn args(&self) -> &Vec<TypeSig> {
        &self.args
    }
}

impl ModifierSig {
    fn new(file: u16, bytes: &mut &[u8]) -> ModifierSig {
        read_unsigned(bytes);
        let definition = TypeDefOrRef::decode(read_unsigned(bytes), file);
        ModifierSig { definition }
    }

    fn vec(file: u16, bytes: &mut &[u8]) -> Vec<ModifierSig> {
        let mut modifiers = Vec::new();
        loop {
            let (element_type, _) = peek_unsigned(bytes);
            if element_type != 32 && element_type != 31 {
                break;
            } else {
                modifiers.push(ModifierSig::new(file, bytes));
            }
        }
        modifiers
    }
}

impl MethodSig {
    pub(crate) fn new(reader: &Reader, method: &MethodDef) -> MethodSig {
        let mut bytes = reader.blob(&method.row, 4);
        let calling_convention = read_unsigned(&mut bytes);
        if calling_convention & 0x10 != 0 {
            read_unsigned(&mut bytes);
        }
        let param_count = read_unsigned(&mut bytes);
        ModifierSig::vec(method.row.file, &mut bytes);
        read_expected(&mut bytes, 0x10);
        let return_type = if read_expected(&mut bytes, 0x01) {
            None
        } else {
            Some(ParamSig {
                name: String::new(),
                input: false,
                by_ref: true,
                definition: TypeSig::new(method.row.file, &mut bytes),
            })
        };
        let mut params = Vec::with_capacity(param_count as usize);
        for param in method.params(reader) {
            if !return_type.is_some() || param.sequence(reader) != 0 {
                params.push(ParamSig::new(
                    param.name(reader).to_string(),
                    param.flags(reader).input(),
                    method.row.file,
                    &mut bytes,
                ));
            }
        }
        MethodSig {
            return_type,
            params,
        }
    }

    pub(crate) fn invalid() -> MethodSig {
        MethodSig {
            return_type: None,
            params: Vec::new(),
        }
    }

    pub fn return_type(&self) -> &Option<ParamSig> {
        &self.return_type
    }

    pub fn params(&self) -> &Vec<ParamSig> {
        &self.params
    }
}

pub(crate) fn field_sig(field: &Field, r: &Reader) -> TypeSig {
    let mut bytes = r.blob(&field.row, 2);
    read_unsigned(&mut bytes);
    ModifierSig::vec(field.row.file, &mut bytes);
    TypeSig::new(field.row.file, &mut bytes)
}

pub(crate) fn constructor_sig(file: u16, mut bytes: &[u8]) -> Vec<ParamSig> {
    let calling_convention = read_unsigned(&mut bytes);
    if calling_convention & 0x10 != 0 {
        read_unsigned(&mut bytes);
    }
    let param_count = read_unsigned(&mut bytes);
    ModifierSig::vec(file, &mut bytes);
    read_expected(&mut bytes, 0x10);
    if !read_expected(&mut bytes, 0x01) {
        TypeSig::new(file, &mut bytes);
    };
    let mut params = Vec::with_capacity(param_count as usize);
    for _ in 0..param_count {
        params.push(ParamSig::from_attribute(file, &mut bytes));
    }
    params
}

impl ArgumentSig {
    pub(crate) fn new(
        r: &Reader,
        file: u16,
        signature_bytes: &[u8],
        mut data_bytes: &[u8],
    ) -> Vec<(String, ArgumentSig)> {
        let params = constructor_sig(file, signature_bytes);
        read_u16(&mut data_bytes);
        let mut args = Vec::with_capacity(params.len());

        for param in params {
            args.push((
                String::new(),
                // TODO: flatten match
                match param.definition.definition {
                    TypeSigType::ElementType(value) => match value {
                        ElementType::I8 => ArgumentSig::I8(read_i8(&mut data_bytes)),
                        ElementType::U8 => ArgumentSig::U8(read_u8(&mut data_bytes)),
                        ElementType::I16 => ArgumentSig::I16(read_i16(&mut data_bytes)),
                        ElementType::U16 => ArgumentSig::U16(read_u16(&mut data_bytes)),
                        ElementType::I32 => ArgumentSig::I32(read_i32(&mut data_bytes)),
                        ElementType::U32 => ArgumentSig::U32(read_u32(&mut data_bytes)),
                        ElementType::I64 => ArgumentSig::I64(read_i64(&mut data_bytes)),
                        ElementType::U64 => ArgumentSig::U64(read_u64(&mut data_bytes)),
                        ElementType::String => ArgumentSig::String(read_string(&mut data_bytes)),
                        _ => panic!(),
                    },
                    TypeSigType::TypeDefOrRef(TypeDefOrRef::TypeRef(value)) => {
                        if value.name(r) == "Type" && value.namespace(r) == "System" {
                            ArgumentSig::TypeDef(r.resolve(&read_string(&mut data_bytes)))
                        } else {
                            panic!();
                        }
                    }
                    _ => panic!(),
                },
            ));
        }

        let named_args = read_u16(&mut data_bytes);

        for _ in 0..named_args {
            read_u8(&mut data_bytes);
            let arg_type = read_u8(&mut data_bytes);

            args.push(match arg_type {
                2 => (
                    read_string(&mut data_bytes),
                    ArgumentSig::Bool(read_u8(&mut data_bytes) != 0),
                ),
                8 => (
                    read_string(&mut data_bytes),
                    ArgumentSig::I32(read_i32(&mut data_bytes)),
                ),
                14 => (
                    read_string(&mut data_bytes),
                    ArgumentSig::String(read_string(&mut data_bytes)),
                ),
                0x50 => (
                    read_string(&mut data_bytes),
                    ArgumentSig::TypeDef(r.resolve(&read_string(&mut data_bytes))),
                ),
                0x55 => {
                    let enum_type = r.resolve(&read_string(&mut data_bytes));
                    (
                        read_string(&mut data_bytes),
                        match enum_type
                            .fields(r)
                            .next()
                            .unwrap()
                            .signature(r)
                            .definition()
                        {
                            TypeSigType::ElementType(value) => match value {
                                ElementType::I32 => ArgumentSig::I32(read_i32(&mut data_bytes)),
                                ElementType::U32 => ArgumentSig::U32(read_u32(&mut data_bytes)),
                                _ => panic!(),
                            },
                            _ => panic!(),
                        },
                    )
                }
                _ => panic!(),
            });
        }

        args
    }
}

impl ParamSig {
    fn new(name: String, input: bool, file: u16, bytes: &mut &[u8]) -> ParamSig {
        ModifierSig::vec(file, bytes);
        let by_ref = read_expected(bytes, 0x10);
        let definition = TypeSig::new(file, bytes);
        ParamSig {
            name: name,
            input,
            by_ref,
            definition,
        }
    }

    fn from_attribute(file: u16, bytes: &mut &[u8]) -> ParamSig {
        ModifierSig::vec(file, bytes);
        let by_ref = read_expected(bytes, 0x10);
        let definition = TypeSig::new(file, bytes);
        ParamSig {
            name: String::new(),
            input: true,
            by_ref,
            definition,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn input(&self) -> bool {
        self.input
    }

    pub fn definition(&self) -> &TypeSig {
        &self.definition
    }

    pub fn array(&self) -> bool {
        self.definition.array
    }

    pub fn by_ref(&self) -> bool {
        self.by_ref
    }
}

impl TypeSigType {
    fn new(file: u16, bytes: &mut &[u8]) -> TypeSigType {
        let element_type = read_unsigned(bytes);

        match element_type {
            0x02 => TypeSigType::ElementType(ElementType::Bool),
            0x03 => TypeSigType::ElementType(ElementType::Char),
            0x04 => TypeSigType::ElementType(ElementType::I8),
            0x05 => TypeSigType::ElementType(ElementType::U8),
            0x06 => TypeSigType::ElementType(ElementType::I16),
            0x07 => TypeSigType::ElementType(ElementType::U16),
            0x08 => TypeSigType::ElementType(ElementType::I32),
            0x09 => TypeSigType::ElementType(ElementType::U32),
            0x0A => TypeSigType::ElementType(ElementType::I64),
            0x0B => TypeSigType::ElementType(ElementType::U64),
            0x0C => TypeSigType::ElementType(ElementType::F32),
            0x0D => TypeSigType::ElementType(ElementType::F64),
            0x0E => TypeSigType::ElementType(ElementType::String),
            0x1C => TypeSigType::ElementType(ElementType::Object),
            0x11 | 0x12 => {
                TypeSigType::TypeDefOrRef(TypeDefOrRef::decode(read_unsigned(bytes), file))
            }
            0x13 => TypeSigType::GenericTypeIndex(read_unsigned(bytes)),
            0x15 => TypeSigType::GenericSig(GenericSig::new(file, bytes)),
            _ => panic!(),
        }
    }
}

impl TypeSig {
    fn new(file: u16, bytes: &mut &[u8]) -> TypeSig {
        let array = read_expected(bytes, 0x1D);
        ModifierSig::vec(file, bytes);
        let definition = TypeSigType::new(file, bytes);
        TypeSig { array, definition }
    }

    pub fn definition(&self) -> &TypeSigType {
        &self.definition
    }

    pub(crate) fn category(&self, r: &Reader) -> ParamCategory {
        if self.array {
            ParamCategory::Array
        } else {
            match &self.definition {
                TypeSigType::ElementType(ElementType::String) => ParamCategory::String,
                TypeSigType::ElementType(ElementType::Object) => ParamCategory::Object,
                TypeSigType::ElementType(_) => ParamCategory::Primitive,
                TypeSigType::GenericSig(_) => ParamCategory::Object,
                TypeSigType::TypeDefOrRef(value) => {
                    if let TypeDefOrRef::TypeRef(value) = value {
                        if value.name(r) == "Guid" && value.namespace(r) == "System" {
                            return ParamCategory::Struct;
                        }
                    }
                    match value.resolve(r).category(r) {
                        TypeCategory::Interface => ParamCategory::Object,
                        TypeCategory::Class => ParamCategory::Object,
                        TypeCategory::Delegate => ParamCategory::Object,
                        TypeCategory::Struct => ParamCategory::Struct,
                        _ => ParamCategory::Enum,
                    }
                }
                _ => ParamCategory::Generic,
            }
        }
    }
}

fn read_expected(bytes: &mut &[u8], expected: u32) -> bool {
    let (element_type, bytes_read) = peek_unsigned(bytes);
    if element_type == expected {
        *bytes = seek(bytes, bytes_read);
        true
    } else {
        false
    }
}

fn read_string(bytes: &mut &[u8]) -> String {
    let length = read_unsigned(bytes);
    let (string_bytes, rest) = bytes.split_at(length as usize);
    *bytes = rest;
    String::from_utf8(string_bytes.into()).unwrap()
}

fn read_i8(bytes: &mut &[u8]) -> i8 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<i8>());
    *bytes = rest;
    i8::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_u8(bytes: &mut &[u8]) -> u8 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<u8>());
    *bytes = rest;
    u8::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_i16(bytes: &mut &[u8]) -> i16 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<i16>());
    *bytes = rest;
    i16::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_u16(bytes: &mut &[u8]) -> u16 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<u16>());
    *bytes = rest;
    u16::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_i32(bytes: &mut &[u8]) -> i32 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<i32>());
    *bytes = rest;
    i32::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_u32(bytes: &mut &[u8]) -> u32 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<u32>());
    *bytes = rest;
    u32::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_i64(bytes: &mut &[u8]) -> i64 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<i64>());
    *bytes = rest;
    i64::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_u64(bytes: &mut &[u8]) -> u64 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<u64>());
    *bytes = rest;
    u64::from_le_bytes(value_bytes.try_into().unwrap())
}

fn seek(bytes: &[u8], bytes_read: usize) -> &[u8] {
    bytes.get(bytes_read..).unwrap()
}

fn peek_unsigned(bytes: &[u8]) -> (u32, usize) {
    let (bytes_read, value) = if bytes[0] & 0x80 == 0 {
        (1, bytes[0] as u32)
    } else if bytes[0] & 0xC0 == 0x80 {
        (2, (((bytes[0] & 0x3F) as u32) << 8) | bytes[1] as u32)
    } else {
        (
            4,
            (((bytes[0] & 0x1F) as u32) << 24)
                | (bytes[1] as u32) << 16
                | (bytes[2] as u32) << 8
                | bytes[3] as u32,
        )
    };
    // TODO: this is backwards from the result above...
    (value, bytes_read)
}

pub fn read_unsigned(bytes: &mut &[u8]) -> u32 {
    let (value, bytes_read) = peek_unsigned(bytes);
    *bytes = seek(bytes, bytes_read);
    value
}

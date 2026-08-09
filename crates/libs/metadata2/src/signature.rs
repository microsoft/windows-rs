use crate::{AnyRowId, BlobId, BlobReader, CodedIndex, Error, Image, TableId, tables};

const CALL_CONVENTION_MASK: u8 = 0x0f;
const CALL_CONVENTION_FIELD: u8 = 0x06;
const CALL_CONVENTION_LOCAL: u8 = 0x07;
const CALL_CONVENTION_PROPERTY: u8 = 0x08;
const CALL_CONVENTION_GENERIC_INSTANCE: u8 = 0x0a;
const GENERIC: u8 = 0x10;

const TYPE_VOID: u8 = 0x01;
const TYPE_BOOLEAN: u8 = 0x02;
const TYPE_CHAR: u8 = 0x03;
const TYPE_I8: u8 = 0x04;
const TYPE_U8: u8 = 0x05;
const TYPE_I16: u8 = 0x06;
const TYPE_U16: u8 = 0x07;
const TYPE_I32: u8 = 0x08;
const TYPE_U32: u8 = 0x09;
const TYPE_I64: u8 = 0x0a;
const TYPE_U64: u8 = 0x0b;
const TYPE_F32: u8 = 0x0c;
const TYPE_F64: u8 = 0x0d;
const TYPE_STRING: u8 = 0x0e;
const TYPE_POINTER: u8 = 0x0f;
const TYPE_BY_REF: u8 = 0x10;
const TYPE_VALUE: u8 = 0x11;
const TYPE_CLASS: u8 = 0x12;
const TYPE_GENERIC_TYPE: u8 = 0x13;
const TYPE_ARRAY: u8 = 0x14;
const TYPE_GENERIC_INSTANCE: u8 = 0x15;
const TYPE_TYPED_REFERENCE: u8 = 0x16;
const TYPE_ISIZE: u8 = 0x18;
const TYPE_USIZE: u8 = 0x19;
const TYPE_FUNCTION_POINTER: u8 = 0x1b;
const TYPE_OBJECT: u8 = 0x1c;
const TYPE_VECTOR: u8 = 0x1d;
const TYPE_GENERIC_METHOD: u8 = 0x1e;
const TYPE_REQUIRED_MODIFIER: u8 = 0x1f;
const TYPE_OPTIONAL_MODIFIER: u8 = 0x20;
const TYPE_SENTINEL: u8 = 0x41;
const TYPE_PINNED: u8 = 0x45;
const MAX_TYPE_DEPTH: usize = 64;

/// A decoded signature blob.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Signature {
    /// A method signature.
    Method(MethodSignature),
    /// A field signature.
    Field(Type),
    /// A property signature.
    Property(PropertySignature),
    /// A local-variable signature.
    Local(LocalSignature),
    /// A generic method-instantiation signature.
    MethodSpec(MethodSpecSignature),
}

/// A method calling convention and its parameter types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodSignature {
    /// Raw ECMA calling-convention flags.
    pub flags: u8,
    /// Generic parameter count when the generic flag is set.
    pub generic_parameter_count: Option<u32>,
    /// Return type.
    pub return_type: Type,
    /// Parameter types.
    pub parameters: Vec<Type>,
    /// Parameter position at which a vararg sentinel appears.
    pub sentinel: Option<usize>,
}

/// A property signature.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropertySignature {
    /// Raw ECMA calling-convention flags.
    pub flags: u8,
    /// Property type.
    pub property_type: Type,
    /// Index parameter types.
    pub parameters: Vec<Type>,
}

/// A local-variable signature.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalSignature {
    /// Local variable types.
    pub variables: Vec<Type>,
}

/// A generic method-instantiation signature.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodSpecSignature {
    /// Concrete generic arguments.
    pub arguments: Vec<Type>,
}

/// A custom modifier attached to a type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomModifier {
    /// Whether the modifier is required rather than optional.
    pub required: bool,
    /// Modifier type.
    pub ty: AnyRowId,
}

/// A decoded ECMA signature type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Type {
    /// Custom modifiers preceding the type.
    pub modifiers: Vec<CustomModifier>,
    /// The underlying type.
    pub kind: TypeKind,
}

/// The underlying form of an ECMA signature type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeKind {
    Void,
    Boolean,
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
    ISize,
    USize,
    Object,
    TypedReference,
    Pointer(Box<Type>),
    ByRef(Box<Type>),
    Value(AnyRowId),
    Class(AnyRowId),
    GenericType(u32),
    GenericMethod(u32),
    Array {
        element: Box<Type>,
        rank: u32,
        sizes: Vec<u32>,
        lower_bounds: Vec<i32>,
    },
    Vector(Box<Type>),
    GenericInstance {
        value_type: bool,
        ty: AnyRowId,
        arguments: Vec<Type>,
    },
    FunctionPointer(Box<MethodSignature>),
    Pinned(Box<Type>),
}

impl Image {
    /// Decodes a method signature blob.
    pub fn method_signature(&self, id: BlobId) -> Result<MethodSignature, Error> {
        let mut reader = self.blob_reader(id)?;
        let signature = read_method(self, &mut reader, 0)?;
        reader.finish()?;
        Ok(signature)
    }

    /// Decodes a field signature blob.
    pub fn field_signature(&self, id: BlobId) -> Result<Type, Error> {
        let mut reader = self.blob_reader(id)?;
        expect_convention(
            &mut reader,
            CALL_CONVENTION_FIELD,
            "invalid field signature",
        )?;
        let ty = read_type(self, &mut reader, 0)?;
        reader.finish()?;
        Ok(ty)
    }

    /// Decodes a property signature blob.
    pub fn property_signature(&self, id: BlobId) -> Result<PropertySignature, Error> {
        let mut reader = self.blob_reader(id)?;
        let flags = reader.read_u8()?;
        if flags & CALL_CONVENTION_MASK != CALL_CONVENTION_PROPERTY {
            return Err(Error::invalid(
                reader.offset() - 1,
                "invalid property signature",
            ));
        }
        let count = reader.read_compressed_u32()?;
        let property_type = read_type(self, &mut reader, 0)?;
        let parameters = read_types(self, &mut reader, count, 0)?;
        reader.finish()?;
        Ok(PropertySignature {
            flags,
            property_type,
            parameters,
        })
    }

    /// Decodes a type-specification signature blob.
    pub fn type_signature(&self, id: BlobId) -> Result<Type, Error> {
        let mut reader = self.blob_reader(id)?;
        let ty = read_type(self, &mut reader, 0)?;
        reader.finish()?;
        Ok(ty)
    }

    /// Decodes a standalone method or local-variable signature blob.
    pub fn standalone_signature(&self, id: BlobId) -> Result<Signature, Error> {
        let mut reader = self.blob_reader(id)?;
        let signature = if reader.peek_u8()? & CALL_CONVENTION_MASK == CALL_CONVENTION_LOCAL {
            Signature::Local(read_local(self, &mut reader)?)
        } else {
            Signature::Method(read_method(self, &mut reader, 0)?)
        };
        reader.finish()?;
        Ok(signature)
    }

    /// Decodes a member-reference method or field signature blob.
    pub fn member_ref_signature(&self, id: BlobId) -> Result<Signature, Error> {
        let mut reader = self.blob_reader(id)?;
        let signature = if reader.peek_u8()? & CALL_CONVENTION_MASK == CALL_CONVENTION_FIELD {
            expect_convention(
                &mut reader,
                CALL_CONVENTION_FIELD,
                "invalid field signature",
            )?;
            Signature::Field(read_type(self, &mut reader, 0)?)
        } else {
            Signature::Method(read_method(self, &mut reader, 0)?)
        };
        reader.finish()?;
        Ok(signature)
    }

    /// Decodes a generic method-instantiation signature blob.
    pub fn method_spec_signature(&self, id: BlobId) -> Result<MethodSpecSignature, Error> {
        let mut reader = self.blob_reader(id)?;
        expect_convention(
            &mut reader,
            CALL_CONVENTION_GENERIC_INSTANCE,
            "invalid method-spec signature",
        )?;
        let count = reader.read_compressed_u32()?;
        let arguments = read_types(self, &mut reader, count, 0)?;
        reader.finish()?;
        Ok(MethodSpecSignature { arguments })
    }

    pub(crate) fn validate_signatures(&self) -> Result<(), Error> {
        for id in self.rows::<tables::Field>() {
            let row = self.view(id).unwrap();
            self.field_signature(row.blob_id(2)?)
                .map_err(|error| row_error(TableId::Field, id.number(), error))?;
        }
        for id in self.rows::<tables::MethodDef>() {
            let row = self.view(id).unwrap();
            self.method_signature(row.blob_id(4)?)
                .map_err(|error| row_error(TableId::MethodDef, id.number(), error))?;
        }
        for id in self.rows::<tables::MemberRef>() {
            let row = self.view(id).unwrap();
            self.member_ref_signature(row.blob_id(2)?)
                .map_err(|error| row_error(TableId::MemberRef, id.number(), error))?;
        }
        for id in self.rows::<tables::StandAloneSig>() {
            let row = self.view(id).unwrap();
            self.standalone_signature(row.blob_id(0)?)
                .map_err(|error| row_error(TableId::StandAloneSig, id.number(), error))?;
        }
        for id in self.rows::<tables::Property>() {
            let row = self.view(id).unwrap();
            self.property_signature(row.blob_id(2)?)
                .map_err(|error| row_error(TableId::Property, id.number(), error))?;
        }
        for id in self.rows::<tables::TypeSpec>() {
            let row = self.view(id).unwrap();
            self.type_signature(row.blob_id(0)?)
                .map_err(|error| row_error(TableId::TypeSpec, id.number(), error))?;
        }
        for id in self.rows::<tables::MethodSpec>() {
            let row = self.view(id).unwrap();
            self.method_spec_signature(row.blob_id(1)?)
                .map_err(|error| row_error(TableId::MethodSpec, id.number(), error))?;
        }
        Ok(())
    }
}

fn row_error(table: TableId, row: u32, source: Error) -> Error {
    Error::Row {
        table: table.schema().name(),
        row,
        source: Box::new(source),
    }
}

fn read_method(
    image: &Image,
    reader: &mut BlobReader<'_>,
    depth: usize,
) -> Result<MethodSignature, Error> {
    check_depth(reader, depth)?;
    let flags = reader.read_u8()?;
    let convention = flags & CALL_CONVENTION_MASK;
    if matches!(
        convention,
        CALL_CONVENTION_FIELD
            | CALL_CONVENTION_LOCAL
            | CALL_CONVENTION_PROPERTY
            | CALL_CONVENTION_GENERIC_INSTANCE
            | 0x0c..=0x0f
    ) {
        return Err(Error::invalid(
            reader.offset() - 1,
            "invalid method calling convention",
        ));
    }
    let generic_parameter_count = if flags & GENERIC != 0 {
        Some(reader.read_compressed_u32()?)
    } else {
        None
    };
    let count = reader.read_compressed_u32()?;
    let return_type = read_type(image, reader, depth + 1)?;
    let mut parameters = Vec::new();
    let mut sentinel = None;
    while parameters.len() < count as usize {
        if reader.peek_u8()? == TYPE_SENTINEL {
            if convention != 0x05 {
                return Err(Error::invalid(
                    reader.offset(),
                    "vararg sentinel appears in a non-vararg signature",
                ));
            }
            if sentinel.replace(parameters.len()).is_some() {
                return Err(Error::invalid(reader.offset(), "duplicate vararg sentinel"));
            }
            reader.read_u8()?;
            continue;
        }
        parameters.push(read_type(image, reader, depth + 1)?);
    }
    Ok(MethodSignature {
        flags,
        generic_parameter_count,
        return_type,
        parameters,
        sentinel,
    })
}

fn read_local(image: &Image, reader: &mut BlobReader<'_>) -> Result<LocalSignature, Error> {
    expect_convention(reader, CALL_CONVENTION_LOCAL, "invalid local signature")?;
    let count = reader.read_compressed_u32()?;
    let mut variables = Vec::new();
    for _ in 0..count {
        let mut ty = read_type(image, reader, 0)?;
        if reader.remaining() != 0 && reader.peek_u8()? == TYPE_PINNED {
            reader.read_u8()?;
            ty = Type {
                modifiers: Vec::new(),
                kind: TypeKind::Pinned(Box::new(ty)),
            };
        }
        variables.push(ty);
    }
    Ok(LocalSignature { variables })
}

fn read_types(
    image: &Image,
    reader: &mut BlobReader<'_>,
    count: u32,
    depth: usize,
) -> Result<Vec<Type>, Error> {
    let mut result = Vec::new();
    for _ in 0..count {
        result.push(read_type(image, reader, depth + 1)?);
    }
    Ok(result)
}

fn read_type(image: &Image, reader: &mut BlobReader<'_>, depth: usize) -> Result<Type, Error> {
    check_depth(reader, depth)?;
    let mut modifiers = Vec::new();
    loop {
        let required = match reader.peek_u8()? {
            TYPE_REQUIRED_MODIFIER => true,
            TYPE_OPTIONAL_MODIFIER => false,
            _ => break,
        };
        let offset = reader.offset();
        reader.read_u8()?;
        let encoded = reader.read_compressed_u32()?;
        let ty = decode_type_ref(image, encoded, offset)?;
        modifiers.push(CustomModifier { required, ty });
    }

    let offset = reader.offset();
    let code = reader.read_u8()?;
    let kind = match code {
        TYPE_VOID => TypeKind::Void,
        TYPE_BOOLEAN => TypeKind::Boolean,
        TYPE_CHAR => TypeKind::Char,
        TYPE_I8 => TypeKind::I8,
        TYPE_U8 => TypeKind::U8,
        TYPE_I16 => TypeKind::I16,
        TYPE_U16 => TypeKind::U16,
        TYPE_I32 => TypeKind::I32,
        TYPE_U32 => TypeKind::U32,
        TYPE_I64 => TypeKind::I64,
        TYPE_U64 => TypeKind::U64,
        TYPE_F32 => TypeKind::F32,
        TYPE_F64 => TypeKind::F64,
        TYPE_STRING => TypeKind::String,
        TYPE_ISIZE => TypeKind::ISize,
        TYPE_USIZE => TypeKind::USize,
        TYPE_OBJECT => TypeKind::Object,
        TYPE_TYPED_REFERENCE => TypeKind::TypedReference,
        TYPE_POINTER => TypeKind::Pointer(Box::new(read_type(image, reader, depth + 1)?)),
        TYPE_BY_REF => TypeKind::ByRef(Box::new(read_type(image, reader, depth + 1)?)),
        TYPE_VALUE => TypeKind::Value(read_type_ref(image, reader)?),
        TYPE_CLASS => TypeKind::Class(read_type_ref(image, reader)?),
        TYPE_GENERIC_TYPE => TypeKind::GenericType(reader.read_compressed_u32()?),
        TYPE_GENERIC_METHOD => TypeKind::GenericMethod(reader.read_compressed_u32()?),
        TYPE_ARRAY => {
            let element = Box::new(read_type(image, reader, depth + 1)?);
            let rank = reader.read_compressed_u32()?;
            let size_count = reader.read_compressed_u32()?;
            if size_count > rank {
                return Err(Error::invalid(
                    reader.offset(),
                    "array size count exceeds rank",
                ));
            }
            let mut sizes = Vec::new();
            for _ in 0..size_count {
                sizes.push(reader.read_compressed_u32()?);
            }
            let bound_count = reader.read_compressed_u32()?;
            if bound_count > rank {
                return Err(Error::invalid(
                    reader.offset(),
                    "array bound count exceeds rank",
                ));
            }
            let mut lower_bounds = Vec::new();
            for _ in 0..bound_count {
                lower_bounds.push(reader.read_compressed_i32()?);
            }
            TypeKind::Array {
                element,
                rank,
                sizes,
                lower_bounds,
            }
        }
        TYPE_VECTOR => TypeKind::Vector(Box::new(read_type(image, reader, depth + 1)?)),
        TYPE_GENERIC_INSTANCE => {
            let value_type = match reader.read_u8()? {
                TYPE_VALUE => true,
                TYPE_CLASS => false,
                _ => {
                    return Err(Error::invalid(
                        reader.offset() - 1,
                        "generic instance does not name a class or value type",
                    ));
                }
            };
            let ty = read_type_ref(image, reader)?;
            let count = reader.read_compressed_u32()?;
            let arguments = read_types(image, reader, count, depth + 1)?;
            TypeKind::GenericInstance {
                value_type,
                ty,
                arguments,
            }
        }
        TYPE_FUNCTION_POINTER => {
            TypeKind::FunctionPointer(Box::new(read_method(image, reader, depth + 1)?))
        }
        TYPE_SENTINEL => {
            return Err(Error::invalid(
                offset,
                "vararg sentinel appears outside parameters",
            ));
        }
        TYPE_PINNED => {
            return Err(Error::invalid(
                offset,
                "pinned marker appears before a type",
            ));
        }
        _ => return Err(Error::invalid(offset, "unsupported signature element type")),
    };
    Ok(Type { modifiers, kind })
}

fn read_type_ref(image: &Image, reader: &mut BlobReader<'_>) -> Result<AnyRowId, Error> {
    let offset = reader.offset();
    let encoded = reader.read_compressed_u32()?;
    decode_type_ref(image, encoded, offset)
}

fn decode_type_ref(image: &Image, encoded: u32, offset: usize) -> Result<AnyRowId, Error> {
    let Some(row) = image.decode_coded(CodedIndex::TypeDefOrRef, encoded, offset)? else {
        return Err(Error::invalid(offset, "null type reference"));
    };
    if !matches!(
        row.table(),
        TableId::TypeDef | TableId::TypeRef | TableId::TypeSpec
    ) {
        return Err(Error::invalid(offset, "invalid type reference"));
    }
    Ok(row)
}

fn expect_convention(
    reader: &mut BlobReader<'_>,
    expected: u8,
    message: &'static str,
) -> Result<(), Error> {
    let offset = reader.offset();
    let actual = reader.read_u8()?;
    if actual & CALL_CONVENTION_MASK == expected && actual & !CALL_CONVENTION_MASK == 0 {
        Ok(())
    } else {
        Err(Error::invalid(offset, message))
    }
}

fn check_depth(reader: &BlobReader<'_>, depth: usize) -> Result<(), Error> {
    if depth <= MAX_TYPE_DEPTH {
        Ok(())
    } else {
        Err(Error::invalid(
            reader.offset(),
            "signature nesting exceeds the supported limit",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image() -> Image {
        Image::new(windows_default::WINRT).unwrap()
    }

    #[test]
    fn rejects_excessive_type_nesting() {
        let image = image();
        let mut bytes = vec![TYPE_POINTER; MAX_TYPE_DEPTH + 1];
        bytes.push(TYPE_I32);
        let mut reader = BlobReader::new(&bytes, 50);
        assert!(matches!(
            read_type(&image, &mut reader, 0),
            Err(Error::Invalid {
                message: "signature nesting exceeds the supported limit",
                ..
            })
        ));
    }

    #[test]
    fn rejects_sentinel_on_non_vararg_method() {
        let image = image();
        let bytes = [0x00, 0x01, TYPE_VOID, TYPE_SENTINEL, TYPE_I32];
        let mut reader = BlobReader::new(&bytes, 50);
        assert!(matches!(
            read_method(&image, &mut reader, 0),
            Err(Error::Invalid {
                message: "vararg sentinel appears in a non-vararg signature",
                ..
            })
        ));
    }
}

use super::*;

const SERIALIZATION_TYPE_TYPE: u8 = 0x50;
const SERIALIZATION_TYPE_OBJECT: u8 = 0x51;
const SERIALIZATION_TYPE_FIELD: u8 = 0x53;
const SERIALIZATION_TYPE_PROPERTY: u8 = 0x54;
const SERIALIZATION_TYPE_ENUM: u8 = 0x55;

/// Identifies a metadata type reference within a database.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TypeIdentity {
    /// Owning metadata image.
    pub file: FileId,
    /// TypeDef, TypeRef, or TypeSpec row.
    pub ty: AnyRowId,
}

/// An enum type encoded by metadata identity or by a named-argument type string.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnumType {
    Metadata(TypeIdentity),
    Named(String),
}

/// A custom-attribute serialization type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttributeType {
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
    Type,
    Object,
    Enum(EnumType),
    Array(Box<Self>),
}

/// The integer storage type of an enum used by a custom attribute.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnumBacking {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
}

/// Resolves enum definitions supplied by metadata dependencies outside a [`Database`].
pub trait EnumResolver {
    /// Returns the enum's integer storage type, or `None` when the dependency is unavailable.
    fn enum_backing(&self, database: &Database, ty: &EnumType) -> Option<EnumBacking>;
}

impl EnumResolver for () {
    fn enum_backing(&self, _: &Database, _: &EnumType) -> Option<EnumBacking> {
        None
    }
}

/// A decoded custom-attribute value.
#[derive(Clone, Debug, PartialEq)]
pub enum AttributeValue {
    Boolean(bool),
    Char(u16),
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
    TypeName(String),
    Enum {
        ty: EnumType,
        value: Box<Self>,
    },
    Boxed {
        ty: AttributeType,
        value: Box<Self>,
    },
    Array {
        element: AttributeType,
        values: Vec<Self>,
    },
    Null(AttributeType),
}

/// Distinguishes named field and property arguments.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NamedArgumentKind {
    Field,
    Property,
}

/// One fixed or named custom-attribute argument.
#[derive(Clone, Debug, PartialEq)]
pub enum AttributeArgument {
    Fixed {
        ty: AttributeType,
        value: AttributeValue,
    },
    Named {
        kind: NamedArgumentKind,
        name: String,
        ty: AttributeType,
        value: AttributeValue,
    },
}

impl Database {
    /// Decodes one custom-attribute value using its constructor signature.
    pub fn attribute(
        &self,
        attribute: Entity<tables::CustomAttribute>,
    ) -> Result<Vec<AttributeArgument>, Error> {
        self.attribute_with(attribute, &())
    }

    /// Decodes one custom-attribute value using an external enum dependency resolver.
    pub fn attribute_with(
        &self,
        attribute: Entity<tables::CustomAttribute>,
        resolver: &impl EnumResolver,
    ) -> Result<Vec<AttributeArgument>, Error> {
        let image = self
            .image(attribute.file())
            .ok_or_else(|| Error::invalid_metadata("invalid attribute file identity"))?;
        let row = image
            .view(attribute.row())
            .ok_or_else(|| Error::invalid_metadata("invalid attribute identity"))?;
        let constructor = row
            .coded(1)?
            .ok_or_else(|| Error::invalid_metadata("null attribute constructor"))?;
        let signature = self.constructor_signature(attribute.file(), constructor)?;
        let mut reader = image.blob_reader(row.blob_id(2)?)?;
        if reader.read_u16()? != 1 {
            return Err(Error::invalid(
                reader.offset() - 2,
                "invalid custom-attribute prolog",
            ));
        }

        let mut arguments = Vec::new();
        for parameter in &signature.parameters {
            let ty = self.attribute_type(attribute.file(), parameter)?;
            let value = self.read_attribute_value(&mut reader, &ty, resolver)?;
            arguments.push(AttributeArgument::Fixed { ty, value });
        }

        let named_count = reader.read_u16()?;
        for _ in 0..named_count {
            let offset = reader.offset();
            let kind = match reader.read_u8()? {
                SERIALIZATION_TYPE_FIELD => NamedArgumentKind::Field,
                SERIALIZATION_TYPE_PROPERTY => NamedArgumentKind::Property,
                _ => return Err(Error::invalid(offset, "invalid named-argument tag")),
            };
            let ty = read_named_type(&mut reader)?;
            let name = read_ser_string(&mut reader)?
                .ok_or_else(|| Error::invalid(reader.offset(), "null named-argument name"))?;
            let value = self.read_attribute_value(&mut reader, &ty, resolver)?;
            arguments.push(AttributeArgument::Named {
                kind,
                name,
                ty,
                value,
            });
        }
        reader.finish()?;
        Ok(arguments)
    }

    /// Decodes every custom attribute in the database.
    pub fn validate_attributes(&self, resolver: &impl EnumResolver) -> Result<(), Error> {
        for (file, image) in self.images().iter().enumerate() {
            let file = FileId::new(file);
            for row in image.rows::<tables::CustomAttribute>() {
                self.attribute_with(Entity::new(file, row), resolver)
                    .map_err(|source| Error::Row {
                        table: TableId::CustomAttribute.schema().name(),
                        row: row.number(),
                        source: Box::new(source),
                    })?;
            }
        }
        Ok(())
    }

    fn constructor_signature(
        &self,
        file: FileId,
        constructor: AnyRowId,
    ) -> Result<MethodSignature, Error> {
        let image = self.image(file).unwrap();
        match constructor.table() {
            TableId::MethodDef => {
                let row = image
                    .row::<tables::MethodDef>(constructor.number())
                    .and_then(|row| image.view(row))
                    .ok_or_else(|| Error::invalid_metadata("invalid attribute constructor"))?;
                image.method_signature(row.blob_id(4)?)
            }
            TableId::MemberRef => {
                let row = image
                    .row::<tables::MemberRef>(constructor.number())
                    .and_then(|row| image.view(row))
                    .ok_or_else(|| Error::invalid_metadata("invalid attribute constructor"))?;
                match image.member_ref_signature(row.blob_id(2)?)? {
                    Signature::Method(signature) => Ok(signature),
                    _ => Err(Error::invalid_metadata(
                        "attribute constructor is not a method",
                    )),
                }
            }
            _ => Err(Error::invalid_metadata(
                "invalid attribute constructor table",
            )),
        }
    }

    fn attribute_type(&self, file: FileId, ty: &Type) -> Result<AttributeType, Error> {
        if !ty.modifiers.is_empty() {
            return Err(Error::invalid_metadata(
                "custom-attribute type has modifiers",
            ));
        }
        Ok(match &ty.kind {
            TypeKind::Boolean => AttributeType::Boolean,
            TypeKind::Char => AttributeType::Char,
            TypeKind::I8 => AttributeType::I8,
            TypeKind::U8 => AttributeType::U8,
            TypeKind::I16 => AttributeType::I16,
            TypeKind::U16 => AttributeType::U16,
            TypeKind::I32 => AttributeType::I32,
            TypeKind::U32 => AttributeType::U32,
            TypeKind::I64 => AttributeType::I64,
            TypeKind::U64 => AttributeType::U64,
            TypeKind::F32 => AttributeType::F32,
            TypeKind::F64 => AttributeType::F64,
            TypeKind::String => AttributeType::String,
            TypeKind::Object => AttributeType::Object,
            TypeKind::Class(ty) if self.type_name(file, *ty)? == Some(("System", "Type")) => {
                AttributeType::Type
            }
            TypeKind::Value(ty) | TypeKind::Class(ty) => {
                AttributeType::Enum(EnumType::Metadata(TypeIdentity { file, ty: *ty }))
            }
            TypeKind::Vector(element) => {
                AttributeType::Array(Box::new(self.attribute_type(file, element)?))
            }
            _ => {
                return Err(Error::invalid_metadata(
                    "invalid custom-attribute parameter type",
                ));
            }
        })
    }

    fn read_attribute_value(
        &self,
        reader: &mut BlobReader<'_>,
        ty: &AttributeType,
        resolver: &impl EnumResolver,
    ) -> Result<AttributeValue, Error> {
        Ok(match ty {
            AttributeType::Boolean => match reader.read_u8()? {
                0 => AttributeValue::Boolean(false),
                1 => AttributeValue::Boolean(true),
                _ => return Err(Error::invalid(reader.offset() - 1, "invalid Boolean value")),
            },
            AttributeType::Char => AttributeValue::Char(reader.read_u16()?),
            AttributeType::I8 => AttributeValue::I8(reader.read_i8()?),
            AttributeType::U8 => AttributeValue::U8(reader.read_u8()?),
            AttributeType::I16 => AttributeValue::I16(reader.read_i16()?),
            AttributeType::U16 => AttributeValue::U16(reader.read_u16()?),
            AttributeType::I32 => AttributeValue::I32(reader.read_i32()?),
            AttributeType::U32 => AttributeValue::U32(reader.read_u32()?),
            AttributeType::I64 => AttributeValue::I64(reader.read_i64()?),
            AttributeType::U64 => AttributeValue::U64(reader.read_u64()?),
            AttributeType::F32 => AttributeValue::F32(reader.read_f32()?),
            AttributeType::F64 => AttributeValue::F64(reader.read_f64()?),
            AttributeType::String => match read_ser_string(reader)? {
                Some(value) => AttributeValue::String(value),
                None => AttributeValue::Null(ty.clone()),
            },
            AttributeType::Type => match read_ser_string(reader)? {
                Some(value) => AttributeValue::TypeName(value),
                None => AttributeValue::Null(ty.clone()),
            },
            AttributeType::Enum(enum_type) => {
                let backing = self.enum_backing(enum_type, resolver)?;
                let value = self.read_attribute_value(reader, &backing, resolver)?;
                AttributeValue::Enum {
                    ty: enum_type.clone(),
                    value: Box::new(value),
                }
            }
            AttributeType::Object => {
                if reader.peek_u8()? == 0xff {
                    reader.read_u8()?;
                    AttributeValue::Null(AttributeType::Object)
                } else {
                    let ty = read_named_type(reader)?;
                    let value = self.read_attribute_value(reader, &ty, resolver)?;
                    AttributeValue::Boxed {
                        ty,
                        value: Box::new(value),
                    }
                }
            }
            AttributeType::Array(element) => {
                let count = reader.read_u32()?;
                if count == u32::MAX {
                    AttributeValue::Null(ty.clone())
                } else {
                    if count as usize > reader.remaining() {
                        return Err(Error::invalid(
                            reader.offset(),
                            "array count exceeds remaining data",
                        ));
                    }
                    let mut values = Vec::new();
                    for _ in 0..count {
                        values.push(self.read_attribute_value(reader, element, resolver)?);
                    }
                    AttributeValue::Array {
                        element: element.as_ref().clone(),
                        values,
                    }
                }
            }
        })
    }

    fn enum_backing(
        &self,
        ty: &EnumType,
        resolver: &impl EnumResolver,
    ) -> Result<AttributeType, Error> {
        enum Definitions<'a> {
            One(Entity<tables::TypeDef>),
            Many(&'a [Entity<tables::TypeDef>]),
        }

        let definitions = match ty {
            EnumType::Metadata(identity) => match self.resolve_type(identity.file, identity.ty)? {
                TypeResolution::Definition(definition) => Definitions::One(definition),
                TypeResolution::Candidates(definitions) => Definitions::Many(definitions),
                TypeResolution::Specification(_) => {
                    return Err(Error::invalid_metadata("enum type is a TypeSpec"));
                }
            },
            EnumType::Named(name) => {
                let name = name.split(',').next().unwrap_or(name).trim();
                let (namespace, name) = name
                    .rsplit_once('.')
                    .ok_or_else(|| Error::invalid_metadata("enum name has no namespace"))?;
                Definitions::Many(self.type_definitions(namespace, name))
            }
        };

        let mut backing = None;
        let mut found = false;
        let mut visit = |definition: Entity<tables::TypeDef>| -> Result<(), Error> {
            found = true;
            let image = self.image(definition.file()).unwrap();
            let value_field = self
                .fields(definition)?
                .find(|field| image.view(*field).unwrap().string(1).unwrap() == "value__")
                .ok_or_else(|| Error::invalid_metadata("enum has no value__ field"))?;
            let field = image.view(value_field).unwrap();
            let ty = enum_backing(&image.field_signature(field.blob_id(2)?)?.kind)?;
            if backing.as_ref().is_some_and(|backing| backing != &ty) {
                return Err(Error::invalid_metadata(
                    "enum definitions have different backing types",
                ));
            }
            backing = Some(ty);
            Ok(())
        };

        match definitions {
            Definitions::One(definition) => visit(definition)?,
            Definitions::Many(definitions) => {
                for definition in definitions {
                    visit(*definition)?;
                }
            }
        }
        if !found {
            if let Some(backing) = resolver.enum_backing(self, ty) {
                return Ok(backing.into());
            }
            let (namespace, name) = match ty {
                EnumType::Metadata(identity) => {
                    self.type_name(identity.file, identity.ty)?.map_or_else(
                        || (String::new(), "<TypeSpec>".to_string()),
                        |(namespace, name)| (namespace.to_string(), name.to_string()),
                    )
                }
                EnumType::Named(name) => {
                    let name = name.split(',').next().unwrap_or(name).trim();
                    name.rsplit_once('.').map_or_else(
                        || (String::new(), name.to_string()),
                        |(namespace, name)| (namespace.to_string(), name.to_string()),
                    )
                }
            };
            return Err(Error::UnresolvedType { namespace, name });
        }
        Ok(backing.unwrap().into())
    }
}

fn read_named_type(reader: &mut BlobReader<'_>) -> Result<AttributeType, Error> {
    let offset = reader.offset();
    Ok(match reader.read_u8()? {
        0x02 => AttributeType::Boolean,
        0x03 => AttributeType::Char,
        0x04 => AttributeType::I8,
        0x05 => AttributeType::U8,
        0x06 => AttributeType::I16,
        0x07 => AttributeType::U16,
        0x08 => AttributeType::I32,
        0x09 => AttributeType::U32,
        0x0a => AttributeType::I64,
        0x0b => AttributeType::U64,
        0x0c => AttributeType::F32,
        0x0d => AttributeType::F64,
        0x0e => AttributeType::String,
        0x1d => AttributeType::Array(Box::new(read_named_type(reader)?)),
        SERIALIZATION_TYPE_TYPE => AttributeType::Type,
        SERIALIZATION_TYPE_OBJECT => AttributeType::Object,
        SERIALIZATION_TYPE_ENUM => AttributeType::Enum(EnumType::Named(
            read_ser_string(reader)?
                .ok_or_else(|| Error::invalid(reader.offset(), "null enum type name"))?,
        )),
        _ => return Err(Error::invalid(offset, "invalid named-argument type")),
    })
}

fn read_ser_string(reader: &mut BlobReader<'_>) -> Result<Option<String>, Error> {
    if reader.peek_u8()? == 0xff {
        reader.read_u8()?;
        return Ok(None);
    }
    let offset = reader.offset();
    let length = reader.read_compressed_u32()? as usize;
    let bytes = reader.read_bytes(length)?;
    let value = std::str::from_utf8(bytes)
        .map_err(|_| Error::invalid(offset, "custom-attribute string is not UTF-8"))?;
    Ok(Some(value.to_string()))
}

fn enum_backing(kind: &TypeKind) -> Result<EnumBacking, Error> {
    Ok(match kind {
        TypeKind::I8 => EnumBacking::I8,
        TypeKind::U8 => EnumBacking::U8,
        TypeKind::I16 => EnumBacking::I16,
        TypeKind::U16 => EnumBacking::U16,
        TypeKind::I32 => EnumBacking::I32,
        TypeKind::U32 => EnumBacking::U32,
        TypeKind::I64 => EnumBacking::I64,
        TypeKind::U64 => EnumBacking::U64,
        _ => return Err(Error::invalid_metadata("invalid enum backing type")),
    })
}

impl From<EnumBacking> for AttributeType {
    fn from(value: EnumBacking) -> Self {
        match value {
            EnumBacking::I8 => Self::I8,
            EnumBacking::U8 => Self::U8,
            EnumBacking::I16 => Self::I16,
            EnumBacking::U16 => Self::U16,
            EnumBacking::I32 => Self::I32,
            EnumBacking::U32 => Self::U32,
            EnumBacking::I64 => Self::I64,
            EnumBacking::U64 => Self::U64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_metadata as old;

    struct FrameworkEnums;

    impl EnumResolver for FrameworkEnums {
        fn enum_backing(&self, database: &Database, ty: &EnumType) -> Option<EnumBacking> {
            let EnumType::Metadata(identity) = ty else {
                return None;
            };
            match database
                .type_name(identity.file, identity.ty)
                .ok()
                .flatten()?
            {
                ("System.Runtime.InteropServices", "CallingConvention") => Some(EnumBacking::I32),
                _ => None,
            }
        }
    }

    #[test]
    fn decodes_committed_attribute_corpus() {
        let database = Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap();
        database.validate_attributes(&FrameworkEnums).unwrap();
    }

    #[test]
    fn decodes_local_enum_backing_and_named_arguments() {
        let mut file = old::writer::File::new("test");
        let system_enum = file.TypeRef("System", "Enum");
        file.TypeDef(
            "Test",
            "Small",
            old::writer::TypeDefOrRef::TypeRef(system_enum),
            old::TypeAttributes::Public | old::TypeAttributes::Sealed,
        );
        file.Field(
            "value__",
            &old::Type::U8,
            old::FieldAttributes::Public | old::FieldAttributes::SpecialName,
        );
        let target = file.TypeDef(
            "Test",
            "Target",
            old::writer::TypeDefOrRef::default(),
            old::TypeAttributes::Public,
        );
        let parent = file.TypeRef("Test", "Attribute");
        let signature = old::Signature {
            types: vec![old::Type::value_named("Test", "Small"), old::Type::String],
            ..Default::default()
        };
        let constructor = file.MemberRef(
            ".ctor",
            &signature,
            old::writer::MemberRefParent::TypeRef(parent),
        );
        file.Attribute(
            old::writer::HasAttribute::TypeDef(target),
            old::writer::AttributeType::MemberRef(constructor),
            &[
                (
                    String::new(),
                    old::Value::EnumValue(
                        old::TypeName::named("Test", "Small"),
                        Box::new(old::Value::U8(7)),
                    ),
                ),
                (String::new(), old::Value::Utf8("fixed".to_string())),
                ("Named".to_string(), old::Value::U16(42)),
            ],
        );

        let database = Database::new([Image::new(file.into_stream()).unwrap()]).unwrap();
        let row = database.images()[0]
            .rows::<tables::CustomAttribute>()
            .next()
            .unwrap();
        let arguments = database
            .attribute(Entity::new(FileId::new(0), row))
            .unwrap();

        assert_eq!(arguments.len(), 3);
        let AttributeArgument::Fixed {
            ty: AttributeType::Enum(ty),
            value:
                AttributeValue::Enum {
                    ty: value_ty,
                    value,
                },
        } = &arguments[0]
        else {
            panic!("expected fixed enum argument");
        };
        assert_eq!(ty, value_ty);
        assert_eq!(value.as_ref(), &AttributeValue::U8(7));
        assert_eq!(
            arguments[1],
            AttributeArgument::Fixed {
                ty: AttributeType::String,
                value: AttributeValue::String("fixed".to_string()),
            }
        );
        assert_eq!(
            arguments[2],
            AttributeArgument::Named {
                kind: NamedArgumentKind::Field,
                name: "Named".to_string(),
                ty: AttributeType::U16,
                value: AttributeValue::U16(42),
            }
        );
    }
}

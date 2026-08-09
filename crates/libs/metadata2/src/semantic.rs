use super::*;

/// The type category represented by a TypeDef row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeCategory {
    Interface,
    Class,
    Enum,
    Delegate,
    Struct,
    Attribute,
}

/// A database-backed semantic view of a TypeDef row.
#[derive(Clone, Copy)]
pub struct TypeDefinition<'a> {
    database: &'a Database,
    entity: Entity<tables::TypeDef>,
}

impl Database {
    /// Returns semantic views of all indexed top-level type definitions.
    pub fn definitions(&self) -> impl Iterator<Item = TypeDefinition<'_>> {
        self.type_names()
            .flat_map(|(_, _, definitions)| definitions.iter().copied())
            .map(|entity| TypeDefinition {
                database: self,
                entity,
            })
    }

    /// Returns a semantic view of one type definition.
    pub fn definition(&self, entity: Entity<tables::TypeDef>) -> Option<TypeDefinition<'_>> {
        self.view(entity).map(|_| TypeDefinition {
            database: self,
            entity,
        })
    }

    /// Returns a semantic view of one field.
    pub fn field(&self, entity: Entity<tables::Field>) -> Option<FieldDefinition<'_>> {
        self.view(entity).map(|_| FieldDefinition {
            database: self,
            entity,
        })
    }

    /// Returns a semantic view of one method.
    pub fn method(&self, entity: Entity<tables::MethodDef>) -> Option<MethodDefinition<'_>> {
        self.view(entity).map(|_| MethodDefinition {
            database: self,
            entity,
        })
    }
}

impl<'a> TypeDefinition<'a> {
    /// Returns the database identity.
    pub const fn entity(self) -> Entity<tables::TypeDef> {
        self.entity
    }

    /// Returns the encoded TypeAttributes flags.
    pub fn flags(self) -> Result<u32, Error> {
        self.row()?.u32(0)
    }

    /// Returns whether the WindowsRuntime flag is set.
    pub fn is_windows_runtime(self) -> Result<bool, Error> {
        Ok(self.flags()? & 0x4000 != 0)
    }

    /// Returns the metadata type name.
    pub fn name(self) -> Result<&'a str, Error> {
        self.row()?.string(1)
    }

    /// Returns the metadata type namespace.
    pub fn namespace(self) -> Result<&'a str, Error> {
        self.row()?.string(2)
    }

    /// Returns the type category implied by the base type.
    pub fn category(self) -> Result<TypeCategory, Error> {
        let Some(extends) = self.row()?.coded(3)? else {
            return Ok(TypeCategory::Interface);
        };
        let Some((namespace, name)) = self.database.type_name(self.entity.file(), extends)? else {
            return Ok(TypeCategory::Class);
        };
        Ok(if namespace == "System" {
            match name {
                "Enum" => TypeCategory::Enum,
                "MulticastDelegate" => TypeCategory::Delegate,
                "ValueType" => TypeCategory::Struct,
                "Attribute" => TypeCategory::Attribute,
                _ => TypeCategory::Class,
            }
        } else {
            TypeCategory::Class
        })
    }

    /// Iterates the fields declared by this type.
    pub fn fields(self) -> Result<impl ExactSizeIterator<Item = FieldDefinition<'a>>, Error> {
        let file = self.entity.file();
        Ok(self
            .database
            .type_members::<tables::Field>(self.entity, 4)?
            .map(move |row| FieldDefinition {
                database: self.database,
                entity: Entity::new(file, row),
            }))
    }

    /// Iterates the methods declared by this type.
    pub fn methods(self) -> Result<impl ExactSizeIterator<Item = MethodDefinition<'a>>, Error> {
        let file = self.entity.file();
        Ok(self
            .database
            .type_members::<tables::MethodDef>(self.entity, 5)?
            .map(move |row| MethodDefinition {
                database: self.database,
                entity: Entity::new(file, row),
            }))
    }

    /// Iterates the custom attributes attached to this type.
    pub fn attributes(
        self,
    ) -> Result<impl ExactSizeIterator<Item = AttributeDefinition<'a>>, Error> {
        let encoded = CodedIndex::HasCustomAttribute
            .encode(TableId::TypeDef, self.entity.row().number())
            .ok_or_else(|| Error::invalid_metadata("TypeDef cannot own custom attributes"))?;
        let file = self.entity.file();
        let image = self
            .database
            .image(file)
            .ok_or_else(|| Error::invalid_metadata("invalid file identity"))?;
        Ok(image
            .matching_rows::<tables::CustomAttribute>(0, encoded)?
            .map(move |row| AttributeDefinition {
                database: self.database,
                entity: Entity::new(file, row),
            }))
    }

    /// Returns the first custom attribute with the given type name.
    pub fn find_attribute(self, name: &str) -> Result<Option<AttributeDefinition<'a>>, Error> {
        for attribute in self.attributes()? {
            if attribute.name()? == Some(name) {
                return Ok(Some(attribute));
            }
        }
        Ok(None)
    }

    /// Returns whether a custom attribute with the given type name is present.
    pub fn has_attribute(self, name: &str) -> Result<bool, Error> {
        Ok(self.find_attribute(name)?.is_some())
    }

    /// Returns the SupportedArchitectureAttribute bit mask, or zero when absent.
    pub fn architectures(self) -> Result<i32, Error> {
        let Some(attribute) = self.find_attribute("SupportedArchitectureAttribute")? else {
            return Ok(0);
        };
        let arguments = attribute.arguments(&())?;
        let Some(AttributeArgument::Fixed { value, .. }) = arguments.first() else {
            return Err(Error::invalid_metadata(
                "SupportedArchitectureAttribute has no fixed argument",
            ));
        };
        match value {
            AttributeValue::I32(value) => Ok(*value),
            AttributeValue::Enum { value, .. } => match value.as_ref() {
                AttributeValue::I32(value) => Ok(*value),
                _ => Err(Error::invalid_metadata(
                    "SupportedArchitectureAttribute enum is not i32-backed",
                )),
            },
            _ => Err(Error::invalid_metadata(
                "SupportedArchitectureAttribute argument is not i32",
            )),
        }
    }

    fn row(self) -> Result<Row<'a, tables::TypeDef>, Error> {
        self.database
            .view(self.entity)
            .ok_or_else(|| Error::invalid_metadata("invalid type definition identity"))
    }
}

/// A database-backed semantic view of a Field row.
#[derive(Clone, Copy)]
pub struct FieldDefinition<'a> {
    database: &'a Database,
    entity: Entity<tables::Field>,
}

impl<'a> FieldDefinition<'a> {
    /// Returns the database identity.
    pub const fn entity(self) -> Entity<tables::Field> {
        self.entity
    }

    /// Returns the encoded FieldAttributes flags.
    pub fn flags(self) -> Result<u16, Error> {
        self.row()?.u16(0)
    }

    /// Returns whether the Literal flag is set.
    pub fn is_literal(self) -> Result<bool, Error> {
        Ok(self.flags()? & 0x40 != 0)
    }

    /// Returns the metadata field name.
    pub fn name(self) -> Result<&'a str, Error> {
        self.row()?.string(1)
    }

    /// Returns the decoded field signature.
    pub fn signature(self) -> Result<Type, Error> {
        let row = self.row()?;
        self.database
            .image(self.entity.file())
            .unwrap()
            .field_signature(row.blob_id(2)?)
    }

    /// Returns the constant attached to this field, when present.
    pub fn constant(self) -> Result<Option<ConstantDefinition<'a>>, Error> {
        let encoded = CodedIndex::HasConstant
            .encode(TableId::Field, self.entity.row().number())
            .ok_or_else(|| Error::invalid_metadata("Field cannot own a constant"))?;
        let file = self.entity.file();
        let image = self
            .database
            .image(file)
            .ok_or_else(|| Error::invalid_metadata("invalid file identity"))?;
        let mut rows = image.matching_rows::<tables::Constant>(1, encoded)?;
        let constant = rows.next().map(|row| ConstantDefinition {
            database: self.database,
            entity: Entity::new(file, row),
        });
        if rows.next().is_some() {
            return Err(Error::invalid_metadata("field has more than one constant"));
        }
        Ok(constant)
    }

    fn row(self) -> Result<Row<'a, tables::Field>, Error> {
        self.database
            .view(self.entity)
            .ok_or_else(|| Error::invalid_metadata("invalid field identity"))
    }
}

/// A database-backed semantic view of a Constant row.
#[derive(Clone, Copy)]
pub struct ConstantDefinition<'a> {
    database: &'a Database,
    entity: Entity<tables::Constant>,
}

/// A decoded Constant table value.
#[derive(Clone, Debug, PartialEq)]
pub enum ConstantValue {
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
    ISize(i64),
    USize(u64),
    F32(f32),
    F64(f64),
    String(String),
    Null,
}

impl ConstantDefinition<'_> {
    /// Returns the database identity.
    pub const fn entity(self) -> Entity<tables::Constant> {
        self.entity
    }

    /// Returns the encoded constant element type.
    pub fn element_type(self) -> Result<u16, Error> {
        self.database
            .view(self.entity)
            .ok_or_else(|| Error::invalid_metadata("invalid constant identity"))?
            .u16(0)
    }

    /// Decodes the constant value.
    pub fn value(self) -> Result<ConstantValue, Error> {
        let row = self
            .database
            .view(self.entity)
            .ok_or_else(|| Error::invalid_metadata("invalid constant identity"))?;
        let image = self.database.image(self.entity.file()).unwrap();
        let mut reader = image.blob_reader(row.blob_id(2)?)?;
        let value = match self.element_type()? as u8 {
            TYPE_BOOLEAN => match reader.read_u8()? {
                0 => ConstantValue::Boolean(false),
                1 => ConstantValue::Boolean(true),
                _ => return Err(Error::invalid(reader.offset() - 1, "invalid Boolean value")),
            },
            TYPE_CHAR => ConstantValue::Char(reader.read_u16()?),
            TYPE_I8 => ConstantValue::I8(reader.read_i8()?),
            TYPE_U8 => ConstantValue::U8(reader.read_u8()?),
            TYPE_I16 => ConstantValue::I16(reader.read_i16()?),
            TYPE_U16 => ConstantValue::U16(reader.read_u16()?),
            TYPE_I32 => ConstantValue::I32(reader.read_i32()?),
            TYPE_U32 => ConstantValue::U32(reader.read_u32()?),
            TYPE_I64 => ConstantValue::I64(reader.read_i64()?),
            TYPE_U64 => ConstantValue::U64(reader.read_u64()?),
            TYPE_ISIZE => ConstantValue::ISize(reader.read_i64()?),
            TYPE_USIZE => ConstantValue::USize(reader.read_u64()?),
            TYPE_F32 => ConstantValue::F32(reader.read_f32()?),
            TYPE_F64 => ConstantValue::F64(reader.read_f64()?),
            TYPE_STRING => {
                let offset = reader.offset();
                let length = reader.remaining();
                if length % 2 != 0 {
                    return Err(Error::invalid(offset, "UTF-16 constant has odd length"));
                }
                let bytes = reader.read_bytes(length)?;
                let units: Vec<_> = bytes
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|unit| u16::from_le_bytes(*unit))
                    .collect();
                ConstantValue::String(
                    String::from_utf16(&units)
                        .map_err(|_| Error::invalid(offset, "invalid UTF-16 constant"))?,
                )
            }
            TYPE_CLASS => {
                if reader.read_u32()? != 0 {
                    return Err(Error::invalid(
                        reader.offset() - 4,
                        "class constant is not null",
                    ));
                }
                ConstantValue::Null
            }
            _ => return Err(Error::invalid_metadata("unsupported constant element type")),
        };
        reader.finish()?;
        Ok(value)
    }
}

/// A database-backed semantic view of a MethodDef row.
#[derive(Clone, Copy)]
pub struct MethodDefinition<'a> {
    database: &'a Database,
    entity: Entity<tables::MethodDef>,
}

/// A database-backed semantic view of a CustomAttribute row.
#[derive(Clone, Copy)]
pub struct AttributeDefinition<'a> {
    database: &'a Database,
    entity: Entity<tables::CustomAttribute>,
}

impl<'a> AttributeDefinition<'a> {
    /// Returns the database identity.
    pub const fn entity(self) -> Entity<tables::CustomAttribute> {
        self.entity
    }

    /// Returns the namespace and name of the attribute type.
    pub fn type_name(self) -> Result<Option<(&'a str, &'a str)>, Error> {
        let row = self
            .database
            .view(self.entity)
            .ok_or_else(|| Error::invalid_metadata("invalid attribute identity"))?;
        let constructor = row
            .coded(1)?
            .ok_or_else(|| Error::invalid_metadata("null attribute constructor"))?;
        let ty = match constructor.table() {
            TableId::MethodDef => {
                let method = self
                    .database
                    .image(self.entity.file())
                    .unwrap()
                    .row::<tables::MethodDef>(constructor.number())
                    .ok_or_else(|| Error::invalid_metadata("invalid attribute constructor"))?;
                self.database
                    .method_owner(self.entity.file(), method)?
                    .row()
                    .number()
            }
            TableId::MemberRef => {
                let image = self.database.image(self.entity.file()).unwrap();
                let member = image
                    .row::<tables::MemberRef>(constructor.number())
                    .and_then(|row| image.view(row))
                    .ok_or_else(|| Error::invalid_metadata("invalid attribute constructor"))?;
                let Some(parent) = member.coded(0)? else {
                    return Err(Error::invalid_metadata("null attribute constructor parent"));
                };
                if parent.table() == TableId::MethodDef {
                    let method = image
                        .row::<tables::MethodDef>(parent.number())
                        .ok_or_else(|| Error::invalid_metadata("invalid constructor parent"))?;
                    self.database
                        .method_owner(self.entity.file(), method)?
                        .row()
                        .number()
                } else {
                    return self.database.type_name(self.entity.file(), parent);
                }
            }
            _ => {
                return Err(Error::invalid_metadata(
                    "invalid attribute constructor table",
                ));
            }
        };
        self.database.type_name(
            self.entity.file(),
            AnyRowId::new(TableId::TypeDef, ty).unwrap(),
        )
    }

    /// Returns the metadata attribute type name.
    pub fn name(self) -> Result<Option<&'a str>, Error> {
        Ok(self.type_name()?.map(|(_, name)| name))
    }

    /// Decodes the attribute's fixed and named arguments.
    pub fn arguments(self, resolver: &impl EnumResolver) -> Result<Vec<AttributeArgument>, Error> {
        self.database.attribute_with(self.entity, resolver)
    }
}

impl<'a> MethodDefinition<'a> {
    /// Returns the database identity.
    pub const fn entity(self) -> Entity<tables::MethodDef> {
        self.entity
    }

    /// Returns the encoded MethodAttributes flags.
    pub fn flags(self) -> Result<u16, Error> {
        self.row()?.u16(2)
    }

    /// Returns the metadata method name.
    pub fn name(self) -> Result<&'a str, Error> {
        self.row()?.string(3)
    }

    /// Returns the decoded method signature.
    pub fn signature(self) -> Result<MethodSignature, Error> {
        let row = self.row()?;
        self.database
            .image(self.entity.file())
            .unwrap()
            .method_signature(row.blob_id(4)?)
    }

    fn row(self) -> Result<Row<'a, tables::MethodDef>, Error> {
        self.database
            .view(self.entity)
            .ok_or_else(|| Error::invalid_metadata("invalid method identity"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_metadata::HasAttributes as _;

    #[test]
    fn type_shapes_match_existing_reader() {
        let database = Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap();
        let mut actual: Vec<_> = database
            .definitions()
            .map(|definition| {
                let mut attributes: Vec<_> = definition
                    .attributes()
                    .unwrap()
                    .map(|attribute| {
                        attribute
                            .type_name()
                            .unwrap()
                            .map(|(_, name)| name.to_string())
                            .unwrap()
                    })
                    .collect();
                attributes.sort();
                (
                    definition.namespace().unwrap().to_string(),
                    definition.name().unwrap().to_string(),
                    category(definition.category().unwrap()),
                    definition.flags().unwrap(),
                    definition.fields().unwrap().len(),
                    definition.methods().unwrap().len(),
                    definition.architectures().unwrap(),
                    definition
                        .fields()
                        .unwrap()
                        .filter(|field| field.constant().unwrap().is_some())
                        .count(),
                    attributes,
                )
            })
            .collect();

        let old = windows_metadata::reader::Index::new(vec![
            windows_metadata::reader::File::new(windows_default::WINRT.to_vec()).unwrap(),
            windows_metadata::reader::File::new(windows_default::WIN32.to_vec()).unwrap(),
        ]);
        let mut expected: Vec<_> = old
            .iter()
            .map(|(_, _, definition)| {
                let mut attributes: Vec<_> = definition
                    .attributes()
                    .map(|attribute| attribute.name().to_string())
                    .collect();
                attributes.sort();
                (
                    definition.namespace().to_string(),
                    definition.name().to_string(),
                    old_category(definition.category()),
                    definition.flags().0,
                    definition.fields().len(),
                    definition.methods().len(),
                    definition.arches(),
                    definition
                        .fields()
                        .filter(|field| field.constant().is_some())
                        .count(),
                    attributes,
                )
            })
            .collect();

        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn constant_values_match_existing_reader() {
        let database = Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap();
        let mut actual = Vec::new();
        for definition in database.definitions() {
            for field in definition.fields().unwrap() {
                if let Some(constant) = field.constant().unwrap() {
                    actual.push((
                        definition.namespace().unwrap().to_string(),
                        definition.name().unwrap().to_string(),
                        field.name().unwrap().to_string(),
                        constant_text(&constant.value().unwrap()),
                    ));
                }
            }
        }

        let old = windows_metadata::reader::Index::new(vec![
            windows_metadata::reader::File::new(windows_default::WINRT.to_vec()).unwrap(),
            windows_metadata::reader::File::new(windows_default::WIN32.to_vec()).unwrap(),
        ]);
        let mut expected = Vec::new();
        for (_, _, definition) in old.iter() {
            for field in definition.fields() {
                if let Some(constant) = field.constant() {
                    expected.push((
                        definition.namespace().to_string(),
                        definition.name().to_string(),
                        field.name().to_string(),
                        old_constant_text(&constant.value()),
                    ));
                }
            }
        }

        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    fn category(value: TypeCategory) -> u8 {
        match value {
            TypeCategory::Interface => 0,
            TypeCategory::Class => 1,
            TypeCategory::Enum => 2,
            TypeCategory::Delegate => 3,
            TypeCategory::Struct => 4,
            TypeCategory::Attribute => 5,
        }
    }

    fn old_category(value: windows_metadata::reader::TypeCategory) -> u8 {
        match value {
            windows_metadata::reader::TypeCategory::Interface => 0,
            windows_metadata::reader::TypeCategory::Class => 1,
            windows_metadata::reader::TypeCategory::Enum => 2,
            windows_metadata::reader::TypeCategory::Delegate => 3,
            windows_metadata::reader::TypeCategory::Struct => 4,
            windows_metadata::reader::TypeCategory::Attribute => 5,
        }
    }

    fn constant_text(value: &ConstantValue) -> String {
        format!("{value:?}")
    }

    fn old_constant_text(value: &windows_metadata::Value) -> String {
        match value {
            windows_metadata::Value::Bool(value) => format!("Boolean({value:?})"),
            windows_metadata::Value::U8(value) => format!("U8({value:?})"),
            windows_metadata::Value::I8(value) => format!("I8({value:?})"),
            windows_metadata::Value::U16(value) => format!("U16({value:?})"),
            windows_metadata::Value::I16(value) => format!("I16({value:?})"),
            windows_metadata::Value::U32(value) => format!("U32({value:?})"),
            windows_metadata::Value::I32(value) => format!("I32({value:?})"),
            windows_metadata::Value::U64(value) => format!("U64({value:?})"),
            windows_metadata::Value::I64(value) => format!("I64({value:?})"),
            windows_metadata::Value::USize(value) => format!("USize({value:?})"),
            windows_metadata::Value::ISize(value) => format!("ISize({value:?})"),
            windows_metadata::Value::F32(value) => format!("F32({value:?})"),
            windows_metadata::Value::F64(value) => format!("F64({value:?})"),
            windows_metadata::Value::Utf16(value) => format!("String({value:?})"),
            rest => panic!("unexpected constant value {rest:?}"),
        }
    }
}

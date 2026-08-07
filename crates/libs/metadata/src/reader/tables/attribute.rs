use super::*;

impl std::fmt::Debug for Attribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("Attribute")
            .field(&self.ctor().parent().name())
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttributeValueError {
    offset: usize,
    message: &'static str,
    unsupported: bool,
}

impl AttributeValueError {
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn is_unsupported(&self) -> bool {
        self.unsupported
    }
}

impl std::fmt::Display for AttributeValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} at byte {}", self.message, self.offset)
    }
}

impl std::error::Error for AttributeValueError {}

impl<'a> Attribute<'a> {
    pub fn name(&self) -> &'a str {
        self.ctor().parent().name()
    }

    pub fn namespace(&self) -> &'a str {
        self.ctor().parent().namespace()
    }

    pub fn parent(&self) -> HasAttribute<'a> {
        self.decode(0)
    }

    pub fn ctor(&self) -> AttributeType<'a> {
        self.decode(1)
    }

    /// Returns the encoded custom-attribute value blob.
    pub fn value_blob(&self) -> &'a [u8] {
        self.file().blob(self.pos(), Self::TABLE, 2)
    }

    pub fn value(&self) -> Vec<(String, Value)> {
        self.try_value_impl(None, true).unwrap()
    }

    pub fn try_value(&self) -> Result<Vec<(String, Value)>, AttributeValueError> {
        self.try_value_impl(None, false)
    }

    pub fn try_value_with_references(
        &self,
        references: &Index,
    ) -> Result<Vec<(String, Value)>, AttributeValueError> {
        self.try_value_impl(Some(references), false)
    }

    fn try_value_impl(
        &self,
        references: Option<&Index>,
        assume_i32_enums: bool,
    ) -> Result<Vec<(String, Value)>, AttributeValueError> {
        let signature = self.ctor().signature(&[]);
        let mut values = Vec::with_capacity(signature.types.len());
        let mut blob = AttributeBlob::new(
            self.value_blob(),
            self.to_row().index,
            references,
            assume_i32_enums,
        );

        if blob.read_u16()? != 1 {
            return Err(blob.invalid_at(0, "invalid custom-attribute prolog"));
        }

        for ty in &signature.types {
            let value = blob.read_value(ty)?;
            values.push((String::new(), value));
        }

        let named_arg_count = blob.read_u16()?;
        values.reserve(named_arg_count as usize);

        for _ in 0..named_arg_count {
            let offset = blob.offset;
            if !matches!(blob.read_u8()?, 0x53 | 0x54) {
                return Err(blob.invalid_at(offset, "invalid named-argument tag"));
            }
            let ty = blob.read_type()?;
            let name = blob.read_string("null named-argument name")?;
            let value = blob.read_value(&ty)?;
            values.push((name, value));
        }

        if blob.offset != blob.bytes.len() {
            return Err(blob.invalid("trailing custom-attribute data"));
        }

        Ok(values)
    }
}

struct AttributeBlob<'a, 'r> {
    bytes: &'a [u8],
    index: &'a Index,
    references: Option<&'r Index>,
    assume_i32_enums: bool,
    offset: usize,
}

impl<'a, 'r> AttributeBlob<'a, 'r> {
    fn new(
        bytes: &'a [u8],
        index: &'a Index,
        references: Option<&'r Index>,
        assume_i32_enums: bool,
    ) -> Self {
        Self {
            bytes,
            index,
            references,
            assume_i32_enums,
            offset: 0,
        }
    }

    fn invalid(&self, message: &'static str) -> AttributeValueError {
        self.invalid_at(self.offset, message)
    }

    fn invalid_at(&self, offset: usize, message: &'static str) -> AttributeValueError {
        AttributeValueError {
            offset,
            message,
            unsupported: false,
        }
    }

    fn unsupported(&self, message: &'static str) -> AttributeValueError {
        AttributeValueError {
            offset: self.offset,
            message,
            unsupported: true,
        }
    }

    fn read<const N: usize>(&mut self) -> Result<[u8; N], AttributeValueError> {
        let Some(bytes) = self.bytes.get(self.offset..self.offset + N) else {
            return Err(self.invalid("truncated custom-attribute value"));
        };
        self.offset += N;
        Ok(bytes.try_into().unwrap())
    }

    fn read_u8(&mut self) -> Result<u8, AttributeValueError> {
        Ok(u8::from_le_bytes(self.read()?))
    }

    fn read_i8(&mut self) -> Result<i8, AttributeValueError> {
        Ok(i8::from_le_bytes(self.read()?))
    }

    fn read_u16(&mut self) -> Result<u16, AttributeValueError> {
        Ok(u16::from_le_bytes(self.read()?))
    }

    fn read_i16(&mut self) -> Result<i16, AttributeValueError> {
        Ok(i16::from_le_bytes(self.read()?))
    }

    fn read_u32(&mut self) -> Result<u32, AttributeValueError> {
        Ok(u32::from_le_bytes(self.read()?))
    }

    fn read_i32(&mut self) -> Result<i32, AttributeValueError> {
        Ok(i32::from_le_bytes(self.read()?))
    }

    fn read_u64(&mut self) -> Result<u64, AttributeValueError> {
        Ok(u64::from_le_bytes(self.read()?))
    }

    fn read_i64(&mut self) -> Result<i64, AttributeValueError> {
        Ok(i64::from_le_bytes(self.read()?))
    }

    fn read_f32(&mut self) -> Result<f32, AttributeValueError> {
        Ok(f32::from_le_bytes(self.read()?))
    }

    fn read_f64(&mut self) -> Result<f64, AttributeValueError> {
        Ok(f64::from_le_bytes(self.read()?))
    }

    fn read_compressed(&mut self) -> Result<Option<usize>, AttributeValueError> {
        let first = self.read_u8()?;
        if first == 0xff {
            return Ok(None);
        }
        if first & 0x80 == 0 {
            return Ok(Some(first.into()));
        }
        if first & 0xc0 == 0x80 {
            let second = self.read_u8()?;
            return Ok(Some((((first & 0x3f) as usize) << 8) | second as usize));
        }
        if first & 0xe0 == 0xc0 {
            let rest = self.read::<3>()?;
            return Ok(Some(
                (((first & 0x1f) as usize) << 24)
                    | ((rest[0] as usize) << 16)
                    | ((rest[1] as usize) << 8)
                    | rest[2] as usize,
            ));
        }
        Err(self.invalid_at(self.offset - 1, "invalid compressed integer"))
    }

    fn read_string(&mut self, null: &'static str) -> Result<String, AttributeValueError> {
        let offset = self.offset;
        let Some(len) = self.read_compressed()? else {
            return Err(self.unsupported(null));
        };
        let Some(bytes) = self.bytes.get(self.offset..self.offset + len) else {
            return Err(self.invalid("truncated custom-attribute string"));
        };
        let value = std::str::from_utf8(bytes)
            .map_err(|_| self.invalid_at(offset, "invalid UTF-8 string"))?
            .to_string();
        self.offset += len;
        Ok(value)
    }

    fn read_type(&mut self) -> Result<Type, AttributeValueError> {
        let offset = self.offset;
        Ok(match self.read_u8()? {
            ELEMENT_TYPE_BOOLEAN => Type::Bool,
            ELEMENT_TYPE_CHAR => Type::Char,
            ELEMENT_TYPE_I1 => Type::I8,
            ELEMENT_TYPE_U1 => Type::U8,
            ELEMENT_TYPE_I2 => Type::I16,
            ELEMENT_TYPE_U2 => Type::U16,
            ELEMENT_TYPE_I4 => Type::I32,
            ELEMENT_TYPE_U4 => Type::U32,
            ELEMENT_TYPE_I8 => Type::I64,
            ELEMENT_TYPE_U8 => Type::U64,
            ELEMENT_TYPE_R4 => Type::F32,
            ELEMENT_TYPE_R8 => Type::F64,
            ELEMENT_TYPE_STRING => Type::String,
            ELEMENT_TYPE_OBJECT => Type::Object,
            ELEMENT_TYPE_SZARRAY => Type::Array(Box::new(self.read_type()?)),
            0x50 => Type::ClassName(TypeName::named("System", "Type")),
            0x55 => {
                let name = self.read_string("null enum type name")?;
                Type::ValueName(type_name(&name))
            }
            _ => return Err(self.invalid_at(offset, "invalid named-argument type")),
        })
    }

    fn read_value(&mut self, ty: &Type) -> Result<Value, AttributeValueError> {
        match ty {
            Type::Bool => match self.read_u8()? {
                0 => Ok(Value::Bool(false)),
                1 => Ok(Value::Bool(true)),
                _ => Err(self.invalid_at(self.offset - 1, "invalid Boolean value")),
            },
            Type::Char => Ok(Value::Char(self.read_u16()?)),
            Type::I8 => Ok(Value::I8(self.read_i8()?)),
            Type::U8 => Ok(Value::U8(self.read_u8()?)),
            Type::I16 => Ok(Value::I16(self.read_i16()?)),
            Type::U16 => Ok(Value::U16(self.read_u16()?)),
            Type::I32 => Ok(Value::I32(self.read_i32()?)),
            Type::U32 => Ok(Value::U32(self.read_u32()?)),
            Type::I64 => Ok(Value::I64(self.read_i64()?)),
            Type::U64 => Ok(Value::U64(self.read_u64()?)),
            Type::F32 => Ok(Value::F32(self.read_f32()?)),
            Type::F64 => Ok(Value::F64(self.read_f64()?)),
            Type::String => Ok(Value::Utf8(
                self.read_string("null strings are not represented")?,
            )),
            Type::ClassName(tn) if tn == ("System", "Type") => Ok(Value::TypeName(type_name(
                &self.read_string("null type names are not represented")?,
            ))),
            Type::ValueName(tn) | Type::ClassName(tn) => {
                let ty = if let Some(ty) = self.enum_underlying_type(tn) {
                    ty
                } else if self.assume_i32_enums {
                    Type::I32
                } else {
                    return Err(self.unsupported("enum backing type is unavailable"));
                };
                let value = match ty {
                    Type::I8 => Value::I8(self.read_i8()?),
                    Type::U8 => Value::U8(self.read_u8()?),
                    Type::I16 => Value::I16(self.read_i16()?),
                    Type::U16 => Value::U16(self.read_u16()?),
                    Type::I32 => Value::I32(self.read_i32()?),
                    Type::U32 => Value::U32(self.read_u32()?),
                    Type::I64 => Value::I64(self.read_i64()?),
                    Type::U64 => Value::U64(self.read_u64()?),
                    _ => return Err(self.invalid("invalid enum backing type")),
                };
                Ok(Value::EnumValue(tn.clone(), Box::new(value)))
            }
            Type::Object => Err(self.unsupported("boxed values are not represented")),
            Type::Array(_) => Err(self.unsupported("array values are not represented")),
            _ => Err(self.invalid("invalid custom-attribute parameter type")),
        }
    }

    fn enum_underlying_type(&self, name: &TypeName) -> Option<Type> {
        enum_underlying_type(self.index, name)
            .or_else(|| enum_underlying_type(self.references?, name))
    }
}

fn enum_underlying_type(index: &Index, name: &TypeName) -> Option<Type> {
    let mut definitions = index.get(&name.namespace, &name.name);
    let definition = definitions.next()?;
    definitions
        .next()
        .is_none()
        .then(|| definition.underlying_type())?
}

fn type_name(name: &str) -> TypeName {
    if let Some(dot) = name.rfind('.') {
        TypeName::named(&name[..dot], &name[dot + 1..])
    } else {
        TypeName::named("", name)
    }
}

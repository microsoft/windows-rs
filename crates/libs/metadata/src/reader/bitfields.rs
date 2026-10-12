use super::*;

/// Logical bit-field evidence on one backing field; no member here allocates storage.
#[derive(Debug, PartialEq)]
pub struct Bitfield {
    pub name: String,
    pub offset: u32,
    pub width: u32,
    pub logical_type: Option<Type>,
    pub annotations: Vec<BitfieldAnnotation>,
}

#[derive(Debug, PartialEq)]
pub struct BitfieldAnnotation {
    pub source: String,
    pub ordinal: i32,
    pub payload: String,
}

#[derive(Debug)]
pub struct BitfieldError(String);

impl std::fmt::Display for BitfieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for BitfieldError {}

/// Maps supported logical scalar leaves to CLR type-valued attribute identities.
pub fn bitfield_type_name(ty: &Type) -> Option<TypeName> {
    let name = match ty {
        Type::Bool => "Boolean",
        Type::Char => "Char",
        Type::I8 => "SByte",
        Type::U8 => "Byte",
        Type::I16 => "Int16",
        Type::U16 => "UInt16",
        Type::I32 => "Int32",
        Type::U32 => "UInt32",
        Type::I64 => "Int64",
        Type::U64 => "UInt64",
        Type::ISize => "IntPtr",
        Type::USize => "UIntPtr",
        Type::ValueName(name) if name.generics.is_empty() => return Some(name.clone()),
        _ => return None,
    };
    Some(TypeName::named("System", name))
}

fn logical_type(name: &TypeName) -> Type {
    if name.namespace == "System" {
        match name.name.as_str() {
            "Boolean" => return Type::Bool,
            "Char" => return Type::Char,
            "SByte" => return Type::I8,
            "Byte" => return Type::U8,
            "Int16" => return Type::I16,
            "UInt16" => return Type::U16,
            "Int32" => return Type::I32,
            "UInt32" => return Type::U32,
            "Int64" => return Type::I64,
            "UInt64" => return Type::U64,
            "IntPtr" => return Type::ISize,
            "UIntPtr" => return Type::USize,
            "Single" => return Type::F32,
            "Double" => return Type::F64,
            "String" => return Type::String,
            "Object" => return Type::Object,
            "Void" => return Type::Void,
            _ => {}
        }
    }
    Type::ValueName(name.clone())
}

fn bitfield_attribute(attr: Attribute<'_>) -> bool {
    attr.namespace() == "Windows.Win32.Metadata"
        && matches!(
            attr.name(),
            "NativeBitfieldAttribute"
                | "NativeBitfieldTypeAttribute"
                | "NativeBitfieldQualifierAttribute"
                | "NativeBitfieldAnnotationAttribute"
        )
}

impl Field<'_> {
    /// Decodes logical members and checks descriptor identities and nonoverlapping storage ranges.
    pub fn bitfields(&self) -> Result<Vec<Bitfield>, BitfieldError> {
        let mut members = BTreeMap::new();
        let mut types = BTreeMap::new();
        let mut qualifiers: BTreeMap<String, BTreeMap<i32, i32>> = BTreeMap::new();
        let mut annotations: BTreeMap<String, BTreeMap<(String, i32), String>> = BTreeMap::new();
        for attr in self.attributes().filter(|attr| bitfield_attribute(*attr)) {
            let args = attr.value();
            if args.iter().any(|(name, _)| !name.is_empty()) {
                return invalid("bit-field descriptor arguments must be positional");
            }
            let values: Vec<_> = args.into_iter().map(|(_, value)| value).collect();
            let Some(Value::Utf8(name)) = values.first() else {
                return invalid("bit-field descriptor requires a member name");
            };
            if name.is_empty() || name == "_" {
                return invalid("bit-field descriptor cannot name padding");
            }
            match (attr.name(), values.as_slice()) {
                (
                    "NativeBitfieldAttribute",
                    [Value::Utf8(_), Value::I64(offset), Value::I64(width)],
                ) if *offset >= 0 && *width > 0 => {
                    let (Ok(offset), Ok(width)) = (u32::try_from(*offset), u32::try_from(*width))
                    else {
                        return invalid("bit-field range exceeds u32");
                    };
                    let member = Bitfield {
                        name: name.clone(),
                        offset,
                        width,
                        logical_type: None,
                        annotations: Vec::new(),
                    };
                    if members.insert(name.clone(), member).is_some() {
                        return invalid("duplicate bit-field member");
                    }
                }
                ("NativeBitfieldTypeAttribute", [Value::Utf8(_), Value::TypeName(ty)])
                    if ty.generics.is_empty() && !ty.name.is_empty() =>
                {
                    if types.insert(name.clone(), logical_type(ty)).is_some() {
                        return invalid("duplicate bit-field logical type");
                    }
                }
                (
                    "NativeBitfieldQualifierAttribute",
                    [Value::Utf8(_), Value::I32(ordinal), Value::I32(qualifier)],
                ) if *ordinal >= 0 && matches!(qualifier, 1 | 2) => {
                    if qualifiers
                        .entry(name.clone())
                        .or_default()
                        .insert(*ordinal, *qualifier)
                        .is_some()
                    {
                        return invalid("duplicate bit-field qualifier ordinal");
                    }
                }
                (
                    "NativeBitfieldAnnotationAttribute",
                    [
                        Value::Utf8(_),
                        Value::Utf8(source),
                        Value::I32(ordinal),
                        Value::Utf8(payload),
                    ],
                ) if !source.is_empty() && *ordinal >= 0 && !payload.is_empty() => {
                    if annotations
                        .entry(name.clone())
                        .or_default()
                        .insert((source.clone(), *ordinal), payload.clone())
                        .is_some()
                    {
                        return invalid("duplicate bit-field annotation identity");
                    }
                }
                _ => return invalid("invalid bit-field descriptor arguments"),
            }
        }
        for name in types
            .keys()
            .chain(qualifiers.keys())
            .chain(annotations.keys())
        {
            if !members.contains_key(name) {
                return invalid("bit-field descriptor has no matching member");
            }
        }
        for (name, member) in &mut members {
            member.logical_type = types.remove(name);
            if let Some(qualifiers) = qualifiers.remove(name) {
                let Some(mut ty) = member.logical_type.take() else {
                    return invalid("bit-field qualifiers require a logical type");
                };
                if qualifiers
                    .keys()
                    .enumerate()
                    .any(|(index, ordinal)| usize::try_from(*ordinal) != Ok(index))
                {
                    return invalid("bit-field qualifier ordinals must be contiguous from zero");
                }
                for (_, qualifier) in qualifiers.into_iter().rev() {
                    ty = if qualifier == 1 {
                        Type::Const(Box::new(ty))
                    } else {
                        Type::Volatile(Box::new(ty))
                    };
                }
                member.logical_type = Some(ty);
            }
            if let Some(evidence) = annotations.remove(name) {
                member.annotations = evidence
                    .into_iter()
                    .map(|((source, ordinal), payload)| BitfieldAnnotation {
                        source,
                        ordinal,
                        payload,
                    })
                    .collect();
            }
        }
        let mut members: Vec<_> = members.into_values().collect();
        members.sort_by_key(|member| member.offset);
        let mut cursor = 0;
        let capacity = match self.ty().unqualified() {
            Type::I8 | Type::U8 | Type::Bool => Some(8),
            Type::I16 | Type::U16 | Type::Char => Some(16),
            Type::I32 | Type::U32 => Some(32),
            Type::I64 | Type::U64 => Some(64),
            _ => None,
        };
        for member in &members {
            if member.offset < cursor {
                return invalid("overlapping bit-field ranges");
            }
            cursor = member
                .offset
                .checked_add(member.width)
                .ok_or_else(|| BitfieldError("bit-field range overflows u32".to_string()))?;
            if capacity.is_some_and(|capacity| cursor > capacity) {
                return invalid("bit-field range exceeds backing storage");
            }
        }
        Ok(members)
    }
}

impl Index {
    /// Checks bit-field descriptors and known logical types without calculating native layout.
    pub fn validate_native_bitfields(&self, reference: Option<&Self>) -> Result<(), BitfieldError> {
        let mut fields = BTreeSet::new();
        for attr in self
            .all_attributes()
            .filter(|attr| bitfield_attribute(*attr))
        {
            let HasAttribute::Field(field) = attr.parent() else {
                return invalid("bit-field descriptor requires a backing field");
            };
            fields.insert(field);
        }
        for field in fields {
            let declaration: TypeDef<'_> = field.parent_row(4);
            if declaration.flags().contains(TypeAttributes::WindowsRuntime) {
                return invalid("native bit-fields are not valid on WinRT declarations");
            }
            if declaration.category() != TypeCategory::Struct
                || declaration.attributes().any(|attr| {
                    attr.namespace() == "Windows.Win32.Metadata"
                        && attr.name() == "NativeTypedefAttribute"
                })
                || field.flags().contains(FieldAttributes::Static)
                || field.flags().contains(FieldAttributes::Literal)
            {
                return invalid(
                    "bit-field backing storage requires a native record instance field",
                );
            }
            let members = field.bitfields()?;
            let arches = if field.arches() == 0 {
                declaration.arches()
            } else if declaration.arches() == 0 {
                field.arches()
            } else {
                declaration.arches() & field.arches()
            };
            if !members.is_empty() {
                let capacity =
                    self.check_bitfield_type(&field.ty(), arches, reference, &mut Vec::new())?;
                if capacity.is_some_and(|bits| {
                    members
                        .iter()
                        .any(|member| member.offset + member.width > bits)
                }) {
                    return invalid("bit-field range exceeds backing storage");
                }
            }
            for member in members {
                if let Some(ty) = member.logical_type {
                    self.check_bitfield_type(&ty, arches, reference, &mut Vec::new())?;
                }
            }
        }
        Ok(())
    }

    fn check_bitfield_type(
        &self,
        ty: &Type,
        arches: i32,
        reference: Option<&Self>,
        visiting: &mut Vec<TypeName>,
    ) -> Result<Option<u32>, BitfieldError> {
        match ty.unqualified() {
            Type::Bool | Type::I8 | Type::U8 => Ok(Some(8)),
            Type::Char | Type::I16 | Type::U16 => Ok(Some(16)),
            Type::I32 | Type::U32 => Ok(Some(32)),
            Type::I64 | Type::U64 => Ok(Some(64)),
            Type::ISize | Type::USize => Ok(None),
            Type::ValueName(name) if name.generics.is_empty() => {
                if visiting.contains(name) {
                    return invalid("cyclic bit-field logical type");
                }
                visiting.push(name.clone());
                let mut bits: Option<u32> = None;
                for definition in self.get(&name.namespace, &name.name).chain(
                    reference
                        .into_iter()
                        .flat_map(|index| index.get(&name.namespace, &name.name)),
                ) {
                    let definition_arches = definition.arches();
                    if arches != 0 && definition_arches != 0 && arches & definition_arches == 0 {
                        continue;
                    }
                    if definition.category() == TypeCategory::Enum
                        || definition.attributes().any(|attr| {
                            attr.namespace() == "Windows.Win32.Metadata"
                                && attr.name() == "NativeTypedefAttribute"
                        })
                    {
                        let Some(underlying) = definition.underlying_type() else {
                            return invalid("bit-field logical type has no scalar target");
                        };
                        let target_arches = if arches == 0 {
                            definition_arches
                        } else if definition_arches == 0 {
                            arches
                        } else {
                            arches & definition_arches
                        };
                        if let Some(capacity) = self.check_bitfield_type(
                            &underlying,
                            target_arches,
                            reference,
                            visiting,
                        )? {
                            bits = Some(bits.map_or(capacity, |bits| bits.min(capacity)));
                        }
                    } else {
                        return invalid("bit-field logical type must be integral or enum");
                    }
                }
                visiting.pop();
                Ok(bits)
            }
            _ => invalid("bit-field logical type must be integral or enum"),
        }
    }
}

fn invalid<T>(message: &str) -> Result<T, BitfieldError> {
    Err(BitfieldError(message.to_string()))
}

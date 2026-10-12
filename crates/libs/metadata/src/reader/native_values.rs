use super::*;

/// A semantic initializer, not a native object-memory image.
#[derive(Debug, Clone, PartialEq)]
pub enum NativeValue {
    Scalar(Value),
    Array(Vec<Self>),
    Record(Vec<(String, Self)>),
    String {
        encoding: NativeStringEncoding,
        units: Vec<u32>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeStringEncoding {
    Narrow,
    Utf8,
    Utf16,
    Utf32,
}

#[derive(Debug)]
pub struct NativeValueError(String);

impl std::fmt::Display for NativeValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for NativeValueError {}

fn invalid<T>(message: &str) -> Result<T, NativeValueError> {
    Err(NativeValueError(message.to_string()))
}

fn value_attribute(attr: Attribute<'_>) -> bool {
    attr.namespace() == "Windows.Win32.Metadata" && attr.name() == "NativeConstantAttribute"
}

impl Field<'_> {
    pub fn has_native_value(&self) -> bool {
        self.attributes().any(value_attribute)
    }

    pub fn native_value(&self) -> Result<Option<NativeValue>, NativeValueError> {
        let mut attrs = self.attributes().filter(|attr| value_attribute(*attr));
        let Some(attr) = attrs.next() else {
            return Ok(None);
        };
        if attrs.next().is_some() {
            return invalid("duplicate native constant descriptor");
        }
        let args = attr.value();
        let [
            (version_name, Value::I32(1)),
            (payload_name, Value::Utf8(payload)),
        ] = args.as_slice()
        else {
            return invalid("native constant requires version 1 and a string payload");
        };
        if !version_name.is_empty() || !payload_name.is_empty() {
            return invalid("native constant arguments must be positional");
        }
        NativeValue::from_payload(payload).map(Some)
    }
}

impl NativeValue {
    /// Encodes version 1's little-endian semantic tree as lowercase hexadecimal.
    pub fn to_payload(&self) -> Result<String, NativeValueError> {
        let mut bytes = Vec::new();
        self.encode(&mut bytes, 0)?;
        use std::fmt::Write;
        let mut payload = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            write!(payload, "{byte:02x}").unwrap();
        }
        Ok(payload)
    }

    pub fn from_payload(payload: &str) -> Result<Self, NativeValueError> {
        if !payload.len().is_multiple_of(2) {
            return invalid("native value payload has an odd hexadecimal length");
        }
        let mut bytes = Vec::with_capacity(payload.len() / 2);
        for pair in payload.as_bytes().as_chunks::<2>().0 {
            let digit = |byte| match byte {
                b'0'..=b'9' => Ok(byte - b'0'),
                b'a'..=b'f' => Ok(byte - b'a' + 10),
                _ => invalid("native value payload requires lowercase hexadecimal"),
            };
            bytes.push(digit(pair[0])? * 16 + digit(pair[1])?);
        }
        let mut input = ValueInput(&bytes);
        let value = input.value(0)?;
        if !input.0.is_empty() {
            return invalid("native value payload has trailing bytes");
        }
        Ok(value)
    }

    fn encode(&self, bytes: &mut Vec<u8>, depth: usize) -> Result<(), NativeValueError> {
        if depth > 64 {
            return invalid("native value nesting exceeds 64");
        }
        let count = |bytes: &mut Vec<u8>, count: usize| {
            let count = u32::try_from(count)
                .map_err(|_| NativeValueError("native value count exceeds u32".into()))?;
            bytes.extend(count.to_le_bytes());
            Ok::<_, NativeValueError>(())
        };
        match self {
            Self::Scalar(value) => {
                macro_rules! scalar {
                    ($tag:expr, $value:expr) => {{
                        bytes.push($tag);
                        bytes.extend($value.to_le_bytes());
                    }};
                }
                match value {
                    Value::Bool(value) => bytes.extend([1, u8::from(*value)]),
                    Value::U8(value) => scalar!(2, value),
                    Value::I8(value) => scalar!(3, value),
                    Value::U16(value) => scalar!(4, value),
                    Value::I16(value) => scalar!(5, value),
                    Value::U32(value) => scalar!(6, value),
                    Value::I32(value) => scalar!(7, value),
                    Value::U64(value) => scalar!(8, value),
                    Value::I64(value) => scalar!(9, value),
                    Value::USize(value) => scalar!(10, value),
                    Value::ISize(value) => scalar!(11, value),
                    Value::F32(value) => scalar!(12, value.to_bits()),
                    Value::F64(value) => scalar!(13, value.to_bits()),
                    _ => return invalid("unsupported native scalar value"),
                }
            }
            Self::Array(values) => {
                bytes.push(20);
                count(bytes, values.len())?;
                for value in values {
                    value.encode(bytes, depth + 1)?;
                }
            }
            Self::Record(fields) => {
                bytes.push(21);
                count(bytes, fields.len())?;
                let mut names = BTreeSet::new();
                for (name, value) in fields {
                    if name.is_empty() || name.contains('\0') || !names.insert(name) {
                        return invalid("native record member names must be nonempty and unique");
                    }
                    count(bytes, name.len())?;
                    bytes.extend(name.as_bytes());
                    value.encode(bytes, depth + 1)?;
                }
            }
            Self::String { encoding, units } => {
                bytes.push(22);
                let (kind, maximum) = match encoding {
                    NativeStringEncoding::Narrow => (0, 0xff),
                    NativeStringEncoding::Utf8 => (1, 0xff),
                    NativeStringEncoding::Utf16 => (2, 0xffff),
                    NativeStringEncoding::Utf32 => (3, u32::MAX),
                };
                bytes.push(kind);
                count(bytes, units.len())?;
                for unit in units {
                    if *unit > maximum {
                        return invalid("native string code unit exceeds its encoding width");
                    }
                    match encoding {
                        NativeStringEncoding::Narrow | NativeStringEncoding::Utf8 => {
                            bytes.push(*unit as u8);
                        }
                        NativeStringEncoding::Utf16 => bytes.extend((*unit as u16).to_le_bytes()),
                        NativeStringEncoding::Utf32 => bytes.extend(unit.to_le_bytes()),
                    }
                }
            }
        }
        Ok(())
    }
}

struct ValueInput<'a>(&'a [u8]);

impl ValueInput<'_> {
    fn take<const N: usize>(&mut self) -> Result<[u8; N], NativeValueError> {
        if self.0.len() < N {
            return invalid("truncated native value payload");
        }
        let (value, rest) = self.0.split_at(N);
        self.0 = rest;
        Ok(value.try_into().unwrap())
    }

    fn count(&mut self) -> Result<usize, NativeValueError> {
        let count = u32::from_le_bytes(self.take()?) as usize;
        if count > self.0.len() {
            return invalid("native value count exceeds remaining payload");
        }
        Ok(count)
    }

    fn value(&mut self, depth: usize) -> Result<NativeValue, NativeValueError> {
        if depth > 64 {
            return invalid("native value nesting exceeds 64");
        }
        use NativeValue::*;
        let value = match self.take::<1>()?[0] {
            1 => match self.take::<1>()?[0] {
                0 => Scalar(Value::Bool(false)),
                1 => Scalar(Value::Bool(true)),
                _ => return invalid("invalid native boolean"),
            },
            2 => Scalar(Value::U8(u8::from_le_bytes(self.take()?))),
            3 => Scalar(Value::I8(i8::from_le_bytes(self.take()?))),
            4 => Scalar(Value::U16(u16::from_le_bytes(self.take()?))),
            5 => Scalar(Value::I16(i16::from_le_bytes(self.take()?))),
            6 => Scalar(Value::U32(u32::from_le_bytes(self.take()?))),
            7 => Scalar(Value::I32(i32::from_le_bytes(self.take()?))),
            8 => Scalar(Value::U64(u64::from_le_bytes(self.take()?))),
            9 => Scalar(Value::I64(i64::from_le_bytes(self.take()?))),
            10 => Scalar(Value::USize(u64::from_le_bytes(self.take()?))),
            11 => Scalar(Value::ISize(i64::from_le_bytes(self.take()?))),
            12 => Scalar(Value::F32(f32::from_bits(u32::from_le_bytes(self.take()?)))),
            13 => Scalar(Value::F64(f64::from_bits(u64::from_le_bytes(self.take()?)))),
            20 => {
                let count = self.count()?;
                let values = (0..count)
                    .map(|_| self.value(depth + 1))
                    .collect::<Result<_, _>>()?;
                Array(values)
            }
            21 => {
                let count = self.count()?;
                let mut fields = Vec::new();
                let mut names = BTreeSet::new();
                for _ in 0..count {
                    let count = self.count()?;
                    let (name, rest) = self.0.split_at(count);
                    let name = std::str::from_utf8(name)
                        .map_err(|_| NativeValueError("invalid native member name UTF-8".into()))?
                        .to_string();
                    self.0 = rest;
                    if name.is_empty() || name.contains('\0') || !names.insert(name.clone()) {
                        return invalid("native record member names must be nonempty and unique");
                    }
                    fields.push((name, self.value(depth + 1)?));
                }
                Record(fields)
            }
            22 => {
                let encoding = match self.take::<1>()?[0] {
                    0 => NativeStringEncoding::Narrow,
                    1 => NativeStringEncoding::Utf8,
                    2 => NativeStringEncoding::Utf16,
                    3 => NativeStringEncoding::Utf32,
                    _ => return invalid("unknown native string encoding"),
                };
                let count = self.count()?;
                let mut units = Vec::new();
                for _ in 0..count {
                    units.push(match encoding {
                        NativeStringEncoding::Narrow | NativeStringEncoding::Utf8 => {
                            u8::from_le_bytes(self.take()?) as u32
                        }
                        NativeStringEncoding::Utf16 => u16::from_le_bytes(self.take()?) as u32,
                        NativeStringEncoding::Utf32 => u32::from_le_bytes(self.take()?),
                    });
                }
                String { encoding, units }
            }
            _ => return invalid("unknown native value tag"),
        };
        Ok(value)
    }
}

impl Index {
    pub fn validate_native_values(&self, reference: Option<&Self>) -> Result<(), NativeValueError> {
        let mut fields = BTreeSet::new();
        for attr in self.all_attributes().filter(|attr| value_attribute(*attr)) {
            let HasAttribute::Field(field) = attr.parent() else {
                return invalid("native constant descriptor requires a field");
            };
            fields.insert(field);
        }
        for field in fields {
            let owner: TypeDef<'_> = field.parent_row(4);
            if owner.name() != "Apis"
                || owner.namespace().is_empty()
                || owner.category() != TypeCategory::Class
                || owner.flags().contains(TypeAttributes::WindowsRuntime)
                || owner.generic_params().next().is_some()
                || owner.arches() != 0
            {
                return invalid("native values require a neutral native Apis container");
            }
            if field.flags() != FieldAttributes::Public | FieldAttributes::Static
                || field.name().is_empty()
                || field.constant().is_some()
                || field.is_imported_data()
                || field.attributes().any(|attr| {
                    attr.name() == "GuidAttribute"
                        && matches!(
                            attr.namespace(),
                            "Windows.Win32.Metadata" | "Windows.Foundation.Metadata"
                        )
                })
            {
                return invalid(
                    "native constant conflicts with scalar, GUID, import, or storage flags",
                );
            }
            let value = field.native_value()?.unwrap();
            self.check_native_value(
                &field.ty(),
                &value,
                field.arches(),
                reference,
                &mut Vec::new(),
                0,
            )?;
        }
        Ok(())
    }

    fn check_native_value(
        &self,
        ty: &Type,
        value: &NativeValue,
        arches: i32,
        reference: Option<&Self>,
        visiting: &mut Vec<TypeName>,
        depth: usize,
    ) -> Result<(), NativeValueError> {
        if depth > 64 {
            return invalid("native value type nesting exceeds 64");
        }
        let ty = ty.unqualified();
        if let Type::ValueName(name) = ty {
            if visiting.contains(name) {
                return invalid("cyclic native value type");
            }
            let definitions: Vec<_> = self
                .get(&name.namespace, &name.name)
                .chain(
                    reference
                        .into_iter()
                        .flat_map(|r| r.get(&name.namespace, &name.name)),
                )
                .filter(|def| arches == 0 || def.arches() == 0 || def.arches() & arches != 0)
                .collect();
            if definitions.is_empty() {
                return invalid("native value requires an available type definition");
            }
            visiting.push(name.clone());
            for def in definitions {
                if def.flags().contains(TypeAttributes::WindowsRuntime)
                    || def.generic_params().next().is_some()
                    || def.is_opaque()
                    || !matches!(def.category(), TypeCategory::Struct | TypeCategory::Enum)
                {
                    return invalid("native value requires a complete nongeneric native type");
                }
                let target_arches = if arches == 0 {
                    def.arches()
                } else if def.arches() == 0 {
                    arches
                } else {
                    arches & def.arches()
                };
                if def.attributes().any(|attr| {
                    attr.namespace() == "Windows.Win32.Metadata"
                        && attr.name() == "NativeTypedefAttribute"
                }) {
                    let fields: Vec<_> = def.fields().collect();
                    if fields.len() != 1 || def.category() != TypeCategory::Struct {
                        return invalid("native value typedef requires one target");
                    }
                    self.check_native_value(
                        &fields[0].ty(),
                        value,
                        target_arches,
                        reference,
                        visiting,
                        depth + 1,
                    )?;
                } else if def.category() == TypeCategory::Enum {
                    let Some(underlying) = def.underlying_type() else {
                        return invalid("native value enum has no underlying type");
                    };
                    self.check_native_value(
                        &underlying,
                        value,
                        target_arches,
                        reference,
                        visiting,
                        depth + 1,
                    )?;
                } else {
                    if def.category() != TypeCategory::Struct
                        || def.flags().contains(TypeAttributes::ExplicitLayout)
                    {
                        return invalid("native aggregate requires a non-union record");
                    }
                    let NativeValue::Record(values) = value else {
                        return invalid("native record requires member values");
                    };
                    let fields: Vec<_> = def
                        .fields()
                        .filter(|field| !field.flags().contains(FieldAttributes::Static))
                        .collect();
                    if fields.len() != values.len()
                        || fields
                            .iter()
                            .zip(values)
                            .any(|(field, (name, _))| field.name() != name)
                    {
                        return invalid(
                            "native record values must cover members in declaration order",
                        );
                    }
                    for (field, (_, value)) in fields.iter().zip(values) {
                        if field.attributes().any(|attr| {
                            attr.namespace() == "Windows.Win32.Metadata"
                                && attr.name() == "NativeBitfieldAttribute"
                        }) {
                            return invalid(
                                "native bit-field aggregate values require logical member contracts",
                            );
                        }
                        self.check_native_value(
                            &field.ty(),
                            value,
                            target_arches,
                            reference,
                            visiting,
                            depth + 1,
                        )?;
                    }
                }
            }
            visiting.pop();
            return Ok(());
        }
        match value {
            NativeValue::Scalar(value) if &value.ty() == ty => {
                if arches & 1 != 0
                    && matches!(value,
                    Value::USize(v) if u32::try_from(*v).is_err())
                    || arches & 1 != 0
                        && matches!(value,
                    Value::ISize(v) if i32::try_from(*v).is_err())
                {
                    return invalid("native-sized value exceeds its architecture width");
                }
                Ok(())
            }
            NativeValue::Array(values) => {
                let Type::ArrayFixed(element, count) = ty else {
                    return invalid("native array value requires a fixed-array type");
                };
                if values.len() != *count {
                    return invalid("native array value length disagrees with its type");
                }
                for value in values {
                    self.check_native_value(
                        element,
                        value,
                        arches,
                        reference,
                        visiting,
                        depth + 1,
                    )?;
                }
                Ok(())
            }
            NativeValue::String { encoding, units } => {
                let element = match ty {
                    Type::ArrayFixed(element, count) if *count == units.len() => element.as_ref(),
                    Type::PtrConst(element, 1) | Type::PtrMut(element, 1) => element.as_ref(),
                    Type::String if *encoding == NativeStringEncoding::Utf16 => return Ok(()),
                    _ => {
                        return invalid(
                            "native string requires a matching array or character pointer",
                        );
                    }
                };
                self.check_native_character(
                    element,
                    *encoding,
                    arches,
                    reference,
                    visiting,
                    depth + 1,
                )
            }
            _ => invalid("native scalar value disagrees with its declared type"),
        }
    }

    fn check_native_character(
        &self,
        ty: &Type,
        encoding: NativeStringEncoding,
        arches: i32,
        reference: Option<&Self>,
        visiting: &mut Vec<TypeName>,
        depth: usize,
    ) -> Result<(), NativeValueError> {
        if depth > 64 {
            return invalid("native value type nesting exceeds 64");
        }
        match (ty.unqualified(), encoding) {
            (Type::U8 | Type::I8, NativeStringEncoding::Narrow | NativeStringEncoding::Utf8)
            | (Type::U16, NativeStringEncoding::Utf16)
            | (Type::U32, NativeStringEncoding::Utf32) => Ok(()),
            (Type::ValueName(name), _) => {
                if visiting.contains(name) {
                    return invalid("cyclic native character typedef");
                }
                let definitions: Vec<_> = self
                    .get(&name.namespace, &name.name)
                    .chain(
                        reference
                            .into_iter()
                            .flat_map(|r| r.get(&name.namespace, &name.name)),
                    )
                    .filter(|def| arches == 0 || def.arches() == 0 || def.arches() & arches != 0)
                    .collect();
                if definitions.is_empty() {
                    return invalid("native string requires an available character typedef");
                }
                visiting.push(name.clone());
                for def in definitions {
                    let fields: Vec<_> = def.fields().collect();
                    if fields.len() != 1
                        || def.category() != TypeCategory::Struct
                        || def.is_opaque()
                        || def.flags().contains(TypeAttributes::WindowsRuntime)
                        || def.generic_params().next().is_some()
                        || !def.attributes().any(|attr| {
                            attr.namespace() == "Windows.Win32.Metadata"
                                && attr.name() == "NativeTypedefAttribute"
                        })
                    {
                        return invalid("native string character type must be a scalar or typedef");
                    }
                    let target_arches = if arches == 0 {
                        def.arches()
                    } else if def.arches() == 0 {
                        arches
                    } else {
                        arches & def.arches()
                    };
                    self.check_native_character(
                        &fields[0].ty(),
                        encoding,
                        target_arches,
                        reference,
                        visiting,
                        depth + 1,
                    )?;
                }
                visiting.pop();
                Ok(())
            }
            _ => invalid("native string encoding disagrees with its declared character type"),
        }
    }
}

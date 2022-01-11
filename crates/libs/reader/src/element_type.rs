use super::*;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum ElementType {
    Void,
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
    ISize,
    USize,
    String,
    GUID,
    IUnknown,
    IInspectable,
    HRESULT,
    TypeName,
    GenericParam(String),
    Array((Box<Signature>, u32)),
    MethodDef(MethodDef),
    Field(Field),
    TypeDef(TypeDef),
}

impl Default for ElementType {
    fn default() -> Self {
        Self::Void
    }
}

impl From<TypeDef> for ElementType {
    fn from(def: TypeDef) -> Self {
        Self::TypeDef(def.with_generics())
    }
}

impl ElementType {
    pub fn size(&self) -> usize {
        match self {
            Self::I64 | Self::U64 | Self::F64 => 2,
            Self::GUID => 4,
            Self::TypeDef(def) => def.size(),
            _ => 1,
        }
    }

    pub fn row(&self) -> &Row {
        match self {
            Self::MethodDef(def) => &def.0,
            Self::Field(def) => &def.0,
            Self::TypeDef(def) => &def.row,
            _ => unimplemented!(),
        }
    }

    pub fn from_code(code: u32) -> Option<Self> {
        match code {
            0x01 => Some(Self::Void),
            0x02 => Some(Self::Bool),
            0x03 => Some(Self::Char),
            0x04 => Some(Self::I8),
            0x05 => Some(Self::U8),
            0x06 => Some(Self::I16),
            0x07 => Some(Self::U16),
            0x08 => Some(Self::I32),
            0x09 => Some(Self::U32),
            0x0a => Some(Self::I64),
            0x0b => Some(Self::U64),
            0x0c => Some(Self::F32),
            0x0d => Some(Self::F64),
            0x18 => Some(Self::ISize),
            0x19 => Some(Self::USize),
            0x0e => Some(Self::String),
            0x1c => Some(Self::IInspectable),
            _ => None,
        }
    }

    pub fn from_string_lossy(name: &str) -> Option<ElementType> {
        match name {
            "bool" => Some(Self::Bool),
            "i8" => Some(Self::I8),
            "u8" => Some(Self::U8),
            "i16" => Some(Self::I16),
            "u16" => Some(Self::U16),
            "i32" => Some(Self::I32),
            "u32" => Some(Self::U32),
            "i64" => Some(Self::I64),
            "u64" => Some(Self::U64),
            "f32" => Some(Self::F32),
            "f64" => Some(Self::F64),
            "isize" => Some(Self::ISize),
            "usize" => Some(Self::USize),
            "HSTRING" => Some(Self::String),
            "GUID" => Some(Self::GUID),
            "IInspectable" => Some(Self::IInspectable),
            _ => None,
        }
    }

    pub fn type_name(&self) -> TypeName {
        match self {
            Self::TypeDef(def) => def.type_name(),
            _ => unimplemented!(),
        }
    }

    pub fn type_signature(&self) -> String {
        match self {
            Self::Bool => "b1".to_owned(),
            Self::Char => "c2".to_owned(),
            Self::I8 => "i1".to_owned(),
            Self::U8 => "u1".to_owned(),
            Self::I16 => "i2".to_owned(),
            Self::U16 => "u2".to_owned(),
            Self::I32 => "i4".to_owned(),
            Self::U32 => "u4".to_owned(),
            Self::I64 => "i8".to_owned(),
            Self::U64 => "u8".to_owned(),
            Self::F32 => "f4".to_owned(),
            Self::F64 => "f8".to_owned(),
            Self::String => "string".to_owned(),
            Self::IInspectable => "cinterface(IInspectable)".to_owned(),
            Self::GUID => "g16".to_owned(),
            Self::HRESULT => "struct(Windows.Foundation.HResult;i4)".to_owned(),
            Self::TypeDef(t) => t.type_signature(),
            _ => unimplemented!(),
        }
    }

    pub fn include_dependencies(&self, include: TypeInclude) {
        match self {
            Self::MethodDef(t) => t.include_dependencies(),
            Self::TypeDef(t) => t.include_dependencies(include),
            Self::Field(t) => t.include_dependencies(None, include),
            Self::Array((signature, _)) => signature.kind.include_dependencies(include),
            _ => {}
        }
    }

    pub fn include_definition(&self, include: TypeInclude) {
        match self {
            Self::TypeDef(t) => t.include_definition(include),
            Self::Array((signature, _)) => signature.kind.include_definition(include),
            _ => {}
        }
    }

    pub fn features(&self, features: &mut BTreeSet<&'static str>, keys: &mut std::collections::HashSet<Row>) {
        match self {
            Self::TypeDef(def) => def.features(features, keys),
            Self::Array((signature, _)) => signature.kind.features(features, keys),
            _ => {}
        }
    }

    pub fn is_nullable(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.is_nullable(),
            Self::IInspectable | Self::IUnknown | Self::MethodDef(_) => true,
            _ => false,
        }
    }

    pub fn is_blittable(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.is_blittable(),
            Self::String | Self::IInspectable | Self::IUnknown | Self::GenericParam(_) => false,
            Self::Array((kind, _)) => kind.is_blittable(),
            _ => true,
        }
    }

    pub fn is_convertible(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.is_convertible(),
            Self::String | Self::IInspectable | Self::GUID | Self::IUnknown | Self::GenericParam(_) => true,
            _ => false,
        }
    }

    pub fn is_callback(&self) -> bool {
        match self {
            Self::TypeDef(def) => def.is_callback(),
            _ => false,
        }
    }

    pub fn is_callback_array(&self) -> bool {
        match self {
            Self::Array((kind, _)) => kind.kind.is_callback(),
            _ => false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.is_primitive(),
            Self::Bool | Self::Char | Self::I8 | Self::U8 | Self::I16 | Self::U16 | Self::I32 | Self::U32 | Self::I64 | Self::U64 | Self::F32 | Self::F64 | Self::ISize | Self::USize | Self::HRESULT => true,
            _ => false,
        }
    }

    pub fn is_udt(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.is_udt(),
            Self::GUID => true,
            _ => false,
        }
    }

    pub fn has_explicit(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.has_explicit(),
            Self::Array((kind, _)) => kind.has_explicit(),
            _ => false,
        }
    }

    pub fn is_handle(&self) -> bool {
        match self {
            Self::TypeDef(def) => def.is_handle(),
            _ => false,
        }
    }

    #[must_use]
    pub fn underlying_type(&self) -> ElementType {
        match self {
            Self::TypeDef(def) => def.underlying_type(),
            Self::HRESULT => ElementType::I32,
            _ => self.clone(),
        }
    }

    pub fn has_replacement(&self) -> bool {
        match self {
            Self::HRESULT => true,
            Self::TypeDef(def) => def.is_handle(),
            _ => false,
        }
    }
}

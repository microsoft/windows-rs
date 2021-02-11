use super::*;

// TODO: this replaces TypeKind, TypeName, and TypeDefinition
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum ElementType {
    NotYetSupported,
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
    Object,
    Guid,
    IUnknown,
    ErrorCode,
    Bool32,
    Matrix3x2,
    TypeName,
    TypeDef(GenericTypeDef),
    MethodDef(MethodDef),
    Field(Field),
    GenericParam(GenericParam),
}

impl ElementType {
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
            _ => None,
        }
    }

    pub fn from_blob(blob: &mut Blob, generics: &[ElementType]) -> Self {
        let code = blob.read_unsigned();

        if let Some(code) = Self::from_code(code) {
            return code;
        }

        match code {
            0x11 | 0x12 => {
                let code = TypeDefOrRef::decode(blob.reader, blob.read_unsigned(), blob.file_index);

                match code {
                    TypeDefOrRef::TypeDef(type_def) => {
                        Self::TypeDef(GenericTypeDef::from_type_def(type_def, Vec::new()))
                    }
                    TypeDefOrRef::TypeRef(type_ref) => match type_ref.full_name() {
                        ("System", "Guid") | ("Windows.Win32.Com", "Guid") => Self::Guid,
                        ("Windows.Win32.Com", "IUnknown") => Self::IUnknown,
                        ("Windows.Foundation", "HResult") => Self::ErrorCode,
                        ("Windows.Win32.Com", "HRESULT") => Self::ErrorCode,
                        ("Windows.Win32.SystemServices", "BOOL") => Self::Bool32,
                        ("Windows.Win32.SystemServices", "LARGE_INTEGER") => Self::I64,
                        ("Windows.Win32.SystemServices", "ULARGE_INTEGER") => Self::U64,
                        ("Windows.Win32.Direct2D", "D2D_MATRIX_3X2_F") => Self::Matrix3x2,
                        ("System", "Type") => Self::TypeName,
                        ("", _) => Self::NotYetSupported,
                        _ => Self::TypeDef(GenericTypeDef::from_type_def(
                            type_ref.resolve(),
                            Vec::new(),
                        )),
                    },
                    _ => panic!("ElementType"),
                }
            }
            0x13 => generics[blob.read_unsigned() as usize].clone(),
            0x14 => Self::NotYetSupported, // arrays
            0x15 => Self::TypeDef(GenericTypeDef::from_blob(blob, generics)),
            _ => panic!("ElementType"),
        }
    }

    pub fn gen_name(&self, gen: Gen) -> TokenStream {
        match self {
            Self::Void => quote! { ::std::ffi::c_void },
            Self::Bool => quote! { bool },
            Self::Char => quote! { u16 },
            Self::I8 => quote! { i8 },
            Self::U8 => quote! { u8 },
            Self::I16 => quote! { i16 },
            Self::U16 => quote! { u16 },
            Self::I32 => quote! { i32 },
            Self::U32 => quote! { u32 },
            Self::I64 => quote! { i64 },
            Self::U64 => quote! { u64 },
            Self::F32 => quote! { f32 },
            Self::F64 => quote! { f64 },
            Self::ISize => quote! { isize },
            Self::USize => quote! { usize },
            Self::String => {
                let windows = gen.windows();
                quote! { #windows HString }
            }
            Self::Object => {
                let windows = gen.windows();
                quote! { #windows Object }
            }
            Self::Guid => {
                let windows = gen.windows();
                quote! { #windows Guid }
            }
            Self::IUnknown => {
                let windows = gen.windows();
                quote! { #windows IUnknown }
            }
            Self::ErrorCode => {
                let windows = gen.windows();
                quote! { #windows ErrorCode }
            }
            Self::Bool32 => {
                let windows = gen.windows();
                quote! { #windows BOOL }
            }
            Self::Matrix3x2 => {
                let windows = gen.windows();
                quote! { #windows foundation::numerics::Matrix3x2 }
            }
            Self::NotYetSupported => {
                let windows = gen.windows();
                quote! { #windows NOT_YET_SUPPORTED_TYPE }
            }
            Self::TypeDef(def) => def.gen_name(gen),
            Self::GenericParam(generic) => generic.gen_name(),
            _ => panic!("ElementType"),
        }
    }

    pub fn signature(&self) -> String {
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
            Self::Object => "cinterface(IInspectable)".to_owned(),
            Self::Guid => "g16".to_owned(),
            Self::TypeDef(def) => def.signature(),
            _ => panic!("ElementType"),
        }
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        match self {
            Self::TypeDef(value) => value.dependencies(),
            Self::MethodDef(value) => value.dependencies(&[]),
            Self::Field(value) => value.dependencies(),
            _ => Vec::new(),
        }
    }

    pub fn gen(&self, gen: Gen) -> TokenStream {
        match self {
            Self::TypeDef(value) => value.gen(gen),
            Self::MethodDef(value) => value.gen(gen),
            Self::Field(value) => value.gen(gen),
            _ => panic!("ElementType"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool() {
        assert_eq!(ElementType::Bool.gen_name(Gen::Absolute).as_str(), "bool");
    }
}

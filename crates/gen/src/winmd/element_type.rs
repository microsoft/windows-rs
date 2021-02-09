use super::*;

// TODO: this needs to replace gen::TypeKind and gen::TypeName
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
    TypeDef(ElementTypeDef),
    GenericParam(&'static str),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ElementTypeDef {
    pub def: TypeDef,
    pub args: Vec<ElementType>,
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

    pub fn from_blob(blob: &mut Blob) -> Self {
        Self::from_blob_with_generics(blob, &[])
    }

    pub fn from_blob_with_generics(blob: &mut Blob, generics: &[ElementType]) -> Self {
        let code = blob.read_unsigned();

        if let Some(code) = Self::from_code(code) {
            return code;
        }

        match code {
            0x11 | 0x12 => {
                let code = TypeDefOrRef::decode(blob.reader, blob.read_unsigned(), blob.file_index);

                match code {
                    TypeDefOrRef::TypeDef(type_def) => Self::TypeDef(ElementTypeDef {
                        def: type_def,
                        args: Vec::new(),
                    }),
                    TypeDefOrRef::TypeRef(type_ref) => match type_ref.name() {
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
                        _ => Self::TypeDef(ElementTypeDef {
                            def: type_ref.resolve(),
                            args: Vec::new(),
                        }),
                    },
                    _ => panic!("Expected a TypeDef or TypeRef"),
                }
            }
            0x13 => generics[blob.read_unsigned() as usize].clone(),
            0x14 => Self::NotYetSupported, // arrays
            0x15 => {
                blob.read_unsigned();
                // TODO: add "read_type_def_or_ref" method to Blob reader.
                let def = TypeDefOrRef::decode(blob.reader, blob.read_unsigned(), blob.file_index)
                    .resolve();
                let mut args = Vec::with_capacity(blob.read_unsigned() as usize);

                for _ in 0..args.capacity() {
                    args.push(Self::from_blob_with_generics(blob, generics));
                }
                
                Self::TypeDef(ElementTypeDef { def, args })
            }
            _ => panic!(format!("Unexpected ElementType: {:x}", code)),
        }
    }

    // pub fn gen_name(&self, gen: &Gen) -> TokenStream {
    //     match self {

    //     }
    // }
}

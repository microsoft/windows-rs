use super::*;

// TODO: this needs to replace gen::TypeKind
#[derive(Debug)]
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
    Object,
    Guid,
    IUnknown,
    ErrorCode,
    Bool32,
    Matrix3x2,
    TypeName,
    TypeDef(TypeDef),
    Generic(&'static str),
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
        let code = blob.read_unsigned();

        if let Some(code) = Self::from_code(code) {
            return code;
        }

        if code == 0x11 || code == 0x12 {
            let code = TypeDefOrRef::decode(blob.reader, blob.read_unsigned(), blob.file_index);

            return match code {
                TypeDefOrRef::TypeDef(type_def) => Self::TypeDef(type_def),
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
                    _ => Self::TypeDef(type_ref.resolve()),
                },
                _ => panic!("Expected a TypeDef or TypeRef"),
            };
        }

        panic!(format!("Unexpected ElementType: {:x}", code));
    }
}

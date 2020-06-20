use crate::blob::Blob;
use crate::codes::{Decode, TypeDefOrRef};
use crate::signatures::TypeSig;

#[allow(dead_code)]
pub enum ElementType {
    Void,
    Boolean,
    Char,
    I1,
    U1,
    I2,
    U2,
    I4,
    U4,
    I8,
    U8,
    R4,
    R8,
    String,

    Ptr(TypeSig),
    ByRef(TypeSig),
    ValueType(TypeDefOrRef),
    Class(TypeDefOrRef),
    Var, // Generic parameter in a type definition, represented as unsigned integer
    Array,
    GenericInst,
    TypedByRef,

    I, // System.IntPtr
    U, // System.UIntPtr

    FnPtr,  // Full method sig
    Object, // System.Object
    SZArray,
    MVar, // Generic parameter in a method definition, represented as unsigned integer
    CModReqd(TypeDefOrRef), // Required modifier
    CModOpt(TypeDefOrRef), // Optional modifier
    Internal,

    Modifier, // Or'd with the following element types
    Sentinel, // Sentinel for vararg method signature,

    Pinned,

    Type,         // System.Type
    TaggedObject, // Boxed object (in custom attributes)
    Field,        // Custom attribute field
    Property,     // Custom attribute property
    Enum,         // Custom attribute enum
}

impl ElementType {
    pub fn from_blob(blob: &mut Blob) -> ElementType {
        let code = blob.read_unsigned();
        match code {
            0x01 => ElementType::Void,
            0x02 => ElementType::Boolean,
            0x03 => ElementType::Char,
            0x04 => ElementType::I1,
            0x05 => ElementType::U1,
            0x06 => ElementType::I2,
            0x07 => ElementType::U2,
            0x08 => ElementType::I4,
            0x09 => ElementType::U4,
            0x0a => ElementType::I8,
            0x0b => ElementType::U8,
            0x0c => ElementType::R4,
            0x0d => ElementType::R8,
            0x0e => ElementType::String,

            0x0f => ElementType::Ptr(TypeSig::from_blob(blob)),
            0x10 => ElementType::ByRef(TypeSig::from_blob(blob)),
            0x11 => {
                ElementType::ValueType(TypeDefOrRef::decode(blob.read_unsigned(), blob.file_index))
            }
            0x12 => ElementType::Class(TypeDefOrRef::decode(blob.read_unsigned(), blob.file_index)),
            0x13 => ElementType::Var,
            0x14 => ElementType::Array,
            0x15 => ElementType::GenericInst,
            0x16 => ElementType::TypedByRef,

            0x18 => ElementType::I,
            0x19 => ElementType::U,

            0x1b => ElementType::FnPtr, // TODO: Followed by full function signature
            0x1c => ElementType::Object,
            0x1d => ElementType::SZArray,
            0x1e => ElementType::MVar,
            0x1f => {
                ElementType::CModReqd(TypeDefOrRef::decode(blob.read_unsigned(), blob.file_index))
            }
            0x20 => {
                ElementType::CModOpt(TypeDefOrRef::decode(blob.read_unsigned(), blob.file_index))
            }
            0x21 => ElementType::Internal,

            0x40 => ElementType::Modifier,
            0x41 => ElementType::Sentinel,

            0x45 => ElementType::Pinned,

            0x50 => ElementType::Type,
            0x51 => ElementType::TaggedObject,
            0x53 => ElementType::Field,
            0x54 => ElementType::Property,
            0x55 => ElementType::Enum,

            unknown_type => panic!(format!("Unexpected ElementType: {:x}", unknown_type)),
        }
    }
}

impl std::fmt::Debug for ElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementType::Void => write!(f, "ElementType::Void"),
            ElementType::Boolean => write!(f, "ElementType::Boolean"),
            ElementType::Char => write!(f, "ElementType::Char"),
            ElementType::I1 => write!(f, "ElementType::I1"),
            ElementType::U1 => write!(f, "ElementType::U1"),
            ElementType::I2 => write!(f, "ElementType::I2"),
            ElementType::U2 => write!(f, "ElementType::U2"),
            ElementType::I4 => write!(f, "ElementType::I4"),
            ElementType::U4 => write!(f, "ElementType::U4"),
            ElementType::I8 => write!(f, "ElementType::I8"),
            ElementType::U8 => write!(f, "ElementType::U8"),
            ElementType::R4 => write!(f, "ElementType::R4"),
            ElementType::R8 => write!(f, "ElementType::R8"),
            ElementType::String => write!(f, "ElementType::String"),

            ElementType::Ptr(_) => write!(f, "ElementType::Ptr"),
            ElementType::ByRef(_) => write!(f, "ElementType::ByRef"),
            ElementType::ValueType(_) => write!(f, "ElementType::ValueType"),
            ElementType::Class(_) => write!(f, "ElementType::Class"),
            ElementType::Var => write!(f, "ElementType::Var"),
            ElementType::Array => write!(f, "ElementType::Array"),
            ElementType::GenericInst => write!(f, "ElementType::GenericInst"),
            ElementType::TypedByRef => write!(f, "ElementType::TypedByRef"),

            ElementType::I => write!(f, "ElementType::I"),
            ElementType::U => write!(f, "ElementType::U"),

            ElementType::FnPtr => write!(f, "ElementType::FnPtr"),
            ElementType::Object => write!(f, "ElementType::Object"),
            ElementType::SZArray => write!(f, "ElementType::SZArray"),
            ElementType::MVar => write!(f, "ElementType::MVar"),
            ElementType::CModReqd(_) => write!(f, "ElementType::CModReqd"),
            ElementType::CModOpt(_) => write!(f, "ElementType::CModOpt"),
            ElementType::Internal => write!(f, "ElementType::Internal"),

            ElementType::Modifier => write!(f, "ElementType::Modifier"),
            ElementType::Sentinel => write!(f, "ElementType::Sentinel"),

            ElementType::Pinned => write!(f, "ElementType::Pinned"),

            ElementType::Type => write!(f, "ElementType::Type"),
            ElementType::TaggedObject => write!(f, "ElementType::TaggedObject"),
            ElementType::Field => write!(f, "ElementType::Field"),
            ElementType::Property => write!(f, "ElementType::Property"),
            ElementType::Enum => write!(f, "ElementType::Enum"),
        }
    }
}

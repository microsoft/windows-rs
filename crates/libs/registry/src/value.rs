use super::*;

/// A registry value.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Value {
    /// A 32-bit unsigned integer value.
    U32(u32),

    /// A 64-bit unsigned integer value.
    U64(u64),

    /// A string value.
    String(String),

    /// An array u8 bytes.
    Bytes(Vec<u8>),

    /// An array of string values.
    MultiString(Vec<String>),

    /// An unknown or unsupported type.
    Unknown(u32),
}

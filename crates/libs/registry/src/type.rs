/// The possible types that a registry value could have.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
    /// A 32-bit unsigned integer value.
    U32,

    /// A 64-bit unsigned integer value.
    U64,

    /// A string value.
    String,

    /// An array u8 bytes.
    Bytes,

    /// An array of string values.
    MultiString,

    /// An unknown or unsupported type.
    Unknown(u32),
}

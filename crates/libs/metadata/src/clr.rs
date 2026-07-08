//! CLR (ECMA-335) signature element type codes, from `corhdr.h`.
//!
//! These are hand-authored rather than generated: `corhdr.h` ships with the .NET
//! SDK, not the Windows SDK, so the in-house Win32 scrape cannot reach it. The
//! values are a frozen part of the CLR metadata ABI.

pub type CorElementType = u8;
pub const ELEMENT_TYPE_ARRAY: CorElementType = 20;
pub const ELEMENT_TYPE_BOOLEAN: CorElementType = 2;
pub const ELEMENT_TYPE_BYREF: CorElementType = 16;
pub const ELEMENT_TYPE_CHAR: CorElementType = 3;
pub const ELEMENT_TYPE_CLASS: CorElementType = 18;
pub const ELEMENT_TYPE_CMOD_OPT: CorElementType = 32;
pub const ELEMENT_TYPE_CMOD_REQD: CorElementType = 31;
pub const ELEMENT_TYPE_GENERICINST: CorElementType = 21;
pub const ELEMENT_TYPE_I: CorElementType = 24;
pub const ELEMENT_TYPE_I1: CorElementType = 4;
pub const ELEMENT_TYPE_I2: CorElementType = 6;
pub const ELEMENT_TYPE_I4: CorElementType = 8;
pub const ELEMENT_TYPE_I8: CorElementType = 10;
pub const ELEMENT_TYPE_OBJECT: CorElementType = 28;
pub const ELEMENT_TYPE_PTR: CorElementType = 15;
pub const ELEMENT_TYPE_R4: CorElementType = 12;
pub const ELEMENT_TYPE_R8: CorElementType = 13;
pub const ELEMENT_TYPE_STRING: CorElementType = 14;
pub const ELEMENT_TYPE_SZARRAY: CorElementType = 29;
pub const ELEMENT_TYPE_U: CorElementType = 25;
pub const ELEMENT_TYPE_U1: CorElementType = 5;
pub const ELEMENT_TYPE_U2: CorElementType = 7;
pub const ELEMENT_TYPE_U4: CorElementType = 9;
pub const ELEMENT_TYPE_U8: CorElementType = 11;
pub const ELEMENT_TYPE_VALUETYPE: CorElementType = 17;
pub const ELEMENT_TYPE_VAR: CorElementType = 19;
pub const ELEMENT_TYPE_VOID: CorElementType = 1;

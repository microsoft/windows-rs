#![doc = include_str!("../readme.md")]
#![expect(non_snake_case, non_upper_case_globals)]

use std::cmp::Ordering;
use std::collections::*;

mod attributes;
pub mod reader;
mod value;
pub mod writer;

pub use attributes::*;
pub use value::*;
mod bindings;
use bindings::*;
pub use bindings::CorElementType;
pub use bindings::{
    ELEMENT_TYPE_ARRAY, ELEMENT_TYPE_BOOLEAN, ELEMENT_TYPE_BYREF, ELEMENT_TYPE_CHAR,
    ELEMENT_TYPE_CLASS, ELEMENT_TYPE_CMOD_OPT, ELEMENT_TYPE_CMOD_REQD,
    ELEMENT_TYPE_GENERICINST, ELEMENT_TYPE_I, ELEMENT_TYPE_I1, ELEMENT_TYPE_I2,
    ELEMENT_TYPE_I4, ELEMENT_TYPE_I8, ELEMENT_TYPE_OBJECT, ELEMENT_TYPE_PTR, ELEMENT_TYPE_R4,
    ELEMENT_TYPE_R8, ELEMENT_TYPE_STRING, ELEMENT_TYPE_SZARRAY, ELEMENT_TYPE_U,
    ELEMENT_TYPE_U1, ELEMENT_TYPE_U2, ELEMENT_TYPE_U4, ELEMENT_TYPE_U8,
    ELEMENT_TYPE_VALUETYPE, ELEMENT_TYPE_VAR, ELEMENT_TYPE_VOID,
};

mod ty;
pub use ty::*;

mod type_name;
pub use type_name::*;

mod signature;
pub use signature::*;

pub use reader::{AsRow, HasAttributes};

pub fn trim_tick(name: &str) -> &str {
    if name.as_bytes().iter().rev().nth(1) == Some(&b'`') {
        &name[..name.len() - 2]
    } else {
        name
    }
}

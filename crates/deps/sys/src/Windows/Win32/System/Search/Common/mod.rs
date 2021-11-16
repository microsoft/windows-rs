#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const COP_IMPLICIT: i32 = 0i32;
pub const COP_EQUAL: i32 = 1i32;
pub const COP_NOTEQUAL: i32 = 2i32;
pub const COP_LESSTHAN: i32 = 3i32;
pub const COP_GREATERTHAN: i32 = 4i32;
pub const COP_LESSTHANOREQUAL: i32 = 5i32;
pub const COP_GREATERTHANOREQUAL: i32 = 6i32;
pub const COP_VALUE_STARTSWITH: i32 = 7i32;
pub const COP_VALUE_ENDSWITH: i32 = 8i32;
pub const COP_VALUE_CONTAINS: i32 = 9i32;
pub const COP_VALUE_NOTCONTAINS: i32 = 10i32;
pub const COP_DOSWILDCARDS: i32 = 11i32;
pub const COP_WORD_EQUAL: i32 = 12i32;
pub const COP_WORD_STARTSWITH: i32 = 13i32;
pub const COP_APPLICATION_SPECIFIC: i32 = 14i32;
pub const CT_AND_CONDITION: i32 = 0i32;
pub const CT_OR_CONDITION: i32 = 1i32;
pub const CT_NOT_CONDITION: i32 = 2i32;
pub const CT_LEAF_CONDITION: i32 = 3i32;

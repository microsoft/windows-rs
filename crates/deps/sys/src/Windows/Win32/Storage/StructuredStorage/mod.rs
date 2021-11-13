#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct JET_API_PTR(pub usize);
impl ::core::marker::Copy for JET_API_PTR {}
impl ::core::clone::Clone for JET_API_PTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct JET_HANDLE(pub usize);
impl ::core::marker::Copy for JET_HANDLE {}
impl ::core::clone::Clone for JET_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct JET_INSTANCE(pub usize);
impl ::core::marker::Copy for JET_INSTANCE {}
impl ::core::clone::Clone for JET_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct JET_SESID(pub usize);
impl ::core::marker::Copy for JET_SESID {}
impl ::core::clone::Clone for JET_SESID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct JET_TABLEID(pub usize);
impl ::core::marker::Copy for JET_TABLEID {}
impl ::core::clone::Clone for JET_TABLEID {
    fn clone(&self) -> Self {
        *self
    }
}

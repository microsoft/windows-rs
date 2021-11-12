#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct JET_API_PTR {
    pub Value: usize,
}
impl ::core::marker::Copy for JET_API_PTR {}
impl ::core::clone::Clone for JET_API_PTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct JET_HANDLE {
    pub Value: usize,
}
impl ::core::marker::Copy for JET_HANDLE {}
impl ::core::clone::Clone for JET_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct JET_INSTANCE {
    pub Value: usize,
}
impl ::core::marker::Copy for JET_INSTANCE {}
impl ::core::clone::Clone for JET_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct JET_SESID {
    pub Value: usize,
}
impl ::core::marker::Copy for JET_SESID {}
impl ::core::clone::Clone for JET_SESID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct JET_TABLEID {
    pub Value: usize,
}
impl ::core::marker::Copy for JET_TABLEID {}
impl ::core::clone::Clone for JET_TABLEID {
    fn clone(&self) -> Self {
        *self
    }
}

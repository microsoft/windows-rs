#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct JET_API_PTR(i32);
#[repr(C)]
pub struct JET_HANDLE(i32);
#[repr(C)]
pub struct JET_INSTANCE(i32);
#[repr(C)]
pub struct JET_SESID(i32);
#[repr(C)]
pub struct JET_TABLEID(i32);

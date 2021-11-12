#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct JET_API_PTR(i32);
pub struct JET_HANDLE(i32);
pub struct JET_INSTANCE(i32);
pub struct JET_SESID(i32);
pub struct JET_TABLEID(i32);

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IQuickLink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IQuickLink {}
impl ::core::clone::Clone for IQuickLink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareOperation {}
impl ::core::clone::Clone for IShareOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareOperation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareOperation2 {}
impl ::core::clone::Clone for IShareOperation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareOperation3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareOperation3 {}
impl ::core::clone::Clone for IShareOperation3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct QuickLink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for QuickLink {}
impl ::core::clone::Clone for QuickLink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShareOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShareOperation {}
impl ::core::clone::Clone for ShareOperation {
    fn clone(&self) -> Self {
        *self
    }
}

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPreviewBuildsManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPreviewBuildsManager {}
impl ::core::clone::Clone for IPreviewBuildsManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPreviewBuildsManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPreviewBuildsManagerStatics {}
impl ::core::clone::Clone for IPreviewBuildsManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPreviewBuildsState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPreviewBuildsState {}
impl ::core::clone::Clone for IPreviewBuildsState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PreviewBuildsManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PreviewBuildsManager {}
impl ::core::clone::Clone for PreviewBuildsManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PreviewBuildsState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PreviewBuildsState {}
impl ::core::clone::Clone for PreviewBuildsState {
    fn clone(&self) -> Self {
        *self
    }
}

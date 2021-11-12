#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILanguageFont(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguageFontGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguageFontGroupFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LanguageFont(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LanguageFontGroup(pub *mut ::core::ffi::c_void);

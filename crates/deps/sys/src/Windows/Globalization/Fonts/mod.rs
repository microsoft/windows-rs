#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ILanguageFont(pub *mut ::core::ffi::c_void);
pub struct ILanguageFontGroup(pub *mut ::core::ffi::c_void);
pub struct ILanguageFontGroupFactory(pub *mut ::core::ffi::c_void);
pub struct LanguageFont(i32);
pub struct LanguageFontGroup(i32);

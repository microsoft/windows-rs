#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILanguageFont(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILanguageFont {}
impl ::core::clone::Clone for ILanguageFont {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILanguageFontGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILanguageFontGroup {}
impl ::core::clone::Clone for ILanguageFontGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILanguageFontGroupFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILanguageFontGroupFactory {}
impl ::core::clone::Clone for ILanguageFontGroupFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LanguageFont(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LanguageFont {}
impl ::core::clone::Clone for LanguageFont {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LanguageFontGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LanguageFontGroup {}
impl ::core::clone::Clone for LanguageFontGroup {
    fn clone(&self) -> Self {
        *self
    }
}

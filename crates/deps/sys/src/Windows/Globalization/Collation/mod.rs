#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CharacterGrouping(i32);
pub struct CharacterGroupings(i32);
pub struct ICharacterGrouping(pub *mut ::core::ffi::c_void);
pub struct ICharacterGroupings(pub *mut ::core::ffi::c_void);
pub struct ICharacterGroupingsFactory(pub *mut ::core::ffi::c_void);

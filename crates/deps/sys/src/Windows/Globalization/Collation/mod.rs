#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CharacterGrouping(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CharacterGroupings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICharacterGrouping(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICharacterGroupings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICharacterGroupingsFactory(pub *mut ::core::ffi::c_void);

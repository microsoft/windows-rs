#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CharacterGrouping(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CharacterGrouping {}
impl ::core::clone::Clone for CharacterGrouping {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CharacterGroupings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CharacterGroupings {}
impl ::core::clone::Clone for CharacterGroupings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICharacterGrouping(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICharacterGrouping {}
impl ::core::clone::Clone for ICharacterGrouping {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICharacterGroupings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICharacterGroupings {}
impl ::core::clone::Clone for ICharacterGroupings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICharacterGroupingsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICharacterGroupingsFactory {}
impl ::core::clone::Clone for ICharacterGroupingsFactory {
    fn clone(&self) -> Self {
        *self
    }
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct ContentAccessRestrictionLevel(i32);
#[repr(transparent)]
pub struct ContentRestrictionsBrowsePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentRestrictionsBrowsePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatedContentDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatedContentDescriptionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatedContentRestrictions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatedContentRestrictionsFactory(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct RatedContentCategory(i32);
#[repr(transparent)]
pub struct RatedContentDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RatedContentRestrictions(pub *mut ::core::ffi::c_void);

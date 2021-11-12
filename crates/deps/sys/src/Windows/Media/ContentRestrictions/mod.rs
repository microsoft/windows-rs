#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ContentAccessRestrictionLevel(i32);
pub struct ContentRestrictionsBrowsePolicy(i32);
pub struct IContentRestrictionsBrowsePolicy(pub *mut ::core::ffi::c_void);
pub struct IRatedContentDescription(pub *mut ::core::ffi::c_void);
pub struct IRatedContentDescriptionFactory(pub *mut ::core::ffi::c_void);
pub struct IRatedContentRestrictions(pub *mut ::core::ffi::c_void);
pub struct IRatedContentRestrictionsFactory(pub *mut ::core::ffi::c_void);
pub struct RatedContentCategory(i32);
pub struct RatedContentDescription(i32);
pub struct RatedContentRestrictions(i32);
